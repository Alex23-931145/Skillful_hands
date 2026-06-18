// Страницы сайта. Каждая — свой файл + свой css в assets/css/<page>.css.
mod home;
mod legal;
mod project_detail;
mod service_detail;
mod services;

pub use home::Home;
pub use legal::{Privacy, Terms};
pub use project_detail::{project_card, ProjectDetail};
pub use service_detail::ServiceDetail;
pub use services::Services;
