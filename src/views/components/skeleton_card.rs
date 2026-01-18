use maud::{Markup, html};

pub fn skeleton_card() -> Markup {
    html! {
        div {
            div class="not-pointer-fine:hidden" {
                div class="skeleton rounded-lg aspect-2/3" {}
            }
            div class="pointer-fine:hidden h-full bg-gray-700 rounded-lg flex flex-col" {
                div class="flex-1 flex flex-col justify-center items-center m-2 gap-2" {
                    div class="w-full flex flex-col items-center gap-1" {
                        div class="skeleton h-4 w-4/5" {}
                        div class="skeleton h-4 w-4/5" {}
                    }
                    div class="skeleton h-3 w-1/4" {}
                }
                div class="skeleton rounded-b-lg rounded-t-none aspect-2/3" {}
            }
        }
    }
}
