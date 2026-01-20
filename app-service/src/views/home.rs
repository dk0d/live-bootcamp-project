use dioxus::prelude::*;

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Home() -> Element {
    rsx! {
        div {
            div {
                class: "flex justify-center p-4 w-full",
                img {
                    id: "protected-img",
                    alt: "Protected Resource",
                    width: "560",
                    height: "350",
                    src: asset!("/assets/default.jpg")
                }
            }
        }
    }
}
