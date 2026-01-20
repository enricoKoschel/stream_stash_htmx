use crate::views::icons::bars_3_solid;
use maud::{Markup, html};

pub fn navbar_dropdown(button_class: &str, button_size: &str) -> Markup {
    html! {
        details class="dropdown dropdown-end" {
            summary class={ "btn btn-square btn-ghost " (button_class) } {
                (bars_3_solid(button_size));
            }
            ul class="menu dropdown-content bg-base-100 rounded-box z-1 w-52 p-2 shadow-sm sm:text-lg" {
                li { a href="/profile" { "Profile"; } }
                div class="divider m-0 px-2" {}
                li { a href="/logout" { "Logout"; } }
            }
        }
    }
}
