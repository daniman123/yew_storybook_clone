use super::layout::ToolBarStates;
use crate::{
    components::ui::button::Button,
    hooks::use_even_hook::use_event,
    utils::event_handlers::{create_toggle_callback, handle_toolbar_key_press},
};
use gloo::utils::window;
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct ToolBarProps {
    pub tool_bar_states: UseStateHandle<ToolBarStates>,
    pub is_toolbar_hidden: bool,
}

#[function_component]
pub fn ToolBar(props: &ToolBarProps) -> Html {
    let tool_bar_states = props.tool_bar_states.clone();
    let is_toolbar_hidden = props.is_toolbar_hidden;

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
    use_event(
        &window(),
        "keypress",
        handle_toolbar_key_press(tool_bar_states.clone()),
    );

    let onclick_fullscreen = create_toggle_callback(&tool_bar_states, |state| {
        state.is_sidebar_hidden = !state.is_sidebar_hidden;
    });

    let onclick_toolbar = create_toggle_callback(&tool_bar_states, |state| {
        state.is_toolbar_hidden = !state.is_toolbar_hidden;
    });

    let onclick_outline = create_toggle_callback(&tool_bar_states, |state| {
        state.is_outlined = !state.is_outlined;
    });

    html! {
        if !is_toolbar_hidden {
            <div class={toolbar_style}>
                <Button styles={button_styles.clone()} label="Full Screen" onclick={onclick_fullscreen}/>
                <Button styles={button_styles.clone()} label="Toggle Toolbar" onclick={onclick_toolbar}/>
                <Button styles={button_styles.clone()} label="Toggle Element Outlines" onclick={onclick_outline}/>
            </div>
        }
    }
}
