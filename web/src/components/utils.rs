use dioxus::prelude::*;

#[component]
pub fn Loading() -> Element {
    rsx! {
        div {
            style: "display: flex; justify-content: center; align-items: center; height: 100%;",
            div {
                class: "spinner-border text-primary",
                role: "status",
                span {
                    class: "visually-hidden", // for screen readers
                    "Loading..."
                }
            }
        }
    }
}


