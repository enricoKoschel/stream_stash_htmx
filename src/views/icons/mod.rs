use maud::{Markup, html};

mod bars_3_solid;
mod calendar_solid;
mod home_solid;
mod no_symbol_solid;

pub use bars_3_solid::bars_3_solid;
pub use calendar_solid::calendar_solid;
pub use home_solid::home_solid;

pub use no_symbol_solid::no_symbol_solid;

pub fn chevron_double_up_solid(class: &str) -> Markup {
    html! {
        svg class=(class) xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" {
            path fill-rule="evenodd" d="M11.47 4.72a.75.75 0 0 1 1.06 0l7.5 7.5a.75.75 0 1 1-1.06 1.06L12 6.31l-6.97 6.97a.75.75 0 0 1-1.06-1.06l7.5-7.5Zm.53 7.59-6.97 6.97a.75.75 0 0 1-1.06-1.06l7.5-7.5a.75.75 0 0 1 1.06 0l7.5 7.5a.75.75 0 1 1-1.06 1.06L12 12.31Z" clip-rule="evenodd" {}
        }
    }
}
