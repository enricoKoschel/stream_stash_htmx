use crate::views::icons::no_symbol_solid;
use maud::{Markup, html};

pub fn image_with_fallback(class: &str, aspect_class: &str, src: Option<&str>) -> Markup {
    let class = format!("{class} {aspect_class}");

    match src {
        Some(src) => html! {
            img class=(class) referrerpolicy="no-referrer" loading="lazy" src=(src) onerror="this.onerror=false; this.classList.add('hidden'); this.nextElementSibling.classList.remove('hidden');";
            (no_symbol_solid(&format!("hidden border {class}")));
        },
        None => html! {
            (no_symbol_solid(&format!("border {class}")));
        },
    }
}
