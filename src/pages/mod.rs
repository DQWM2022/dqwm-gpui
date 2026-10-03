pub mod dashboard;
pub mod system;

use gpui_kit::{AnyElement, IntoElement};

use crate::nav::Nav;

pub fn page_for(nav: Nav) -> AnyElement {
    match nav {
        Nav::Dashboard => dashboard::DashboardPage.into_any_element(),
        Nav::Users => system::users::UsersPage.into_any_element(),
        Nav::Roles => system::roles::RolesPage.into_any_element(),
        Nav::Permissions => system::permissions::PermissionsPage.into_any_element(),
    }
}
