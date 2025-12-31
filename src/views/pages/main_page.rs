use crate::views::components::MediaCard;
use crate::views::layouts::MainLayout;
use hypertext::prelude::*;

pub fn main_page(cards: &[MediaCard]) -> impl Renderable {
    maud! {
        MainLayout {
            div class="grid grid-cols-9 gap-5 m-2" {
                @for card in cards {
                    (card)
                }
            }
        }
    }
}
