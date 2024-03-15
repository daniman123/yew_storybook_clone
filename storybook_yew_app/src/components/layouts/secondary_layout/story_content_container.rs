use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct StoryContainerProps {
    pub children: Html,
    pub story_content_style: String,
    pub toolbar_style: String,
    pub sidebar_style: String,
}

#[function_component]
pub fn StoryContainer(props: &StoryContainerProps) -> Html {
    let StoryContainerProps {
        children,
        story_content_style,
        toolbar_style,
        sidebar_style,
    } = props;
    html! {
        <section
            id={<std::string::String as Clone>::clone(story_content_style)}
            class={classes!(toolbar_style,"pl-1","pt-1","border","border-black","bg-white", <std::string::String as Clone>::clone(sidebar_style))}>
            {children}
        </section>
    }
}
