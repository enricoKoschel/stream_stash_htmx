use maud::{Markup, html};

// TODO: Media card as struct?
pub fn card_collection(cards: &[Markup], after_search: bool, only_cards: bool) -> Markup {
    let rendered_cards = if !cards.is_empty() {
        Some(html! {
            @for card in cards {
                (card);
            }
        })
    } else {
        None
    };

    if only_cards {
        rendered_cards.unwrap_or(html! {})
    } else {
        html! {
            @if let Some(rendered_cards) = rendered_cards {
                div class="grid gap-4
                grid-cols-[repeat(auto-fill,minmax(150px,1fr))]
                lg:grid-cols-[repeat(auto-fill,minmax(200px,1fr))]
                2xl:grid-cols-[repeat(auto-fill,minmax(250px,1fr))]"
                {
                    (rendered_cards);
                }
            } @else {
                div class="flex justify-center" {
                    @if after_search {
                        h2 class="text-4xl" { "No search results for this query"; }
                    } @else {
                        h2 class="text-4xl" { "No media to display"; }
                    }
                }
            }
        }
    }
}
