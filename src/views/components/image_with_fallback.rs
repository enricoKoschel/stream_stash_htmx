use maud::{Markup, html};

pub fn image_with_fallback(class: &str, src: &str) -> Markup {
    html! {
        div {
            img class=(class) loading="lazy" src=(src) onerror="this.onerror=false; this.classList.add('hidden'); this.nextElementSibling.classList.remove('hidden');";
            img class={ "hidden border " (class) } loading="lazy" src="/static/heroicons/no-symbol-solid-white.svg";
        }
    }
}
