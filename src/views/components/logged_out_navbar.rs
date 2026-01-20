use crate::views::icons::home_solid;
use maud::{Markup, html};

pub fn logged_out_navbar() -> Markup {
    html! {
        // Mobile navbar
        div class="sm:hidden fixed top-0 w-full z-1 bg-base-200 shadow-md flex px-2 py-1" {
            div class="flex-1" {
                a class="btn btn-square btn-ghost btn-sm" href="/" {
                    (home_solid("size-6"));
                }
            }
            img class="w-6" src="/static/logos/StreamStashNoText.svg";
            div class="flex-1" {}
        }

        // Desktop navbar
        div class="hidden sm:flex navbar fixed top-0 z-1 bg-base-200 shadow-md" {
            div class="flex-1" {
                a class="btn btn-square btn-ghost text-xl" href="/" {
                    (home_solid("size-8"));
                }
            }
            img class="w-96" src="/static/logos/StreamStashWithTextWhite.svg";
            div class="flex-1" {}
        }
    }
}
