use yew::prelude::*;
use crate::models::Outside;
use crate::frontend::components::navigation::Navigation;
use crate::frontend::components::card::{Card, CardHeader, CardContent, CardFooter};
use crate::frontend::components::modal::Modal;
use crate::frontend::services::storage;

#[function_component(OutsidePage)]
pub fn outside_page() -> Html {
    let outsides = use_state(|| {
        let mut data = storage::get_outsides();
        data.sort_by(|a, b| b.percentage.cmp(&a.percentage));
        data
    });
    let show_modal = use_state(|| false);
    let edit_uuid = use_state(|| None::<String>);
    let name_input = use_state(|| String::new());
    let context_input = use_state(|| String::new());
    let url_input = use_state(|| String::new());
    let percentage_input = use_state(|| 0);

    let _refresh_data = {
        let outsides = outsides.clone();
        Callback::from(move |_: MouseEvent| {
            let mut outsides_data = storage::get_outsides();
            outsides_data.sort_by(|a, b| b.percentage.cmp(&a.percentage));
            outsides.set(outsides_data);
        })
    };

    let show_add_modal = {
        let show_modal = show_modal.clone();
        let edit_uuid = edit_uuid.clone();
        let name_input = name_input.clone();
        let context_input = context_input.clone();
        let url_input = url_input.clone();
        let percentage_input = percentage_input.clone();
        Callback::from(move |_: MouseEvent| {
            edit_uuid.set(None);
            name_input.set(String::new());
            context_input.set(String::new());
            url_input.set(String::new());
            percentage_input.set(0);
            show_modal.set(true);
        })
    };

    let show_edit_modal = {
        let show_modal = show_modal.clone();
        let edit_uuid = edit_uuid.clone();
        let name_input = name_input.clone();
        let context_input = context_input.clone();
        let url_input = url_input.clone();
        let percentage_input = percentage_input.clone();
        let outsides = outsides.clone();
        Callback::from(move |uuid: String| {
            if let Some(outside) = outsides.iter().find(|o| o.uuid == uuid) {
                name_input.set(outside.name.clone());
                context_input.set(outside.context.clone());
                url_input.set(outside.url.clone());
                percentage_input.set(outside.percentage);
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

    let on_name_change = {
        let name_input = name_input.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            name_input.set(input.value());
        })
    };

    let on_context_change = {
        let context_input = context_input.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlTextAreaElement = e.target_unchecked_into();
            context_input.set(input.value());
        })
    };

    let on_url_change = {
        let url_input = url_input.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            url_input.set(input.value());
        })
    };

    let on_percentage_change = {
        let percentage_input = percentage_input.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            percentage_input.set(input.value().parse().unwrap_or(0));
        })
    };

    let submit_outside = {
        let name_input = name_input.clone();
        let context_input = context_input.clone();
        let url_input = url_input.clone();
        let percentage_input = percentage_input.clone();
        let edit_uuid = edit_uuid.clone();
        let outsides = outsides.clone();
        let show_modal = show_modal.clone();
        Callback::from(move |_| {
            let name = (*name_input).clone();
            let context = (*context_input).clone();
            let url = (*url_input).clone();
            let percentage = *percentage_input;
            let uuid = (*edit_uuid).clone();
            
            let outside = Outside {
                uuid: uuid.clone().unwrap_or_else(|| uuid::Uuid::new_v4().to_string()),
                name,
                context,
                url,
                percentage,
            };

            let result = if let Some(ref uuid) = uuid {
                storage::update_outside(uuid, outside)
            } else {
                storage::add_outside(outside)
            };

            match result {
                Ok(_) => {
                    let mut outsides_data = storage::get_outsides();
                    outsides_data.sort_by(|a, b| b.percentage.cmp(&a.percentage));
                    outsides.set(outsides_data);
                    web_sys::console::log_1(&"Outside saved successfully!".into());
                }
                Err(e) => web_sys::console::log_1(&format!("Failed to save outside: {}", e).into()),
            }
            
            show_modal.set(false);
        })
    };

    let delete_outside_callback = {
        let outsides = outsides.clone();
        Callback::from(move |uuid: String| {
            match storage::delete_outside(&uuid) {
                Ok(_) => {
                    let mut outsides_data = storage::get_outsides();
                    outsides_data.sort_by(|a, b| b.percentage.cmp(&a.percentage));
                    outsides.set(outsides_data);
                    web_sys::console::log_1(&"Outside deleted successfully!".into());
                }
                Err(e) => web_sys::console::log_1(&format!("Failed to delete outside: {}", e).into()),
            }
        })
    };

    html! {
        <>
            <Navigation title="Relf" />
            
            <div class="cards-container">
                {
                    outsides.iter().map(|outside| {
                        let edit_callback = show_edit_modal.reform({
                            let uuid = outside.uuid.clone();
                            move |_: MouseEvent| uuid.clone()
                        });
                        let delete_callback = delete_outside_callback.reform({
                            let uuid = outside.uuid.clone();
                            move |_: MouseEvent| uuid.clone()
                        });
                        
                        html! {
                            <Card key={outside.uuid.clone()} class="outside-card">
                                <CardHeader>
                                    <h3>{&outside.name}</h3>
                                </CardHeader>
                                <CardContent>
                                    <p>{&outside.context}</p>
                                </CardContent>
                                <CardFooter>
                                    <div class="card-meta">
                                        <span class="percentage">{format!("{}%", outside.percentage)}</span>
                                    </div>
                                    <div class="card-actions">
                                        <a href={outside.url.clone()} target="_blank" class="url-link">{"🔗"}</a>
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
                    <label for="name">{"Name:"}</label>
                    <input 
                        type="text" 
                        id="name" 
                        required=true
                        value={(*name_input).clone()}
                        oninput={on_name_change}
                    />
                    
                    <label for="context">{"Context:"}</label>
                    <textarea 
                        id="context" 
                        required=true
                        value={(*context_input).clone()}
                        oninput={on_context_change}
                    />
                    
                    <label for="url">{"URL:"}</label>
                    <input 
                        type="text" 
                        id="url" 
                        required=true
                        value={(*url_input).clone()}
                        oninput={on_url_change}
                    />
                    
                    <label for="percentage">{"Percentage:"}</label>
                    <input 
                        type="number" 
                        id="percentage" 
                        min="0" 
                        max="100" 
                        required=true
                        value={(*percentage_input).to_string()}
                        oninput={on_percentage_change}
                    />
                    
                    <button type="button" id="submit-btn" onclick={submit_outside}>{"Submit"}</button>
                </form>
            </Modal>
            
        </>
    }
}