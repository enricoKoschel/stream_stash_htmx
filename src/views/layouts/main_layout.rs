use crate::data_source::search::SearchQuery;
use crate::views::components::main_footer;
use crate::views::components::main_navbar;
use maud::{Markup, html};

pub fn main_layout(search_query: Option<&SearchQuery>, children: Markup) -> Markup {
    html! {
        (main_navbar(search_query));
        // Margin on the bottom for the footer
        div class="p-4 mb-14" {
            (children);
        }
        // TODO: Looks bad on mobile
        (main_footer());
    }
}
