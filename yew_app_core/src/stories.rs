use crate::components::example_components::{
    Card, CardProps, InputFields, InputFieldsProps, JumboTron, JumboTronProps, SearchInput,
    SearchInputProps, SearchInputSmall, SearchInputSmallProps,
};
use crate::{component_registry::ComponentsRegistry, register_components};
use yew::prelude::*;

pub fn render_stories(mut components_registry: ComponentsRegistry) -> ComponentsRegistry {
    register_components!(
        components_registry,
        (Card, CardProps {}),
        (
            InputFields,
            InputFieldsProps {
                test_prop: "First name".into()
            }
        ),
        (JumboTron, JumboTronProps {}),
        (SearchInput, SearchInputProps {}),
        (SearchInputSmall, SearchInputSmallProps {})
    );
    components_registry
}
