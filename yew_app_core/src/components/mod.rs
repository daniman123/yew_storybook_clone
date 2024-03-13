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

#[derive(PartialEq, Properties)]
pub struct NewTestCompProps {}

#[function_component]
pub fn NewTestComp(props: &NewTestCompProps) -> Html {
    let NewTestCompProps {} = props;
    html! {
        <div> {"Testing NewTestComp Lib Comp"} </div>
    }
}

#[derive(PartialEq, Properties)]
pub struct GrabMeGGCompProps {}

#[function_component]
pub fn GrabMeGGComp(props: &GrabMeGGCompProps) -> Html {
    let GrabMeGGCompProps {} = props;
    html! {
        <h3> {" DATAER GIGI"} </h3>
    }
}
