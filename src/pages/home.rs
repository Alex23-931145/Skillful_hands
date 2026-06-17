use dioxus::prelude::*;

use crate::components::{ContactForm, Icon, ReviewCard, ScrollLink, ServiceCard};
use crate::pages::project_tiles;
use crate::Route;

// Hero-фон — реальное фото объекта. Через asset!(), чтобы base_path
// (GitHub Pages project-page /Skillful_hands/) корректно префиксил путь,
// а не грузил с корня сайта (иначе 404 на под-пути).
const HERO_IMG: Asset = asset!("/assets/img/hero.jpg");
// Фон CTA-баннера — реальный объект (через asset!() ради base-path на GitHub Pages).
const CTA_IMG: Asset = asset!("/assets/img/foto_web/island-retreat-exterior-after-07.jpg");
// Профиль компании в Google Maps (живые отзывы). Бейдж и CTA ведут сюда.
const GOOGLE_REVIEWS_URL: &str = "https://maps.app.goo.gl/9NdFEvng8LDeybFVA";

/// Лендинг (Pencil C5tVy «Sweet Yards Landing» → ребренд Skillful Hands).
#[component]
pub fn Home() -> Element {
    rsx! {
        main {
            Hero {}
            Projects {}
            ServicesSection {}
            WhyUs {}
            Owners {}
            Testimonials {}
            CtaBand {}
            ContactForm {}
        }
    }
}

// ---------- Hero (hv0zC) ----------
#[component]
fn Hero() -> Element {
    rsx! {
        section {
            class: "hero",
            style: "background-image: linear-gradient(180deg, rgba(10,9,7,0.15) 0%, rgba(10,9,7,0.34) 52%, rgba(10,9,7,0.95) 100%), url({HERO_IMG});",
            div { class: "hero__inner",
                div { class: "hero__eyebrow",
                    Icon { name: "map-pin".to_string(), size: 14 }
                    span { "Sunshine Coast, BC  ·  Since 2024" }
                }
                h1 { class: "hero__headline", "Custom Construction — One Call Does It All" }
                p { class: "hero__sub",
                    "Custom renovations, decks, outdoor living spaces, and landscaping on the Sunshine Coast."
                }
                div { class: "hero__ctas",
                    Link { to: Route::Services {}, class: "btn", "Explore our services" }
                    ScrollLink { target: "contact", class: "btn btn--light", "Get a free quote" }
                }
            }
        }
    }
}

// ---------- Projects (GpsRO) ----------
// Плитка проекта → переход на /projects/:slug (детальная страница-кейс).
#[component]
fn ProjectTile(slug: String, label: String, img: String, #[props(default = false)] big: bool) -> Element {
    rsx! {
        Link {
            to: Route::ProjectDetail { slug },
            class: if big { "tile tile--big" } else { "tile" },
            style: "background-image:url('{img}')",
            span { class: "tile__chip", "{label}" }
        }
    }
}

#[component]
fn Projects() -> Element {
    // 3 реальных проекта (slug, подпись, фото) из project_detail.rs.
    // Раскладка big+2: первый — крупный, остальные два — в правой колонке.
    let tiles = project_tiles();
    rsx! {
        section { id: "projects", class: "home-section home-section--surface",
            div { class: "wrap",
                div { class: "sec-head",
                    div { class: "sec-head__left",
                        span { class: "sec-eyebrow", "OUR WORK" }
                        h2 { class: "sec-title", "See some of our past projects" }
                    }
                }
                div { class: "gallery-top",
                    if let Some((slug , label , img)) = tiles.first() {
                        ProjectTile {
                            big: true,
                            slug: slug.to_string(),
                            label: label.to_string(),
                            img: img.clone(),
                        }
                    }
                    div { class: "gallery-top__right",
                        for (slug , label , img) in tiles.iter().skip(1) {
                            ProjectTile {
                                key: "{slug}",
                                slug: slug.to_string(),
                                label: label.to_string(),
                                img: img.clone(),
                            }
                        }
                    }
                }
            }
        }
    }
}

