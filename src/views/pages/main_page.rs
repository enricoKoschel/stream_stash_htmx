use crate::views::components::card_collection;
use crate::views::layouts::main_layout;
use maud::Markup;

// TODO: Media item as struct?
pub fn main_page(cards: &[Markup]) -> Markup {
    main_layout(card_collection(cards, false, false))
}
