use yew::prelude::*;
use crate::components::container::Container;

#[function_component]
pub fn App() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

    html! {
        <Container>
        <h1 class="text-3xl font-bold underline"> { "Hello world!" } </h1>
            <button {onclick} class="h-12 px-6 m-2 text-lg bg-sky-400">{ "+1" }</button>
            <p>{ *counter }</p>
        </Container>
    }
}