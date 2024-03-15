use yew::prelude::*;

#[hook]
pub fn use_state_on_click_set_bool() -> (UseStateHandle<bool>, Callback<MouseEvent>) {
    let is_state_bool = use_state(|| false);
    let onclick: Callback<MouseEvent> = {
        let is_state_bool = is_state_bool.clone();
        Callback::from(move |_| is_state_bool.set(!*is_state_bool))
    };

    (is_state_bool, onclick)
}
