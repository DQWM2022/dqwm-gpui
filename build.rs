use std::env;
use std::error::Error;
use std::fs::File;
use std::io::BufWriter;
use std::path::{Path, PathBuf};

use ico::{IconDir, IconDirEntry, IconImage, ResourceType};
use image::imageops::FilterType;
use winresource::WindowsResource;

/// 生成的 ico 包含这些尺寸，覆盖 Windows 任务栏、开始菜单、Alt+Tab 各场景
const SIZES: &[u32] = &[16, 32, 48, 64, 128, 256];

fn png_to_ico(png_path: &Path, ico_path: &Path) -> Result<(), Box<dyn Error>> {
    let png = image::open(png_path)?;
    let mut icon_dir = IconDir::new(ResourceType::Icon);

    for &size in SIZES {
        // 每个尺寸单独缩放，Lanczos3 保证小图边缘干净
        let resized = png.resize_exact(size, size, FilterType::Lanczos3);
        let rgba = resized.to_rgba8();
        let (w, h) = rgba.dimensions();

        let image = IconImage::from_rgba_data(w, h, rgba.into_raw());
        let entry = IconDirEntry::encode(&image)?;
        icon_dir.add_entry(entry);
    }

    let file = File::create(ico_path)?;
    let mut writer = BufWriter::new(file);
    icon_dir.write(&mut writer)?;
    Ok(())
}

fn main() {
    // 只在 Windows 目标上做资源编译
    if env::var_os("CARGO_CFG_WINDOWS").is_none() {
        return;
    }

    // 改源文件后要重新跑 build.rs
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=assets/logo.png");

    // ico 写到 OUT_DIR，不污染项目目录
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR not set"));
    let ico_path = out_dir.join("app.ico");

    // 路径相对于 Cargo.toml 所在目录
    png_to_ico(Path::new("assets/logo.png"), &ico_path).expect("构建阶段 >> logo.png 转 ico 失败");

    WindowsResource::new()
        .set_icon(ico_path.to_str().expect("ico path not utf-8"))
        .compile()
        .expect("构建阶段 >> 图标嵌入二进制文件失败");
}
