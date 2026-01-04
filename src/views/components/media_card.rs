use hypertext::prelude::*;

#[component]
pub fn media_card<'a>(
    title: &'a str,
    year: &'a str,
    poster_url: &'a str,
    media_page_url: &'a str,
) -> impl Renderable {
    maud! {
        a class="relative group not-pointer-fine:hidden" href=media_page_url {
            img class="rounded-lg brightness-100 group-hover:brightness-28 transition-[filter]" src=poster_url;

            div class="absolute inset-0 m-4 flex flex-col justify-center opacity-0 group-hover:opacity-100 transition-[opacity]" {
                h2 class="text-lg font-medium text-center mb-3 line-clamp-5 text-ellipsis wrap-anywhere " { (title) }
                p class="text-md text-center" { (year) }
            }
        }
        a class="pointer-fine:hidden bg-gray-700 rounded-lg flex flex-col" href=media_page_url {
            div class="flex-1 flex flex-col justify-center m-2" {
                h2 class="text-md font-medium text-center mb-3 line-clamp-3 text-ellipsis wrap-anywhere" { (title) }
                p class="text-sm text-center" { (year) }
            }

            img class="rounded-b-lg" src=poster_url;
        }
    }
}
