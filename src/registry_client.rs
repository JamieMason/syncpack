use {
  log::{debug, error},
  reqwest::{header::ACCEPT, Client, StatusCode},
  serde::{Deserialize, Serialize},
  std::collections::BTreeMap,
  thiserror::Error,
};

#[derive(Error, Debug)]
pub enum RegistryError {
  #[error("Failed to fetch package '{name}': {source}")]
  FetchError {
    name: String,
    #[source]
    source: Box<dyn std::error::Error + Send + Sync>,
  },

  #[error("HTTP error for package '{name}': {status}")]
  HttpError { name: String, status: StatusCode },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PackageMeta {
  pub name: String,
  pub time: BTreeMap<String, String>,
}

/// A trait defining the interface for a registry client
#[async_trait::async_trait]
pub trait RegistryClient: std::fmt::Debug + Send + Sync {
  /// Fetch latest version of a given dep
  async fn fetch(&self, dependency_name: &str) -> Result<PackageMeta, RegistryError>;
}

/// The real implementation of RegistryClientTrait which makes actual network
/// requests
#[derive(Debug)]
pub struct LiveRegistryClient {
  pub client: Client,
}

#[async_trait::async_trait]
impl RegistryClient for LiveRegistryClient {
  async fn fetch(&self, dependency_name: &str) -> Result<PackageMeta, RegistryError> {
    let url = format!("https://registry.npmjs.org/{}", dependency_name);
    let req = self.client.get(&url).header(ACCEPT, "application/json");
    debug!("GET {url}");
    match req.send().await {
      Ok(res) => match res.status() {
        StatusCode::OK => match res.json::<PackageMeta>().await {
          Ok(package_meta) => Ok(package_meta),
          Err(err) => Err(RegistryError::FetchError {
            name: dependency_name.to_string(),
            source: Box::new(err),
          }),
        },
        status => Err(RegistryError::HttpError {
          name: dependency_name.to_string(),
          status,
        }),
      },
      Err(err) => Err(RegistryError::FetchError {
        name: dependency_name.to_string(),
        source: Box::new(err),
      }),
    }
  }
}

impl LiveRegistryClient {
  pub fn new() -> Self {
    LiveRegistryClient { client: Client::new() }
  }
}
