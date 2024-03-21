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

pub fn create_toggle_callback(
    tool_bar_states: &UseStateHandle<ToolBarStates>,
    toggle_field: fn(&mut ToolBarStates),
) -> Callback<MouseEvent> {
    let is_state_bool = tool_bar_states.clone();
    Callback::from(move |_| {
        let mut new_state = is_state_bool.deref().clone();
        toggle_field(&mut new_state);
        is_state_bool.set(new_state);
    })
}