use crate::views::components::logged_out_navbar;
use crate::views::components::main_footer;
use maud::{Markup, html};

pub fn logged_out_layout(children: Markup) -> Markup {
    html! {
        (logged_out_navbar());
        // Padding for the navbar and footer
        div class="px-4 pt-14 sm:pt-20 pb-14 sm:pb-18" {
            (children);
        }
        (main_footer());
    }
}
