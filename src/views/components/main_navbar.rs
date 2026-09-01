use crate::data_source::MEDIA_TYPES;
use crate::data_source::search::SearchQuery;
use crate::views::icons::{bars_3_solid, home_solid, magnifying_glass_solid, x_mark_solid};
use maud::{Markup, html};

fn dropdown(button_class: &str, button_size: &str) -> Markup {
    html! {
        details class="dropdown dropdown-end" {
            summary class={"btn btn-square btn-ghost " (button_class)} {
                (bars_3_solid(button_size));
            }
            ul class="menu dropdown-content bg-base-100 rounded-box z-1 w-52 p-2 shadow-sm sm:text-lg" {
                li { a href="/profile" { "Profile"; } }
                div class="divider m-0 px-2" {}
                li { a href="/logout" { "Logout"; } }
            }
        }
    }
}

pub fn main_navbar(search_query: Option<&SearchQuery>) -> Markup {
    // Necessary to avoid double shadow when search result count bar is present
    let shadow_class = if search_query.is_none() {
        "shadow-md"
    } else {
        ""
    };

    html! {
        // Mobile navbar
        div class={"sm:hidden fixed top-0 w-full z-2 bg-base-200 flex justify-between items-center px-2 py-1 " (shadow_class)} {
            div class="flex-1" {
                a class="btn btn-square btn-ghost btn-sm" href="/" {
                    (home_solid("size-6"));
                }
            }
            div class="flex-1 flex justify-center" {
                img class="w-6" src="/static/logos/StreamStashNoText.svg";
            }
            div class="flex-1 flex justify-end gap-1" {
                button class="btn btn-square btn-ghost btn-sm" onclick="document.getElementById('mobile-search-modal').showModal()" {
                    (magnifying_glass_solid("size-6"));
                }
                (dropdown("btn-sm", "size-6"));
            }
        }

        // Mobile search modal
        dialog id="mobile-search-modal" class="modal modal-top sm:hidden" {
            div class="modal-box flex flex-col gap-4" {
                div class="flex justify-between" {
                    h3 class="font-bold text-lg" { "Search"; }
                    form method="dialog" {
                        button class="btn btn-square btn-ghost btn-sm" {
                            (x_mark_solid("size-6"));
                        }
                    }
                }
                form class="flex flex-col gap-2" hx-get="/search" hx-push-url="true" autocomplete="off" {
                    input class="input w-full outline-0" name="q" type="text" placeholder="Search..." value=[search_query.map(|s| &s.q)];
                    select class="select w-full cursor-pointer outline-0" name="t" {
                        @for media_type in MEDIA_TYPES {
                            option selected[search_query.is_some_and(|s| s.t == media_type)] { (media_type); }
                        }
                    }
                    button class="btn btn-primary w-full" type="submit" { "Search"; }
                }
            }
            // Hidden form, closes the dialog when pressing outside it
            form method="dialog" class="modal-backdrop" {
                button {}
            }
        }

        // Desktop navbar
        div class={"hidden sm:flex navbar fixed top-0 z-2 bg-base-200 " (shadow_class)} {
            div class="flex-1 flex gap-2" {
                a class="btn btn-square btn-ghost text-xl" href="/" {
                    (home_solid("size-8"));
                }
                img class="hidden lg:block w-[min(20vw,24rem)]" src="/static/logos/StreamStashWithTextWhite.svg";
                img class="w-8 hidden max-lg:block" src="/static/logos/StreamStashNoText.svg";
            }
            form class="flex-1 flex gap-2 justify-center" hx-get="/search" hx-push-url="true" autocomplete="off" {
                div class="flex-1 flex join" {
                    input class="flex-1 input outline-0 min-w-60 join-item" name="q" type="text" placeholder="Search" value=[search_query.map(|s| &s.q)];
                    select class="select outline-0 w-30 cursor-pointer join-item" name="t" {
                        @for media_type in MEDIA_TYPES {
                            option selected[search_query.is_some_and(|s| s.t == media_type)] { (media_type); }
                        }
                    }
                }

                input class="btn btn-primary btn-soft" type="submit" value="Search";
            }
            div class="flex-1 flex justify-end" {
                (dropdown("btn-md", "size-8"));
            }
        }
    }
}
