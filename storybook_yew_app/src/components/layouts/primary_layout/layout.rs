use crate::components::layouts::primary_layout::side_bar::SideBar;
use crate::components::layouts::primary_layout::tool_bar::ToolBar;
use crate::components::layouts::secondary_layout::story_content_container::StoryContainer;
use crate::hooks::use_even_hook::use_event;
use crate::hooks::use_state_set_on_click::use_state_on_click_set_bool;
use crate::utils::event_handlers::handle_toolbar_key_press;
use gloo::utils::window;
use std::collections::HashMap;
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct PrimaryLayoutProps {
    pub children: Html,
    pub stories: HashMap<String, Html>,
}

#[function_component]
pub fn PrimaryLayout(props: &PrimaryLayoutProps) -> Html {
    let stories = props.stories.clone();

    let (is_sidebar_hidden, onclick_fullscreen) = use_state_on_click_set_bool();
    let (is_toolbar_hidden, onclick_toolbar) = use_state_on_click_set_bool();
    let (is_outlined, onclick_outline) = use_state_on_click_set_bool();

    use_event(
        &window(),
        "keypress",
        handle_toolbar_key_press(&is_sidebar_hidden, &is_toolbar_hidden, &is_outlined),
    );

    let sidebar_style = if *is_sidebar_hidden {
        "w-[100dvw]"
    } else {
        "w-[85dvw]"
    };

    let toolbar_style = if *is_toolbar_hidden {
        "h-[100dvh]"
    } else {
        "h-[95dvh]"
    };

    let story_content_style = if *is_outlined {
        "storybook-root-story-colored"
    } else {
        "storybook-root-story"
    };

    html! {
        <main class="flex h-full w-full">
            if !*is_sidebar_hidden {
                <section class="h-[100dvh] w-[15dvw] bg-slate-300  drop-shadow-xl">
                    <SideBar stories={stories}/>
                </section>
            }
            <section class={classes!("h-[100dvh]", sidebar_style, "bg-slate-300")}>
                if !*is_toolbar_hidden {
                    <ToolBar
                        {onclick_fullscreen}
                        {onclick_toolbar}
                        {onclick_outline}
                    />
                }
                <StoryContainer
                    {story_content_style}
                    { toolbar_style}
                    { sidebar_style}
                >
                    {props.children.clone()}
                </StoryContainer>
            </section>
        </main>
    }
}
