use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ModalProps {
    pub show: bool,
    pub on_close: Callback<()>,
    pub title: String,
    pub children: Children,
}

#[function_component(Modal)]
pub fn modal(props: &ModalProps) -> Html {
    let close_modal = {
        let on_close = props.on_close.clone();
        Callback::from(move |_| {
            on_close.emit(());
        })
    };

    let stop_propagation = Callback::from(|e: MouseEvent| {
        e.stop_propagation();
    });

    html! {
        <div class={classes!("modal", props.show.then_some("show"))} onclick={close_modal}>
            <div class="modal-content" onclick={stop_propagation}>
                if !props.title.is_empty() {
                    <h2>{&props.title}</h2>
                }
                {props.children.clone()}
            </div>
        </div>
    }
}