use dioxus::prelude::*;

// /// The props for the [`Form`] component
// #[derive(Props, Clone, PartialEq)]
// pub struct FormProps {
//     /// The id of the element that this label is associated with
//     pub html_for: ReadSignal<String>,
//
//     /// Additional attributes to apply to the label element
//     #[props(extends = GlobalAttributes)]
//     pub attributes: Vec<Attribute>,
//
//     /// The children of the label element
//     pub children: Element,
// }

#[component]
pub fn Form(
    class: Option<String>,
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let class = class.unwrap_or("flex flex-col gap-4 bg-background".to_string());
    rsx! {
        form { class, {children} }
    }
}
