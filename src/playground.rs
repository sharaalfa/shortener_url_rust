use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Debug, PartialEq)]
pub enum ShortenerError {
    InvalidUrl,
    SlugAlreadyInUse,
    SlugNotFound,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Slug(pub String);

#[derive(Clone, Debug, PartialEq)]
pub struct Url(pub String);

#[derive(Debug, Clone, PartialEq)]
pub struct ShortLink {
    pub slug: Slug,
    pub url: Url,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Stats {
    pub link: ShortLink,
    pub redirects: u64,
}

pub mod commands {
    use super::{ShortLink, ShortenerError, Slug, Url};

    pub trait CommandHandler {
        fn handle_create_short_link(
            &mut self,
            url: Url,
            slug: Option<Slug>,
        ) -> Result<ShortLink, ShortenerError>;

        fn handle_redirect(
            &mut self,
            slug: Slug,
        ) -> Result<ShortLink, ShortenerError>;
    }
}

pub mod queries {
    use super::{ShortenerError, Slug, Stats};

    pub trait QueryHandler {
        fn get_stats(&self, slug: Slug) -> Result<Stats, ShortenerError>;
    }
}

#[derive(Debug, Clone)]
enum Event {
    #[allow(dead_code)]
    LinkCreated(ShortLink),
    #[allow(dead_code)]
    RedirectCountIncremented(Slug),
}

pub struct UrlShortenerService {
    events: Arc<Mutex<Vec<Event>>>,
    state: Arc<Mutex<HashMap<Slug, (Url, u64)>>>,
}

impl UrlShortenerService {
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
            Ok(Stats { link: short_link, redirects: *redirects })
        } else {
            Err(ShortenerError::SlugNotFound)
        }
    }
}

fn main() {
    use commands::CommandHandler;
    use queries::QueryHandler;

    let mut service = UrlShortenerService::new();

    // Создание короткой ссылки
    let url = Url("https://example.com".to_string());
    let slug = Slug("custom_slug".to_string());

    match service.handle_create_short_link(url.clone(), Some(slug.clone())) {
        Ok(short_link) => println!("Created short link: {:?}", short_link),
        Err(e) => println!("Error: {:?}", e),
    }

    // Перенаправление по короткой ссылке
    match service.handle_redirect(slug.clone()) {
        Ok(short_link) => println!("Redirected to: {:?}", short_link),
        Err(e) => println!("Error: {:?}", e),
    }

    // Получение статистики по ссылке
    match service.get_stats(slug.clone()) {
        Ok(stats) => println!("Stats: {:?}", stats),
        Err(e) => println!("Error: {:?}", e),
    }
}