use dioxus::prelude::*;

use crate::components::{ContactForm, Icon, ScrollLink};
use crate::pages::project_card;
use crate::Route;

/// Детальная страница услуги (Pencil DOYUn/Ir8HP/MJ0tT/ArMJd/sgGfF).
/// Один параметрический шаблон + данные на 4 услуги, выбор по slug.
#[component]
pub fn ServiceDetail(slug: String) -> Element {
    let data = service_by_slug(&slug);

    rsx! {
        main {
            // A. Detail Hero
            section { class: "sd-hero",
                div { class: "wrap sd-hero__inner",
                    div { class: "sd-hero__left",
                        div { class: "sd-crumb",
                            Link { to: Route::Services {}, class: "sd-crumb__root", "Services" }
                            Icon { name: "chevron-right".to_string(), size: 15 }
                            span { class: "sd-crumb__current", "{data.name}" }
                        }
                        h1 { class: "sd-hero__title", "{data.name}" }
                        p { class: "sd-hero__intro", "{data.intro}" }
                        div { class: "sd-hero__btns",
                            ScrollLink { target: "contact", class: "btn", "Get a free quote" }
                            a { href: "tel:+17782396704", class: "btn btn--outline", "Call (778) 239-6704" }
                        }
                    }
                    div {
                        class: "sd-hero__photo",
                        style: "background-image:url('{data.hero_img}')",
                    }
                }
            }

            // B. Related projects (реальные проекты — портфолио сразу под hero)
            if !data.related_projects.is_empty() {
                section { class: "sd-related",
                    div { class: "wrap sd-related__inner",
                        div { class: "sd-head",
                            span { class: "sd-eyebrow", "CASE STUDIES" }
                            h2 { class: "sd-h2", "Related projects" }
                        }
                        div { class: "sd-related__grid",
                            for slug in data.related_projects.iter().copied() {
                                if let Some((s , chip , img)) = project_card(slug) {
                                    Link {
                                        key: "{s}",
                                        to: Route::ProjectDetail { slug: s.to_string() },
                                        class: "sd-related__card",
                                        style: "background-image:url('{img}')",
                                        span { class: "sd-related__chip",
                                            "{chip}"
                                            Icon { name: "arrow-up-right".to_string(), size: 15 }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // C. Gallery — работы этой категории (полные фото / coming soon)
            section { class: "svc-gallery",
                div { class: "wrap svc-gallery__inner",
                    div { class: "sd-head",
                        span { class: "sd-eyebrow", "OUR WORK" }
                        h2 { class: "sd-h2", "{data.gallery_title}" }
                    }
                    if data.gallery.is_empty() {
                        div { class: "svc-empty",
                            div { class: "svc-empty__title", "Photos coming soon" }
                            p { class: "svc-empty__text",
                                "We're putting this gallery together — new project photos are on the way. Get in touch to talk about your project."
                            }
                            ScrollLink { target: "contact", class: "btn", "Get a free quote" }
                        }
                    } else {
                        div { class: "pd-gallery-grid",
                            for img in data.gallery.iter() {
                                img {
                                    key: "{img}",
                                    class: "pd-photo",
                                    src: "{img}",
                                    alt: "{data.name}",
                                    loading: "lazy",
                                }
                            }
                        }
                    }
                }
            }

            // D. Features
            section { class: "sd-features",
                div { class: "wrap sd-features__inner",
                    div { class: "sd-head",
                        span { class: "sd-eyebrow", "WHAT'S INCLUDED" }
                        h2 { class: "sd-h2", "{data.features_title}" }
                    }
                    div { class: "sd-feature-grid",
                        for f in data.features.iter() {
                            div { key: "{f.title}", class: "sd-feature",
                                span { class: "sd-feature__icon", Icon { name: f.icon.to_string(), size: 24 } }
                                div { class: "sd-feature__text",
                                    h3 { class: "sd-feature__title", "{f.title}" }
                                    p { class: "sd-feature__desc", "{f.desc}" }
                                }
                            }
                        }
                    }
                }
            }

            // D. Process (generic — идентичен на всех услугах)
            Process {}

            // E. Closing CTA
            section { class: "sd-cta",
                div { class: "wrap sd-cta__inner",
                    h2 { class: "sd-cta__title", "{data.closing_title}" }
                    ScrollLink { target: "contact", class: "btn btn--light", "Contact now" }
                }
            }

            // F. Contact
            ContactForm {}
        }
    }
}

// ---------- Process (generic, идентичен на всех услугах) ----------
#[component]
fn Process() -> Element {
    rsx! {
        section { class: "sd-process",
            div { class: "wrap sd-process__inner",
                div { class: "sd-head",
                    span { class: "sd-eyebrow", "HOW IT WORKS" }
                    h2 { class: "sd-h2", "A simple, no-surprises process" }
                }
                div { class: "sd-steps",
                    for step in PROCESS_STEPS.iter() {
                        div { key: "{step.0}", class: "sd-step",
                            div { class: "sd-step__num", "{step.0}" }
                            h3 { class: "sd-step__title", "{step.1}" }
                            p { class: "sd-step__desc", "{step.2}" }
                        }
                    }
                }
            }
        }
    }
}

// ============================================================
//  Данные услуг
// ============================================================

#[derive(Clone, PartialEq)]
struct Feature {
    icon: &'static str,
    title: &'static str,
    desc: &'static str,
}

#[derive(Clone, PartialEq)]
struct ServiceData {
    name: &'static str,
    intro: &'static str,
    hero_img: String,
    features_title: &'static str,
    features: Vec<Feature>,
    /// Заголовок галереи работ категории.
    gallery_title: &'static str,
    /// Фото работ этой категории (полные, без обрезки). Пусто → «coming soon».
    gallery: Vec<String>,
    closing_title: &'static str,
    /// Связанные проекты (slug) — кросс-ссылки на страницы проектов.
    related_projects: Vec<&'static str>,
}

// Шаги процесса одинаковы на всех услугах — общий набор.
const PROCESS_STEPS: [(&str, &str, &str); 4] = [
    ("01", "Consultation", "We visit your property, listen to your goals, and assess the site."),
    ("02", "Design & Planning", "We map out a clear plan, design direction, and a transparent quote."),
    ("03", "Construction", "Our crew builds to spec — tidy, on schedule, and to a high standard."),
    ("04", "Project Completion", "We review every detail with you and hand over a result you're proud of."),
];

fn feature(icon: &'static str, title: &'static str, desc: &'static str) -> Feature {
    Feature { icon, title, desc }
}

fn service_by_slug(slug: &str) -> ServiceData {
    match slug {
        "outdoor-living" => outdoor_living(),
        "landscaping" => landscaping(),
        // renovations + неизвестный slug → первая услуга
        _ => renovations(),
    }
}

/// Хелпер: URL веб-фото. public_dir=assets отдаётся с корня домена,
/// поэтому assets/img/foto_web/<name> доступен как /img/foto_web/<name>
/// (как og:image /img/hero.jpg). Базовый путь не нужен — сайт на своём домене.
fn ph(name: &str) -> String {
    format!("/img/foto_web/{name}")
}

fn renovations() -> ServiceData {
    ServiceData {
        name: "Renovations",
        intro: "Professional renovation services for residential properties — interior, exterior, and complete home renovations and restoration, backed by custom design and planning.",
        hero_img: asset!("/assets/img/foto_web/island-retreat-interior-after-07.jpg").to_string(),
        features_title: "Everything a quality renovation needs",
        features: vec![
            feature("ruler", "Custom design & planning", "We plan layout, grades, and finishes before any work begins."),
            feature("hammer", "Interior renovations", "Kitchens, baths, flooring, and full interior refreshes."),
            feature("layers", "Exterior renovations", "Siding, decks, facades, and weather-ready exterior work."),
            feature("sparkles", "Complete restoration", "Full home renovations and restoration, done right."),
        ],
        gallery_title: "Recent renovation work",
        gallery: [
            "island-retreat-interior-after-04.jpg",
            "island-retreat-interior-after-05.jpg",
            "island-retreat-interior-after-06.jpg",
            "island-retreat-interior-after-10.jpg",
            "island-retreat-interior-after-12.jpg",
            "island-retreat-interior-after-19.jpg",
            "island-retreat-interior-after-21.jpg",
            "island-retreat-interior-after-24.jpg",
            "island-retreat-interior-after-15.jpg",
            "island-retreat-interior-after-18.jpg",
            "island-retreat-interior-after-20.jpg",
            "bathroom-after-01.jpg",
            "bathroom-after-02.jpg",
            "bathroom-after-03.jpg",
            "utility-room-after-04.jpg",
            "utility-room-after-06.jpg",
            "utility-room-after-07.jpg",
            "custom-woodwork-01.jpg",
            "custom-woodwork-02.jpg",
        ].into_iter().map(ph).collect(),
        closing_title: "Ready to start your renovation project?",
        related_projects: vec!["rustic-utility-room", "island-retreat"],
    }
}

fn outdoor_living() -> ServiceData {
    ServiceData {
        name: "Decks, BBQ & Pergola",
        intro: "Custom decks, BBQ areas, pergolas, and patio zones — built for durability, function, and relaxed outdoor living on the coast.",
        hero_img: asset!("/assets/img/foto_web/bbq-area-after-04.jpg").to_string(),
        features_title: "Built for how you live outdoors",
        features: vec![
            feature("hammer", "Custom decks & patios", "Durable, beautiful decks and patio zones built to fit your space."),
            feature("trees", "Pergolas & shade", "Pergolas and shade structures for comfort and style."),
            feature("sparkles", "BBQ & outdoor kitchens", "Built-in BBQ and outdoor kitchens for hosting."),
            feature("shield-check", "Built to last", "Quality materials that stand up to the coast climate."),
        ],
        gallery_title: "Recent decks, BBQ & pergola work",
        gallery: [
            "bbq-area-after-06.jpg",
            "bbq-area-after-03.jpg",
            "bbq-area-after-02.jpg",
            "bbq-area-after-05.jpg",
            "bbq-area-after-01.jpg",
            "bbq-area-after-07.jpg",
            "bbq-area-after-08.jpg",
            "bbq-area-during-01.jpg",
            "bbq-area-during-03.jpg",
            "bbq-area-during-05.jpg",
            "island-retreat-exterior-after-02.jpg",
            "island-retreat-exterior-after-03.jpg",
            "island-retreat-exterior-after-04.jpg",
            "island-retreat-exterior-after-07.jpg",
            "island-retreat-exterior-after-08.jpg",
            "island-retreat-exterior-after-09.jpg",
            "island-retreat-exterior-after-05.jpg",
        ].into_iter().map(ph).collect(),
        closing_title: "Ready to build your outdoor space?",
        related_projects: vec!["waterfront-bbq", "island-retreat"],
    }
}

fn landscaping() -> ServiceData {
    ServiceData {
        name: "Landscaping",
        intro: "Landscaping and property improvement for residential and waterfront properties across the Sunshine Coast.",
        hero_img: asset!("/assets/img/foto_web/scenery-01.jpg").to_string(),
        features_title: "Property improvement, inside out",
        features: vec![
            feature("leaf", "Residential landscaping", "Yards, plantings, and grounds that elevate your property."),
            feature("droplets", "Waterfront work", "Improvement projects for waterfront and shoreline properties."),
            feature("ruler", "Site & grounds", "Grading, hardscape, and site improvement."),
            feature("sparkles", "Property improvement", "Finishing touches that boost value and enjoyment."),
        ],
        gallery_title: "Recent landscaping work",
        // Фото будут позже — пустая галерея покажет блок «coming soon».
        gallery: Vec::new(),
        closing_title: "Ready to transform your property?",
        related_projects: vec!["island-retreat"],
    }
}
