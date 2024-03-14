use std::collections::HashMap;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::router::{switch, Route};

#[derive(PartialEq, Properties)]
pub struct AppProps {
    pub stories: HashMap<String, Html>,
}

#[function_component]
pub fn App(props: &AppProps) -> Html {
    let stories = props.stories.clone();
    html! {
        <BrowserRouter>
            <Switch<Route> render={move |route: Route| switch(route, stories.clone())} />
        </BrowserRouter>
    }
}
