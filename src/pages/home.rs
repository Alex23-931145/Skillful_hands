use dioxus::prelude::*;

use crate::components::{ContactForm, Icon, ReviewCard, ScrollLink};
use crate::Route;

// Hero-фон — реальное фото объекта. Через asset!(), чтобы base_path
// (GitHub Pages project-page /Skillful_hands/) корректно префиксил путь,
// а не грузил с корня сайта (иначе 404 на под-пути).
const HERO_IMG: Asset = asset!("/assets/img/hero.jpg");
// Фон CTA-баннера — реальный объект (через asset!() ради base-path на GitHub Pages).
const CTA_IMG: Asset = asset!("/assets/img/foto_web/island-retreat-exterior-after-07.jpg");
// Профиль компании в Google Maps (живые отзывы). Бейдж и CTA ведут сюда.
const GOOGLE_REVIEWS_URL: &str = "https://maps.app.goo.gl/9NdFEvng8LDeybFVA";
// Личное фото основателя (из HEIC → web-JPG). Показываем целиком, без обрезки.
const FOUNDER_IMG: Asset = asset!("/assets/img/founder.jpg");

// Фото-обложки 3 категорий (= hero соответствующих услуг, выбраны клиентом).
const SVC_RENO_IMG: Asset = asset!("/assets/img/foto_web/island-retreat-interior-after-22.jpg");
const SVC_DECK_IMG: Asset = asset!("/assets/img/foto_web/bbq-area-after-08.jpg");
const SVC_LAND_IMG: Asset = asset!("/assets/img/foto_web/landscaping-01.jpg");

/// Лендинг (Pencil C5tVy «Sweet Yards Landing» → ребренд Skillful Hands).
#[component]
pub fn Home() -> Element {
    rsx! {
        main {
            Hero {}
            WorkServices {}
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
            style: "background-image: linear-gradient(180deg, rgba(10,9,7,0.22) 0%, rgba(10,9,7,0.52) 50%, rgba(10,9,7,0.96) 100%), url({HERO_IMG});",
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

// ---------- Work / Services (объединённая секция) ----------
// 3 категории-услуги как фото-карточки → /services/:slug (там галерея работ).
// Заменяет прежние секции Projects + ServicesSection. id="projects" сохранён,
// чтобы ссылка «Projects» в футере вела сюда.
#[component]
fn WorkServices() -> Element {
    let services = [
        ("Renovations", "Interiors, kitchens, baths, and complete home renovations.", "renovations", SVC_RENO_IMG),
        ("Decks, BBQ & Pergola", "Custom decks, BBQ areas, pergolas, and patio zones.", "outdoor-living", SVC_DECK_IMG),
        ("Landscaping", "Property and waterfront landscaping. New photos coming soon.", "landscaping", SVC_LAND_IMG),
    ];
    rsx! {
        section { id: "projects", class: "home-section home-section--bg",
            div { class: "wrap",
                div { class: "sec-head",
                    div { class: "sec-head__left",
                        span { class: "sec-eyebrow", "OUR WORK" }
                        h2 { class: "sec-title", "See what we build" }
                    }
                    p { class: "sec-note", "Three things we do — tap any to see the full gallery of real projects." }
                }
                div { class: "work-grid",
                    for (title , desc , slug , img) in services {
                        Link {
                            key: "{slug}",
                            to: Route::ServiceDetail { slug: slug.to_string() },
                            class: "work-card",
                            div { class: "work-card__photo", style: "background-image:url({img})" }
                            div { class: "work-card__body",
                                h3 { class: "work-card__title", "{title}" }
                                p { class: "work-card__desc", "{desc}" }
                                span { class: "work-card__link",
                                    "View work"
                                    Icon { name: "arrow-right".to_string(), size: 15 }
                                }
                            }
                        }
                    }
                }
                div { class: "work-cta",
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
            img { class: "owner-card__photo", src: "{img}", alt: "{name}" }
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
                        span { class: "sec-eyebrow", "ABOUT US" }
                        h2 { class: "sec-title", "About Skillful Hands Solutions LTD" }
                    }
                    p { class: "sec-note", "Custom construction built on hard work, honesty, and quality craftsmanship." }
                }
                div { class: "owners",
                    div { class: "owner-card owner-card--founder",
                        img { class: "owner-card__photo", src: "{FOUNDER_IMG}", alt: "Aleksandr Dudchenko" }
                        div { class: "owner-card__body",
                            h3 { class: "owner-card__name", "Aleksandr Dudchenko" }
                            div { class: "owner-card__role", "Owner & Founder" }
                            p { class: "owner-card__bio", "My name is Aleksandr Dudchenko, and I am the founder of Skillful Hands Solutions LTD." }
                            p { class: "owner-card__bio", "I was born and raised in Ukraine in a farming family, where I learned the value of hard work, honesty, and dedication from an early age. Before immigrating to Canada, I studied Psychology at university in Ukraine." }
                            p { class: "owner-card__bio", "After moving to Canada, I lived in Saskatchewan, where I worked in the agricultural, oil, and construction industries. It was there that I founded a company specializing in agricultural fencing, which continues to operate successfully to this day. This experience helped me develop strong practical skills, discipline, and a deep understanding of project management and teamwork." }
                            p { class: "owner-card__bio", "After five years in Saskatchewan, I relocated to British Columbia and discovered my true passion for custom construction. In 2024, I founded Skillful Hands Solutions LTD with the goal of creating unique, high-quality projects built to stand the test of time." }
                            p { class: "owner-card__bio", "Today, I lead a team of experienced and dedicated professionals who successfully deliver projects of all sizes and levels of complexity. From home renovations and outdoor living spaces to custom cottages, landscaping, and waterfront projects, we are committed to quality craftsmanship, attention to detail, and complete customer satisfaction. We approach every project with responsibility, professionalism, and genuine passion for our work." }
                            p { class: "owner-card__bio", "At Skillful Hands Solutions LTD, we believe that construction is more than simply building structures. It is about creating comfortable, functional, and beautiful spaces that people can enjoy for years to come. For our team, no project is too challenging — every project is completed with dedication, professionalism, and pride in our craftsmanship." }
                        }
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
            style: "background-image: linear-gradient(0deg, #0a0907b3, #0a09078c), url({CTA_IMG});",
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
