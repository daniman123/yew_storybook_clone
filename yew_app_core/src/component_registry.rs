use std::collections::HashMap;
use yew::prelude::*;

#[derive(Default, Clone)]
pub struct ComponentsRegistry {
    pub components_map: HashMap<String, Html>,
}

impl ComponentsRegistry {
    pub fn new() -> Self {
        ComponentsRegistry {
            ..Default::default()
        }
    }

    pub fn register_component(&mut self, k: impl Into<String>, v: Html) {
        self.components_map.insert(k.into(), v);
    }

    pub fn get_components(&self) -> HashMap<String, Html> {
        self.components_map.clone()
    }
}

#[macro_export]
macro_rules! register_components {
    ($registry:expr, $(($component:ident, $props:expr)),*) => {
        $(
            $registry.register_component(stringify!($component), html! {<$component ..$props />});
        )*
    };
}
