use std::collections::HashMap;

use yew::prelude::*;
use yew_router::prelude::*;

use crate::router::Route;

#[derive(PartialEq, Properties)]
pub struct SideBarProps {
    pub stories: HashMap<String, Html>,
}

#[function_component]
pub fn SideBar(props: &SideBarProps) -> Html {
    let SideBarProps { stories } = props;
    html! {
        <div class="w-full px-2">
            <h2 class="p-2 text-3xl font-semibold border-b-4 rounded text-center">
                {"Stories"}
            </h2>
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
