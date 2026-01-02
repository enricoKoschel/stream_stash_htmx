use crate::views::components::MainFooter;
use crate::views::components::MainNavbar;
use hypertext::prelude::*;

#[component]
pub fn main_layout<R: Renderable>(children: &R) -> impl Renderable {
    maud! {
        MainNavbar;
        // Margin on the bottom for the footer
        div class="p-4 mb-14" {
            (children)
        }
        // TODO: Looks bad on mobile
        MainFooter;
    }
}
