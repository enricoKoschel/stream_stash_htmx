use crate::views::layouts::logged_out_layout;
use maud::{Markup, html};

pub fn about_page() -> Markup {
    logged_out_layout(html! {
        div class="flex flex-col gap-2 items-center" {
            h1 class="text-6xl font-medium" { "About"; }
            div class="flex flex-col gap-8 items-center" {
                img class="w-100" src="/static/logos/StreamStashWithTextWhite.svg";
                div class="flex items-center whitespace-pre-wrap" {
                    p {
                        "This product uses the ";
                        a class="link" href="https://themoviedb.org" target="_blank" { "TMDB"; }
                        " API, but is not endorsed or certified by ";
                        a class="link" href="https://themoviedb.org" target="_blank" { "TMDB"; }
                        ".";
                    }
                    img class="w-20 ml-4" src="/static/logos/TMDB.svg";
                }
                div class="flex flex-col items-center gap-1" {
                    p class="text-sm text-base-content/70" {
                        "Built on " (env!("BUILD_TIMESTAMP"));
                    }
                    p class="text-xs text-base-content/60 font-mono" {
                        "Commit: " (env!("GIT_HASH"));
                    }
                }
            }
        }
    })
}
