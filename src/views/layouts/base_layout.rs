use crate::views::components::error_modal;
use maud::{DOCTYPE, Markup, html};

pub fn base_layout(google_client_id: &str, login_redirect_url: &str, children: Markup) -> Markup {
    html! {
        (DOCTYPE);
        html {
            head {
                script src="/static/htmx.min.js" async {}
                link rel="stylesheet" href="/static/styles.css";

                meta name="viewport" content="width=device-width, height=device-height, minimum-scale=1.0, initial-scale=1.0, maximum-scale=1.0, user-scalable=no";

                link rel="icon" type="image/png" sizes="128x128" href="/static/icons/favicon-128x128.png";
                link rel="icon" type="image/png" sizes="96x96" href="/static/icons/favicon-96x96.png";
                link rel="icon" type="image/png" sizes="32x32" href="/static/icons/favicon-32x32.png";
                link rel="icon" type="image/png" sizes="16x16" href="/static/icons/favicon-16x16.png";
                link rel="icon" type="image/svg" href="/static/logos/StreamStashNoText.svg";
                link rel="icon" type="image/ico" href="/static/favicon.ico";

                title { "Stream Stash" }

                // Has to be at the bottom and whithout async or defer
                // Otherwise a red background appears while loading
                script src="https://accounts.google.com/gsi/client" {}
            }
            body hx-boost="true" {
                div id="g_id_onload" class="hidden"
                    data-client_id=(google_client_id)
                    data-context="signin"
                    data-ux_mode="popup"
                    data-login_uri=(login_redirect_url)
                    data-auto_prompt="false" {}

                noscript class="text-6xl flex justify-center items-center w-screen h-screen text-center" {
                    "This site requires JavaScript to function. Please enable JavaScript in your browser settings.";
                }
                div class="noscript:hidden" {
                    (children);
                }

                (error_modal());
            }
        }
    }
}
