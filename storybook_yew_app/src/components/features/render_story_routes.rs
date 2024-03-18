use crate::router::Route;
use std::collections::HashMap;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(PartialEq, Properties)]
pub struct StoryRoutesProps {
    pub stories: HashMap<String, Html>,
    pub current_story_id: String,
}

#[function_component]
pub fn StoryRoutes(props: &StoryRoutesProps) -> Html {
    let StoryRoutesProps {
        stories,
        current_story_id,
    } = props;

    html! {
        { for stories.keys().map(|id| {
            html! {
                <Link<Route>  to={Route::Story { id: id.to_string() }}>
                    <div class={classes!("group","w-full", "flex", "relative","h-full", "justify-center", "border-l-4", if current_story_id == id {"border-indigo-500"}else{"border-slate-300"})}>
                        <p class="w-4/5 group-hover:text-indigo-800 group-hover:bg-indigo-100 rounded p-2 font-medium text-base text-left truncate">
                            {id}
                        </p>
                        if current_story_id == id {
                            <div class="flex-initial flex justify-center items-center w-2 p-2">
                                <div class="h-2 aspect-square bg-indigo-500 rounded-full"></div>
                            </div>
                        }
                    </div>
                </Link<Route>>
            }
        }) }
    }
}
