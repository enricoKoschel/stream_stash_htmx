use crate::views::layouts::logged_out_layout;
use maud::{Markup, html};

pub fn privacy_page() -> Markup {
    logged_out_layout(html! {
        div class="max-w-4xl flex flex-col mx-auto gap-8" {
            h1 class="text-4xl font-bold" { "Privacy Policy"; }
            p class="text-base-content/70" { "Last updated: January 2026"; }

            section class="flex flex-col gap-4" {
                h2 class="text-2xl font-semibold" { "1. Introduction"; }
                p class="text-base-content/80" {
                    "Welcome to Stream Stash. We respect your privacy and are committed to protecting your personal data. ";
                    "This privacy policy explains how we collect, use, and safeguard your information when you use our service.";
                }
            }

            section class="flex flex-col gap-4" {
                h2 class="text-2xl font-semibold" { "2. Information We Collect"; }

                div class="flex flex-col gap-2" {
                    h3 class="text-xl font-medium" { "2.1 Account Information"; }
                    p class="text-base-content/80" {
                        "When you sign in with Google, we receive and store:";
                    }
                    ul class="list-disc list-inside text-base-content/80 ml-4" {
                        li { "Your Google account identifier (unique ID)"; }
                        li { "Your email address"; }
                        li { "Your name (if provided)"; }
                        li { "Your profile picture URL (if provided)"; }
                    }
                }

                div class="flex flex-col gap-2" {
                    h3 class="text-xl font-medium" { "2.2 Usage Data"; }
                    p class="text-base-content/80" {
                        "We collect information about how you interact with our service, including:";
                    }
                    ul class="list-disc list-inside text-base-content/80 ml-4" {
                        li { "Movies and TV shows you search for"; }
                        li { "Media items you add to your watchlist or mark as watched"; }
                        li { "Information you add to these media items, such as rating and watch history"; }
                        li { "Your preferences and settings"; }
                    }
                }
            }

            section class="flex flex-col gap-4" {
                h2 class="text-2xl font-semibold" { "3. How We Use Your Information"; }
                p class="text-base-content/80" {
                    "We use the collected information to:";
                }
                ul class="list-disc list-inside text-base-content/80 ml-4" {
                    li { "Provide and maintain your account"; }
                    li { "Save your watchlist and viewing history"; }
                    li { "Personalize your experience"; }
                    li { "Improve our service"; }
                }
            }

            section class="flex flex-col gap-4" {
                h2 class="text-2xl font-semibold" { "4. Third-Party Services"; }

                div class="flex flex-col gap-2" {
                    h3 class="text-xl font-medium" { "4.1 Google Sign-In"; }
                    p class="text-base-content/80" {
                        "We use Google Sign-In for authentication. When you sign in, Google's privacy policy applies to the data they collect. ";
                        "You can review Google's privacy policy at ";
                        a class="link" href="https://policies.google.com/privacy" target="_blank" { "policies.google.com/privacy"; }
                        ".";
                    }
                }

                div class="flex flex-col gap-2" {
                    h3 class="text-xl font-medium" { "4.2 The Movie Database (TMDB)"; }
                    p class="text-base-content/80" {
                        "We use The Movie Database (TMDB) API to provide movie and TV show information. ";
                        "This product uses the TMDB API but is not endorsed or certified by TMDB. ";
                        "TMDB's privacy policy can be found at ";
                        a class="link" href="https://www.themoviedb.org/privacy-policy" target="_blank" { "themoviedb.org/privacy-policy"; }
                        ".";
                    }
                }
            }

            section class="flex flex-col gap-4" {
                h2 class="text-2xl font-semibold" { "5. Data Storage and Security"; }
                p class="text-base-content/80" {
                    "Your data is stored securely and we implement appropriate technical measures to protect your personal information. ";
                    "However, no method of transmission over the Internet is 100% secure, and we cannot guarantee absolute security.";
                }
            }

            section class="flex flex-col gap-4" {
                h2 class="text-2xl font-semibold" { "6. Data Retention"; }
                p class="text-base-content/80" {
                    "We retain your personal data for as long as your account is active or as needed to provide you with our services. ";
                    "You may request deletion of your account and associated data at any time.";
                }
            }

            section class="flex flex-col gap-4" {
                h2 class="text-2xl font-semibold" { "7. Your Rights"; }
                p class="text-base-content/80" {
                    "You have the right to:";
                }
                ul class="list-disc list-inside text-base-content/80 ml-4" {
                    li { "Access your personal data"; }
                    li { "Request correction of your data"; }
                    li { "Request deletion of your account and data"; }
                    li { "Withdraw consent at any time"; }
                }
            }

            section class="flex flex-col gap-4" {
                h2 class="text-2xl font-semibold" { "8. Cookies"; }
                p class="text-base-content/80" {
                    "We use essential cookies to maintain your session and authentication state. ";
                    "These cookies are necessary for the service to function properly.";
                }
            }

            section class="flex flex-col gap-4" {
                h2 class="text-2xl font-semibold" { "9. Changes to This Policy"; }
                p class="text-base-content/80" {
                    "We may update this privacy policy from time to time. ";
                    "We will post the new policy on this page and update the \"Last updated\" date.";
                }
            }

            section class="flex flex-col gap-4" {
                h2 class="text-2xl font-semibold" { "10. Contact Us"; }
                p class="text-base-content/80" {
                    "If you have any questions about this privacy policy, please contact us at ";
                    a class="link" href="mailto:streamstashapp@gmail.com" target="_blank" { "streamstashapp@gmail.com"; }
                    ".";
                }
            }
        }
    })
}
