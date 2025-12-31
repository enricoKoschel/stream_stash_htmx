use hypertext::prelude::*;

#[component]
pub fn media_card<'a>(
    title: &'a str,
    year: &'a str,
    poster_url: &'a str,
    media_page_url: &'a str,
) -> impl Renderable {
    maud! {
        a class="relative w-50 group" href=media_page_url {
            img class="rounded-lg brightness-100 group-hover:brightness-28 transition-[filter]" src=poster_url;

            div class="absolute inset-0 mx-3 flex flex-col justify-center opacity-0 group-hover:opacity-100 transition-[opacity]" {
                h2 class="text-lg font-medium text-white text-center mb-3" { (title) }
                p class="text-white text-md text-center" { (year) }
            }
        }
    }
}
