use {
  crate::{context::Context, effects::ui::Ui},
  reqwest::{header::ACCEPT, Client},
  serde::{Deserialize, Serialize},
  std::{collections::BTreeMap, error::Error, fmt},
};

// Custom key type to reverse the order
#[derive(Eq, PartialEq)]
struct Descending<T>(T);

impl<T: Ord> Ord for Descending<T> {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    other.0.cmp(&self.0)
  }
}

impl<T: Ord> PartialOrd for Descending<T> {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl<T: fmt::Debug> fmt::Debug for Descending<T> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    self.0.fmt(f)
  }
}

impl<T: Serialize> Serialize for Descending<T> {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    self.0.serialize(serializer)
  }
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for Descending<T> {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>,
  {
    T::deserialize(deserializer).map(Descending)
  }
}

#[derive(Serialize, Deserialize, Debug)]
struct Root {
  total_rows: u64,
  offset: u64,
  rows: Vec<Row>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Row {
  id: String,
  key: String,
  doc: Doc,
}

#[derive(Serialize, Deserialize, Debug)]
struct Doc {
  name: String,
  #[serde(rename = "dist-tags")]
  dist_tags: BTreeMap<Descending<String>, String>,
  time: BTreeMap<Descending<String>, String>,
}

/// Run the update command side effects
pub async fn run(ctx: Context) -> Result<Context, Box<dyn Error>> {
  let ui = Ui { ctx: &ctx };
  let client = Client::new();

  let response = client
    .post("https://replicate.npmjs.com/registry/_all_docs?include_docs=true")
    .header(ACCEPT, "application/json")
    .json(&serde_json::json!({
      "keys": ["tightrope", "react"],
      "limit": 1,
    }))
    .send()
    .await?;

  // Ensure the request was successful
  if response.status().is_success() {
    // Parse the response body as JSON
    let json: Root = response.json().await?;
    println!("Response JSON: {:#?}", json);
  } else {
    println!("Request failed with status: {}", response.status());
  }

  Ok(ctx)
}
