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
    stories::render_stories,
};

fn main() {
    let components_registry = ComponentsRegistry::new();
    let components_registry = render_stories(components_registry);

    let story_components = components_registry.get_components();
    let app = html! {<App stories={story_components} />};

    yew::Renderer::<Root>::with_props(RootProps { children: app }).render();
}
