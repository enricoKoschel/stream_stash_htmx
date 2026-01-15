use maud::{Markup, html};

pub fn main_footer() -> Markup {
    // TODO: Looks bad on mobile
    html! {
        footer class="flex justify-end fixed bottom-0 w-full bg-base-200 shadow-md p-2 sm:p-4" {
            a class="link" href="/about" { "About" }
        }
    }
}
