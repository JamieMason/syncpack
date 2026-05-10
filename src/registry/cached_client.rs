use {
  crate::{
    dependency::UpdateUrl,
    disk::DiskIo,
    registry::{
      cache::Cache,
      client::{AllPackageVersions, RegistryClient, RegistryError},
    },
  },
  log::debug,
  std::{path::PathBuf, sync::Arc},
};

/// Wraps any `RegistryClient` with an on-disk cache. The cache is
/// loaded eagerly on construction and persisted to disk in `Drop`,
/// so callers don't manage cache lifecycle.
#[derive(Debug)]
pub struct CachedRegistryClient<R, D>
where
  R: RegistryClient,
  D: DiskIo + std::fmt::Debug + Send + Sync + 'static,
{
  inner: R,
  cache: Cache,
  io: Arc<D>,
}

impl<R, D> CachedRegistryClient<R, D>
where
  R: RegistryClient,
  D: DiskIo + std::fmt::Debug + Send + Sync + 'static,
{
  pub fn new(inner: R, io: Arc<D>, cache_filepath: PathBuf) -> Self {
    let cache = Cache::load(&*io, cache_filepath);
    Self { inner, cache, io }
  }
}

#[async_trait::async_trait]
impl<R, D> RegistryClient for CachedRegistryClient<R, D>
where
  R: RegistryClient,
  D: DiskIo + std::fmt::Debug + Send + Sync + 'static,
{
  async fn fetch(&self, update_url: &UpdateUrl) -> Result<Arc<AllPackageVersions>, RegistryError> {
    if let Some(hit) = self.cache.lookup(&update_url.url) {
      debug!("registry cache hit for {}", update_url.url);
      return Ok(hit);
    }
    let fresh = self.inner.fetch(update_url).await?;
    self.cache.store(&update_url.url, Arc::clone(&fresh));
    Ok(fresh)
  }
}

impl<R, D> Drop for CachedRegistryClient<R, D>
where
  R: RegistryClient,
  D: DiskIo + std::fmt::Debug + Send + Sync + 'static,
{
  fn drop(&mut self) {
    self.cache.save(&*self.io);
  }
}
