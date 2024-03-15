mod app;
mod router;
mod components;
mod hooks;
mod utils;
extern crate yew_app_core;
use app::App;
use yew::prelude::*;
use yew_app_core::{
    app::{Root, RootProps},
    component_registry::ComponentsRegistry,
    components::{NewTestComp, TestComp,GrabMeGGComp},
};

fn main() {
    let mut components_registry = ComponentsRegistry::new();
    components_registry.register_component("Test", html! {<TestComp/>});
    components_registry.register_component("NewTestComp", html! {<NewTestComp/>});
    components_registry.register_component("GrabMeGGComp", html! {<GrabMeGGComp/>});
    let story_components = components_registry.get_components();

    let app = html! {<App stories={story_components} />};

    yew::Renderer::<Root>::with_props(RootProps { children: app }).render();
}
