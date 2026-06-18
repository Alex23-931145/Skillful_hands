use dioxus::prelude::*;

use crate::components::{ContactForm, Icon, ScrollLink};
use crate::Route;

/// Детальная страница проекта (кейс). Один параметрический шаблон + данные на
/// 3 проекта, выбор по slug. Вёрстка переиспользует sd-* классы страниц услуг
/// (hero, head, cta) + собственные pd-* (overview, features, before/after, gallery).
#[component]
pub fn ProjectDetail(slug: String) -> Element {
    let data = project_by_slug(&slug);

    rsx! {
        main {
            // A. Hero
            section { class: "sd-hero",
                div { class: "wrap sd-hero__inner",
                    div { class: "sd-hero__left",
                        div { class: "sd-crumb",
                            Link { to: Route::Home {}, class: "sd-crumb__root", "Home" }
                            Icon { name: "chevron-right".to_string(), size: 15 }
                            span { class: "sd-crumb__current", "Projects" }
                        }
                        h1 { class: "sd-hero__title", "{data.name}" }
                        p { class: "sd-hero__intro", "{data.intro}" }
                        div { class: "pd-meta",
                            div { class: "pd-meta__item",
                                Icon { name: "map-pin".to_string(), size: 17 }
                                span { "{data.location}" }
                            }
                            div { class: "pd-meta__item",
                                Icon { name: "shield-check".to_string(), size: 17 }
                                span { "Completed by " strong { "{data.completed_by}" } }
                            }
                        }
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

            // B. Overview (narrative)
            section { class: "pd-overview",
                div { class: "wrap pd-overview__inner",
                    div { class: "sd-head",
                        span { class: "sd-eyebrow", "PROJECT OVERVIEW" }
                        h2 { class: "sd-h2", "{data.overview_title}" }
                    }
                    div { class: "pd-prose",
                        for para in data.overview.iter() {
                            p { key: "{para}", "{para}" }
                        }
                    }
                }
            }

            // C. Features (bullet list)
            section { class: "pd-features",
                div { class: "wrap pd-features__inner",
                    div { class: "sd-head",
                        span { class: "sd-eyebrow", "WHAT WE BUILT" }
                        h2 { class: "sd-h2", "{data.features_title}" }
                    }
                    ul { class: "pd-feature-list",
                        for item in data.features.iter() {
                            li { key: "{item}", class: "pd-feature-item",
                                span { class: "pd-feature-item__icon", Icon { name: "check".to_string(), size: 17 } }
                                span { "{item}" }
                            }
                        }
                    }
                }
            }

            // D. Before & After (только если есть пары «до/после»)
            if !data.comparisons.is_empty() {
                section { class: "pd-ba",
                    div { class: "wrap pd-ba__inner",
                        div { class: "sd-head",
                            span { class: "sd-eyebrow", "TRANSFORMATION" }
                            h2 { class: "sd-h2", "Before & after" }
                        }
                        div { class: "pd-ba__grid",
                            for c in data.comparisons.iter() {
                                div { key: "{c.label}", class: "pd-ba__pair",
                                    span { class: "pd-ba__label", "{c.label}" }
                                    div { class: "pd-ba__images",
                                        div { class: "pd-ba__shot",
                                            img { class: "pd-ba__img", src: "{c.before}", alt: "{c.label} — before", loading: "lazy" }
                                            span { class: "pd-ba__badge pd-ba__badge--before", "Before" }
                                        }
                                        div { class: "pd-ba__shot",
                                            img { class: "pd-ba__img", src: "{c.after}", alt: "{c.label} — after", loading: "lazy" }
                                            span { class: "pd-ba__badge pd-ba__badge--after", "After" }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // E. Gallery — гибкая сетка под остальные фото
            section { class: "pd-gallery",
                div { class: "wrap pd-gallery__inner",
                    h2 { class: "sd-h2", "Project gallery" }
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

            // F. Related services (кросс-ссылки на услуги)
            if !data.related_services.is_empty() {
                section { class: "pd-related",
                    div { class: "wrap pd-related__inner",
                        div { class: "sd-head",
                            span { class: "sd-eyebrow", "EXPLORE" }
                            h2 { class: "sd-h2", "Services behind this project" }
                        }
                        div { class: "pd-related__links",
                            for (slug , label) in data.related_services.iter() {
                                Link {
                                    key: "{slug}",
                                    to: Route::ServiceDetail { slug: slug.to_string() },
                                    class: "pill-link",
                                    "{label}"
                                    Icon { name: "arrow-up-right".to_string(), size: 16 }
                                }
                            }
                        }
                    }
                }
            }

            // G. Closing CTA
            section { class: "sd-cta",
                div { class: "wrap sd-cta__inner",
                    h2 { class: "sd-cta__title", "Have a project like this in mind?" }
                    ScrollLink { target: "contact", class: "btn btn--light", "Contact now" }
                }
            }

            // H. Contact
            ContactForm {}
        }
    }
}

// ============================================================
//  Данные проектов
// ============================================================

/// Пара «до/после» для блока Before & After.
#[derive(Clone, PartialEq)]
struct Comparison {
    before: String,
    after: String,
    label: &'static str,
}

#[derive(Clone, PartialEq)]
struct ProjectData {
    slug: &'static str,
    /// Подпись на плитке главной + хлебная крошка.
    chip: &'static str,
    name: &'static str,
    /// Лид в hero (1–2 предложения).
    intro: &'static str,
    hero_img: String,
    overview_title: &'static str,
    /// Повествовательные абзацы.
    overview: Vec<&'static str>,
    features_title: &'static str,
    features: Vec<&'static str>,
    /// Пары «до/после». Пусто → блок не рендерится (напр. BBQ — новая постройка).
    comparisons: Vec<Comparison>,
    gallery: Vec<String>,
    location: &'static str,
    completed_by: &'static str,
    /// Связанные услуги (slug, label) — кросс-ссылки на страницы услуг.
    related_services: Vec<(&'static str, &'static str)>,
}

fn all_projects() -> Vec<ProjectData> {
    vec![rustic_utility_room(), waterfront_bbq(), island_retreat()]
}

/// (slug, подпись, фото) одного проекта — для кросс-ссылок со страниц услуг.
pub fn project_card(slug: &str) -> Option<(&'static str, &'static str, String)> {
    all_projects()
        .into_iter()
        .find(|p| p.slug == slug)
        .map(|p| (p.slug, p.chip, p.hero_img))
}

fn project_by_slug(slug: &str) -> ProjectData {
    match slug {
        "waterfront-bbq" => waterfront_bbq(),
        "island-retreat" => island_retreat(),
        // rustic-utility-room + неизвестный slug → первый проект
        _ => rustic_utility_room(),
    }
}

// Реальные фото — assets/img/foto_web/ (web-оптимизированы). Через asset!()
// ради base-path-safe путей на GitHub Pages (project-page /Skillful_hands/).
fn rustic_utility_room() -> ProjectData {
    ProjectData {
        slug: "rustic-utility-room",
        chip: "Rustic Utility & Storage Room",
        name: "Custom Rustic Utility & Storage Room Renovation",
        intro: "This custom utility and storage room was completely rebuilt and transformed on a remote island along the coast of British Columbia.",
        hero_img: asset!("/assets/img/foto_web/utility-room-after-04.jpg").to_string(),
        overview_title: "Handcrafted, down to the last detail",
        overview: vec![
            "What makes this project unique is that every visible element was custom-designed and handcrafted specifically for this space. From the rustic cabinetry and storage systems to the custom doors, countertops, trim work, and utility integration, every detail was carefully planned and built by hand.",
            "The original space was outdated, worn, and lacked both functionality and organization. The goal was to create a practical utility room while maintaining the character and atmosphere of the surrounding cabin.",
            "Every cabinet, door, panel, and finishing detail was individually built and installed by hand. The entire design was developed in the signature Skillful Hands rustic style, combining functionality, durability, and traditional craftsmanship.",
            "Working on a remote island requires extensive planning and logistics. Every material, tool, fixture, and component had to be transported, fabricated, and installed with precision, making projects like this far more demanding than conventional renovations.",
            "This project demonstrates our commitment to creating practical spaces that are not only functional but also visually unique, with craftsmanship visible in every detail.",
        ],
        features_title: "Project features include",
        features: vec![
            "Complete structural renovation and rebuilding",
            "Custom handcrafted rustic cabinetry",
            "Custom-built storage systems",
            "Handcrafted rustic doors and trim work",
            "Integrated laundry area",
            "Utility sink and plumbing integration",
            "New flooring installation",
            "Improved organization and storage capacity",
            "Custom hardware and finishing details",
            "Fully coordinated mechanical and utility systems",
        ],
        comparisons: vec![
            Comparison {
                before: asset!("/assets/img/foto_web/utility-room-before-01.jpg").to_string(),
                after: asset!("/assets/img/foto_web/utility-room-after-01.jpg").to_string(),
                label: "Utility & storage room",
            },
            Comparison {
                before: asset!("/assets/img/foto_web/utility-room-before-02.jpg").to_string(),
                after: asset!("/assets/img/foto_web/utility-room-after-06.jpg").to_string(),
                label: "Laundry area",
            },
        ],
        gallery: vec![
            asset!("/assets/img/foto_web/utility-room-during-01.jpg").to_string(),
            asset!("/assets/img/foto_web/utility-room-after-02.jpg").to_string(),
            asset!("/assets/img/foto_web/utility-room-after-03.jpg").to_string(),
            asset!("/assets/img/foto_web/utility-room-after-05.jpg").to_string(),
            asset!("/assets/img/foto_web/utility-room-after-07.jpg").to_string(),
            asset!("/assets/img/foto_web/utility-room-after-08.jpg").to_string(),
        ],
        location: "Remote Island, Sunshine Coast, British Columbia",
        completed_by: "Skillful Hands Solutions LTD",
        related_services: vec![("renovations", "Renovations")],
    }
}

fn waterfront_bbq() -> ProjectData {
    ProjectData {
        slug: "waterfront-bbq",
        chip: "Waterfront BBQ & Entertainment Area",
        name: "Custom Waterfront BBQ & Outdoor Entertainment Area",
        intro: "This custom-built outdoor BBQ and entertainment area was designed, fabricated, and constructed on a remote island along the coast of British Columbia.",
        hero_img: asset!("/assets/img/foto_web/bbq-area-after-06.jpg").to_string(),
        overview_title: "An outdoor kitchen built to last for generations",
        overview: vec![
            "What makes this project truly unique is that nearly everything visible in these photos was handcrafted and custom-built. From the welded steel framework and custom cabinetry to the cedar finishes, storage compartments, cooking stations, and architectural details, every element was carefully designed specifically for this location.",
            "Working on a remote island presents challenges far beyond those of a typical construction project. Every material, every tool, every appliance, and every custom component had to be planned, transported, fabricated, and installed with precision. In an isolated environment where logistics, craftsmanship, and problem-solving are equally important, there is no room for compromise.",
            "The main purpose of this project was to create an outdoor kitchen that remains clean, protected, and functional in all weather conditions, while providing shelter from rain and maximizing enjoyment of the waterfront setting.",
            "Every detail of this project reflects a commitment to quality craftsmanship. From the largest structural elements to the smallest finishing details, every piece was built with care, attention, and pride in workmanship.",
            "This project represents more than construction — it is the creation of a unique gathering place where family and friends can enjoy the beauty of the coastline in a space designed and built to last for generations.",
        ],
        features_title: "Project features include",
        features: vec![
            "Custom welded steel structure",
            "Handcrafted cedar cladding and finish work",
            "Premium gas BBQ integration",
            "Traditional wood-fired cooking area",
            "Custom-built cabinetry and storage systems",
            "Built-in dishwasher requested by the client",
            "Natural stone countertops",
            "Firewood storage compartments",
            "Automatically lifting overhead canopy system",
            "Waterfront dining and entertainment area",
            "Custom metal and woodworking details throughout",
        ],
        // Новая постройка — «before» нет, показываем путь от стального каркаса к готовому.
        comparisons: vec![],
        gallery: vec![
            // процесс: сварной стальной каркас павильона
            asset!("/assets/img/foto_web/bbq-area-during-01.jpg").to_string(),
            asset!("/assets/img/foto_web/bbq-area-during-02.jpg").to_string(),
            asset!("/assets/img/foto_web/bbq-area-during-03.jpg").to_string(),
            asset!("/assets/img/foto_web/bbq-area-during-04.jpg").to_string(),
            asset!("/assets/img/foto_web/bbq-area-during-05.jpg").to_string(),
            // готовая BBQ-кухня
            asset!("/assets/img/foto_web/bbq-area-after-01.jpg").to_string(),
            asset!("/assets/img/foto_web/bbq-area-after-02.jpg").to_string(),
            asset!("/assets/img/foto_web/bbq-area-after-03.jpg").to_string(),
            asset!("/assets/img/foto_web/bbq-area-after-04.jpg").to_string(),
            asset!("/assets/img/foto_web/bbq-area-after-05.jpg").to_string(),
            asset!("/assets/img/foto_web/bbq-area-after-07.jpg").to_string(),
            asset!("/assets/img/foto_web/bbq-area-after-08.jpg").to_string(),
        ],
        location: "Remote Island, Sunshine Coast, British Columbia",
        completed_by: "Skillful Hands Solutions LTD",
        related_services: vec![("outdoor-living", "Decks, BBQ & Pergola")],
    }
}

fn island_retreat() -> ProjectData {
    ProjectData {
        slug: "island-retreat",
        chip: "Remote Island Retreat",
        name: "Remote Island Retreat Reconstruction",
        intro: "This project involved the complete reconstruction and modernization of a remote island retreat located on the British Columbia coastline.",
        hero_img: asset!("/assets/img/foto_web/island-retreat-exterior-after-04.jpg").to_string(),
        overview_title: "Complex construction, far from the mainland",
        overview: vec![
            "Due to the property's isolated location, all materials, tools, equipment, and supplies had to be carefully coordinated and transported by water. Every phase of the project required detailed planning, logistics management, and efficient execution in a challenging remote environment.",
            "The goal was to transform an aging island cabin into a comfortable, modern, and functional waterfront retreat while preserving its rustic character and natural surroundings.",
            "This project demonstrates our ability to successfully manage complex construction projects in remote locations where transportation, logistics, and planning are just as important as craftsmanship and quality workmanship.",
        ],
        features_title: "The scope of work included",
        features: vec![
            "Complete interior renovation and reconstruction",
            "Structural improvements and rebuilding of interior spaces",
            "Custom kitchen design and installation",
            "Custom-built cabinetry and storage solutions",
            "Flooring installation throughout the property",
            "Loft and upper-level reconstruction",
            "Electrical and lighting upgrades",
            "Interior finishing and custom woodworking",
            "Space optimization and functional redesign",
        ],
        comparisons: vec![
            Comparison {
                before: asset!("/assets/img/foto_web/island-retreat-exterior-before-02.jpg").to_string(),
                after: asset!("/assets/img/foto_web/island-retreat-exterior-after-07.jpg").to_string(),
                label: "Cabin exterior",
            },
            Comparison {
                before: asset!("/assets/img/foto_web/island-retreat-exterior-after-02.jpg").to_string(),
                after: asset!("/assets/img/foto_web/island-retreat-exterior-after-03.jpg").to_string(),
                label: "Waterfront deck",
            },
            Comparison {
                before: asset!("/assets/img/foto_web/island-retreat-interior-before-08.jpg").to_string(),
                after: asset!("/assets/img/foto_web/island-retreat-interior-after-07.jpg").to_string(),
                label: "Kitchen",
            },
            Comparison {
                before: asset!("/assets/img/foto_web/bathroom-after-01.jpg").to_string(),
                after: asset!("/assets/img/foto_web/bathroom-after-02.jpg").to_string(),
                label: "Bathroom",
            },
        ],
        gallery: vec![
            // процесс стройки (демонтаж / каркас / черновой пол)
            asset!("/assets/img/foto_web/island-retreat-interior-during-03.jpg").to_string(),
            asset!("/assets/img/foto_web/island-retreat-interior-during-08.jpg").to_string(),
            asset!("/assets/img/foto_web/island-retreat-interior-during-13.jpg").to_string(),
            // экстерьер: дом и новые палубы у воды
            asset!("/assets/img/foto_web/island-retreat-exterior-after-05.jpg").to_string(),
            asset!("/assets/img/foto_web/island-retreat-exterior-after-06.jpg").to_string(),
            asset!("/assets/img/foto_web/island-retreat-exterior-after-08.jpg").to_string(),
            asset!("/assets/img/foto_web/island-retreat-exterior-after-09.jpg").to_string(),
            // жилые зоны: гостиная, столовая, лофт, спальни
            asset!("/assets/img/foto_web/island-retreat-interior-after-04.jpg").to_string(),
            asset!("/assets/img/foto_web/island-retreat-interior-after-05.jpg").to_string(),
            asset!("/assets/img/foto_web/island-retreat-interior-after-06.jpg").to_string(),
            asset!("/assets/img/foto_web/island-retreat-interior-after-10.jpg").to_string(),
            asset!("/assets/img/foto_web/island-retreat-interior-after-12.jpg").to_string(),
            asset!("/assets/img/foto_web/island-retreat-interior-after-15.jpg").to_string(),
            asset!("/assets/img/foto_web/island-retreat-interior-after-19.jpg").to_string(),
            asset!("/assets/img/foto_web/island-retreat-interior-after-20.jpg").to_string(),
            asset!("/assets/img/foto_web/island-retreat-interior-after-21.jpg").to_string(),
            asset!("/assets/img/foto_web/island-retreat-interior-after-22.jpg").to_string(),
            asset!("/assets/img/foto_web/island-retreat-interior-after-24.jpg").to_string(),
            // санузел
            asset!("/assets/img/foto_web/island-retreat-interior-after-18.jpg").to_string(),
            asset!("/assets/img/foto_web/island-retreat-interior-after-25.jpg").to_string(),
            asset!("/assets/img/foto_web/island-retreat-interior-after-27.jpg").to_string(),
            // детали: кастомные деревянные элементы
            asset!("/assets/img/foto_web/island-retreat-interior-after-13.jpg").to_string(),
            asset!("/assets/img/foto_web/custom-woodwork-01.jpg").to_string(),
            asset!("/assets/img/foto_web/custom-woodwork-02.jpg").to_string(),
        ],
        location: "Remote Island, Sunshine Coast, British Columbia",
        completed_by: "Skillful Hands Solutions LTD",
        related_services: vec![
            ("renovations", "Renovations"),
            ("outdoor-living", "Decks, BBQ & Pergola"),
        ],
    }
}
