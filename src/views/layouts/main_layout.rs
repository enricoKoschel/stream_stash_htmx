use crate::views::components::MainFooter;
use crate::views::components::MainNavbar;
use hypertext::prelude::*;

#[component]
// TODO: Actual search query struct
pub fn main_layout<'a, R: Renderable>(
    search_query: (&'a str, &'a str),
    children: &R,
) -> impl Renderable {
    maud! {
        MainNavbar search_query=search_query;
        // Margin on the bottom for the footer
        div class="p-4 mb-14" {
            (children)
        }
        // TODO: Looks bad on mobile
        MainFooter;
    }
}
