#[cfg(test)]
#[path = "client_test.rs"]
mod client_test;

use {
  crate::dependency::UpdateUrl,
  log::debug,
  reqwest::{Client, StatusCode, header::ACCEPT},
  serde::{Deserialize, Serialize},
  serde_json::Value,
  std::{
    collections::{BTreeMap, HashMap},
    sync::Arc,
    time::Duration,
  },
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
  /// Per-version publish timestamps (ISO 8601). Also contains
  /// non-version keys `created` / `modified` which we ignore.
  #[serde(default)]
  pub time: BTreeMap<String, String>,
}

/// All available versions of a package from the npm registry
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AllPackageVersions {
  pub name: String,
  pub versions: Vec<String>,
  /// Map of version → ISO 8601 publish timestamp. May be empty if the
  /// registry response did not include `time`.
  #[serde(default)]
  pub times: HashMap<String, String>,
}

/// A trait defining the interface for a registry client
#[async_trait::async_trait]
pub trait RegistryClient: std::fmt::Debug + Send + Sync {
  /// Fetch every published version of `update_url`. Implementations are
  /// free to satisfy hits from a cache (see `CachedRegistryClient`).
  async fn fetch(&self, update_url: &UpdateUrl) -> Result<Arc<AllPackageVersions>, RegistryError>;
}

/// The real implementation of `RegistryClient` which makes network
/// requests against the npm registry. Pure HTTP — caching is layered on
/// via `CachedRegistryClient`.
#[derive(Debug)]
pub struct LiveRegistryClient {
  pub client: Client,
}

#[async_trait::async_trait]
impl RegistryClient for LiveRegistryClient {
  async fn fetch(&self, update_url: &UpdateUrl) -> Result<Arc<AllPackageVersions>, RegistryError> {
    let req = self.client.get(&update_url.url).header(ACCEPT, "application/json");
    debug!("GET {update_url:?}");
    match req.send().await {
      Ok(res) => match res.status() {
        StatusCode::OK => match res.json::<PackageMeta>().await {
          Ok(package_meta) => {
            let versions: Vec<String> = package_meta
              .versions
              .into_iter()
              .filter(|(_, metadata)| {
                // Filter out deprecated versions by checking if "deprecated" field exists
                metadata.get("deprecated").is_none()
              })
              .map(|(version, _)| version)
              .collect();
            let times = package_meta
              .time
              .into_iter()
              .filter(|(k, _)| k != "created" && k != "modified")
              .collect();
            Ok(Arc::new(AllPackageVersions {
              name: package_meta.name,
              versions,
              times,
            }))
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
