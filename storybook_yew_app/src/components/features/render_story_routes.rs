use crate::router::Route;
use std::collections::HashMap;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(PartialEq, Properties)]
pub struct StoryRoutesProps {
    pub stories: HashMap<String, Html>,
}

#[function_component]
pub fn StoryRoutes(props: &StoryRoutesProps) -> Html {
    let StoryRoutesProps { stories } = props;
    html! {
        { for stories.keys().map(|id| {
            html! {
                <Link<Route>  to={Route::Story { id: id.to_string() }}>
                    <div class="group w-full flex justify-center border-l-4 border-slate-300">
                        <p class="w-4/5 group-hover:text-indigo-800 group-hover:bg-indigo-100 rounded p-2 font-medium text-base text-left truncate">
                            {id}
                        </p>
                    </div>
                </Link<Route>>
            }
        })}
    }
}
