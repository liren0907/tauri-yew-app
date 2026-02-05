use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/media")]
    Media,
    #[at("/video")]
    Video,
    #[at("/image")]
    Image,
    #[at("/chat")]
    Chat,
}
