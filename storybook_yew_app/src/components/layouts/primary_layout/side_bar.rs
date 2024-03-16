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
            <div class="w-full py-3">
                { for stories.keys().map(|id| {
                    html! {
                        <Link<Route>  to={Route::Story { id: id.to_string() }}>
                            <div class="group w-full flex justify-center">
                                <p class="w-3/4 group-hover:border-b-2 font-semibold text-center">
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
