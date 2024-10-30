use crate::service::models::{Slug, Stats};
use crate::service::errors::ShortenerError;

/// Trait for query handlers.
pub trait QueryHandler {
    /// Returns the [`Stats`] for a specific [`ShortLink`], such as the
    /// number of redirects (clicks).
    ///
    /// [`ShortLink`]: super::ShortLink
    fn get_stats(&self, slug: Slug) -> Result<Stats, ShortenerError>;
}