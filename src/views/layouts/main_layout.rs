use crate::views::components::MainNavbar;
use hypertext::prelude::*;

#[component]
pub fn main_layout<R: Renderable>(children: &R) -> impl Renderable {
    maud! {
        MainNavbar;
        (children)
    }
}
