use crate::{routes::search::SearchQuery, views::layouts::main_layout};
use maud::{Markup, html};

pub fn media_page(search_query: Option<&SearchQuery>, media_type: &str, id: &str) -> Markup {
    main_layout(
        search_query,
        html! {
            (media_type);
            br;
            (id);
        },
    )
}
