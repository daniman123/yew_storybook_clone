use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct ToolBarProps {
    pub onclick_fullscreen: Callback<MouseEvent>,
    pub onclick_toolbar: Callback<MouseEvent>,
    pub onclick_outline: Callback<MouseEvent>,
}

#[function_component]
pub fn ToolBar(props: &ToolBarProps) -> Html {
    let ToolBarProps {
        onclick_fullscreen,
        onclick_outline,
        onclick_toolbar,
    } = props;
    html! {
        <div class="h-[5dvh] pl-2 flex shadow-2xl gap-2">
            <button onclick={onclick_fullscreen} class="border rounded-md text-xs bg-slate-400">{ "Full Screen" }</button>
            <button onclick={onclick_toolbar} class="border rounded-md text-xs bg-slate-400">{ "Toggle Toolbar" }</button>
            <button onclick={onclick_outline} class="border rounded-md text-xs bg-slate-400">{ "Toggle Element Outlines" }</button>
        </div>
    }
}
