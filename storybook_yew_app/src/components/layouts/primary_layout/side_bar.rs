use crate::{components::ui::title::TitleBanner, router::Route};
use std::{collections::HashMap, vec};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(PartialEq, Properties)]
pub struct SideBarProps {
    pub stories: HashMap<String, Html>,
}

#[function_component]
pub fn SideBar(props: &SideBarProps) -> Html {
    let SideBarProps { stories } = props;

    let title_style = vec![
        "text-3xl".to_string(),
        "border-b-4".to_string(),
        "rounded".to_string(),
    ];

    html! {
        <div class="w-full px-2">
            <TitleBanner label={"Stories"} styles={title_style} />
            <div class="w-full py-5">
                { for stories.keys().map(|id| {
                    html! {
                        <Link<Route>  to={Route::Story { id: id.to_string() }}>
                            <div class="group w-full flex justify-center border-l-4 border-slate-500">
                                <p class="w-3/4 group-hover:bg-slate-400 rounded p-2 text-lg text-left truncate">
                                    {id}
                                </p>
                            </div>
                        </Link<Route>>
                    }
                })}
            </div>
        </div>
    }
}
