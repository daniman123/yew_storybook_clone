use std::collections::HashMap;
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct AppProps {
    pub stories: HashMap<String, Html>,
}

#[function_component]
pub fn App(props: &AppProps) -> Html {
    let AppProps { stories } = props;
    html! {
        <div>
            { for stories.iter().map(|(_key,item)| item.clone()) }
        </div>
    }
}
