use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct CardProps {
    pub children: Children,
    #[prop_or_default]
    pub class: String,
}

#[function_component(Card)]
pub fn card(props: &CardProps) -> Html {
    html! {
        <div class={classes!("card", props.class.clone())}>
            {props.children.clone()}
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct CardHeaderProps {
    pub children: Children,
}

#[function_component(CardHeader)]
pub fn card_header(props: &CardHeaderProps) -> Html {
    html! {
        <div class="card-header">
            {props.children.clone()}
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct CardContentProps {
    pub children: Children,
}

#[function_component(CardContent)]
pub fn card_content(props: &CardContentProps) -> Html {
    html! {
        <div class="card-content">
            {props.children.clone()}
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct CardFooterProps {
    pub children: Children,
}

#[function_component(CardFooter)]
pub fn card_footer(props: &CardFooterProps) -> Html {
    html! {
        <div class="card-footer">
            {props.children.clone()}
        </div>
    }
}