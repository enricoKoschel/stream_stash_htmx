use crate::data_source::search::SearchQuery;
use crate::views::layouts::main_layout;
use maud::{Markup, html};

// TODO: Media card as struct?
pub fn card_page(search_query: Option<&SearchQuery>, cards: &[Markup]) -> Markup {
    main_layout(
        search_query,
        html! {
            @if !cards.is_empty() {
                div class="grid gap-4
                    grid-cols-[repeat(auto-fill,minmax(150px,1fr))]
                    lg:grid-cols-[repeat(auto-fill,minmax(200px,1fr))]
                    2xl:grid-cols-[repeat(auto-fill,minmax(250px,1fr))]"
                {
                    @for card in cards {
                        (card)
                    }
                }
            } @else {
                div class="flex justify-center" {
                    @if search_query.is_some() {
                        h2 class="text-4xl" { "No search results for this query"; }
                    } @else {
                        h2 class="text-4xl" { "No media to display"; }
                    }
                }
            }
        },
    )
}
