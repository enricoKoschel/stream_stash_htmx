use crate::views::icons::github_logo_solid;
use maud::{Markup, html};

pub fn main_footer() -> Markup {
    html! {
        footer class="flex justify-end gap-4 fixed bottom-0 w-full bg-base-200 shadow-md p-2 sm:p-4 text-sm sm:text-base" {
            a class="flex gap-2 link items-center" href="https://github.com/enricoKoschel/stream_stash_htmx" target="_blank" {
                (github_logo_solid("size-5 sm:size-6"));
                "View on GitHub";
            }
            a class="link" href="/privacy" { "Privacy Policy" }
            a class="link" href="/about" { "About" }
        }
    }
}
