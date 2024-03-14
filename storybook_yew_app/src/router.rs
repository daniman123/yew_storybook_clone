use std::collections::HashMap;

use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::layouts::primary_layout::layout::PrimaryLayout;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/story/:id")]
    Story { id: String },
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: Route, stories: HashMap<String, Html>) -> Html {
    match routes {
        Route::Home => html! { 
            <PrimaryLayout stories={stories}>
                <div>
                    {"Content"}
                </div>
            </PrimaryLayout>
        },
        Route::Story { id } => html! {
            <PrimaryLayout stories={stories.clone()}>
                <section id="story-root">
                    {
                        stories.get(&id).unwrap().clone()
                    }
                </section>
            </PrimaryLayout>
        },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}
