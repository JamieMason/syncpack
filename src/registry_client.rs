use {
  crate::dependency::UpdateUrl,
  log::debug,
  npmrc_config_rs::{Credentials, NpmrcConfig},
  reqwest::{
    header::{ACCEPT, AUTHORIZATION},
    Client, StatusCode, Url,
  },
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

  #[error("Failed to load .npmrc config: {0}")]
  NpmrcLoadError(#[from] npmrc_config_rs::Error),
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
  pub npmrc: NpmrcConfig,
}

#[async_trait::async_trait]
impl RegistryClient for LiveRegistryClient {
  async fn fetch(&self, update_url: &UpdateUrl) -> Result<AllPackageVersions, RegistryError> {
    // Build full URL from package name using .npmrc config
    let registry_base = self.registry_url(&update_url.package_name);
    let full_url = registry_base
      .join(&update_url.package_name)
      .map_err(|e| RegistryError::FetchError {
        url: update_url.package_name.clone(),
        source: Box::new(e),
      })?;
    let url_str = full_url.to_string();

    // Build request with auth if credentials exist
    let mut req = self.client.get(full_url.clone()).header(ACCEPT, "application/json");
    if let Some(creds) = self.npmrc.credentials_for(&registry_base) {
      req = match &creds {
        Credentials::Token { token, .. } => req.bearer_auth(token),
        Credentials::BasicAuth { .. } | Credentials::LegacyAuth { .. } => {
          if let Some(header) = creds.basic_auth_header() {
            req.header(AUTHORIZATION, format!("Basic {header}"))
          } else {
            req
          }
        }
        Credentials::ClientCertOnly(_) => req,
      };
    }

    debug!("GET {url_str}");
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
            url: url_str,
            source: Box::new(err),
          }),
        },
        status => Err(RegistryError::HttpError { url: url_str, status }),
      },
      Err(err) => Err(RegistryError::FetchError {
        url: url_str,
        source: Box::new(err),
      }),
    }
  }
}

impl LiveRegistryClient {
  pub fn new() -> Result<Self, RegistryError> {
    let npmrc = NpmrcConfig::load()?;
    Ok(LiveRegistryClient {
      client: Client::builder()
        .connect_timeout(Duration::from_secs(5))
        .timeout(Duration::from_secs(30))
        .build()
        .expect("Failed to build reqwest client"),
      npmrc,
    })
  }

  /// Resolve registry URL for a package, with JSR fallback
  pub fn registry_url(&self, package_name: &str) -> Url {
    let url = self.npmrc.registry_for(package_name);
    // Fallback: @jsr/* uses npm.jsr.io if not explicitly configured
    if package_name.starts_with("@jsr/") && url.host_str() == Some("registry.npmjs.org") {
      return Url::parse("https://npm.jsr.io/").unwrap();
    }
    url
  }
}
