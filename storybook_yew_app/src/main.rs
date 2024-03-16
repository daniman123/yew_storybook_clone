mod app;
mod components;
mod hooks;
mod router;
mod utils;
extern crate yew_app_core;
use app::App;
use yew::prelude::*;
use yew_app_core::{
    app::{Root, RootProps},
    component_registry::ComponentsRegistry,
    components::{Card, CardWithImage, GrabMeGGComp, InputFields, NewTestComp, TestComp},
};

fn main() {
    let mut components_registry = ComponentsRegistry::new();
    components_registry.register_component("Test", html! {<TestComp/>});
    components_registry.register_component("NewTestComp", html! {<NewTestComp/>});
    components_registry.register_component("GrabMeGGComp", html! {<GrabMeGGComp/>});
    components_registry.register_component("Card", html! {<Card/>});
    components_registry.register_component("CardWithImage", html! {<CardWithImage/>});
    components_registry.register_component("InputFields", html! {<InputFields/>});

    let story_components = components_registry.get_components();
    let app = html! {<App stories={story_components} />};

    yew::Renderer::<Root>::with_props(RootProps { children: app }).render();
}
