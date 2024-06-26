use dioxus::prelude::*;
use dioxus_material::{use_theme, TextButton};
use lookbook::preview;

/// Buttons let people take action and make choices with one tap.
#[preview]
pub fn TextButtonPreview(
    cx: Scope,
    /// Label for the text button.
    #[lookbook(default = "Label")]
    label: &'a str,

    /// Background color of the container (optional).
    #[lookbook(default = &*use_theme(cx).primary_color)]
    color: &'a str,

    /// Background color of the container (optional).
    #[lookbook(default = &*use_theme(cx).border_radius_medium)]
    border_radius: &'a str,
) -> Element {
    rsx!(TextButton {
        color,
        border_radius,
        onpress: |_| {},
        label
    })
}
