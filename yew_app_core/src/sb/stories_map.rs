use std::collections::HashMap;
use yew::prelude::*;

#[derive(Default)]
pub struct StoriesMap {
    pub map: HashMap<String, HashMap<String, Html>>,
}

impl StoriesMap {
    pub fn new() -> Self {
        StoriesMap {
            ..Default::default()
        }
    }

    pub fn register_component(
        &mut self,
        component_category: String,
        component_name: String,
        component: Html,
    ) {
        let mut component_data = HashMap::new();
        component_data.insert(component_name, component);

        self.map
            .entry(component_category)
            .or_insert_with(|| component_data);
    }
}

// -TODO:
// struct Stories
// map: HashMap<String,HashMap<String, Html>>

// fn populate_map(component_category:String, component_name:String, component:Html) -> void
// - cat_hash:HashMap<String, Html> = map.get(component_category)
// - cat_hash.insert(component_name, component)
