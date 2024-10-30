use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::service::{commands, queries};
use crate::service::errors::ShortenerError;
use crate::service::event::Event;
use crate::service::models::{ShortLink, Slug, Stats, Url};

/// CQRS and Event Sourcing-based service implementation
pub struct UrlShortenerService {
    pub(crate) events: Arc<Mutex<Vec<Event>>>,
    pub(crate) state: Arc<Mutex<HashMap<Slug, (Url, u64)>>>,
}

impl UrlShortenerService {
    /// Creates a new instance of the service
    pub fn new() -> Self {
        Self {
            events: Arc::new(Mutex::new(Vec::new())),
            state: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    fn generate_slug(&self) -> Slug {
        Slug(format!("slug_{}", self.events.lock().unwrap().len()))
    }
}

impl commands::CommandHandler for UrlShortenerService {
    fn handle_create_short_link(
        &mut self,
        url: Url,
        slug: Option<Slug>,
    ) -> Result<ShortLink, ShortenerError> {
        let slug = slug.unwrap_or_else(|| self.generate_slug());
        let short_link = ShortLink { slug: slug.clone(), url: url.clone() };

        if self.state.lock().unwrap().contains_key(&slug) {
            return Err(ShortenerError::SlugAlreadyInUse);
        }

        self.events.lock().unwrap().push(Event::LinkCreated(short_link.clone()));
        self.state.lock().unwrap().insert(slug, (url, 0));

        Ok(short_link)
    }

    fn handle_redirect(
        &mut self,
        slug: Slug,
    ) -> Result<ShortLink, ShortenerError> {
        let mut state = self.state.lock().unwrap();
        if let Some((url, redirects)) = state.get_mut(&slug) {
            let short_link = ShortLink { slug: slug.clone(), url: url.clone() };
            self.events.lock().unwrap().push(Event::RedirectCountIncremented(slug.clone()));
            *redirects += 1;
            Ok(short_link)
        } else {
            Err(ShortenerError::SlugNotFound)
        }
    }
}

impl queries::QueryHandler for UrlShortenerService {
    fn get_stats(&self, slug: Slug) -> Result<Stats, ShortenerError> {
        let state = self.state.lock().unwrap();
        if let Some((url, redirects)) = state.get(&slug) {
            let short_link = ShortLink { slug: slug.clone(), url: url.clone() };
            Ok(Stats{link: short_link, redirects: *redirects})
        } else {
            Err(ShortenerError::SlugNotFound)
        }
    }
}

