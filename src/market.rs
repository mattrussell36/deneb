use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub market_id: String,
}

#[function_component]
pub fn Market(props: &Props) -> Html {
    use_effect_with_deps(|_| {

    }, props.market_id.clone());

    if props.market_id == "" {
        return html! {
            <h1 class="text-xl text-center mt-20">{"No market selected"}</h1>
        }
    }

    html! { 
        <div>{"market_id: "}{props.market_id.clone()}</div>
    }
}