use {
  crate::{context::Context, effects::ui::Ui},
  reqwest::{header::ACCEPT, Client},
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
  let url = format!("https://registry.npmjs.org/{}", "lodash");
  let req = client.get(url).header(ACCEPT, "application/json");

  match req.send().await {
    Ok(res) => {
      match res.status() {
        reqwest::StatusCode::OK => match res.json::<PackageMeta>().await {
          Ok(json) => {
            println!("Response JSON: {:#?}", json);
          }
          Err(err) => {
            println!("Failed to parse JSON: {}", err);
          }
        },
        status => {
          println!("Request failed with status: {}", status);
        }
      };
    }
    Err(err) => {
      println!("Request failed with error: {}", err);
    }
  };

  ctx
}
