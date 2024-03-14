use crate::components::layouts::primary_layout::side_bar::SideBar;
use gloo::events::EventListener;
use gloo::utils::window;
use std::collections::HashMap;
use wasm_bindgen::JsCast;
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct PrimaryLayoutProps {
    pub children: Html,
    pub stories: HashMap<String, Html>,
}

#[function_component]
pub fn PrimaryLayout(props: &PrimaryLayoutProps) -> Html {
    let stories = props.stories.clone();
    let children = props.children.clone();

    let is_sidebar_hidden = use_state(|| false);
    let onclick = {
        let is_sidebar_hidden = is_sidebar_hidden.clone();
        Callback::from(move |_| is_sidebar_hidden.set(!*is_sidebar_hidden))
    };

    let expanded_sidebar_style = "w-[85dvw]";
    let minimized_sidebar_style = "w-[100dvw]";

    let is_toolbar_hidden = use_state(|| false);
    let onclick_toolbar = {
        let is_toolbar_hidden = is_toolbar_hidden.clone();
        Callback::from(move |_| is_toolbar_hidden.set(!*is_toolbar_hidden))
    };

    let expanded_toolbar_style = "h-[95dvh]";
    let minimized_toolbar_style = "h-[100dvh]";

    let is_sidebar_hidden_clone = is_sidebar_hidden.clone();
    let is_toolbar_hidden_clone = is_toolbar_hidden.clone();
    
    use_effect(move || {
        let listener = EventListener::new(&window(), "keypress", move |event: &Event| {
            if let Some(keyboard_event) = event.dyn_ref::<yew::KeyboardEvent>() {
                let key_pressed = keyboard_event.key();

                if key_pressed == "f" {
                    is_sidebar_hidden_clone.set(!*is_sidebar_hidden_clone)
                };
                if key_pressed == "t" {
                    is_toolbar_hidden_clone.set(!*is_toolbar_hidden_clone)
                };
            }
        });

        move || drop(listener)
    });

    html! {
        <main class="flex h-full w-full">
            if !*is_sidebar_hidden {
                <section class="h-[100dvh] w-[15dvw] bg-slate-300 shadow-2xl">
                    <SideBar stories={stories}/>
                </section>
            }
            <section class={classes!("h-[100dvh]", if !*is_sidebar_hidden {expanded_sidebar_style}else{minimized_sidebar_style}, "bg-slate-300")}>
                if !*is_toolbar_hidden {
                    <div class="h-[5dvh] flex shadow-2xl">
                        <button {onclick}>{ "Full Screen" }</button>
                        <button onclick={onclick_toolbar}>{ "Toggle Toolbar" }</button>
                    </div>
                }
                <section class={classes!(if !*is_toolbar_hidden {expanded_toolbar_style}else{minimized_toolbar_style},"pl-1","pt-1","border","border-black","bg-white", if !*is_sidebar_hidden {expanded_sidebar_style}else{minimized_sidebar_style})}>
                    {children}
                </section>
            </section>
        </main>
    }
}
