use gpui_kit::component::IconName;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Nav {
    Dashboard,
    Users,
    Roles,
    Permissions,
}

impl Nav {
    pub fn label(self) -> &'static str {
        match self {
            Nav::Dashboard => "仪表盘",
            Nav::Users => "用户管理",
            Nav::Roles => "角色管理",
            Nav::Permissions => "权限管理",
        }
    }

    pub fn icon(self) -> IconName {
        match self {
            Nav::Dashboard => IconName::LayoutDashboard,
            Nav::Users => IconName::User,
            Nav::Roles => IconName::CircleUser,
            Nav::Permissions => IconName::Settings,
        }
    }

    /// 面包屑路径，从一级到当前页
    pub fn breadcrumb(self) -> &'static [&'static str] {
        match self {
            Nav::Dashboard => &["仪表盘"],
            Nav::Users => &["系统管理", "用户管理"],
            Nav::Roles => &["系统管理", "角色管理"],
            Nav::Permissions => &["系统管理", "权限管理"],
        }
    }

    /// 固定在标签栏、不可关闭的页面
    pub fn is_pinned(self) -> bool {
        matches!(self, Nav::Dashboard)
    }

    /// 侧边栏「系统管理」下的二级导航
    pub fn system_children() -> [Nav; 3] {
        [Nav::Users, Nav::Roles, Nav::Permissions]
    }
}
