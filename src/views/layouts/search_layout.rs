use crate::data_source::search::SearchQuery;
use crate::views::components::main_footer;
use crate::views::components::main_navbar;
use maud::{Markup, html};

pub fn search_results_count_bar(
    shown_results: i32,
    total_results: i32,
    oob_children: Option<Markup>,
) -> Markup {
    match oob_children {
        Some(oob_children) => html! {
            p id="search-results-count" class="text-lg" hx-swap-oob="true" {
                "Results shown: " (shown_results) " of " (total_results);
            }
            (oob_children);
        },
        None => html! {
            // top-* for navbar
            div class="fixed top-10 sm:top-16 inset-x-0 w-full py-2 z-1 bg-base-200 shadow-md flex justify-center" {
                div class="relative" {
                    p id="search-results-count" class="text-lg" {
                        "Results shown: " (shown_results) " of " (total_results);
                    }
                    span id="search-loading-indicator" class="loading loading-spinner loading-lg htmx-indicator absolute left-full ml-4 top-0" {}
                }
            }
        },
    }
}

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
        div class="px-4 pt-25 sm:pt-31 pb-14 sm:pb-18" {
            (children);
        }
        (main_footer());
    }
}
