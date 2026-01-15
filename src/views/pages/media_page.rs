use crate::views::layouts::main_layout;
use crate::views::{components::image_with_fallback, icons::calendar_solid};
use maud::{Markup, html};

// TODO: Media item as struct?
pub fn media_page(
    title: &str,
    overview: &str,
    release_date: &str,
    poster_url: Option<&str>,
    backdrop_url: Option<&str>,
) -> Markup {
    main_layout(html! {
        // top-16 because of the navbar
        // TODO: Fallback image looks bad here
        (image_with_fallback("fixed inset-0 top-34 sm:top-16 w-full h-full -z-1 opacity-20 object-cover", "aspect-16/9", backdrop_url));

        div class="flex flex-col md:flex-row justify-center pt-8 gap-6 md:gap-10 lg:gap-12" {
            div class="flex-shrink-0 mx-auto md:mx-0" {
                (image_with_fallback("w-48 sm:w-56 md:w-64 lg:w-72 rounded-lg shadow-2xl ring-1 ring-white/10", "aspect-2/3", poster_url));
            }

            div class="flex flex-col gap-4 md:gap-6 text-center max-md:items-center md:text-left" {
                h1 class="text-3xl sm:text-4xl lg:text-5xl font-bold max-w-4xl" {
                    (title);
                }

                div class="flex flex-col items-center md:items-start gap-2 w-fit" {
                    div class="flex items-center gap-2 justify-center md:justify-start text-base text-base-content/70" {
                        (calendar_solid("size-4"));
                        span { (release_date); }
                    }

                    div class="w-full h-1 bg-primary rounded-full" {}
                }

                div class="flex flex-col gap-2 max-w-4xl" {
                    h2 class="text-lg font-semibold text-base-content/90" { "Overview"; }
                    p class="text-base-content/80 text-sm sm:text-base lg:text-lg" {
                        (overview);
                    }
                }
            }
        }
    })
}
