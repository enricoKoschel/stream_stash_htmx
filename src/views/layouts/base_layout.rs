use hypertext::prelude::*;

#[component]
pub fn base_layout<R: Renderable>(children: &R) -> impl Renderable {
    maud! {
        !DOCTYPE
        html {
            head {
                script src="static/htmx.min.js" {}
                link rel="stylesheet" href="static/styles.css";

                title { "Stream Stash" }
            }
            body {
                (children)
            }
        }
    }
}