// ---------- Services (MMmfm) ----------
#[component]
fn ServicesSection() -> Element {
    let services = [
        ("hammer", "Renovations", "Interior, exterior, and complete home renovations and restoration — with custom design and planning.", "renovations"),
        ("trees", "Outdoor Living Spaces", "Custom decks, pergolas, and BBQ areas built for durability, function, and relaxed outdoor living.", "outdoor-living"),
        ("house", "Cottages & Cabins", "Custom cottages, cabins, guest houses, and other unique custom-built structures.", "cottages-cabins"),
        ("leaf", "Landscaping", "Landscaping and property improvement for residential and waterfront properties.", "landscaping"),
    ];
    rsx! {
        section { id: "services", class: "home-section home-section--bg",
            div { class: "wrap",
                div { class: "sec-head",
                    div { class: "sec-head__left",
                        span { class: "sec-eyebrow", "WHAT WE DO" }
                        h2 { class: "sec-title", "Custom construction, one trusted team" }
                    }
                    p { class: "sec-note", "From renovations to outdoor living — one local crew that does it all on the Sunshine Coast." }
                }
                div { class: "service-grid",
                    for (icon , title , desc , slug) in services {
                        ServiceCard {
                            key: "{slug}",
                            icon: icon.to_string(),
                            title: title.to_string(),
                            desc: desc.to_string(),
                            slug: slug.to_string(),
                        }
                    }
                    // CTA-карточка вместо 6-й услуги
                    div { class: "service-cta",
                        div {
                            h3 { class: "service-cta__title", "Not sure where to start?" }
                            p { class: "service-cta__desc",
                                "Tell us about your project and we'll map out a plan that fits your space and budget."
                            }
                        }
                        ScrollLink { target: "contact", class: "btn service-cta__btn",
                            "Book a consult"
                            Icon { name: "arrow-right".to_string(), size: 16 }
                        }
                    }
                }
            }
        }
    }
}

// ---------- Why Us (C5Ccj) ----------
#[component]
fn WhyUs() -> Element {
    let values = [
        ("award", "Quality Craftsmanship", "Exceptional craftsmanship and attention to detail on every project."),
        ("heart-handshake", "Customer Satisfaction", "Clear communication and reliable service from first call to final walkthrough."),
        ("truck", "Remote-Area Ready", "Experienced with logistics and delivery in remote, hard-to-access locations."),
        ("shield-check", "Licensed & Insured", "Fully insured and committed to integrity on every job."),
    ];
    let stats = [
        ("12+", "Years of experience"),
        ("100%", "Custom — built to your vision"),
        ("4", "Core construction services"),
        ("2024", "Locally established, BC"),
    ];
    rsx! {
        section { id: "why", class: "home-section home-section--forest",
            div { class: "wrap",
                div { class: "sec-head",
                    div { class: "sec-head__left",
                        span { class: "sec-eyebrow sec-eyebrow--soft", "WHY SKILLFUL HANDS" }
                        h2 { class: "sec-title sec-title--light", "Built on craftsmanship, trust, and 12 years of experience" }
                    }
                    p { class: "sec-note sec-note--light", "We treat every project like our own — and stand behind the result." }
                }
                div { class: "values",
                    for (icon , title , desc) in values {
                        div { key: "{title}", class: "value-card",
                            span { class: "value-card__icon", Icon { name: icon.to_string(), size: 24 } }
                            h3 { class: "value-card__title", "{title}" }
                            p { class: "value-card__desc", "{desc}" }
                        }
                    }
                }
                div { class: "stats",
                    for (num , label) in stats {
                        div { key: "{label}", class: "stat",
                            div { class: "stat__num", "{num}" }
                            div { class: "stat__label", "{label}" }
                        }
                    }
                }
            }
        }
    }
}

// ---------- Owners (ukTs8) ----------
#[component]
fn OwnerCard(name: String, role: String, bio: String, img: String) -> Element {
    rsx! {
        div { class: "owner-card owner-card--founder",
            div { class: "owner-card__photo", style: "background-image:url('{img}')" }
            div { class: "owner-card__body",
                h3 { class: "owner-card__name", "{name}" }
                div { class: "owner-card__role", "{role}" }
                p { class: "owner-card__bio", "{bio}" }
            }
        }
    }
}

