use yew::prelude::*;

use crate::components::ui::button::Button;

#[derive(PartialEq, Properties)]
pub struct ToolBarProps {
    pub onclick_fullscreen: Callback<MouseEvent>,
    pub onclick_toolbar: Callback<MouseEvent>,
    pub onclick_outline: Callback<MouseEvent>,
}

#[function_component]
pub fn ToolBar(props: &ToolBarProps) -> Html {
    let ToolBarProps {
        onclick_fullscreen,
        onclick_outline,
        onclick_toolbar,
    } = props;

    let button_styles = vec!["".to_string()];

    let toolbar_style = classes!(
        "h-[5dvh]",
        "pl-2",
        "py-2",
        " flex",
        "gap-2",
        "border-b",
        "border-gray-200",
        "shadow-md"
    );

    html! {
        <div class={toolbar_style}>
            <Button styles={button_styles.clone()} label="Full Screen" onclick={onclick_fullscreen}/>
            <Button styles={button_styles.clone()} label="Toggle Toolbar" onclick={onclick_toolbar}/>
            <Button styles={button_styles.clone()} label="Toggle Element Outlines" onclick={onclick_outline}/>
        </div>
    }
}
