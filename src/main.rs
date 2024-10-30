use shortener_url_rust::service::commands::CommandHandler;
use shortener_url_rust::service::models::{Slug, Url};
use shortener_url_rust::service::queries::QueryHandler;
use shortener_url_rust::service::url_shortener::UrlShortenerService;

fn main() {
    let mut service = UrlShortenerService::new();


    let url = Url("http://example.com".to_string());
    let slug = Slug("custom_slug".to_string());

    match service.handle_create_short_link(url.clone(), Some(slug.clone())) {
        Ok(short_link) => println!("Created short_link: {:#?}", short_link),
        Err(e) => println!("Error creating short_link: {:#?}", e),
    }

    match service.handle_redirect(slug.clone()) {
        Ok(short_link) => println!("Redirect to: {:#?}", short_link),
        Err(e) => println!("Error redirecting short link: {:#?}", e),
    }

    match service.get_stats(slug.clone()) {
        Ok(stats) => println!("Stats: {:#?}", stats),
        Err(e) => println!("Error retrieving stats: {:#?}", e),
    }
}
