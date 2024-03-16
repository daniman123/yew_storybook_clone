use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct TitleBannerProps {
    pub label: String,
    pub styles: Vec<String>,
}

#[function_component]
pub fn TitleBanner(props: &TitleBannerProps) -> Html {
    let TitleBannerProps { label, styles } = props;
    let mut title_style = classes!("p-2", "font-bold", "text-center");
    title_style.extend(styles);

    html! {
        <h2 class={title_style}>
            {label}
        </h2>
    }
}
