use crate::layouts::root_layout::*;
use crate::pages::*;
use dioxus::prelude::*;
use dioxus_translate::Language;

#[derive(Clone, Routable, Debug, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[nest("/:lang")]
    #[layout(RootLayout)]
    #[route("/")]
    HomePage { lang: Language },
    #[end_layout]
    #[end_nest]

    #[redirect("/", || Route::HomePage { lang: Language::default() })]
    #[route("/:..route")]
    NotFoundPage { route: Vec<String> },
}
