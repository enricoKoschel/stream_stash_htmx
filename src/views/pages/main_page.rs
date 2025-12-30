use crate::views::components::MediaCard;
use crate::views::layouts::MainLayout;
use hypertext::prelude::*;

pub fn main_page() -> impl Renderable {
    maud! {
        MainLayout {
            div class="grid grid-cols-9 gap-5 m-2" {
                @for _ in 0..50 {
                    MediaCard title="Harry Potter and the Philosopher's Stone" year="2001" poster_url="https://image.tmdb.org/t/p/w600_and_h900_bestv2/wuMc08IPKEatf9rnMNXvIDxqP4W.jpg" media_page_url="/media/1";
                    MediaCard title="Breaking Bad" year="2008" poster_url="https://image.tmdb.org/t/p/w600_and_h900_bestv2/ztkUQFLlC19CCMYHW9o1zWhJRNq.jpg" media_page_url="/media/2";
                }
            }
        }
    }
}
