use crate::views::layouts::MainLayout;
use hypertext::prelude::*;

// TODO: Actual search query and search results struct
pub fn search_page(search_query: (&str, &str), search_results: &str) -> impl Renderable {
    maud! {
        MainLayout search_query=search_query {
            (search_results)
        }
    }
}
