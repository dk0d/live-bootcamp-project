use dioxus::prelude::*;

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Login() -> Element {
    rsx! {
        div {
            "Login Page"
        }
    }
}
