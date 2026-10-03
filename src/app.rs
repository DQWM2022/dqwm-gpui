use crate::nav::Nav;
use crate::pages;
use gpui_kit::base::StyledExt as _;
use gpui_kit::component::menu::ContextMenuExt;
use gpui_kit::component::{
    button::{Button, ButtonVariants as _}, h_flex, input::{Input, InputState}, sidebar::{Sidebar, SidebarCollapsible, SidebarHeader, SidebarMenu, SidebarMenuItem}, v_flex, ActiveTheme as _,
    Icon,
    IconName,
    Sizable,
    Theme,
    ThemeMode,
};
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::{
    actions, div, img, px, AnyElement, AppContext as _, Context,
    CursorStyle, Entity, InteractiveElement, IntoElement, MouseButton, ParentElement as _,
    Render, StatefulInteractiveElement, Styled as _, Window, WindowControlArea,
};

actions!(
    tabs,
    [CloseAllTabs, CloseLeftTabs, CloseRightTabs, CloseOtherTabs,]
);

pub struct AdminApp {
    active: Nav,
    tabs: Vec<Nav>,
    settings_open: bool,
    logged_in: bool,
    user_menu_open: bool,
    username: Entity<InputState>,
    password: Entity<InputState>,
}

impl AdminApp {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let username = cx.new(|cx| InputState::new(window, cx).placeholder("用户名"));
        let password = cx.new(|cx| InputState::new(window, cx).placeholder("密码"));

