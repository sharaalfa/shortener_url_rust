pub mod service;
mod playground;

#[cfg(test)]
mod unit_tests{
    use crate::service::url_shortener::UrlShortenerService;
    use crate::service::commands::CommandHandler;
    use crate::service::queries::QueryHandler;
    use crate::service::errors::ShortenerError;
    use crate::service::models::{Slug, Url};
    #[test]
    fn test_create_short_link(){
        let mut short_link = UrlShortenerService::new();
        let url = Url("http://example.com".to_string());
        let slug = Slug("slug".to_string());

        let result = short_link.handle_create_short_link(url.clone(), Some(slug.clone()));
        assert!(result.is_ok());
    }

    #[test]
    fn test_create_short_link_with_existing_slug(){
        let mut short_link = UrlShortenerService::new();
        let url = Url("https://example.com".to_string());
        let slug = Slug("custom_slug".to_string());

        let result1 = short_link.handle_create_short_link(url.clone(), Some(slug.clone()));
        assert!(result1.is_ok());

        let result2 = short_link.handle_create_short_link(url.clone(), Some(slug.clone()));
        assert!(result2.is_err());
        assert_eq!(result2.unwrap_err(), ShortenerError::SlugAlreadyInUse);
    }

    #[test]
    fn test_handle_redirect(){
        let mut short_link = UrlShortenerService::new();
        let url = Url("http://example.com".to_string());
        let slug = Slug("custom_slug".to_string());

        let result = short_link.handle_create_short_link(url.clone(), Some(slug.clone()));
        assert!(result.is_ok());

        let result = short_link.handle_redirect(slug.clone());
        assert!(result.is_ok());

        let short_link = result.unwrap();
        assert_eq!(short_link.url,url);
        assert_eq!(short_link.slug, slug);

    }

    #[test]
    fn test_get_stats(){
        let mut short_link = UrlShortenerService::new();
        let url = Url("http://example.com".to_string());
        let slug = Slug("custom_slug".to_string());

        let result = short_link.handle_create_short_link(url.clone(), Some(slug.clone()));
        assert!(result.is_ok());

        let result = short_link.handle_redirect(slug.clone());
        assert!(result.is_ok());

        let result = short_link.get_stats(slug.clone());

        let stats = result.unwrap();
        assert_eq!(stats.link.url,url);
        assert_eq!(stats.link.slug,slug);
        assert_eq!(stats.redirects, 1)
    }
}