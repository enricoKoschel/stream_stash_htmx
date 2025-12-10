use hypertext::prelude::*;

#[component]
pub fn main_page<R: Renderable>(children: &R) -> impl Renderable {
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

#[component]
pub fn simple_component() -> impl Renderable {
    maud! {
        p.text-xl { "Press here" }
        button .btn .btn-primary .btn-xl { "Button" }
    }
}
