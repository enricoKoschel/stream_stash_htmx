use crate::views::icons::plus_solid;
use maud::{Markup, html};

pub fn add_media_to_list_button(media_url: &str) -> Markup {
    html! {
        button class="btn btn-primary w-full" hx-put=(media_url) hx-target="this" hx-swap="outerHTML" onclick="document.getElementById('media-history-section').classList.remove('hidden')" {
            (plus_solid("size-6 sm:size-8 stroke-white"));
            p class="text-base sm:text-lg" { "Add to list"; }
            // Dummy for visual alignment
            div class="size-2" {}
        }
    }
}
