use crate::views::components::image_with_fallback;
use crate::views::layouts::main_layout;
use maud::{Markup, html};

// TODO: Media item as struct?
pub fn media_page(
    title: &str,
    overview: &str,
    release_date: &str,
    poster_url: Option<&str>,
    backdrop_url: Option<&str>,
) -> Markup {
    main_layout(
        None,
        html! {
            // top-16 because of the navbar
            // TODO: Fallback image looks bad here
            (image_with_fallback("fixed inset-0 top-16 w-full h-full -z-1 opacity-20 object-cover", "aspect-16/9", backdrop_url));

            div class="flex" {
                (image_with_fallback("", "aspect-2/3", poster_url));
                div class="text-3xl" { (title); }
                div class="text-3xl" { (overview); }
                div class="text-3xl" { (release_date); }
            }
        },
    )
}
