use crate::views::components::{error_modal, revoke_failed_modal};
use maud::{DOCTYPE, Markup, PreEscaped, html};

const GOOGLE_SIGN_IN_JS: &str = r#"
let googleLoginState = "";

async function initGoogleSignIn() {
    const clientId = document.querySelector('meta[name="google-client-id"]').content;
    const nonceRes = await fetch("/login/nonce", { method: "POST" });
    if (!nonceRes.ok) {
        console.error("Failed to fetch login nonce:", nonceRes.status);
        return;
    }

    const { nonce, state } = await nonceRes.json();
    googleLoginState = state;

    google.accounts.id.initialize({
        client_id: clientId,
        ux_mode: "popup",
        auto_select: false,
        nonce,
        callback: onGoogleCredential,
    });
    renderGoogleButtons();
}

function renderGoogleButtons() {
    document.querySelectorAll(".g_id_signin").forEach((el) => {
        google.accounts.id.renderButton(el, {
            type: "standard",
            shape: "rectangular",
            theme: "outline_dark",
            size: "large",
            state: googleLoginState,
            text: "singin_with",
        });
    });
}

async function onGoogleCredential(response) {
    try {
        const res = await fetch("/login", {
            method: "POST",
            headers: { "Content-Type": "application/x-www-form-urlencoded" },
            body: new URLSearchParams({
                credential: response.credential,
                state: response.state ?? "",
            }),
        });
        if (res.ok) {
            window.location.assign("/");
            return;
        }
        console.error("Login failed:", res.status, await res.text());
    } catch (e) {
        console.error("Login failed:", e);
    }
    alert("Login failed. Please try again.");
    // The nonce is single-use, so request a fresh one
    initGoogleSignIn();
}

document.addEventListener("account-deleted", (event) => {
    const id = event.detail.googleAccountId;
    const reload = () => window.location.assign("/");
    if (!id || !window.google?.accounts?.id) {
        reload();
        return;
    }

    // Revoking fails if the browser has no usable Google session (e.g. blocked third-party
    // cookies), so the user is pointed to their Google account to remove the app manually
    let finished = false;
    const finish = (successful) => {
        if (finished) return;
        finished = true;
        if (successful) {
            reload();
        } else {
            document.getElementById("revoke-failed-modal").showModal();
        }
    };

    setTimeout(() => finish(false), 5000);
    google.accounts.id.revoke(id, (result) => {
        if (!result.successful) console.error("Google revoke failed:", result.error);
        finish(result.successful);
    });
});

window.onGoogleLibraryLoad = initGoogleSignIn;
document.addEventListener("htmx:afterSettle", () => {
    if (window.google?.accounts?.id) renderGoogleButtons();
});
"#;

pub fn page_title(title: Option<&str>) -> Markup {
    html! {
        @if let Some(title) = title {
            title { (title) " - Stream Stash"; }
        } @else {
            title { "Stream Stash"; }
        }
    }
}

pub fn base_layout(title: Option<&str>, google_client_id: &str, children: Markup) -> Markup {
    html! {
        (DOCTYPE);
        html {
            head {
                script src="/static/htmx.min.js" {}
                link rel="stylesheet" href="/static/styles.css";

                meta
                    name="viewport"
                    content="width=device-width,
                             height=device-height,
                             initial-scale=1.0";

                link
                    rel="icon"
                    type="image/png"
                    sizes="128x128"
                    href="/static/icons/favicon-128x128.png";
                link
                    rel="icon"
                    type="image/png"
                    sizes="96x96"
                    href="/static/icons/favicon-96x96.png";
                link
                    rel="icon"
                    type="image/png"
                    sizes="32x32"
                    href="/static/icons/favicon-32x32.png";
                link
                    rel="icon"
                    type="image/png"
                    sizes="16x16"
                    href="/static/icons/favicon-16x16.png";
                link
                    rel="icon"
                    type="image/svg"
                    href="/static/logos/StreamStashNoText.svg";
                link
                    rel="icon"
                    type="image/ico"
                    href="/static/favicon.ico";

                meta name="google-client-id" content=(google_client_id);

                (page_title(title));

                // Has to be at the bottom and without async or defer
                // Otherwise a red background appears while loading
                script { (PreEscaped(GOOGLE_SIGN_IN_JS)) }
                script src="https://accounts.google.com/gsi/client" {}
            }
            body hx-boost="true" hx-target="#main-content" {
                noscript class="text-6xl flex justify-center items-center
                                w-screen h-screen text-center"
                {
                    "This site requires JavaScript to function.
                    Please enable JavaScript in your browser settings.";
                }
                div id="main-content" class="noscript:hidden" {
                    (children);
                }

                (error_modal());
                (revoke_failed_modal());
            }
        }
    }
}
