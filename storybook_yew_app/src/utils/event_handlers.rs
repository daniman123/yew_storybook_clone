use wasm_bindgen::JsCast;
use yew::prelude::*;

pub fn handle_toolbar_key_press(
    is_sidebar_hidden: &UseStateHandle<bool>,
    is_toolbar_hidden: &UseStateHandle<bool>,
    is_outlined: &UseStateHandle<bool>,
) -> impl Fn(Event) {
    let is_sidebar_hidden = is_sidebar_hidden.clone();
    let is_toolbar_hidden = is_toolbar_hidden.clone();
    let is_outlined: UseStateHandle<bool> = is_outlined.clone();
    move |event: Event| {
        if let Some(keyboard_event) = event.dyn_ref::<yew::KeyboardEvent>() {
            match keyboard_event.key().as_str() {
                "f" => is_sidebar_hidden.set(!*is_sidebar_hidden),
                "t" => is_toolbar_hidden.set(!*is_toolbar_hidden),
                "o" => is_outlined.set(!*is_outlined),
                _ => {}
            }
        }
    }
}
