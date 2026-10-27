use maud::{Markup, html};

pub fn revoke_failed_modal() -> Markup {
    html! {
        dialog id="revoke-failed-modal" class="modal" {
            div class="modal-box flex flex-col gap-4" {
                h3 class="font-bold text-2xl" { "Account deleted"; }
                p class="text-lg" {
                    "Your account has been deleted. We could not automatically remove
                    Stream Stash from the apps connected to your Google account.
                    You can remove it manually in your ";
                    a
                        class="link link-primary"
                        href="https://myaccount.google.com/connections"
                        target="_blank"
                        rel="noopener noreferrer"
                        hx-boost="false"
                    {
                        "Google account settings";
                    }
                    ".";
                }
                div class="modal-action" {
                    button
                        class="btn btn-primary w-full sm:text-lg"
                        onclick="window.location.assign('/')"
                    {
                        "Close";
                    }
                }
            }
        }
    }
}
