use crate::views::icons::trash_solid;
use maud::{Markup, html};

pub fn watch_state_dropdown(state: &str, states: &[&str], media_url: &str) -> Markup {
    html! {
        form id="watch-state-dropdown" class="flex gap-2 w-full" hx-patch=(media_url) hx-trigger="change" autocomplete="off" {
            select name="state" class="flex-1 select select-md sm:select-lg outline-0" {
                @for &media_state in states {
                    option selected[state == media_state] { (media_state); }
                }
            }

            button class="btn btn-error btn-square btn-md sm:btn-lg" type="button" onclick="document.getElementById('confirm-delete-media-modal').showModal()" {
                (trash_solid("size-6 sm:size-8"));
            }
        }
    }
}
