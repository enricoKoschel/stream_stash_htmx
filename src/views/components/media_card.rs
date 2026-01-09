use crate::views::components::image_with_fallback;
use maud::{Markup, html};

pub fn media_card(
    title: &str,
    year: &str,
    poster_url: Option<&str>,
    media_page_url: &str,
    next_page_url: Option<&str>,
) -> Markup {
    let children = html! {
        a class="relative group not-pointer-fine:hidden" href=(media_page_url) {
            (image_with_fallback("rounded-lg brightness-100 group-hover:brightness-28 transition-[filter]", "aspect-2/3", poster_url));

            div class="absolute inset-0 m-4 flex flex-col justify-center opacity-0 group-hover:opacity-100 transition-[opacity]" {
                h2 class="text-lg font-medium text-center mb-3 line-clamp-5 text-ellipsis wrap-anywhere" { (title); }
                p class="text-md text-center" { (year); }
            }
        }
        a class="pointer-fine:hidden h-full bg-gray-700 rounded-lg flex flex-col" href=(media_page_url) {
            div class="flex-1 flex flex-col justify-center m-2" {
                h2 class="text-md font-medium text-center mb-3 line-clamp-3 text-ellipsis wrap-anywhere" { (title); }
                p class="text-sm text-center" { (year); }
            }

            (image_with_fallback("rounded-b-lg", "aspect-2/3", poster_url));
        }
    };

    html! {
        @if let Some(next_page_url) = next_page_url {
            div hx-get=(next_page_url) hx-trigger="revealed" hx-swap="afterend" {
                (children)
            }
        } @else {
            div {
                (children)
            }
        }
    }
}
