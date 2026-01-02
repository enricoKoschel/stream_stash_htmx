use hypertext::prelude::*;

pub fn about_page() -> impl Renderable {
    maud! {
        div class="p-12 flex flex-col items-center" {
            h1 class="text-6xl font-medium mb-4" { "About" }
            img class="w-100 mb-12" src="static/logos/StreamStashWithTextWhite.svg";
            div class="flex items-center whitespace-pre-wrap mb-12" {
                p { "This product uses the " }
                a class="link" href="https://themoviedb.org" target="_blank" { "TMDB" }
                p { " API, but is not endorsed or certified by " }
                a class="link" href="https://themoviedb.org" target="_blank" { "TMDB" }
                p { "." }
                img class="w-20 ml-4" src="static/logos/TMDB.svg";
            }
            // TODO: Link to GitHub Repo here
            a class="btn text-xl" href="/" { "Go Home" }
        }
    }
}
