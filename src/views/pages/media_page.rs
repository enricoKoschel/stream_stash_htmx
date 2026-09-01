use crate::data_source::MediaType;
use crate::data_source::db::MediaHistoryEntry;
use crate::views::components::{
    add_media_to_list_button, media_history_entry, watch_state_dropdown,
};
use crate::views::icons::plus_solid;
use crate::views::layouts::main_layout;
use crate::views::{components::image_with_fallback, icons::calendar_solid};
use maud::{Markup, html};

fn history_section(
    history: &[MediaHistoryEntry],
    media_url: &str,
    media_type: MediaType,
    hidden: bool,
) -> Markup {
    let history_url = format!("{}/history", media_url);

    // 4 different heights:
    //   Small & TV Show - stacked dates
    //   Small & Movie - single line dates
    //   Big & TV Show - single line dates
    //   Big & Movie - single line dates
    let list_height = match media_type {
        MediaType::Movies => "max-h-135",
        MediaType::TvShows => "max-h-161 sm:max-h-135",
    };

    html! {
        div id="media-history-section" class={"w-full max-w-5xl mx-auto mt-12 " @if hidden { "hidden"; }} {
            div class="flex justify-between items-center mb-4" {
                h2 class="text-xl sm:text-2xl font-semibold" { "Watch History"; }
                button class="btn btn-primary btn-md sm:btn-lg" hx-post=(&history_url) hx-target="#watch-history-list" hx-swap="afterbegin" {
                    (plus_solid("size-5 sm:size-6"));
                    "Add Entry";
                }
            }

            div class="group" {
                div id="watch-history-list" class={"flex flex-col gap-3 overflow-y-auto pr-2 " (list_height)}{
                    @for entry in history {
                        (media_history_entry(entry, &history_url, media_type));
                    }
                }

                p id="watch-history-empty-label" class="hidden group-has-[#watch-history-list:empty]:block text-base-content/70 text-lg sm:text-2xl text-center mt-4" {
                    "No watch history yet. Add an entry to get started!";
                }
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
// TODO: Media item as struct?
pub fn media_page(
    title: &str,
    overview: &str,
    release_date: &str,
    poster_url: Option<&str>,
    backdrop_url: Option<&str>,
    state: Option<&str>,
    states: &[&str],
    media_url: &str,
    history: &[MediaHistoryEntry],
    media_type: MediaType,
) -> Markup {
    main_layout(html! {
        // top-* for navbar
        (image_with_fallback("fixed inset-0 top-10 sm:top-16 w-full h-full -z-1 opacity-20 object-cover", "aspect-16/9", backdrop_url));

        div class="flex flex-col md:flex-row justify-center pt-0 sm:pt-8 gap-6 md:gap-10 lg:gap-12" {
            div class="flex-shrink-0 flex flex-col gap-2 sm:gap-4 mx-auto md:mx-0" {
                (image_with_fallback("w-48 sm:w-56 md:w-64 lg:w-72 rounded-lg shadow-2xl ring-1 ring-white/10", "aspect-2/3", poster_url));

                @if let Some(state) = state {
                    (watch_state_dropdown(state, states, media_url));
                } @else {
                    (add_media_to_list_button(media_url));
                }
            }

            div class="flex flex-col gap-4 md:gap-6 text-center max-md:items-center md:text-left" {
                h1 class="text-3xl sm:text-4xl lg:text-5xl font-bold max-w-4xl" {
                    (title);
                }

                div class="flex flex-col items-center md:items-start gap-2 w-fit" {
                    div class="flex items-center gap-2 justify-center md:justify-start text-base text-base-content/70" {
                        (calendar_solid("size-4"));
                        span { (release_date); }
                    }

                    div class="w-full h-1 bg-primary rounded-full" {}
                }

                div class="flex flex-col gap-2 max-w-4xl" {
                    h2 class="text-lg font-semibold text-base-content/90" { "Overview"; }
                    p class="text-base-content/80 text-sm sm:text-base lg:text-lg" {
                        (overview);
                    }
                }
            }
        }

        (history_section(history, media_url, media_type, !state.is_some()));

        dialog id="confirm-delete-media-modal" class="modal" {
            div class="modal-box flex flex-col gap-8" {
                div class="flex flex-col gap-4 sm:gap-3 text-xl sm:text-2xl" {
                    p { "Are you sure you want to delete this item from your list?"; }
                    p { "This will delete the current state and all history entries."; }
                }
                div class="flex gap-2 w-full" {
                    div class="flex-2 hidden sm:block" {}
                    button class="flex-1 btn btn-primary" onclick="document.getElementById('confirm-delete-media-modal').close()" {
                        "Cancel";
                    }
                    button class="flex-1 btn btn-error" onclick="document.getElementById('confirm-delete-media-modal').close(); document.getElementById('media-history-section').classList.add('hidden')" hx-delete=(media_url) hx-target="#watch-state-dropdown" hx-swap="outerHTML" {
                        "Confirm";
                    }
                }
            }
            // Hidden form, closes the dialog when pressing outside it
            form method="dialog" class="modal-backdrop" {
                button {}
            }
        }

        dialog id="confirm-delete-history-entry-modal" class="modal" {
            div class="modal-box flex flex-col gap-8" {
                div class="flex flex-col gap-4 sm:gap-3 text-xl sm:text-2xl" {
                    p { "Are you sure you want to delete this history entry?"; }
                }
                div class="flex gap-2 w-full" {
                    div class="flex-2 hidden sm:block" {}
                    button class="flex-1 btn btn-primary" onclick="document.getElementById('confirm-delete-history-entry-modal').close()" {
                        "Cancel";
                    }
                    // hx-delete and hx-target are set dynamically for each history entry
                    button id="confirm-delete-history-entry-button" class="flex-1 btn btn-error" onclick="document.getElementById('confirm-delete-history-entry-modal').close()" hx-swap="outerHTML" {
                        "Confirm";
                    }
                }
            }
            // Hidden form, closes the dialog when pressing outside it
            form method="dialog" class="modal-backdrop" {
                button {}
            }
        }
    })
}
