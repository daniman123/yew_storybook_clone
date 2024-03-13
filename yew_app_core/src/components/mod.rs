use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct TestCompProps {}

#[function_component]
pub fn TestComp(props: &TestCompProps) -> Html {
    let TestCompProps {} = props;
    html! {
        <div> {"Testing Lib Comp"} </div>
    }
}
