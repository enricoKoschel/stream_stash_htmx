use crate::data_source::search::SearchQuery;
use crate::views::components::main_footer;
use crate::views::components::main_navbar;
use crate::views::components::search_results_count_bar;
use maud::{Markup, html};

pub fn search_layout(
    search_query: &SearchQuery,
    shown_results: i32,
    total_results: i32,
    children: Markup,
) -> Markup {
    html! {
        (main_navbar(Some(search_query)));
        (search_results_count_bar(shown_results, total_results, None));
        // Padding for the navbar and footer
        div class="px-4 pt-51 sm:pt-31 pb-18" {
            (children);
        }
        (main_footer());
    }
}
