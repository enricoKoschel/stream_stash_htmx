use maud::{Markup, html};

pub fn main_footer() -> Markup {
    html! {
        footer class="flex justify-end gap-4 fixed bottom-0 w-full bg-base-200 shadow-md p-2 sm:p-4" {
            a class="link" href="/privacy" { "Privacy Policy" }
            a class="link" href="/about" { "About" }
        }
    }
}
