use dioxus::prelude::*;

use crate::components::button::{Button, ButtonVariant};
use crate::components::card::{Card, CardContent, CardDescription, CardHeader, CardTitle};
use crate::form::Form;

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Login(class: Option<String>) -> Element {
    let mut mode = use_signal(|| "login".to_string());
    let class = class.unwrap_or("p-8 mx-auto max-w-md".to_string());

    rsx! {
        div { class,
            Card {
                CardHeader {
                    CardTitle { "Login" }
                    CardDescription { "Please enter your credentials to log in." }
                }
                CardContent {

                    div { class: "flex flex-col gap-4",
                        Form {
                            input {
                                hidden: true,
                                value: mode().to_string(),
                                name: "mode",
                            }
                            input {
                                r#type: "email",
                                name: "email",
                                placeholder: "Email",
                                class: "w-full p-2 border border-primary rounded-lg",
                            }
                            input {
                                r#type: "password",
                                name: "password",
                                placeholder: "Password",
                                class: "w-full p-2 border border-primary rounded-lg",
                            }
                            if mode() == "signup" {
                                input {
                                    r#type: "password",
                                    name: "confirm_password",
                                    placeholder: "Confirm Password",
                                    class: "w-full p-2 mb-4 border border-primary rounded-lg",
                                }
                            }
                            Button { r#type: "submit".to_string(), "Submit" }
                        }

                        div { class: "text-sm text-center mt-4 transition-all duration-200",
                            if mode() == "login" {
                                div {
                                    "Don't have an account? "
                                    a {
                                        class: "text-primary hover:underline cursor-pointer transition-all duration-200",
                                        onclick: move |_| mode.set("signup".to_string()),
                                        "Sign up"
                                    }
                                }
                            } else {
                                div {
                                    "Already have an account? "
                                    a {
                                        class: "text-primary hover:underline cursor-pointer transition-all duration-200",
                                        onclick: move |_| mode.set("login".to_string()),
                                        "Log in"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
