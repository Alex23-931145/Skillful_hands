use dioxus::prelude::*;

use crate::components::Icon;

/// Инициалы из имени (макс. 2 буквы) для аватара-заглушки.
/// "James R." → "JR", "David & Lin" → "DL".
fn initials(name: &str) -> String {
    name.split_whitespace()
        .filter_map(|w| w.chars().next())
        .filter(|c| c.is_alphabetic())
        .take(2)
        .collect::<String>()
        .to_uppercase()
}

/// Карточка отзыва (Pencil Sq5op).
#[component]
pub fn ReviewCard(
    quote: String,
    name: String,
    role: String,
    #[props(default = 5)] stars: i32,
) -> Element {
    let avatar = initials(&name);
    rsx! {
        div { class: "review-card",
            div { class: "review-card__stars",
                for i in 0..stars {
                    Icon { key: "{i}", name: "star".to_string(), size: 18 }
                }
            }
            p { class: "review-card__quote", "\u{201C}{quote}\u{201D}" }
            div { class: "review-card__author",
                div { class: "review-card__avatar", "{avatar}" }
                div {
                    div { class: "review-card__name", "{name}" }
                    div { class: "review-card__role", "{role}" }
                }
            }
        }
    }
}
