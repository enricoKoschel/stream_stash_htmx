use maud::{Markup, PreEscaped, html};

pub fn error_modal() -> Markup {
    html! {
        dialog id="error-modal" class="modal" {
            div class="modal-box flex flex-col gap-4" {
                h3 class="font-bold text-2xl text-error" { "Error"; }
                p id="error-modal-message" class="text-lg" {
                    "An error occurred while processing your request.";
                }
                div class="modal-action" {
                    button class="btn btn-primary w-full sm:text-lg" onclick="document.getElementById('error-modal').close()" {
                        "Close";
                    }
                }
            }
            // Hidden form, closes the dialog when pressing outside it
            form method="dialog" class="modal-backdrop" {
                button {}
            }
        }

        script {
            (PreEscaped(r#"
                document.body.addEventListener('htmx:responseError', function(evt) {
                    const statusCode = evt.detail.xhr.status;
                    const statusText = evt.detail.xhr.statusText;
                    let errorMessage = 'An error occurred while processing your request.';

                    // Customize error message based on status code
                    if (statusCode === 400) {
                        errorMessage = 'Bad request. Please check your input and try again.';
                    } else if (statusCode === 401) {
                        errorMessage = 'You are not authorized. Please log in and try again.';
                    } else if (statusCode === 403) {
                        errorMessage = 'Access forbidden. You do not have permission to perform this action.';
                    } else if (statusCode === 404) {
                        errorMessage = 'The requested resource was not found.';
                    } else if (statusCode === 429) {
                        errorMessage = 'Too many requests. Please wait a moment and try again.';
                    } else if (statusCode >= 500) {
                        errorMessage = 'A server error occurred. Please try again later.';
                    } else if (statusCode >= 400) {
                        errorMessage = `Error ${statusCode}: ${statusText || 'An error occurred'}`;
                    }

                    // Try to get more specific error message from response if available
                    try {
                        const responseText = evt.detail.xhr.responseText;
                        if (responseText && responseText.length > 0 && responseText.length < 200) {
                            // Use response text if it's not too long and not HTML
                            if (!responseText.trim().startsWith('<')) {
                                errorMessage = responseText;
                            }
                        }
                    } catch (e) {
                        // Keep default message if parsing fails
                    }

                    document.getElementById('error-modal-message').textContent = errorMessage;
                    document.getElementById('error-modal').showModal();
                });
            "#));
        }
    }
}
