#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod app;
mod nav;
mod pages;

use gpui_kit::assets::Assets as ComponentAssets;
use gpui_kit::private::anyhow;
use gpui_kit::{
    px, size, App, AppContext, AssetSource, Bounds,
    SharedString, TitlebarOptions, WindowBackgroundAppearance, WindowBounds, WindowOptions,
};
use rust_embed::RustEmbed;
use std::borrow::Cow;

// 把 assets/ 目录下所有文件编进二进制
#[derive(RustEmbed)]
#[folder = "./assets"]
struct AppFiles;

struct AppAssets;

impl AssetSource for AppAssets {
    fn load(&self, path: &str) -> anyhow::Result<Option<Cow<'static, [u8]>>> {
        if path.is_empty() {
            return Ok(None);
        }
        if let Some(file) = AppFiles::get(path) {
            return Ok(Some(file.data));
        }
        // 回退到 gpui-kit 的内置图标
        ComponentAssets.load(path)
    }

    fn list(&self, path: &str) -> anyhow::Result<Vec<SharedString>> {
        let mut paths = ComponentAssets.list(path)?;
        paths.extend(AppFiles::iter().filter_map(|p| p.starts_with(path).then(|| p.into())));
        paths.sort();
        paths.dedup();
        Ok(paths)
    }
}

fn main() {
    gpui_kit::application()
        .with_assets(AppAssets)
        .run(|cx: &mut App| {
            gpui_kit::init(cx);

            let bounds = Bounds::centered(None, size(px(800.), px(600.)), cx);
            let opts = WindowOptions {
                app_id: Some("com.dqwm.gpui".into()),
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                window_min_size: Some(size(px(600.), px(400.))),
                window_background: WindowBackgroundAppearance::MicaBackdrop,
                // Windows 拖动的前提：窗口必须"可移动"
                is_movable: true,
                titlebar: Some(TitlebarOptions {
                    title: Some("道起微末".into()),
                    appears_transparent: true,
                    ..Default::default()
                }),
                // macOS：声明应用自己处理标题栏拖动，避免点击延迟
                app_owns_titlebar_drag: true,
                ..Default::default()
            };

            gpui_kit::open_window(opts, cx, |window, cx| {
                cx.new(|cx| app::AdminApp::new(window, cx))
            })
            .expect("打开窗口失败！");
        });
}
