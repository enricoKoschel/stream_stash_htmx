use crate::{
    data_source::{
        MediaType,
        db::{DATE_FORMAT, MediaHistoryEntry},
    },
    views::icons::{calendar_solid, trash_solid},
};
use maud::{Markup, PreEscaped, html};

pub fn media_history_entry(
    entry: &MediaHistoryEntry,
    history_url: &str,
    media_type: MediaType,
) -> Markup {
    let entry_url = format!("{}/{}", history_url, entry.id);
    let start_date = entry
        .start_date
        .map(|start_date| start_date.format(DATE_FORMAT).expect("Date should format"));
    let end_date = entry
        .end_date
        .map(|end_date| end_date.format(DATE_FORMAT).expect("Date should format"));

    html! {
        div id=(format!("watch-history-entry-{}", entry.id)) class="bg-base-200 shadow-md rounded-xl" {
            div class="flex flex-col p-4 gap-5" {
                form
                    class="flex flex-col gap-3"
                    hx-patch=(&entry_url)
                    hx-trigger="change"
                    autocomplete="off"
                {
                    div class="flex items-center" {
                        // Title
                        input
                            class="input input-lg input-ghost w-full outline-0"
                            type="text"
                            name="title"
                            value=[entry.title.as_ref()]
                            onkeydown="if (event.key === 'Enter') { event.preventDefault(); }"
                            placeholder="Entry title" {}

                        // Star rating
                        div class="rating rating-lg rating-half" {
                            @for rating in 1..=10 {
                                input
                                    class={
                                        "mask mask-star-2 bg-orange-400 "
                                        @if rating % 2 == 1 { "mask-half-1" } @else { "mask-half-2" }
                                    }
                                    type="radio"
                                    name="rating"
                                    value=(rating)
                                    checked[entry.rating == Some(rating)]
                                    onmousedown="this.dataset.wasChecked = this.checked"
                                    onclick="if (this.dataset.wasChecked === 'true') { this.checked = false; this.form.dispatchEvent(new Event('change', { bubbles: true })); }" {}
                            }
                        }
                    }

                    // Comment
                    textarea
                        class="textarea textarea-md outline-0 resize-none w-full min-h-auto"
                        name="comment"
                        rows="2"
                        placeholder="Entry comment"
                    {
                        @if let Some(comment) = entry.comment.as_ref() {
                            (comment);
                        }
                    }

                    // No `value` attribute on the date inputs on purpose:
                    // setting it also sets the input's defaultValue,
                    // which triggers a Safari on iOS bug where tapping
                    // the native clear button doesn't clear the field,
                    // but resets it to the value fields contents
                    div class="flex max-sm:flex-col max-sm:gap-3 sm:items-center justify-between" {
                        // Start date
                        div class="flex gap-3 w-50" {
                            label class="flex items-center gap-1 text-sm text-base-content/70" {
                                (calendar_solid("size-4"));
                                @match media_type {
                                    MediaType::TvShows => "Start",
                                    MediaType::Movies => "Watched",
                                }
                            }
                            input id=(format!("start-date-{}", entry.id)) class="input outline-0" type="date" name="start_date" {}
                        }

                        // End date
                        // Movies only have 1 date
                        div class={"flex gap-4.5 sm:gap-3 w-50 justify-start sm:justify-end " @if media_type == MediaType::Movies { "hidden"; }} {
                            label class="flex items-center gap-1 text-sm text-base-content/70" {
                                (calendar_solid("size-4"));
                                "End";
                            }
                            input id=(format!("end-date-{}", entry.id)) class="input outline-0" type="date" name="end_date" {}
                        }
                    }
                }

                // Delete button
                div {
                    button
                        type="button"
                        class="btn btn-error btn-md btn-soft w-full"
                        onclick=(format!(r#"
                            const btn = document.getElementById('confirm-delete-history-entry-button');
                            btn.setAttribute('hx-delete', '{}');
                            btn.setAttribute('hx-target', '#watch-history-entry-{}');
                            htmx.process(btn);
                            document.getElementById('confirm-delete-history-entry-modal').showModal();
                        "#,
                            entry_url,
                            entry.id,
                        ))
                    {
                        (trash_solid("size-5"));
                        "Remove";
                    }
                }
            }

            script {
                (PreEscaped(format!(
                    r#"
                        (function () {{
                            const startInput = document.getElementById('start-date-{0}');
                            const endInput = document.getElementById('end-date-{0}');
                            if (startInput) {{ startInput.value = '{1}'; }}
                            if (endInput) {{ endInput.value = '{2}'; }}
                        }})();
                    "#,
                    entry.id,
                    start_date.unwrap_or_default(),
                    end_date.unwrap_or_default(),
                )))
            }
        }
    }
}
