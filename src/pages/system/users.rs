use gpui_kit::base::StyledExt as _;
use gpui_kit::component::{v_flex, ActiveTheme as _};
use gpui_kit::{div, App, IntoElement, ParentElement as _, RenderOnce, Styled as _, Window};

#[derive(IntoElement)]
pub struct UsersPage;

impl RenderOnce for UsersPage {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        v_flex()
            .gap_3()
            .child(div().text_lg().font_semibold().child("用户管理"))
            .child(
                div()
                    .text_sm()
                    .text_color(theme.muted_foreground)
                    .child("这里放用户列表。"),
            )
    }
}
