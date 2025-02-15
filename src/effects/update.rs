use {
  crate::{context::Context, effects::ui::Ui},
  log::{debug, error},
  reqwest::{header::ACCEPT, Client, StatusCode},
  serde::{Deserialize, Serialize},
  std::{collections::BTreeMap, sync::Arc},
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

/// Run the update command side effects
pub async fn run(ctx: Context) -> Context {
  let ui = Ui { ctx: &ctx };
  let client = Arc::new(Client::new());
  let max_concurrent_requests = 2;
  let semaphore = Arc::new(Semaphore::new(max_concurrent_requests));

  let package_names = vec![
    "lodash",
    "react",
    "react-dom",
    "react-router",
    "react-router-dom",
    "react-scripts",
    "typescript",
    "webpack",
    "webpack-cli",
    "webpack-dev-server",
    "webpack-merge",
    "webpack-node-externals",
    "webpackbar",
    "workbox-webpack-plugin",
    "write-file-webpack-plugin",
    "yargs",
    "yargs-parser",
    "yup",
    "zone.js",
  ];

  let mut handles: Vec<JoinHandle<Option<PackageMeta>>> = vec![];

  for name in package_names {
    let permit = Arc::clone(&semaphore).acquire_owned().await;
    let client = Arc::clone(&client);
    handles.push(spawn(async move {
      let _permit = permit;
      get_package_meta(&client, name).await
    }));
  }

  for handle in handles {
    if let Some(package_meta) = handle.await.unwrap() {
      println!("DONE: {}", package_meta.name);
    }
  }

  ctx
}

async fn get_package_meta(client: &Client, name: &str) -> Option<PackageMeta> {
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
