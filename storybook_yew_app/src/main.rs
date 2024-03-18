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
    components::example_components::{
        Card, CardProps, InputFields, InputFieldsProps, JumboTron, JumboTronProps, SearchInput,
        SearchInputProps, SearchInputSmall, SearchInputSmallProps,
    },
    register_components,
};

fn main() {
    let mut components_registry = ComponentsRegistry::new();

    register_components!(
        components_registry,
        (Card, CardProps {}),
        (
            InputFields,
            InputFieldsProps {
                test_prop: "First name".into()
            }
        ),
        (JumboTron, JumboTronProps {}),
        (SearchInput, SearchInputProps {}),
        (SearchInputSmall, SearchInputSmallProps {})
    );

    let story_components = components_registry.get_components();
    let app = html! {<App stories={story_components} />};

    yew::Renderer::<Root>::with_props(RootProps { children: app }).render();
}
