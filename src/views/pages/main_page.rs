use crate::views::components::MediaCard;
use crate::views::layouts::MainLayout;
use hypertext::prelude::*;

pub fn main_page(cards: &[MediaCard]) -> impl Renderable {
    maud! {
        // TODO: Do this with optional?
        MainLayout search_query=("", "Movies") {
            @if !cards.is_empty() {
                div class="grid gap-4
                    grid-cols-[repeat(auto-fill,minmax(150px,1fr))]
                    lg:grid-cols-[repeat(auto-fill,minmax(200px,1fr))]
                    2xl:grid-cols-[repeat(auto-fill,minmax(250px,1fr))]" {
                    @for card in cards {
                        (card)
                    }
                }
            } @else {
                div class="flex justify-center" {
                    h2 class="text-4xl" { "No media to display" }
                }
            }
        }
    }
}
