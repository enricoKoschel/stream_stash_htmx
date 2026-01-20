use maud::{Markup, html};

// TODO: Fix the button not loading after a boosted navigation to /login
pub fn google_login_button() -> Markup {
    // overflow-hidden, rounded-sm and -m-[2px] are needed to hide an ugly white border around the button
    html! {
        div class="overflow-hidden rounded-sm h-[40px]" {
            div class="g_id_signin -m-[2px]" data-type="standard" data-shape="rectangular" data-theme="filled_black" data-size="large" {
                p class="px-1 text-sm text-center text-base-content/50" {
                    "If you see this text, the Sign in with Google button failed to load.";
                    br;
                    "Reload the page to try again.";
                }
            }
        }
    }
}
