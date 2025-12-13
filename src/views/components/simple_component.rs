use hypertext::prelude::*;

#[component]
pub fn simple_component() -> impl Renderable {
    maud! {
        p.text-xl { "Press here" }
        button .btn .btn-primary .btn-xl hx-get="/test" hx-target="body" { "Button" }
    }
}

pub fn simple_component2() -> impl Renderable {
    maud! {
        p.text-xl { "Press here 2" }
        button .btn .btn-primary .btn-xl hx-get="/" hx-target="body" { "Button" }
    }
}
