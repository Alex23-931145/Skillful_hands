use dioxus::prelude::*;

use crate::components::Icon;

/// Секция «Contact» (Pencil yzRSx). Без бэкенда — статическая форма.
#[component]
pub fn ContactForm() -> Element {
    let details = [
        ("phone", "PHONE", "(778) 239-6704"),
        ("mail", "EMAIL", "skillfulhandsbc@gmail.com"),
        ("map-pin", "SERVICE AREA", "Sunshine Coast, BC"),
    ];
    let svc = [
        "Renovations",
        "Outdoor Living Spaces",
        "Cottages & Cabins",
        "Landscaping",
    ];

    rsx! {
        section { id: "contact", class: "contact",
            div { class: "contact__inner",
                div { class: "contact__left",
                    span { class: "eyebrow", "GET IN TOUCH" }
                    h2 { class: "contact__title", "Let's talk about your project" }
                    p { class: "contact__para",
                        "Tell us a little about your project and we'll get back to you within one business day with next steps."
                    }
                    div { class: "contact__details",
                        for (icon , label , value) in details {
                            div { class: "contact__row",
                                span { class: "contact__icon", Icon { name: icon.to_string() } }
                                div {
                                    div { class: "contact__row-label", "{label}" }
                                    div { class: "contact__row-value", "{value}" }
                                }
                            }
                        }
                    }
                }
                form {
                    class: "form-card",
                    onsubmit: move |e| e.prevent_default(),
                    div { class: "form-row",
                        div { class: "field",
                            label { class: "field__label", "First name" }
                            input { r#type: "text", placeholder: "Jane" }
                        }
                        div { class: "field",
                            label { class: "field__label", "Last name" }
                            input { r#type: "text", placeholder: "Doe" }
                        }
                    }
                    div { class: "field",
                        label { class: "field__label", "Email" }
                        input { r#type: "email", placeholder: "jane@email.com" }
                    }
                    div { class: "field",
                        label { class: "field__label", "Phone" }
                        input { r#type: "tel", placeholder: "(778) 000-0000" }
                    }
                    div { class: "field",
                        label { class: "field__label", "Service" }
                        select {
                            option { value: "", disabled: true, selected: true, "Select a service" }
                            for s in svc {
                                option { value: "{s}", "{s}" }
                            }
                        }
                    }
                    div { class: "field",
                        label { class: "field__label", "Message" }
                        textarea { placeholder: "Tell us about your project..." }
                    }
                    button { r#type: "submit", class: "form-submit",
                        "Send message"
                        Icon { name: "arrow-right".to_string() }
                    }
                }
            }
        }
    }
}
