use dioxus::prelude::*;
use lookbook::LookBook;

mod previews;
use previews::{ButtonPreview, ChipPreview, TabRowPreview, TextButtonPreview, TextFieldPreview};

#[component]
fn Home(cx: Scope) -> Element {
    rsx!(
        div { padding: "20px",
            h1 { "Dioxus Material" }
            h5 { "Material You design library for dioxus." }
            a { href: "https://github.com/matthunz/dioxus-material", "Github" }

            div { margin_top: "20px",
                "Made with "
                a { href: "https://github.com/matthunz/lookbook", "Lookbook" }
                "."
            }
        }
    )
}

fn app(cx: Scope) -> Element {
    rsx!(LookBook {
        home: Home,
        previews: [
            ButtonPreview,
            ChipPreview,
            TabRowPreview,
            TextButtonPreview,
            TextFieldPreview
        ]
    })
}

fn main() {
    dioxus_web::launch(app);
}
