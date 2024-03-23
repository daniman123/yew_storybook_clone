use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct StoryContainerProps {
    pub children: Html,
    pub is_outlined: bool,
}

#[function_component]
pub fn StoryContainer(props: &StoryContainerProps) -> Html {
    let StoryContainerProps {
        children,
        is_outlined,
    } = props;

    let section_styling = classes!(
        "relative",
        "z-0",
        "max-w-full",
        "h-full",
        "pl-2",
        "pt-2",
        "bg-white",
    );

    html! {
        <section
        id={if *is_outlined {"storybook-root-story-colored"} else {"storybook-root-story"}}
        class={section_styling}>
            {children}
        </section>
    }
}
