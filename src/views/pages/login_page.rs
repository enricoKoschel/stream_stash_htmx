use crate::views::components::{card_collection, google_login_button, skeleton_card};
use crate::views::layouts::main_layout;
use maud::{Markup, html};

pub fn login_page() -> Markup {
    let cards = vec![skeleton_card(); 10];

    main_layout(html! {
        dialog id="login-modal" class="modal modal-open" {
            div class="modal-box" {
                h2 class="text-2xl font-bold text-center mb-6" { "Sign in to Stream Stash" }
                div class="flex justify-center" {
                    (google_login_button());
                }
            }
        }

        div class="opacity-50" {
            (card_collection(&cards, false, false));
        }
    })
}
