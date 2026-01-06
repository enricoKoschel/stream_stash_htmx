use crate::views::components::image_with_fallback;
use maud::{Markup, html};

pub fn media_card(
    title: &str,
    year: &str,
    poster_url: Option<&str>,
    media_page_url: &str,
) -> Markup {
    // Sets the poster url to "" if it is None, this causes the backup image to be shown
    let poster_url = poster_url.unwrap_or_default();

    html! {
        a class="relative group not-pointer-fine:hidden" href=(media_page_url) {
            (image_with_fallback("aspect-2/3 rounded-lg brightness-100 group-hover:brightness-28 transition-[filter]", poster_url));

            div class="absolute inset-0 m-4 flex flex-col justify-center opacity-0 group-hover:opacity-100 transition-[opacity]" {
                h2 class="text-lg font-medium text-center mb-3 line-clamp-5 text-ellipsis wrap-anywhere" { (title); }
                p class="text-md text-center" { (year); }
            }
        }
        a class="pointer-fine:hidden bg-gray-700 rounded-lg flex flex-col" href=(media_page_url) {
            div class="flex-1 flex flex-col justify-center m-2" {
                h2 class="text-md font-medium text-center mb-3 line-clamp-3 text-ellipsis wrap-anywhere" { (title); }
                p class="text-sm text-center" { (year); }
            }

            (image_with_fallback("aspect-2/3 rounded-b-lg", poster_url));
        }
    }
}