#[component]
fn Owners() -> Element {
    rsx! {
        section { id: "about", class: "home-section home-section--surface",
            div { class: "wrap",
                div { class: "sec-head",
                    div { class: "sec-head__left",
                        span { class: "sec-eyebrow", "MEET THE FOUNDER" }
                        h2 { class: "sec-title", "Meet Aleksandr, founder of Skillful Hands" }
                    }
                    p { class: "sec-note", "Hands-on craftsmanship and a commitment to doing the job right." }
                }
                div { class: "owners",
                    OwnerCard {
                        name: "Aleksandr Dudchenko",
                        role: "Owner & Founder",
                        bio: "Aleksandr Dudchenko is the founder of Skillful Hands Solutions LTD. He specializes in custom construction, renovations, landscaping, and outdoor living projects, with a strong focus on quality craftsmanship and customer satisfaction.",
                        img: "https://images.unsplash.com/photo-1602752709993-9ab17ac8cf0d?crop=entropy&cs=tinysrgb&fit=max&fm=jpg&q=80&w=1080",
                    }
                }
            }
        }
    }
}

// ---------- Testimonials (yGEU9) ----------
#[component]
fn Testimonials() -> Element {
    let reviews = [
        ("James R.", "Sechelt, BC", "They handled our full home renovation start to finish. Professional, on time, and the craftsmanship is outstanding."),
        ("Megan T.", "Gibsons, BC", "Our new deck and outdoor living space turned out beautifully — done right the first time. We use it every single day."),
        ("David & Lin", "Roberts Creek, BC", "Honest pricing and incredible attention to detail. Skillful Hands is the only crew we'll call from now on."),
    ];
    rsx! {
        section { id: "testimonials", class: "home-section home-section--bg",
            div { class: "wrap",
                div { class: "sec-head",
                    div { class: "sec-head__left",
                        span { class: "sec-eyebrow", "WHAT CLIENTS SAY" }
                        h2 { class: "sec-title", "Loved by homeowners on the Coast" }
                    }
                    a {
                        class: "google-badge",
                        href: GOOGLE_REVIEWS_URL,
                        target: "_blank",
                        rel: "noopener",
                        "aria-label": "Read Skillful Hands reviews on Google",
                        Icon { name: "star".to_string(), size: 18 }
                        span { "5.0 on Google Reviews" }
                    }
                }
                div { class: "reviews",
                    for (name , role , quote) in reviews {
                        ReviewCard {
                            key: "{name}",
                            name: name.to_string(),
                            role: role.to_string(),
                            quote: quote.to_string(),
                        }
                    }
                }
                a {
                    class: "reviews-cta",
                    href: GOOGLE_REVIEWS_URL,
                    target: "_blank",
                    rel: "noopener",
                    "See all reviews on Google"
                    Icon { name: "arrow-up-right".to_string(), size: 16 }
                }
            }
        }
    }
}

// ---------- CTA Band (GeAR4) ----------
#[component]
fn CtaBand() -> Element {
    rsx! {
        section {
            class: "cta-band",
            style: "background-image: linear-gradient(0deg, #0a0907e6, #0a0907e6), url({CTA_IMG});",
            div { class: "cta-band__inner",
                span { class: "sec-eyebrow sec-eyebrow--soft", "LET'S BUILD SOMETHING GREAT" }
                h2 { class: "cta-band__headline", "Ready to start your project?" }
                p { class: "cta-band__sub",
                    "Book a free on-site consultation and get a clear, no-pressure plan for your project."
                }
                div { class: "cta-band__btns",
                    ScrollLink { target: "contact", class: "btn btn--light", "Get a free quote" }
                    a { href: "tel:+17782396704", class: "btn btn--outline",
                        "Call (778) 239-6704"
                    }
                }
            }
        }
    }
}
