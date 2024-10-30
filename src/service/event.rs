use crate::service::models::{ShortLink, Slug};

#[derive(Debug,Clone, PartialEq)]
pub enum Event {
    LinkCreated(ShortLink),
    RedirectCountIncremented(Slug),
}