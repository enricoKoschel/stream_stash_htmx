use crate::views::components::SimpleComponent;
use crate::views::layouts::MainLayout;
use hypertext::prelude::*;

pub fn main_page() -> impl Renderable {
    maud! {
        MainLayout {
            SimpleComponent;
        }
    }
}
