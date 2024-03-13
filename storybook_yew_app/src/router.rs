use std::collections::HashMap;

use yew::prelude::*;
use yew_router::prelude::*;

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
        Route::Home => html! { { for stories.keys().map(|id| {
            html! {
                <h3>
                    <Link<Route>  to={Route::Story { id: id.to_string() }}>
                        {id}
                    </Link<Route>>
                </h3>
            }
        })} },
        Route::Story { id } => html! {
            <div>
                {
                    stories.get(&id).unwrap().clone()
                }
            </div>
        },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}
