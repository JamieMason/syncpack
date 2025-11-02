use {
  crate::{
    config::Config,
    dependency::UpdateUrl,
    instance::Instance,
    packages::Packages,
    registry_client::{AllPackageVersions, RegistryClient, RegistryError},
    specifier::Specifier,
    version_group::VersionGroup,
  },
  indicatif::{MultiProgress, ProgressBar, ProgressStyle},
  log::debug,
  std::{
    collections::{HashMap, HashSet},
    rc::Rc,
    sync::Arc,
    time::Duration,
  },
  tokio::{
    sync::Semaphore,
    task::{spawn, JoinHandle},
  },
};

/// The central data structure that owns all project data.
///
/// Context is created once and flows through the 3-phase pipeline using Rust's
/// ownership system. Each phase takes ownership and returns it:
/// 1. Context::create() - Read files, collect dependencies
/// 2. visit_packages() - Assign InstanceState to each instance
/// 3. command::run() - Process instances, consume Context
///
/// See PATTERNS.md "Ownership and Borrowing" section for details.
#[derive(Debug)]
pub struct Context {
  /// All default configuration with user config applied
  pub config: Config,
  /// The internal names of all failed updates
  pub failed_updates: Vec<String>,
  /// Every instance in the project.
  /// Rc<Instance> is used for single-threaded reference counting - instances
  /// are shared across version groups without expensive cloning.
  /// See .cursorrules for when to use Rc vs Arc.
  pub instances: Vec<Rc<Instance>>,
  /// Every package.json in the project
  pub packages: Packages,
  /// Registry client for fetching package metadata.
  /// Arc<dyn RegistryClient> is used because the client is shared across
  /// async tasks (crosses thread boundaries).
  pub registry_client: Option<Arc<dyn RegistryClient>>,
  /// All updates from the npm registry which have been chosen either by the
  /// user via a prompt or automatically by choosing the latest version
  pub updates_by_internal_name: HashMap<String, Vec<Rc<Specifier>>>,
  /// All version groups, their dependencies, and their instances
  pub version_groups: Vec<VersionGroup>,
}

impl Context {
  /// Phase 1 of the 3-phase pipeline: Create Context (read-only).
  ///
  /// This function reads all configuration and package.json files, collects
  /// all dependency instances, and assigns them to version groups.
  ///
  /// Important: InstanceState is NOT assigned here - all instances start as
  /// Unknown. States are assigned later in visit_packages() (Phase 2).
  ///
  /// Called from: src/main.rs
  /// Next step: visit_packages() in src/visit_packages.rs
  /// See also: .cursorrules for critical invariants
  pub fn create(config: Config, packages: Packages, registry_client: Option<Arc<dyn RegistryClient>>) -> Self {
    let mut instances = vec![];
    let updates_by_internal_name = HashMap::new();
    let all_dependency_types = config.rcfile.get_all_dependency_types();
    let cli_filters = config.cli.get_filters(&packages, &all_dependency_types);
    let dependency_groups = config.rcfile.get_dependency_groups(&packages, &all_dependency_types);
    let semver_groups = config.rcfile.get_semver_groups(&packages, &all_dependency_types);
    let mut version_groups = config.rcfile.get_version_groups(&packages, &all_dependency_types);

    let failed_updates = Vec::new();

    packages.get_all_instances(&all_dependency_types, |mut descriptor| {
      let dependency_group = dependency_groups.iter().find(|alias| alias.can_add(&descriptor));

      if let Some(group) = dependency_group {
        descriptor.internal_name = group.label.clone();
      }

      descriptor.matches_cli_filter = match cli_filters.as_ref() {
        Some(filters) => filters.can_add(&descriptor),
        None => true,
      };

      let preferred_semver_range = semver_groups
        .iter()
        .find(|group| group.selector.can_add(&descriptor))
        .and_then(|group| group.range.clone());

      let version_group = version_groups.iter_mut().find(|group| group.selector.can_add(&descriptor));

      let instance = Rc::new(Instance::new(descriptor, preferred_semver_range));

      instances.push(Rc::clone(&instance));

      if let Some(group) = version_group {
        group.add_instance(instance);
      }
    });

    Self {
      config,
      failed_updates,
      instances,
      packages,
      registry_client,
      updates_by_internal_name,
      version_groups,
    }
  }

  pub fn get_version_groups(&self) -> impl Iterator<Item = &VersionGroup> {
    self.version_groups.iter().filter(|group| group.matches_cli_filter)
  }

  /// Fetch every version specifier ever published for all updateable
  /// dependencies in the project.
  pub async fn fetch_all_updates(&mut self) {
    let client = Arc::clone(self.registry_client.as_ref().expect("Registry client not initialized"));
    let semaphore = Arc::new(Semaphore::new(self.config.rcfile.max_concurrent_requests));
    let progress_bars = Arc::new(MultiProgress::new());
    let mut handles: Vec<(String, JoinHandle<Result<AllPackageVersions, RegistryError>>)> = vec![];
    let mut all_updates_by_internal_name: HashMap<String, Vec<Rc<Specifier>>> = HashMap::new();
    let mut failed_updates: Vec<String> = vec![];

    for update_url in self.get_unique_update_urls() {
      let permit = Arc::clone(&semaphore).acquire_owned().await;
      let client = Arc::clone(&client);
      let progress_bars = Arc::clone(&progress_bars);

      handles.push((
        update_url.internal_name.clone(),
        spawn(async move {
          let _permit = permit;
          let progress_bar = progress_bars.add(ProgressBar::new_spinner());
          progress_bar.enable_steady_tick(Duration::from_millis(100));
          progress_bar.set_style(ProgressStyle::default_spinner());
          progress_bar.set_message(update_url.internal_name.clone());
          let package_meta = client.fetch(&update_url).await;
          progress_bar.finish_and_clear();
          progress_bars.remove(&progress_bar);
          package_meta
        }),
      ));
    }

    for (internal_name, handle) in handles {
      match handle.await {
        Ok(result) => match result {
          Ok(package_meta) => {
            let all_updates = all_updates_by_internal_name.entry(internal_name.clone()).or_default();
            for version in package_meta.versions.iter() {
              if !version.contains("created") && !version.contains("modified") {
                all_updates.push(Specifier::new(version));
              }
            }
          }
          Err(err) => {
            debug!("{err}");
            failed_updates.push(internal_name);
          }
        },
        Err(err) => {
          debug!("{err}");
          failed_updates.push(internal_name);
        }
      }
    }

    self.updates_by_internal_name = all_updates_by_internal_name;
    self.failed_updates = failed_updates;
  }

  /// Return a list of every dependency we should query the registry for
  /// updates. We use internal names in order to support dependency groups,
  /// where many dependencies can be aliased as one.
  fn get_unique_update_urls(&self) -> HashSet<UpdateUrl> {
    self
      .version_groups
      .iter()
      .filter(|group| group.matches_cli_filter)
      .fold(HashSet::new(), |mut unique_update_urls, group| {
        group.get_update_urls().inspect(|update_urls| {
          update_urls.iter().for_each(|url| {
            unique_update_urls.insert(url.clone());
          });
        });
        unique_update_urls
      })
  }
}
