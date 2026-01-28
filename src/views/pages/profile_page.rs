use crate::views::{components::image_with_fallback, layouts::main_layout};
use maud::{Markup, html};

pub fn profile_page(
    email: Option<&str>,
    username: Option<&str>,
    picture_url: Option<&str>,
    created_at: &str,
) -> Markup {
    let email = email.unwrap_or("N/A");
    let username = username.unwrap_or("N/A");

    main_layout(html! {
        div class="flex flex-col items-center gap-6" {
            h1 class="text-6xl font-medium" { "Profile"; }
            // Padding for visual allignment
            div class="flex max-sm:flex-col max-sm:items-center gap-4 md:pl-25" {
                (image_with_fallback("size-25 rounded-full", "aspect-square", picture_url));
                div class="flex flex-col justify-center gap-2 text-xl" {
                    "E-Mail: " (email);
                    br;
                    "Username: " (username);
                    br;
                    "Account created at: " (created_at);
                }
            }
            btn class="btn btn-error"
                onclick="document.getElementById('confirm-delete-input').value = ''; document.getElementById('delete-account-modal').showModal()"
            {
                "Delete account";
            }
            // TODO: Stats (how many of each media type in each state?), import/export data
        }

        // Confirm account deletion modal
        dialog id="delete-account-modal" class="modal" {
            div class="modal-box flex flex-col gap-8" {
                div class="flex flex-col gap-4 sm:gap-3 text-xl sm:text-2xl" {
                    p { "Are you sure you want to delete your account?"; }
                    p { "THIS ACTION CANNOT BE UNDONE!"; }
                    p { "Type DELETE to confirm."; }
                }
                form class="flex flex-col gap-2" hx-post="/deleteAccount"
                    hx-on:show-account-not-deleted-modal="document.getElementById('account-not-deleted-modal').showModal()"
                    hx-on:show-account-deletion-failed-modal="document.getElementById('account-deletion-failed-modal').showModal()"
                {
                    input id="confirm-delete-input" class="input outline-0 w-full" name="confirm" type="text" placeholder="DELETE";
                    div class="flex gap-2 w-full" {
                        div class="flex-2 hidden sm:block" {}
                        button class="flex-1 btn btn-primary" type="reset" onclick="document.getElementById('delete-account-modal').close()" {
                            "Cancel";
                        }
                        button class="flex-1 btn btn-error" type="submit" onclick="document.getElementById('delete-account-modal').close()" {
                            "Confirm";
                        }
                    }
                }
            }
            // Hidden form, closes the dialog when pressing outside it
            form method="dialog" class="modal-backdrop" {
                button {}
            }
        }

        // Account not deleted modal
        dialog id="account-not-deleted-modal" class="modal" {
            div class="modal-box flex flex-col gap-8" {
                p class="text-xl sm:text-2xl" {
                    "Your account was NOT deleted!";
                }
                button class="btn btn-primary" onclick="document.getElementById('account-not-deleted-modal').close()" {
                    "Okay";
                }
            }
            // Hidden form, closes the dialog when pressing outside it
            form method="dialog" class="modal-backdrop" {
                button {}
            }
        }

        // Account deletion failed modal
        dialog id="account-deletion-failed-modal" class="modal" {
            div class="modal-box flex flex-col gap-8" {
                p class="text-xl sm:text-2xl" {
                    "There was an error while trying to delete your account. Please try again.";
                }
                button class="btn btn-primary" onclick="document.getElementById('account-deletion-failed-modal').close()" {
                    "Okay";
                }
            }
            // Hidden form, closes the dialog when pressing outside it
            form method="dialog" class="modal-backdrop" {
                button {}
            }
        }
    })
}
