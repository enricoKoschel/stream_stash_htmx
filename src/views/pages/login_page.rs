use std::collections::HashMap;

use crate::data_source::{MEDIA_STATES_MOVIE, MediaState, MediaType};
use crate::views::components::{google_login_button, skeleton_card};
use crate::views::layouts::main_layout;
use crate::views::pages::main_page;
use maud::{Markup, html};

pub fn login_page() -> Markup {
    let cards = HashMap::from([
        (MediaState::Planned, vec![skeleton_card(); 5]),
        (MediaState::Finished, vec![skeleton_card(); 8]),
        (MediaState::Dropped, vec![skeleton_card(); 2]),
    ]);

    main_layout(html! {
        dialog id="login-modal" class="modal modal-open" {
            div class="modal-box flex flex-col gap-6" {
                h2 class="text-2xl font-bold text-center" {
                    "Sign in to Stream Stash";
                }
                div class="flex justify-center" {
                    (google_login_button());
                }
                div class="flex justify-center gap-4 text-sm" {
                    a class="link" href="/privacy" { "Privacy Policy"; }
                    a class="link" href="/about" { "About"; }
                }
            }
        }

        div class="opacity-50" {
            (main_page(&cards, MediaType::Movies, &MEDIA_STATES_MOVIE, true));
        }
    })
}
