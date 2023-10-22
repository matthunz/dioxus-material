use dioxus::prelude::*;
use dioxus_material::{Button, TextButton, TextField};
use lookbook::{register, Look, LookBook};

#[component]
fn ButtonPage(cx: Scope) -> Element {
    let label = use_state(cx, || String::from("Filled Button"));

    render!(
        Look {
            name: "Button",
            controls: render!(
                TextField { label : "Label", value : label, onchange : move | event : FormEvent |
                label.set(event.data.value.clone()) }
            ),
            Button { onpress: |_| {}, &*** label }
        }
    )
}

#[component]
fn TextButtonPage(cx: Scope) -> Element {
    let label = use_state(cx, || String::from("Text Button"));

    render!(
        Look {
            name: "TextButton",
            controls: render!(
                TextField { label : "Label", value : label, onchange : move | event : FormEvent |
                label.set(event.data.value.clone()) }
            ),
            TextButton { onpress: |_| {}, &*** label }
        }
    )
}

#[component]
fn TextFieldPage(cx: Scope) -> Element {
    let value = use_state(cx, || String::from("Text Field"));
    let label = use_state(cx, || String::from("Label"));

    render!(
        Look {
            name: "TextField",
            controls: render!(
                TextField { label : "Label", value : label, onchange : move | event : FormEvent |
                label.set(event.data.value.clone()) }
            ),
            TextField {
                label: &**label,
                value: value,
                onchange: move |event: FormEvent| value.set(event.data.value.clone())
            }
        }
    )
}

fn app(cx: Scope) -> Element {
    register("Button", ButtonPage);
    register("TextButton", TextButtonPage);
    register("TextField", TextFieldPage);

    #[cfg(feature = "pages")]
    let prefix = "/dioxus-material/lookbook";

    #[cfg(not(feature = "pages"))]
    let prefix = "";

    render!( LookBook { prefix: prefix } )
}

fn main() {
    dioxus_web::launch(app);
}
