use hypertext::prelude::*;

#[component]
pub fn base_layout<R: Renderable>(children: &R) -> impl Renderable {
    maud! {
        !DOCTYPE
        html {
            head {
                script src="/static/htmx.min.js" {}
                link rel="stylesheet" href="/static/styles.css";

                meta name="viewport"
                    content="width=device-width, height=device-height, minimum-scale=1.0, initial-scale=1.0, maximum-scale=1.0, user-scalable=no";

                link rel="icon" type="image/png" sizes="128x128" href="/static/icons/favicon-128x128.png";
                link rel="icon" type="image/png" sizes="96x96" href="/static/icons/favicon-96x96.png";
                link rel="icon" type="image/png" sizes="32x32" href="/static/icons/favicon-32x32.png";
                link rel="icon" type="image/png" sizes="16x16" href="/static/icons/favicon-16x16.png";
                link rel="icon" type="image/svg" href="/static/logos/StreamStashNoText.svg";
                link rel="icon" type="image/ico" href="/static/favicon.ico";

                title { "Stream Stash" }
            }
            body hx-boost="true" {
                (children)
            }
        }
    }
}
