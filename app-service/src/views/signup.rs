use dioxus::prelude::*;

#[component]
pub fn Signup() -> Element {
    rsx! {
            div {
            class: "flex flex",
                h1 { "Signup Page" }
            }
    }
}
