use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct RootProps {
    pub children: Html,
}

#[function_component]
pub fn Root(props: &RootProps) -> Html {
    let RootProps { children } = props;
    html! {{ children.clone() }}
}
