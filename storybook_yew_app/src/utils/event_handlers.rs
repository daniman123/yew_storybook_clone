use crate::components::layouts::primary_layout::layout::ToolBarStates;
use std::ops::Deref;
use wasm_bindgen::JsCast;
use yew::prelude::*;

pub fn handle_toolbar_key_press(tool_bar_states: UseStateHandle<ToolBarStates>) -> impl Fn(Event) {
    move |event: Event| {
        if let Some(keyboard_event) = event.dyn_ref::<KeyboardEvent>() {
            let mut new_state = tool_bar_states.deref().clone();

            match keyboard_event.key().as_str() {
                "f" => new_state.is_sidebar_hidden = !new_state.is_sidebar_hidden,
                "t" => new_state.is_toolbar_hidden = !new_state.is_toolbar_hidden,
                "o" => new_state.is_outlined = !new_state.is_outlined,
                _ => {}
            }

            tool_bar_states.set(new_state);
        }
    }
}
