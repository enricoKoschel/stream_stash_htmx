use hypertext::prelude::*;

#[component]
pub fn main_navbar() -> impl Renderable {
    maud! {
        div class="navbar bg-base-200 shadow-md grid grid-cols-3" {
            div class="flex" {
                // TODO: Link to home (full reload?)
                a class="btn btn-ghost text-xl mr-2" {
                    img class="size-8" src="static/heroicons/home-solid-white.svg";
                }
                img class="md:w-40 lg:w-3xs xl:w-xs hidden md:block" src="static/logos/StreamStashWithTextWhite.svg";
                img class="w-8 hidden sm:max-md:block" src="static/logos/StreamStashNoText.svg";
            }
            div class="flex justify-center" {
                input class="input input-bordered w-xl min-w-20" type="text" placeholder="Search";
            }
            div class="flex justify-end text-end" {
                button class="btn btn-square btn-ghost" {
                    img class="size-8" src="static/heroicons/bars-4-solid-white.svg";
                }
            }
        }
    }
}
