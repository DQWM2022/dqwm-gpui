#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod utils;

use gpui_kit::component::{ActiveTheme, Icon};
use gpui_kit::prelude::*;
use gpui_kit::*;

compose_assets!(AppAssets, "./assets");

pub struct RootView {}

impl RootView {
    fn new(_window: &mut Window, _cx: &mut Context<Self>) -> Self {
        Self {}
    }
}

impl Render for RootView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();

        div()
            .relative()
            .size_full()
            .bg(theme.background)
            .flex()
            .items_center()
            .justify_center()
            .child(Icon::default().path("qiang_black.svg").size(px(200.)))
    }
}

fn main() {
    application().with_assets(AppAssets).run(|cx: &mut App| {
        init(cx);

        let bounds = Bounds::centered(None, size(px(376.), px(731.)), cx);
        let opts = WindowOptions {
            app_id: Some("com.dqwm.gpui".into()),
            window_bounds: Some(WindowBounds::Windowed(bounds)),
            window_min_size: Some(size(px(376.), px(731.))),
            titlebar: Some(TitlebarOptions {
                title: Some("道起微末".into()),
                ..Default::default()
            }),
            ..Default::default()
        };

        open_window(opts, cx, |window, cx| {
            cx.new(|cx| RootView::new(window, cx))
        })
        .expect("打开窗口失败！");
    });
}
