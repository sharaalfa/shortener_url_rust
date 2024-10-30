use crate::service::models::{ShortLink, Slug, Url};
use crate::service::errors::ShortenerError;

/// Trait for command handlers.
pub trait CommandHandler {
    /// Creates a new short link. It accepts the original url and an
    /// optional [`Slug`]. If a [`Slug`] is not provided, the service will generate
    /// one. Returns the newly created [`ShortLink`].
    ///
    /// ## Errors
    ///
    /// See [`ShortenerError`].
    fn handle_create_short_link(
        &mut self,
        url: Url,
        slug: Option<Slug>,
    ) -> Result<ShortLink, ShortenerError>;

    /// Processes a redirection by [`Slug`], returning the associated
    /// [`ShortLink`] or a [`ShortenerError`].
    fn handle_redirect(
        &mut self,
        slug: Slug,
    ) -> Result<ShortLink, ShortenerError>;
}