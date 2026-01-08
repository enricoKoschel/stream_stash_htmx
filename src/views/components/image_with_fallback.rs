use maud::{Markup, html};

pub fn image_with_fallback(class: &str, src: Option<&str>) -> Markup {
    match src {
        Some(src) => html! {
            img class=(class) loading="lazy" src=(src) onerror="this.onerror=false; this.classList.add('hidden'); this.nextElementSibling.classList.remove('hidden');";
            img class={ "hidden border " (class) } loading="lazy" src="/static/heroicons/no-symbol-solid-white.svg";
        },
        None => html! {
            img class={ "border " (class) } loading="lazy" src="/static/heroicons/no-symbol-solid-white.svg";
        },
    }
}
