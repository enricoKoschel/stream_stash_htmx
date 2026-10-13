use crate::data_source::{MEDIA_TYPES, MediaState, MediaType};
use crate::views::components::card_collection;
use crate::views::layouts::main_layout;
use maud::{Markup, html};
use std::collections::HashMap;

fn media_type_tab_header(selected_media_type: MediaType) -> Markup {
    html! {
        div class="tabs tabs-box sm:tabs-lg mb-4" {
            @for media_type in MEDIA_TYPES {
                a
                    class={
                        "tab grow "
                        @if media_type == selected_media_type { "tab-active" }
                    }
                    href=(format!("/?t={}", media_type))
                {
                    (media_type);
                }
            }
        }
    }
}

fn collapse_checkbox(
    selected_media_type: MediaType,
    media_state: MediaState,
    not_logged_in: bool,
) -> Markup {
    if not_logged_in {
        html! {
            input
                class="peer p-0 h-10"
                type="checkbox"
                checked
                autocomplete="off";
        }
    } else {
        html! {
            input
                class="peer p-0 h-10"
                type="checkbox"
                checked
                data-storage-key=(format!("collapse-open-{}-{}",
                        selected_media_type, media_state))
                onchange="try {
                localStorage.setItem(this.dataset.storageKey,
                    this.checked)
            } catch(e) {}";
            script {
                (maud::PreEscaped(
                        "(() => {
                        const i = document.currentScript.previousElementSibling;
                        try{
                            const v = localStorage.getItem(i.dataset.storageKey);
                            if (v !== null) i.checked = v === 'true';
                        } catch(e) {}
                    })()"
                ))
            }
        }
    }
}

pub fn main_page(
    cards: &HashMap<MediaState, Vec<Markup>>,
    selected_media_type: MediaType,
    media_states: &[MediaState],
    not_logged_in: bool,
) -> Markup {
    let content = html! {
        (media_type_tab_header(selected_media_type));

        // TODO: Allow changing order and which are open/closed by default
        @for media_state in media_states {
            div
                class={
                    "collapse "
                    @if cards.contains_key(media_state) { "has-checked:mb-2" }
                    @else { "has-checked:mb-4" }
                }
            {
                (collapse_checkbox(selected_media_type, *media_state, not_logged_in));

                div class="collapse-title font-semibold rounded-xl bg-base-200
                           py-2 px-4 mb-2 flex items-center gap-3
                           peer-not-checked:text-base-content/20"
                {
                    div class="flex-1 h-px bg-current" {}
                    (media_state);
                    div class="flex-1 h-px bg-current" {}
                }
                div class="collapse-content pb-0" {
                    (card_collection(cards.get(media_state)
                                     .map(Vec::as_slice)
                                     .unwrap_or(&[]), false, false)
                    );
                }
            }
        }
    };

    if not_logged_in {
        content
    } else {
        main_layout(content)
    }
}
