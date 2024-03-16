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

    let section_styling = classes!(toolbar_style, "pl-2", "pt-2", "bg-white", sidebar_style);

    html! {
        <section
            id={story_content_style.clone()}
            class={section_styling}>
            {children}
        </section>
    }
}