        Self {
            active: Nav::Dashboard,
            tabs: vec![Nav::Dashboard],
            settings_open: false,
            logged_in: false,
            user_menu_open: false,
            username,
            password,
        }
    }

    fn open_tab(&mut self, nav: Nav, cx: &mut Context<Self>) {
        if !self.tabs.contains(&nav) {
            self.tabs.push(nav);
        }
        self.active = nav;
        cx.notify();
    }

    fn close_tab(&mut self, nav: Nav, cx: &mut Context<Self>) {
        if nav.is_pinned() || self.tabs.len() <= 1 {
            return;
        }
        let Some(idx) = self.tabs.iter().position(|&n| n == nav) else {
            return;
        };
        self.tabs.remove(idx);
        if self.active == nav {
            let next = idx.min(self.tabs.len() - 1);
            self.active = self.tabs[next];
        }
        cx.notify();
    }

    fn on_close_all_tabs(&mut self, _: &CloseAllTabs, _: &mut Window, cx: &mut Context<Self>) {
        self.tabs.retain(|n| n.is_pinned());
        if let Some(&first) = self.tabs.first() {
            self.active = first;
        }
        cx.notify();
    }

    fn on_close_left_tabs(&mut self, _: &CloseLeftTabs, _: &mut Window, cx: &mut Context<Self>) {
        let Some(idx) = self.tabs.iter().position(|&n| n == self.active) else {
            return;
        };
        self.tabs = self
            .tabs
            .iter()
            .enumerate()
            .filter(|(i, nav)| *i >= idx || nav.is_pinned())
            .map(|(_, nav)| *nav)
            .collect();
        cx.notify();
    }

    fn on_close_right_tabs(&mut self, _: &CloseRightTabs, _: &mut Window, cx: &mut Context<Self>) {
        let Some(idx) = self.tabs.iter().position(|&n| n == self.active) else {
            return;
        };
        self.tabs = self
            .tabs
            .iter()
            .enumerate()
            .filter(|(i, nav)| *i <= idx || nav.is_pinned())
            .map(|(_, nav)| *nav)
            .collect();
        cx.notify();
    }

    fn on_close_other_tabs(&mut self, _: &CloseOtherTabs, _: &mut Window, cx: &mut Context<Self>) {
        let active = self.active;
        self.tabs.retain(|&n| n == active || n.is_pinned());
        cx.notify();
    }

    // ---- 自定义标题栏 ----

    fn render_titlebar(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();

        let drag_area = h_flex()
            .id("titlebar-drag")
            .flex_1()
            .h_full()
            .cursor(CursorStyle::Arrow)
            .window_control_area(WindowControlArea::Drag)
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|_, _, window, _| {
                    window.start_window_move();
                }),
            );

        let minimize_btn = h_flex()
            .id("win-minimize")
            .w(px(42.))
            .h_full()
            .items_center()
            .justify_center()
            .cursor(CursorStyle::Arrow)
            .hover(|s| s.bg(theme.muted))
            .on_click(cx.listener(|_, _, window, _| {
                window.minimize_window();
            }))
            .child(Icon::new(IconName::Minus).small());

        let close_btn = h_flex()
            .id("win-close")
            .w(px(42.))
            .h_full()
            .items_center()
            .justify_center()
            .cursor(CursorStyle::Arrow)
            .hover(|s| s.bg(theme.danger).text_color(theme.danger_foreground))
            .on_click(cx.listener(|_, _, window, _| {
                window.remove_window();
            }))
            .child(Icon::new(IconName::Close).small());

        h_flex()
            .w_full()
            .h(px(32.))
            .items_center()
            .bg(theme.background)
            .child(drag_area)
            .child(minimize_btn)
            .child(close_btn)
    }

    // ---- 登录界面 ----

    fn render_login(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();

        div()
            .size_full()
            .flex()
            .items_center()
            .justify_center()
            .bg(theme.background)
            .child(
                v_flex()
                    .w(px(360.))
                    .p_6()
                    .gap_4()
                    .rounded(theme.radius)
                    .bg(theme.popover)
                    .border_1()
                    .border_color(theme.border)
                    .child(
                        v_flex()
                            .gap_2()
                            .items_center()
                            .child(img("logo.png").w(px(48.)).h(px(48.)))
                            .child(div().text_lg().font_semibold().child("道起微末"))
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(theme.muted_foreground)
                                    .child("请登录以继续"),
                            ),
                    )
                    .child(Input::new(&self.username).w_full())
                    .child(Input::new(&self.password).w_full())
                    .child(
                        Button::new("login")
                            .primary()
                            .label("登录")
                            .w_full()
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.logged_in = true;
                                cx.notify();
                            })),
                    ),
            )
    }

    // ---- 用户下拉菜单 ----

    fn render_user_menu(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();

        v_flex()
            .absolute()
            .top(px(78.))
            .right(px(16.))
            .min_w(px(140.))
            .py_1()
            .rounded(theme.radius)
            .bg(theme.popover)
            .border_1()
            .border_color(theme.border)
            .child(
                div()
                    .id("menu-logout")
                    .w_full()
                    .px_3()
                    .py_2()
                    .cursor(CursorStyle::PointingHand)
                    .hover(|s| s.bg(theme.muted))
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.user_menu_open = false;
                        this.logged_in = false;
                        cx.notify();
                    }))
                    .child(div().text_sm().child("退出登录")),
            )
    }

    // ---- 面包屑 + 右上角用户区 ----

    fn render_breadcrumb(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        let crumbs = self.active.breadcrumb();
        let settings_open = self.settings_open;
        let is_dark = matches!(theme.mode, ThemeMode::Dark);

        let mut trail = h_flex().gap_2().items_center();
        for (i, crumb) in crumbs.iter().enumerate() {
            let is_last = i + 1 == crumbs.len();
            if i > 0 {
                trail = trail.child(
                    div()
                        .text_sm()
                        .text_color(theme.muted_foreground)
                        .child("/"),
                );
            }
            if is_last {
                trail = trail.child(div().font_semibold().child(*crumb));
            } else {
                trail = trail.child(
                    div()
                        .text_sm()
                        .text_color(theme.muted_foreground)
                        .child(*crumb),
                );
            }
        }

        let gear = h_flex()
            .id("settings-toggle")
            .items_center()
            .justify_center()
            .p_1()
            .rounded(theme.radius)
            .cursor(CursorStyle::PointingHand)
            .when(settings_open, |el| el.bg(theme.accent))
            .on_click(cx.listener(|this, _, _, cx| {
                this.settings_open = !this.settings_open;
                cx.notify();
            }))
            .child(Icon::new(IconName::Settings));

        let theme_toggle = h_flex()
            .id("theme-toggle")
            .items_center()
            .justify_center()
            .p_1()
            .rounded(theme.radius)
            .cursor(CursorStyle::PointingHand)
            .on_click(cx.listener(|_this, _, _, cx| {
                let next = if matches!(cx.theme().mode, ThemeMode::Dark) {
                    ThemeMode::Light
                } else {
                    ThemeMode::Dark
                };
                let _ = Theme::change(next, None, cx);
            }))
            .child(Icon::new(if is_dark {
                IconName::Moon
            } else {
                IconName::Sun
            }));

        let user_trigger = h_flex()
            .id("user-trigger")
            .gap_2()
            .items_center()
            .px_2()
            .py_1()
            .rounded(theme.radius)
            .cursor(CursorStyle::PointingHand)
            .hover(|s| s.bg(theme.muted))
            .on_click(cx.listener(|this, _, _, cx| {
                this.user_menu_open = !this.user_menu_open;
                cx.notify();
            }))
            .child(div().text_sm().child("admin"))
            .child(Icon::new(IconName::ChevronDown).small());

        let user_area = h_flex()
            .gap_2()
            .items_center()
            .child(gear)
            .child(theme_toggle)
            .child(user_trigger);

        v_flex()
            .w_full()
            .child(
                h_flex()
                    .w_full()
                    .h(px(48.))
                    .px_4()
                    .items_center()
                    .justify_between()
                    .bg(theme.background)
                    .child(trail)
                    .child(user_area),
            )
            .child(div().w_full().h(px(1.)).bg(theme.border))
    }

    // ---- 标签栏 ----

    fn render_tabs(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        let active = self.active;
        let can_close_any = self.tabs.len() > 1;

        let tabs: Vec<_> = self
            .tabs
            .iter()
            .map(|&nav| {
                let is_active = nav == active;
                let can_close = can_close_any && !nav.is_pinned();

                h_flex()
                    .id(("tab", nav as usize))
                    .h(px(28.))
                    .px_3()
                    .gap_2()
                    .items_center()
                    .rounded(theme.radius)
                    .when(is_active, |el| {
                        el.bg(theme.background).text_color(theme.foreground)
                    })
                    .when(!is_active, |el| el.text_color(theme.muted_foreground))
                    .on_mouse_down(
                        MouseButton::Right,
                        cx.listener(move |this, _, _, cx| {
                            if this.active != nav {
                                this.active = nav;
                                cx.notify();
                            }
                        }),
                    )
                    .on_click(cx.listener(move |this, _, _, cx| {
                        this.active = nav;
                        cx.notify();
                    }))
                    .context_menu(|menu, _, _| {
                        menu.menu("全部关闭", Box::new(CloseAllTabs))
                            .separator()
                            .menu("关闭左侧", Box::new(CloseLeftTabs))
                            .menu("关闭右侧", Box::new(CloseRightTabs))
                            .separator()
                            .menu("关闭其他", Box::new(CloseOtherTabs))
                    })
                    .child(Icon::new(nav.icon()))
                    .child(div().text_sm().child(nav.label()))
                    .when(can_close, |el| {
                        el.child(
                            h_flex()
                                .id(("close", nav as usize))
                                .items_center()
                                .justify_center()
                                .on_click(cx.listener(move |this, _, _, cx| {
                                    cx.stop_propagation();
                                    this.close_tab(nav, cx);
                                }))
                                .child(Icon::new(IconName::Close)),
                        )
                    })
            })
            .collect();

        v_flex()
            .w_full()
            .child(
                h_flex()
                    .w_full()
                    .h(px(40.))
                    .px_2()
                    .gap_1()
                    .items_center()
                    .bg(theme.muted)
                    .children(tabs),
            )
            .child(div().w_full().h(px(1.)).bg(theme.border))
    }

    // ---- 右侧配置浮层 ----

    fn render_settings_panel(&self, cx: &Context<Self>) -> impl IntoElement {
        let theme = cx.theme();

        v_flex()
            .w(px(280.))
            .h_full()
            .p_4()
            .gap_3()
            .bg(theme.popover)
            .border_l_1()
            .border_color(theme.border)
            .child(div().font_semibold().child("设置"))
            .child(
                div()
                    .text_sm()
                    .text_color(theme.muted_foreground)
                    .child("配置项待实现。"),
            )
    }

    // ---- 侧边栏 ----

    fn render_sidebar(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let active = self.active;

        let dashboard_item = SidebarMenuItem::new(Nav::Dashboard.label())
            .icon(Nav::Dashboard.icon())
            .active(active == Nav::Dashboard)
            .on_click(cx.listener(|this, _, _, cx| {
                this.open_tab(Nav::Dashboard, cx);
            }));

        let system_submenu: Vec<_> = Nav::system_children()
            .iter()
            .map(|&nav| {
                SidebarMenuItem::new(nav.label())
                    .icon(nav.icon())
                    .active(active == nav)
                    .on_click(cx.listener(move |this, _, _, cx| {
                        this.open_tab(nav, cx);
                    }))
            })
            .collect();

        Sidebar::new("main-sidebar")
            .w(px(220.))
            .collapsible(SidebarCollapsible::None)
            .header(
                SidebarHeader::new().child(
                    h_flex()
                        .gap_2()
                        .items_center()
                        .child(img("logo.png").w(px(24.)).h(px(24.)))
                        .child(div().font_semibold().child("道起微末")),
                ),
            )
            .child(
                SidebarMenu::new().child(dashboard_item).child(
                    SidebarMenuItem::new("系统管理")
                        .icon(IconName::Settings)
                        .children(system_submenu),
                ),
            )
    }
}

