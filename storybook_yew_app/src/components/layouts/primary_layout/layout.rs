use crate::components::layouts::primary_layout::side_bar::SideBar;
use crate::components::layouts::primary_layout::tool_bar::ToolBar;
use crate::components::layouts::secondary_layout::story_content_container::StoryContainer;
use std::collections::HashMap;
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct PrimaryLayoutProps {
    pub children: Html,
    pub stories: HashMap<String, Html>,
    pub id: Option<String>,
}

#[derive(Default, Clone, PartialEq)]
pub struct ToolBarStates {
    pub is_sidebar_hidden: bool,
    pub is_toolbar_hidden: bool,
    pub is_outlined: bool,
}

#[function_component]
pub fn PrimaryLayout(props: &PrimaryLayoutProps) -> Html {
    let stories = props.stories.clone();
    let id = props.id.clone();

    let tool_bar_states = use_state(ToolBarStates::default);

    let ToolBarStates {
        is_outlined,
        is_sidebar_hidden,
        is_toolbar_hidden,
    } = *tool_bar_states;

    let sidebar_style = if is_sidebar_hidden {
        "w-[100dvw]"
    } else {
        "w-[85dvw]"
    };

    let toolbar_style = if is_toolbar_hidden {
        "h-[100dvh]"
    } else {
        "h-[95dvh]"
    };

    let story_content_style = if is_outlined {
        "storybook-root-story-colored"
    } else {
        "storybook-root-story"
    };

    html! {
        <main class="flex h-full w-full">
            if !is_sidebar_hidden {
                <section class="h-[100dvh] w-[15dvw] bg-gray-50 border-r border-gray-200 shadow-md">
                    <SideBar stories={stories} id={id.unwrap()} />
                </section>
            }
            <section class={classes!("h-[100dvh]", sidebar_style, "bg-white")}>
                    <ToolBar
                        {tool_bar_states}
                        {is_toolbar_hidden}
                    />
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
