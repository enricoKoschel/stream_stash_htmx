use crate::data_source::{MEDIA_TYPES, MediaType};
use crate::views::components::card_collection;
use crate::views::layouts::main_layout;
use maud::{Markup, html};

pub fn main_page(cards: &[Markup], selected_media_type: MediaType, without_layout: bool) -> Markup {
    let content = html! {
        div class="tabs tabs-box sm:tabs-lg mb-4" {
            @for media_type in MEDIA_TYPES {
                a
                    class={
                        "tab "
                        @if media_type == selected_media_type { "tab-active" }
                    }
                    href=(format!("/?t={}", media_type))
                {
                    (media_type);
                }
            }
        }

        (card_collection(cards, false, false));
    };

    if without_layout {
        content
    } else {
        main_layout(content)
    }
}
