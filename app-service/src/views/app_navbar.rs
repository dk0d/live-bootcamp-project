use crate::Route;
use crate::components::navbar::{Navbar, NavbarItem};
use dioxus::prelude::*;

/// The Navbar component that will be rendered on all pages of our app since every page is under the layout.
///
///
/// This layout component wraps the UI of [Route::Home] and [Route::Blog] in a common navbar. The contents of the Home and Blog
/// routes will be rendered under the outlet inside this component
#[component]
pub fn AppNavbar() -> Element {
    rsx! {
        Navbar {
            class: "flex flex-row items-center m-12 shadow-md",
            aria_label: "Main Navigation",
            NavbarItem {
                index: 0usize,
                value: "home",
                to: Route::Home {},
                img {
                    id: "logo",
                    alt: "LGR Auth Logo",
                    src: asset!("/assets/lgr_logo.png"),
                    class: "size-10"
                }
            }

            // NavbarItem {
            //     class: "flex flex-1" ,
            //     index: 1usize,
            //     value: "signup",
            //     to: Route::Signup,
            //     div {
            //         "Signup"
            //     }
            // }

            NavbarItem {
                class: "flex flex-1" ,
                index: 1usize,
                value: "login",
                to: Route::Login,
                div {
                    "Login"
                }
            }
        }

        // The `Outlet` component is used to render the next component inside the layout. In this case, it will render either
        // the [`Home`] or [`Blog`] component depending on the current route.
        Outlet::<Route> {}
    }
}
