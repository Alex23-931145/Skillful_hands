use dioxus::prelude::*;

use crate::components::{CookieBanner, Footer, Header};
use crate::pages::{Home, Privacy, ProjectDetail, ServiceDetail, Services, Terms};

const MAIN_CSS: Asset = asset!("/assets/main.css");
// Все стили грузим на уровне App (он не размонтируется при навигации),
// иначе пер-страничный document::Stylesheet подгружается асинхронно при
// первом рендере страницы → рывок без стилей (FOUC) на первом переходе.
const HOME_CSS: Asset = asset!("/assets/css/home.css");
const SERVICES_CSS: Asset = asset!("/assets/css/services.css");
const SERVICE_CSS: Asset = asset!("/assets/css/service.css");
const PROJECT_CSS: Asset = asset!("/assets/css/project.css");
const LEGAL_CSS: Asset = asset!("/assets/css/legal.css");
// Favicon через asset!() — base-path-safe на project-page /Skillful_hands/.
const FAVICON: Asset = asset!("/assets/img/logo.png");

/// Маршруты сайта. Layout оборачивает все страницы шапкой/подвалом/cookie.
#[derive(Routable, Clone, PartialEq)]
pub enum Route {
    #[layout(Layout)]
    #[route("/")]
    Home {},
    #[route("/about")]
    About {},
    #[route("/services")]
    Services {},
    #[route("/services/:slug")]
    ServiceDetail { slug: String },
    #[route("/projects/:slug")]
    ProjectDetail { slug: String },
    #[route("/privacy")]
    Privacy {},
    #[route("/terms")]
    Terms {},
}

#[component]
pub fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", r#type: "image/png", href: FAVICON }
        document::Stylesheet { href: MAIN_CSS }
        document::Stylesheet { href: HOME_CSS }
        document::Stylesheet { href: SERVICES_CSS }
        document::Stylesheet { href: SERVICE_CSS }
        document::Stylesheet { href: PROJECT_CSS }
        document::Stylesheet { href: LEGAL_CSS }
        Router::<Route> {}
    }
}

#[component]
fn Layout() -> Element {
    // Сброс скролла наверх при смене роута (SPA не делает это сам).
    // Якорный скролл внутри страницы — не смена роута, эффект не триггерит.
    let route = use_route::<Route>();
    use_effect(use_reactive((&route,), move |(_,)| {
        spawn(async move {
            let _ = document::eval("window.scrollTo(0, 0);").await;
        });
    }));

    rsx! {
        Header {}
        match route {
            Route::About {} => rsx! { Home {} },
            _ => rsx! { Outlet::<Route> {} },
        }
        Footer {}
        CookieBanner {}
    }
}
