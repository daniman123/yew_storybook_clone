use crate::components::layouts::primary_layout::layout::ToolBarStates;
use std::ops::Deref;
use wasm_bindgen::JsCast;
use yew::prelude::*;

pub fn handle_toolbar_key_press(tool_bar_states: &UseStateHandle<ToolBarStates>) -> impl Fn(Event) {
    let tool_bar_states = tool_bar_states.clone();

    move |event: Event| {
        if let Some(keyboard_event) = event.dyn_ref::<yew::KeyboardEvent>() {
            match keyboard_event.key().as_str() {
                "f" => tool_bar_states.set(ToolBarStates {
                    is_sidebar_hidden: !tool_bar_states.is_sidebar_hidden,
                    ..tool_bar_states.deref().clone()
                }),
                "t" => tool_bar_states.set(ToolBarStates {
                    is_toolbar_hidden: !tool_bar_states.is_toolbar_hidden,
                    ..tool_bar_states.deref().clone()
                }),
                "o" => tool_bar_states.set(ToolBarStates {
                    is_outlined: !tool_bar_states.is_outlined,
                    ..tool_bar_states.deref().clone()
                }),
                _ => {}
            }
        }
    }
}
