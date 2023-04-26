use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ContainerProps {
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component]
pub fn Container(props: &ContainerProps) -> Html {
    html! {
        <div class={classes!(
            "container",
            props.class.clone()
        )}>
            {for props.children.iter()}
        </div>
    }
}