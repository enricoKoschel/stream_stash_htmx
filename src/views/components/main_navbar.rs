use hypertext::prelude::*;

#[component]
pub fn main_navbar() -> impl Renderable {
    maud! {
        div class="navbar sticky top-0 z-1 bg-base-200 shadow-md flex" {
            div class="flex-1 flex gap-2" {
                a class="btn btn-square btn-ghost text-xl" href="/" {
                    img class="size-8" src="static/heroicons/home-solid-white.svg";
                }
                img class="hidden lg:block w-[min(20vw,24rem)]" src="static/logos/StreamStashWithTextWhite.svg";
                img class="w-8 hidden sm:max-lg:block" src="static/logos/StreamStashNoText.svg";
            }
            form class="sm:flex-1 flex gap-1 max-sm:flex-col max-sm:w-80 max-sm:min-w-26 max-sm:mx-4 justify-center" hx-get="/search" hx-target="body" hx-push-url="true" {
                input class="sm:flex-1 input min-h-10 outline-0 sm:min-w-60 max-sm:w-full" name="q" type="text" placeholder="Search";
                select class="select outline-0 w-full sm:w-26 cursor-pointer" name="t" {
                    option { "Movies" }
                    option { "TV Shows" }
                }
                input class="btn btn-primary btn-soft" type="submit" value="Search";
            }
            div class="flex-1 flex justify-end" {
                button class="btn btn-square btn-ghost" {
                    img class="size-8" src="static/heroicons/bars-4-solid-white.svg";
                }
            }
        }
    }
}
