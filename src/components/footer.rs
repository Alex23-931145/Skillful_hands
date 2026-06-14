use dioxus::prelude::*;

use crate::components::{Icon, Logo, ScrollLink};
use crate::Route;

/// Подвал (Pencil G5CvcW). На каждой странице через Layout.
#[component]
pub fn Footer() -> Element {
    let services = [
        ("Renovations", "renovations"),
        ("Outdoor Living", "outdoor-living"),
        ("Cottages & Cabins", "cottages-cabins"),
        ("Landscaping", "landscaping"),
    ];

    rsx! {
        footer { class: "footer",
            div { class: "footer__inner",
                div { class: "footer__top",
                    // Бренд
                    div { class: "footer__brand",
                        Logo {}
                        p { class: "footer__tagline",
                            "Custom construction, renovations, and outdoor living on the Sunshine Coast, BC — 12+ years of Canadian construction experience."
                        }
                        div { class: "footer__socials",
                            a { class: "footer__social", href: "https://www.facebook.com/share/18u35u7YjK/?mibextid=wwXIfr", target: "_blank", rel: "noopener", "aria-label": "Facebook", Icon { name: "facebook".to_string() } }
                            a { class: "footer__social", href: "https://www.instagram.com/skillfull_hands_solutions_ltd", target: "_blank", rel: "noopener", "aria-label": "Instagram", Icon { name: "instagram".to_string() } }
                        }
                    }
                    // Services
                    div { class: "footer__col",
                        span { class: "footer__col-title", "SERVICES" }
                        for (label , slug) in services {
                            Link { to: Route::ServiceDetail { slug: slug.to_string() }, "{label}" }
                        }
                    }
                    // Company
                    div { class: "footer__col",
                        span { class: "footer__col-title", "COMPANY" }
                        ScrollLink { target: "about", "About Us" }
                        ScrollLink { target: "projects", "Projects" }
                        ScrollLink { target: "testimonials", "Reviews" }
                    }
                    // Contact
                    div { class: "footer__col",
                        span { class: "footer__col-title", "CONTACT" }
                        a { href: "tel:+17782396704", "(778) 239-6704" }
                        a { href: "mailto:skillfulhandsbc@gmail.com", "skillfulhandsbc@gmail.com" }
                        span { "Sunshine Coast, BC" }
                    }
                }
                div { class: "footer__bottom",
                    span { "© 2026 Skillful Hands Solutions Ltd. All rights reserved." }
                    span {
                        Link { to: Route::Privacy {}, "Privacy Policy" }
                        "   ·   "
                        Link { to: Route::Terms {}, "Terms of Service" }
                    }
                }
            }
        }
    }
}
