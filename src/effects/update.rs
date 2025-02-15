use {
  crate::{context::Context, effects::ui::Ui, specifier::Specifier},
  colored::Colorize,
  indicatif::{MultiProgress, ProgressBar, ProgressStyle},
  log::{debug, error},
  reqwest::{header::ACCEPT, Client, StatusCode},
  serde::{Deserialize, Serialize},
  std::{collections::BTreeMap, sync::Arc, time::Duration},
  tokio::{
    sync::Semaphore,
    task::{spawn, JoinHandle},
  },
};

#[derive(Serialize, Deserialize, Debug)]
struct PackageMeta {
  name: Arc<str>,
  #[serde(rename = "dist-tags")]
  dist_tags: BTreeMap<Arc<str>, Arc<str>>,
  time: BTreeMap<Arc<str>, Arc<str>>,
}

pub enum UpdateGreediness {
  /// "*.*.*"
  Any,
  /// "1.*.*"
  Minor,
  /// "1.2.*"
  Patch,
}

/// Run the update command side effects
pub async fn run(ctx: Context) -> Context {
  let ctx = fetch_all_updates(ctx).await;

  // @TODO: move to cli_config and/or update_groups config
  let greediness = UpdateGreediness::Minor;

  fn is_any_update(current: &Specifier, newer: &Specifier) -> bool {
    if let (Specifier::BasicSemver(current_semver), Specifier::BasicSemver(newer_semver)) = (current, newer) {
      newer > current
    } else {
      false
    }
  }

  fn is_minor_update(current: &Specifier, newer: &Specifier) -> bool {
    if let (Specifier::BasicSemver(current_semver), Specifier::BasicSemver(newer_semver)) = (current, newer) {
      newer_semver.node_version.major == current_semver.node_version.major && newer > current
    } else {
      false
    }
  }

  fn is_patch_update(current: &Specifier, newer: &Specifier) -> bool {
    if let (Specifier::BasicSemver(current_semver), Specifier::BasicSemver(newer_semver)) = (current, newer) {
      newer_semver.node_version.major == current_semver.node_version.major
        && newer_semver.node_version.minor == current_semver.node_version.minor
        && newer > current
    } else {
      false
    }
  }

  ctx.instances.iter().for_each(|instance| {
    if instance.descriptor.matches_cli_filter {
      let name = &instance.descriptor.name;
      let current = &instance.descriptor.specifier;

      if let Specifier::BasicSemver(current_semver) = current {
        let newer_versions: Option<Vec<&Specifier>> = ctx.update_versions.get(name).map(|all_versions| {
          all_versions
            .iter()
            .filter(|version| match greediness {
              UpdateGreediness::Any => is_any_update(current, version),
              UpdateGreediness::Minor => is_minor_update(current, version),
              UpdateGreediness::Patch => is_patch_update(current, version),
            })
            .collect()
        });

        if let Some(newer_versions) = newer_versions {
          if let Some(latest) = newer_versions.last() {
            if current >= latest {
              let current = current.get_raw().green();
              println!("{name} {current}");
            } else {
              let current = current.get_raw().red();
              let arrow = "→".dimmed();
              let latest = latest.get_raw().green();
              println!("{name} {current} {arrow} {latest}");
            }
          }
        } else {
          debug!("{name} No updates found");
        }
      }
    }
  });

  ctx
}

/// Fetch latest versions of all packages
async fn fetch_all_updates(mut ctx: Context) -> Context {
  let ui = Ui { ctx: &ctx };
  let client = Arc::new(Client::new());
  let semaphore = Arc::new(Semaphore::new(ctx.config.rcfile.max_concurrent_requests));
  let progress_bars = Arc::new(MultiProgress::new());
  let mut handles_by_instance_name: BTreeMap<String, JoinHandle<Option<PackageMeta>>> = BTreeMap::new();

  for instance in ctx.instances.iter() {
    if !instance.descriptor.matches_cli_filter {
      continue;
    }
    let name = instance.descriptor.name.clone();
    if !handles_by_instance_name.contains_key(&name) {
      let permit = Arc::clone(&semaphore).acquire_owned().await;
      let client = Arc::clone(&client);
      let progress_bars = Arc::clone(&progress_bars);

      handles_by_instance_name.insert(
        name.clone(),
        spawn(async move {
          let _permit = permit;
          let progress_bar = progress_bars.add(ProgressBar::new_spinner());
          progress_bar.enable_steady_tick(Duration::from_millis(100));
          progress_bar.set_style(ProgressStyle::default_spinner());
          progress_bar.set_message(name.clone());
          let package_meta = fetch_updates(&client, &name).await;
          progress_bar.finish_and_clear();
          progress_bars.remove(&progress_bar);
          package_meta
        }),
      );
    }
  }

  for (name, handle) in handles_by_instance_name {
    let update_versions = ctx.update_versions.entry(name.clone()).or_default();
    if let Some(package_meta) = handle.await.unwrap() {
      for (version, _timestamp) in package_meta.time.iter() {
        if !version.contains("created") && !version.contains("modified") {
          update_versions.push(Specifier::new(version, None));
        }
      }
    }
  }

  progress_bars.clear().unwrap();

  ctx
}

/// Fetch latest version of a given package
async fn fetch_updates(client: &Client, name: &str) -> Option<PackageMeta> {
  let url = format!("https://registry.npmjs.org/{}", name);
  let req = client.get(&url).header(ACCEPT, "application/json");
  debug!("GET {url}");
  match req.send().await {
    Ok(res) => match res.status() {
      StatusCode::OK => match res.json::<PackageMeta>().await {
        Ok(package_meta) => Some(package_meta),
        Err(err) => {
          error!("{err}: {url}");
          None
        }
      },
      status => {
        error!("{status}: {url}");
        None
      }
    },
    Err(err) => {
      error!("{err}: {url}");
      None
    }
  }
}
