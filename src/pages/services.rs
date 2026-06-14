use dioxus::prelude::*;

use crate::components::{ContactForm, Icon, ScrollLink};
use crate::Route;

/// Страница услуг (Pencil a79RGt «Services Page» → ребренд Skillful Hands).
#[component]
pub fn Services() -> Element {
    rsx! {
        main {
            ServicesHero {}
            ServicesList {}
            ClosingCta {}
            ContactForm {}
        }
    }
}

// ---------- Services Hero ----------
#[component]
fn ServicesHero() -> Element {
    rsx! {
        section { class: "svc-hero",
            div { class: "wrap",
                div { class: "svc-hero__inner",
                    span { class: "svc-hero__eyebrow", "SERVICES" }
                    h1 { class: "svc-hero__title", "Our construction services" }
                    p { class: "svc-hero__sub",
                        "From custom renovations to cottages, decks, and landscaping, Skillful Hands Solutions delivers quality craftsmanship across the Sunshine Coast and surrounding islands of British Columbia."
                    }
                }
            }
        }
    }
}

// ---------- Services List ----------
#[component]
fn ServicesList() -> Element {
    // (num, title, slug, desc, image base URL). Зигзаг — по индексу строки.
    let rows = [
        (
            "01",
            "Renovations",
            "renovations",
            "Professional renovation services for residential properties — interior, exterior, complete home renovations and restoration, plus custom design and planning.",
            "https://images.unsplash.com/photo-1761897190222-d222d88851c9",
        ),
        (
            "02",
            "Outdoor Living Spaces",
            "outdoor-living",
            "Custom deck, pergola, and BBQ area construction designed for durability, functionality, relaxation, and outdoor living.",
            "https://images.unsplash.com/photo-1735657438299-7d543a1b8cc2",
        ),
        (
            "03",
            "Cottages & Cabins",
            "cottages-cabins",
            "Custom cottages, cabins, guest houses, and other unique custom-built structures, built to last in any setting.",
            "https://images.unsplash.com/photo-1604609165742-58e1b9cf0457",
        ),
        (
            "04",
            "Landscaping",
            "landscaping",
            "Landscaping and property improvement services for residential and waterfront properties.",
            "https://images.unsplash.com/photo-1621385236063-8544bb086619",
        ),
    ];
    rsx! {
        section { class: "svc-list",
            div { class: "wrap",
                for (i , (num , title , slug , desc , img)) in rows.into_iter().enumerate() {
                    div {
                        key: "{slug}",
                        // Чётный индекс (строки 1,3,5) — фото слева (исходный порядок);
                        // нечётный (2,4) — reverse, фото уезжает вправо.
                        class: if i % 2 == 1 { "svc-row svc-row--rev" } else { "svc-row" },
                        div {
                            class: "svc-row__photo",
                            style: "background-image:url('{img}?crop=entropy&cs=tinysrgb&fit=max&fm=jpg&q=80&w=1080')",
                        }
                        div { class: "svc-row__text",
                            div { class: "svc-row__top",
                                span { class: "svc-row__num", "{num}" }
                                h2 { class: "svc-row__title", "{title}" }
                            }
                            p { class: "svc-row__desc", "{desc}" }
                            Link {
                                to: Route::ServiceDetail { slug: slug.to_string() },
                                class: "see-service",
                                "See service"
                                Icon { name: "arrow-right".to_string(), size: 16 }
                            }
                        }
                    }
                }
            }
        }
    }
}

// ---------- Closing CTA ----------
#[component]
fn ClosingCta() -> Element {
    rsx! {
        section { class: "svc-cta",
            div { class: "wrap",
                div { class: "svc-cta__inner",
                    h2 { class: "svc-cta__title",
                        "Looking for a qualified, reliable, and honest construction team?"
                    }
                    ScrollLink { target: "contact", class: "btn btn--light", "Contact now" }
                }
            }
        }
    }
}
