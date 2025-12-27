use {
  crate::dependency::UpdateUrl,
  log::debug,
  reqwest::{header::ACCEPT, Client, StatusCode},
  serde::{Deserialize, Serialize},
  serde_json::Value,
  std::{collections::BTreeMap, time::Duration},
  thiserror::Error,
};

#[derive(Error, Debug)]
pub enum RegistryError {
  #[error("Failed to fetch package '{url}': {source}")]
  FetchError {
    url: String,
    #[source]
    source: Box<dyn std::error::Error + Send + Sync>,
  },

  #[error("HTTP error for package '{url}': {status}")]
  HttpError { url: String, status: StatusCode },
}

/// Registry responses such as https://registry.npmjs.org/colors
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PackageMeta {
  pub name: String,
  pub versions: BTreeMap<String, Value>,
}

/// All available versions of a package from the npm registry
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AllPackageVersions {
  pub name: String,
  pub versions: Vec<String>,
}

/// A trait defining the interface for a registry client
#[async_trait::async_trait]
pub trait RegistryClient: std::fmt::Debug + Send + Sync {
  /// Fetch latest version of a given dep
  async fn fetch(&self, update_url: &UpdateUrl) -> Result<AllPackageVersions, RegistryError>;
}

/// The real implementation of RegistryClientTrait which makes actual network
/// requests
#[derive(Debug)]
pub struct LiveRegistryClient {
  pub client: Client,
}

#[async_trait::async_trait]
impl RegistryClient for LiveRegistryClient {
  async fn fetch(&self, update_url: &UpdateUrl) -> Result<AllPackageVersions, RegistryError> {
    let req = self.client.get(&update_url.url).header(ACCEPT, "application/json");
    debug!("GET {update_url:?}");
    match req.send().await {
      Ok(res) => match res.status() {
        StatusCode::OK => match res.json::<PackageMeta>().await {
          Ok(package_meta) => {
            let versions = package_meta
              .versions
              .into_iter()
              .filter(|(_, metadata)| {
                // Filter out deprecated versions by checking if "deprecated" field exists
                metadata.get("deprecated").is_none()
              })
              .map(|(version, _)| version)
              .collect();
            Ok(AllPackageVersions {
              name: package_meta.name,
              versions,
            })
          }
          Err(err) => Err(RegistryError::FetchError {
            url: update_url.url.to_string(),
            source: Box::new(err),
          }),
        },
        status => Err(RegistryError::HttpError {
          url: update_url.url.to_string(),
          status,
        }),
      },
      Err(err) => Err(RegistryError::FetchError {
        url: update_url.url.to_string(),
        source: Box::new(err),
      }),
    }
  }
}

impl LiveRegistryClient {
  pub fn new() -> Self {
    LiveRegistryClient {
      client: Client::builder()
        .connect_timeout(Duration::from_secs(5))
        .timeout(Duration::from_secs(30))
        .build()
        .expect("Failed to build reqwest client"),
    }
  }
}
