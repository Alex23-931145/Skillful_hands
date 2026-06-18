use dioxus::prelude::*;

use crate::components::ScrollLink;
use crate::Route;

const LOGO: Asset = asset!("/assets/img/logo.png");

/// Шапка сайта (Pencil HiEqB). Sticky, на узком экране — бургер-меню.
#[component]
pub fn Header() -> Element {
    let mut open = use_signal(|| false);
    let close = move |_| open.set(false);

    rsx! {
        header { class: if open() { "header is-open" } else { "header" },
            div { class: "header__inner",
                Link { to: Route::Home {}, onclick: close, class: "logo",
                    img { class: "header__logo", src: LOGO, alt: "" }
                    span { class: "logo__word",
                        span { class: "logo__name", "Skillful Hands" }
                        span { class: "logo__sub", "SOLUTIONS LTD." }
                    }
                }
                div { class: "header__menu",
                    nav { class: "nav",
                        Link { to: Route::Home {}, onclick: close, "Home" }
                        Link { to: Route::Services {}, onclick: close, "Services" }
                        ScrollLink { target: "projects", onnav: close, "Projects" }
                        ScrollLink { target: "about", onnav: close, "About Us" }
                    }
                    ScrollLink { target: "contact", class: "btn header__cta", onnav: close, "Contact Now" }
                }
                button {
                    class: "header__burger",
                    "aria-label": "Menu",
                    "aria-expanded": if open() { "true" } else { "false" },
                    onclick: move |_| open.set(!open()),
                    span { class: "burger" }
                }
            }
        }
    }
}
