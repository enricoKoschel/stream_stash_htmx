use crate::{
    data_source::search::SearchQuery,
    views::{components::card_collection, layouts::search_layout},
};
use maud::{Markup, html};

// TODO: Media item as struct?
pub fn search_page(
    search_query: &SearchQuery,
    shown_results: i32,
    total_results: i32,
    cards: &[Markup],
) -> Markup {
    search_layout(
        search_query,
        shown_results,
        total_results,
        html! {
            (card_collection(cards, true, false));
        },
    )
}
