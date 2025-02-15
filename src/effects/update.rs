use {
  crate::{context::Context, effects::ui::Ui},
  log::error,
  reqwest::{header::ACCEPT, Client, StatusCode},
  serde::{Deserialize, Serialize},
  std::{collections::BTreeMap, rc::Rc},
};

#[derive(Serialize, Deserialize, Debug)]
struct PackageMeta {
  name: Rc<str>,
  #[serde(rename = "dist-tags")]
  dist_tags: BTreeMap<Rc<str>, Rc<str>>,
  time: BTreeMap<Rc<str>, Rc<str>>,
}

/// Run the update command side effects
pub async fn run(ctx: Context) -> Context {
  let ui = Ui { ctx: &ctx };
  let client = Client::new();

  get_package_meta(&client, "lodash").await.inspect(|x| println!("{x:#?}"));

  ctx
}

async fn get_package_meta(client: &Client, name: &str) -> Option<PackageMeta> {
  let url = format!("https://registry.npmjs.org/{}", name);
  let req = client.get(&url).header(ACCEPT, "application/json");

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
