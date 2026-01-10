use crate::data_source::MEDIA_TYPES;
use crate::data_source::search::SearchQuery;
use maud::{Markup, html};

pub fn main_navbar(search_query: Option<&SearchQuery>) -> Markup {
    html! {
        div class="navbar sticky top-0 z-1 bg-base-200 shadow-md flex" {
            div class="flex-1 flex gap-2" {
                a class="btn btn-square btn-ghost text-xl" href="/" {
                    img class="size-8" src="/static/heroicons/home-solid-white.svg";
                }
                img class="hidden lg:block w-[min(20vw,24rem)]" src="/static/logos/StreamStashWithTextWhite.svg";
                img class="w-8 hidden sm:max-lg:block" src="/static/logos/StreamStashNoText.svg";
            }
            form class="sm:flex-1 flex gap-1 max-sm:flex-col max-sm:w-80 max-sm:min-w-26 max-sm:mx-4 justify-center" hx-get="/search" hx-target="body" hx-push-url="true" {
                input class="sm:flex-1 input min-h-10 outline-0 sm:min-w-60 max-sm:w-full" name="q" type="text" placeholder="Search" value=[search_query.map(|s| &s.q)];
                select class="select outline-0 w-full sm:w-26 cursor-pointer" name="t" {
                    @for media_type in MEDIA_TYPES {
                        option selected[search_query.is_some_and(|s| s.t == media_type)] { (media_type) }
                    }
                }
                input class="btn btn-primary btn-soft" type="submit" value="Search";
            }
            div class="flex-1 flex justify-end" {
                button class="btn btn-square btn-ghost" {
                    img class="size-8" src="/static/heroicons/bars-4-solid-white.svg";
                }
            }
        }
    }
}