impl Render for AdminApp {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if !self.logged_in {
            return v_flex()
                .size_full()
                .bg(cx.theme().background)
                .child(self.render_titlebar(cx))
                .child(
                    v_flex()
                        .flex_1()
                        .min_h_0()
                        .w_full()
                        .child(self.render_login(cx)),
                )
                .into_any_element();
        }

        let menu: Option<AnyElement> = if self.user_menu_open {
            Some(self.render_user_menu(cx).into_any_element())
        } else {
            None
        };

        let close_layer: Option<AnyElement> = if self.user_menu_open {
            Some(
                div()
                    .id("menu-close-layer")
                    .absolute()
                    .top_0()
                    .right_0()
                    .bottom_0()
                    .left_0()
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.user_menu_open = false;
                        cx.notify();
                    }))
                    .into_any_element(),
            )
        } else {
            None
        };

        let theme = cx.theme();
        let active = self.active;
        let settings_open = self.settings_open;

        let content = v_flex().flex_1().min_h_0().min_w_0().w_full().p_4().child(
            v_flex()
                .flex_1()
                .min_h_0()
                .w_full()
                .p_6()
                .rounded(theme.radius)
                .bg(theme.popover)
                .border_1()
                .border_color(theme.border)
                .child(pages::page_for(active)),
        );

        let main = h_flex()
            .relative()
            .flex_1()
            .min_h_0()
            .items_stretch()
            .w_full()
            .child(content)
            .when(settings_open, |el| {
                el.child(
                    div()
                        .absolute()
                        .top_0()
                        .right_0()
                        .bottom_0()
                        .child(self.render_settings_panel(cx)),
                )
            });

        h_flex()
            .relative()
            .items_stretch()
            .size_full()
            .key_context("MainApp")
            .on_action(cx.listener(Self::on_close_all_tabs))
            .on_action(cx.listener(Self::on_close_left_tabs))
            .on_action(cx.listener(Self::on_close_right_tabs))
            .on_action(cx.listener(Self::on_close_other_tabs))
            .bg(theme.background)
            .text_color(theme.foreground)
            .child(self.render_sidebar(cx))
            .child(
                v_flex()
                    .flex_1()
                    .min_w_0()
                    .h_full()
                    .child(self.render_titlebar(cx))
                    .child(self.render_breadcrumb(cx))
                    .child(self.render_tabs(cx))
                    .child(main),
            )
            .when_some(close_layer, |el, layer| el.child(layer))
            .when_some(menu, |el, m| el.child(m))
            .into_any_element()
    }
}
