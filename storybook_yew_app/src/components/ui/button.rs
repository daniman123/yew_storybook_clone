use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct ButtonProps {
    pub onclick: Callback<MouseEvent>,
    pub label: String,
    pub styles: Vec<String>,
}

#[function_component]
pub fn Button(props: &ButtonProps) -> Html {
    let ButtonProps {
        onclick,
        label,
        styles,
    } = props;

    let mut button_style = classes!(
        "rounded-md",
        "text-xs",
        "text-white",
        "font-semibold",
        "px-2",
        "hover:bg-blue-900",
        "bg-blue-800"
    );
    button_style.extend(styles);

    html! {
        <button {onclick} class={button_style}>{label}</button>
    }
}
