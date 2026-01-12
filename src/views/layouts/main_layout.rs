use crate::views::components::main_footer;
use crate::views::components::main_navbar;
use maud::{Markup, html};

pub fn main_layout(children: Markup) -> Markup {
    html! {
        (main_navbar(None));
        // Padding for the navbar and footer
        div class="px-4 pt-40 sm:pt-20 pb-18" {
            (children);
        }
        (main_footer());
    }
}
