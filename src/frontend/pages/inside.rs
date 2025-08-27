use yew::prelude::*;
use crate::models::Inside;
use crate::frontend::components::navigation::Navigation;
use crate::frontend::components::card::{Card, CardContent, CardFooter};
use crate::frontend::components::modal::Modal;
use crate::frontend::services::storage;

#[function_component(InsidePage)]
pub fn inside_page() -> Html {
    let insides = use_state(|| {
        let mut data = storage::get_insides();
        data.sort_by(|a, b| b.date.cmp(&a.date));
        data
    });
    let show_modal = use_state(|| false);
    let edit_uuid = use_state(|| None::<String>);
    let context_input = use_state(|| String::new());

    let _refresh_data = {
        let insides = insides.clone();
        Callback::from(move |_: MouseEvent| {
            let mut insides_data = storage::get_insides();
            insides_data.sort_by(|a, b| b.date.cmp(&a.date));
            insides.set(insides_data);
        })
    };

    let show_add_modal = {
        let show_modal = show_modal.clone();
        let edit_uuid = edit_uuid.clone();
        let context_input = context_input.clone();
        Callback::from(move |_: MouseEvent| {
            edit_uuid.set(None);
            context_input.set(String::new());
            show_modal.set(true);
        })
    };

    let show_edit_modal = {
        let show_modal = show_modal.clone();
        let edit_uuid = edit_uuid.clone();
        let context_input = context_input.clone();
        let insides = insides.clone();
        Callback::from(move |uuid: String| {
            if let Some(inside) = insides.iter().find(|i| i.uuid == uuid) {
                context_input.set(inside.context.clone());
                edit_uuid.set(Some(uuid));
                show_modal.set(true);
            }
        })
    };

    let close_modal = {
        let show_modal = show_modal.clone();
        Callback::from(move |_| {
            show_modal.set(false);
        })
    };

    let on_context_change = {
        let context_input = context_input.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlTextAreaElement = e.target_unchecked_into();
            context_input.set(input.value());
        })
    };

    let submit_inside = {
        let context_input = context_input.clone();
        let edit_uuid = edit_uuid.clone();
        let insides = insides.clone();
        let show_modal = show_modal.clone();
        Callback::from(move |_| {
            let context = (*context_input).clone();
            let uuid = (*edit_uuid).clone();
            
            let inside = Inside {
                uuid: uuid.clone().unwrap_or_else(|| uuid::Uuid::new_v4().to_string()),
                context,
                date: chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            };

            let result = if let Some(ref uuid) = uuid {
                storage::update_inside(uuid, inside)
            } else {
                storage::add_inside(inside)
            };

            match result {
                Ok(_) => {
                    let mut insides_data = storage::get_insides();
                    insides_data.sort_by(|a, b| b.date.cmp(&a.date));
                    insides.set(insides_data);
                    web_sys::console::log_1(&"Inside saved successfully!".into());
                }
                Err(e) => web_sys::console::log_1(&format!("Failed to save inside: {}", e).into()),
            }
            
            show_modal.set(false);
        })
    };

    let delete_inside_callback = {
        let insides = insides.clone();
        Callback::from(move |uuid: String| {
            match storage::delete_inside(&uuid) {
                Ok(_) => {
                    let mut insides_data = storage::get_insides();
                    insides_data.sort_by(|a, b| b.date.cmp(&a.date));
                    insides.set(insides_data);
                    web_sys::console::log_1(&"Inside deleted successfully!".into());
                }
                Err(e) => web_sys::console::log_1(&format!("Failed to delete inside: {}", e).into()),
            }
        })
    };

    html! {
        <>
            <Navigation title="Relf" />
            
            <div class="cards-container">
                {
                    insides.iter().map(|inside| {
                        let edit_callback = show_edit_modal.reform({
                            let uuid = inside.uuid.clone();
                            move |_| uuid.clone()
                        });
                        let delete_callback = delete_inside_callback.reform({
                            let uuid = inside.uuid.clone();
                            move |_| uuid.clone()
                        });
                        
                        html! {
                            <Card key={inside.uuid.clone()} class="inside-card">
                                <CardContent>
                                    <p>{&inside.context}</p>
                                </CardContent>
                                <CardFooter>
                                    <div class="card-meta">
                                        <span class="date">{&inside.date}</span>
                                    </div>
                                    <div class="card-actions">
                                        <button class="edit-btn" onclick={edit_callback}>{"✏️"}</button>
                                        <button class="delete-btn" onclick={delete_callback}>{"🗑️"}</button>
                                    </div>
                                </CardFooter>
                            </Card>
                        }
                    }).collect::<Html>()
                }
            </div>
            
            <button class="fab" onclick={show_add_modal}>{"+"}</button>
            
            <Modal 
                show={*show_modal} 
                on_close={close_modal} 
                title={""}
            >
                <form class="modal-form" onsubmit={Callback::from(|e: SubmitEvent| e.prevent_default())}>
                    <label for="context">{"Context:"}</label>
                    <textarea 
                        id="context" 
                        required=true
                        value={(*context_input).clone()}
                        oninput={on_context_change}
                        rows="10"
                    />
                    
                    <button type="button" id="submit-btn" onclick={submit_inside}>{"Submit"}</button>
                </form>
            </Modal>
            
        </>
    }
}