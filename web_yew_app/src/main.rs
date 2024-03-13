extern crate yew_app_core;
use yew::prelude::*;
use yew_app_core::{
    app::{Root, RootProps},
    components::TestComp,
};

#[derive(PartialEq, Properties)]
pub struct AppProps {}

#[function_component]
pub fn App(props: &AppProps) -> Html {
    let AppProps {} = props;
    html! {
        <>
            <h2> {"Main App!"} </h2>
            <TestComp />
        </>
    }
}

fn main() {
    yew::Renderer::<Root>::with_props(RootProps {
        children: html! {<App/>},
    })
    .render();
}
