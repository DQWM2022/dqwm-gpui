/// 组合自定义资源与 GPUI Kit 内置资源
///
/// # 用法
/// ```ignore
/// compose_assets!(AppAssets, "./assets");
/// ```
#[macro_export]
macro_rules! compose_assets {
    ($name:ident, $path:literal) => {
        #[derive(::rust_embed::RustEmbed)]
        #[folder = $path]
        struct $name;

        impl ::gpui_kit::AssetSource for $name {
            fn load(
                &self,
                path: &str,
            ) -> ::gpui_kit::private::anyhow::Result<Option<std::borrow::Cow<'static, [u8]>>> {
                if path.is_empty() {
                    return Ok(None);
                }
                if let Some(file) = <$name>::get(path) {
                    return Ok(Some(file.data));
                }
                ::gpui_kit::assets::Assets.load(path)
            }

            fn list(
                &self,
                path: &str,
            ) -> ::gpui_kit::private::anyhow::Result<Vec<::gpui_kit::SharedString>> {
                let mut paths = ::gpui_kit::assets::Assets.list(path)?;
                paths.extend(<$name>::iter().filter_map(|p| p.starts_with(path).then(|| p.into())));
                paths.sort();
                paths.dedup();
                Ok(paths)
            }
        }
    };
}
