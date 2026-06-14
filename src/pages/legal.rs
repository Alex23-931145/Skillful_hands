use dioxus::prelude::*;

/// Privacy Policy (Pencil FV65D). Title Band + Content; Header/Footer — из Layout.
#[component]
pub fn Privacy() -> Element {
    let sections = [
        (
            "Introduction",
            "Skillful Hands Solutions Ltd. (\"we\", \"us\", \"our\") respects your privacy. This policy explains what information we collect, how we use it, and the rights you have regarding it.",
        ),
        (
            "Information we collect",
            "We collect information you provide directly — such as your name, email, phone number, and project details when you submit our contact form — along with basic technical data like your IP address and browser type.",
        ),
        (
            "How we use your information",
            "We use your information to respond to inquiries, prepare quotes, deliver our services, and improve our website. We never sell your personal information to third parties.",
        ),
        (
            "Cookies",
            "Our website uses cookies to remember your preferences and understand how visitors use the site. You can accept or decline non-essential cookies at any time using our cookie banner, or disable them in your browser settings.",
        ),
        (
            "Sharing & disclosure",
            "We share information only with trusted service providers who help us operate our business, or when required by law. All partners are bound by confidentiality obligations.",
        ),
        (
            "Your rights",
            "You may request access to, correction of, or deletion of your personal information at any time. To exercise these rights, simply contact us using the details below.",
        ),
        (
            "Contact us",
            "Questions about this policy? Reach us at skillfulhandsbc@gmail.com or (778) 239-6704. Skillful Hands Solutions Ltd., serving the Sunshine Coast, BC.",
        ),
    ];

    rsx! {
        main {
            LegalBand {
                title: "Privacy Policy".to_string(),
                updated: "Last updated June 12, 2026".to_string(),
            }
            LegalContent { sections: sections.to_vec() }
        }
    }
}

/// Terms of Service (Pencil PHUHz). Title Band + Content; Header/Footer — из Layout.
#[component]
pub fn Terms() -> Element {
    let sections = [
        (
            "Agreement to terms",
            "By accessing or using the Skillful Hands Solutions Ltd. website, you agree to be bound by these Terms of Service. If you do not agree, please discontinue use of our site.",
        ),
        (
            "Use of our website",
            "You may use our website for lawful purposes only. You agree not to misuse the site, attempt to disrupt it, or use it in any way that could harm Skillful Hands or other visitors.",
        ),
        (
            "Quotes & estimates",
            "Any quotes or estimates provided through this website are preliminary and subject to an on-site assessment. Final pricing is confirmed in a written agreement before any work begins.",
        ),
        (
            "Intellectual property",
            "All content on this site — including text, images, logos, and project photos — is the property of Skillful Hands Solutions Ltd. and may not be reproduced without our written permission.",
        ),
        (
            "Limitation of liability",
            "Our website is provided \"as is\" without warranties of any kind. Skillful Hands is not liable for any damages arising from your use of the site or reliance on its content.",
        ),
        (
            "Changes to these terms",
            "We may update these Terms from time to time. Continued use of the website after changes are posted constitutes acceptance of the revised Terms.",
        ),
        (
            "Contact us",
            "Questions about these Terms? Reach us at skillfulhandsbc@gmail.com or (778) 239-6704. Skillful Hands Solutions Ltd., serving the Sunshine Coast, BC.",
        ),
    ];

    rsx! {
        main {
            LegalBand {
                title: "Terms of Service".to_string(),
                updated: "Last updated June 12, 2026".to_string(),
            }
            LegalContent { sections: sections.to_vec() }
        }
    }
}

/// Title Band — тёмная полоса с крупным заголовком и датой обновления.
#[component]
fn LegalBand(title: String, updated: String) -> Element {
    rsx! {
        section { class: "legal-band",
            div { class: "wrap legal-band__inner",
                h1 { class: "legal-band__title", "{title}" }
                p { class: "legal-band__updated", "{updated}" }
            }
        }
    }
}

/// Content — узкая колонка с чередой заголовков и абзацев.
#[component]
fn LegalContent(sections: Vec<(&'static str, &'static str)>) -> Element {
    rsx! {
        section { class: "legal-content",
            div { class: "legal-content__col",
                for (heading , body) in sections {
                    div { key: "{heading}", class: "legal-section",
                        h2 { class: "legal-section__heading", "{heading}" }
                        p { class: "legal-section__body", "{body}" }
                    }
                }
            }
        }
    }
}
