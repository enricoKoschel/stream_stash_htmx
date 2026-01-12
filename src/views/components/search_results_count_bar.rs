use maud::{Markup, html};

pub fn search_results_count_bar(shown_results: i32, total_results: i32) -> Markup {
    html! {
        div id="search-results-count-bar" class="fixed top-36 sm:top-16 inset-x-0 w-full py-2 z-1 bg-base-200 shadow-md flex justify-center" {
            div class="relative" {
                p class="text-lg" {
                    "Results shown: " (shown_results) " of " (total_results);
                }
                span id="search-loading-indicator" class="loading loading-spinner loading-lg htmx-indicator absolute left-full ml-4 top-0" {}
            }
        }
    }
}
