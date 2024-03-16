extern crate yew_app_core;
mod app;
mod components;
mod hooks;
mod router;
mod utils;
use app::App;
use yew::prelude::*;
use yew_app_core::{
    app::{Root, RootProps},
    component_registry::ComponentsRegistry,
    components::example_stories::{Card, InputFields},
};

fn main() {
    let mut components_registry = ComponentsRegistry::new();
    components_registry.register_component("Card", html! {<Card/>});
    components_registry.register_component("InputFields", html! {<InputFields/>});

    let story_components = components_registry.get_components();
    let app = html! {<App stories={story_components} />};

    yew::Renderer::<Root>::with_props(RootProps { children: app }).render();
}
