use crate::components::{features::render_story_routes::StoryRoutes, ui::title::TitleBanner};
use std::{collections::HashMap, vec};
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct SideBarProps {
    pub stories: HashMap<String, Html>,
}

#[function_component]
pub fn SideBar(props: &SideBarProps) -> Html {
    let title_style = vec!["text-3xl".to_string()];
    let segment_width = "w-full";

    html! {
        <div class={classes!(segment_width,"px-2")}>
            <div class={classes!(segment_width)}>
                <TitleBanner label={"StoryBook"} styles={title_style} />
            </div>
            <div class={classes!(segment_width,"py-5")}>
                <StoryRoutes stories={props.stories.clone()} />
            </div>
        </div>
    }
}
