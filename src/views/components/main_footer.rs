use hypertext::prelude::*;

#[component]
pub fn main_footer() -> impl Renderable {
    maud! {
        footer class="sticky bottom-0 bg-base-200 shadow-md p-4 flex justify-end" {
                a class="link" href="/about" { "About" }
        }
    }
}
