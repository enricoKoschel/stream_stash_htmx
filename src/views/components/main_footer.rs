use hypertext::prelude::*;

#[component]
pub fn main_footer() -> impl Renderable {
    maud! {
        footer class="fixed bottom-0 w-full bg-base-200 shadow-md p-4 flex justify-end" {
            a class="link" href="/about" { "About" }
        }
    }
}
