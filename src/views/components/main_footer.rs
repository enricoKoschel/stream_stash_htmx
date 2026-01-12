use maud::{Markup, html};

pub fn main_footer() -> Markup {
    // TODO: Looks bad on mobile
    html! {
        footer class="fixed bottom-0 w-full bg-base-200 shadow-md p-4 flex justify-end" {
            a class="link" href="/about" { "About" }
        }
    }
}
