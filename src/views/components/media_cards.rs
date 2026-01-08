use maud::{Markup, html};

// TODO: Media card as struct?
pub fn media_cards(cards: &[Markup]) -> Markup {
    html! {
        @for card in cards {
            (card)
        }
    }
}
