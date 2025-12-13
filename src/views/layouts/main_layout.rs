use hypertext::prelude::*;

#[component]
pub fn main_layout<R: Renderable>(children: &R) -> impl Renderable {
    maud! {
        header {
            p { "Test" }
        }
        (children)
    }
}
