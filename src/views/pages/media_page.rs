use maud::{Markup, html};

use crate::views::layouts::main_layout;

// TODO: Actual search query struct
pub fn media_page(search_query: (&str, &str), media_type: &str, id: &str) -> Markup {
    main_layout(
        search_query,
        html! {
            (media_type);
            br;
            (id);
        },
    )
}
