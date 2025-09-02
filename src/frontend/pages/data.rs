use yew::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::{window, HtmlInputElement, HtmlTextAreaElement};
use js_sys;
use crate::frontend::components::navigation::Navigation;
use crate::frontend::services::storage;

#[function_component(Data)]
pub fn data() -> Html {
    let outsides = use_state(|| {
        let mut data = storage::get_outsides();
        data.sort_by(|a, b| b.percentage.cmp(&a.percentage));
        data
    });
    let insides = use_state(|| {
        let mut data = storage::get_insides();
        data.sort_by(|a, b| b.date.cmp(&a.date));
        data
    });
    let json_content = use_state(|| storage::export_to_json());
    let show_import_modal = use_state(|| false);
    let show_import_outside_modal = use_state(|| false);
    let show_import_inside_modal = use_state(|| false);
    let import_json = use_state(|| String::new());
    let import_outside_json = use_state(|| String::new());
    let import_inside_json = use_state(|| String::new());
    let file_input_ref = use_node_ref();
    let textarea_ref = use_node_ref();
    let outside_textarea_ref = use_node_ref();
    let inside_textarea_ref = use_node_ref();

    // Refresh/Reset all data to defaults
    let refresh_data = {
        let outsides = outsides.clone();
        let insides = insides.clone();
        let json_content = json_content.clone();
        Callback::from(move |_: MouseEvent| {
            match storage::reset_to_defaults() {
                Ok(_) => {
                    let mut outside_data = storage::get_outsides();
                    outside_data.sort_by(|a, b| b.percentage.cmp(&a.percentage));
                    outsides.set(outside_data);
                    
                    let mut inside_data = storage::get_insides();
                    inside_data.sort_by(|a, b| b.date.cmp(&a.date));
                    insides.set(inside_data);
                    
                    json_content.set(storage::export_to_json());
                    web_sys::console::log_1(&"All data reset to defaults!".into());
                }
                Err(e) => {
                    web_sys::console::log_1(&format!("Failed to reset data: {}", e).into());
                }
            }
        })
    };

    // Copy all JSON data to clipboard
    let copy_json = {
        let json_content = json_content.clone();
        Callback::from(move |_| {
            let text_to_copy = (*json_content).clone();
            let escaped_text = text_to_copy
                .replace('\\', "\\\\")
                .replace('`', "\\`")
                .replace('$', "\\$")
                .replace('\'', "\\'")
                .replace('\n', "\\n")
                .replace('\r', "\\r");
            
            // Mobile-friendly clipboard copying with fallback methods
            let copy_script = format!(r#"
                (async function() {{
                    try {{
                        // Modern Clipboard API (preferred)
                        if (navigator.clipboard && navigator.clipboard.writeText) {{
                            await navigator.clipboard.writeText(`{}`);
                            console.log('JSON copied to clipboard!');
                            return;
                        }}
                        
                        // Fallback for older browsers/mobile
                        const textArea = document.createElement('textarea');
                        textArea.value = `{}`;
                        textArea.style.position = 'fixed';
                        textArea.style.left = '-9999px';
                        textArea.style.top = '-9999px';
                        document.body.appendChild(textArea);
                        
                        textArea.focus();
                        textArea.select();
                        textArea.setSelectionRange(0, 99999); // For mobile devices
                        
                        const successful = document.execCommand('copy');
                        document.body.removeChild(textArea);
                        
                        if (successful) {{
                            console.log('JSON copied to clipboard!');
                        }} else {{
                            throw new Error('Copy command failed');
                        }}
                    }} catch (err) {{
                        console.error('Copy failed:', err);
                        console.log('Copy failed. Please copy manually from the text above.');
                    }}
                }})();
            "#, escaped_text, escaped_text);
            
            let _ = js_sys::eval(&copy_script);
        })
    };

    // Save JSON to file
    let save_json = Callback::from(|_| {
        storage::download_json();
    });

    // Show import modal
    let show_modal = {
        let show_import_modal = show_import_modal.clone();
        let import_json = import_json.clone();
        Callback::from(move |_| {
            import_json.set(String::new());
            show_import_modal.set(true);
        })
    };

    // Show import outside modal
    let show_outside_modal = {
        let show_import_outside_modal = show_import_outside_modal.clone();
        let import_outside_json = import_outside_json.clone();
        Callback::from(move |_| {
            import_outside_json.set(String::new());
            show_import_outside_modal.set(true);
        })
    };

    // Show import inside modal
    let show_inside_modal = {
        let show_import_inside_modal = show_import_inside_modal.clone();
        let import_inside_json = import_inside_json.clone();
        Callback::from(move |_| {
            import_inside_json.set(String::new());
            show_import_inside_modal.set(true);
        })
    };

    // Close import modal
    let close_modal = {
        let show_import_modal = show_import_modal.clone();
        Callback::from(move |_| {
            show_import_modal.set(false);
        })
    };

    // Close import outside modal
    let close_outside_modal = {
        let show_import_outside_modal = show_import_outside_modal.clone();
        Callback::from(move |_| {
            show_import_outside_modal.set(false);
        })
    };

    // Close import inside modal
    let close_inside_modal = {
        let show_import_inside_modal = show_import_inside_modal.clone();
        Callback::from(move |_| {
            show_import_inside_modal.set(false);
        })
    };

    // Handle textarea change
    let on_textarea_change = {
        let import_json = import_json.clone();
        Callback::from(move |e: Event| {
            let input: HtmlTextAreaElement = e.target_unchecked_into();
            import_json.set(input.value());
        })
    };

    // Handle outside textarea change
    let on_outside_textarea_change = {
        let import_outside_json = import_outside_json.clone();
        Callback::from(move |e: Event| {
            let input: HtmlTextAreaElement = e.target_unchecked_into();
            import_outside_json.set(input.value());
        })
    };

    // Handle inside textarea change
    let on_inside_textarea_change = {
        let import_inside_json = import_inside_json.clone();
        Callback::from(move |e: Event| {
            let input: HtmlTextAreaElement = e.target_unchecked_into();
            import_inside_json.set(input.value());
        })
    };

    // Append JSON data
    let append_data = {
        let import_json = import_json.clone();
        let outsides = outsides.clone();
        let insides = insides.clone();
        let json_content = json_content.clone();
        let show_import_modal = show_import_modal.clone();
        Callback::from(move |_| {
            let json_str = (*import_json).clone();
            if !json_str.is_empty() {
                match storage::append_from_json(&json_str) {
                    Ok(_) => {
                        let mut outside_data = storage::get_outsides();
                        outside_data.sort_by(|a, b| b.percentage.cmp(&a.percentage));
                        outsides.set(outside_data);
                        
                        let mut inside_data = storage::get_insides();
                        inside_data.sort_by(|a, b| b.date.cmp(&a.date));
                        insides.set(inside_data);
                        
                        json_content.set(storage::export_to_json());
                        show_import_modal.set(false);
                        web_sys::console::log_1(&"Data appended successfully!".into());
                    }
                    Err(e) => {
                        web_sys::console::log_1(&format!("Append failed: {}", e).into());
                        // Show error to user
                        if let Some(window) = window() {
                            let _ = window.alert_with_message(&format!("Append failed: {}", e));
                        }
                    }
                }
            }
        })
    };

    // Import JSON data
    let import_data = {
        let import_json = import_json.clone();
        let outsides = outsides.clone();
        let insides = insides.clone();
        let json_content = json_content.clone();
        let show_import_modal = show_import_modal.clone();
        Callback::from(move |_| {
            let json_str = (*import_json).clone();
            if !json_str.is_empty() {
                match storage::import_from_json(&json_str) {
                    Ok(_) => {
                        let mut outside_data = storage::get_outsides();
                        outside_data.sort_by(|a, b| b.percentage.cmp(&a.percentage));
                        outsides.set(outside_data);
                        
                        let mut inside_data = storage::get_insides();
                        inside_data.sort_by(|a, b| b.date.cmp(&a.date));
                        insides.set(inside_data);
                        
                        json_content.set(storage::export_to_json());
                        show_import_modal.set(false);
                        web_sys::console::log_1(&"Data imported successfully!".into());
                    }
                    Err(e) => {
                        web_sys::console::log_1(&format!("Import failed: {}", e).into());
                        // Show error to user
                        if let Some(window) = window() {
                            let _ = window.alert_with_message(&format!("Import failed: {}", e));
                        }
                    }
                }
            }
        })
    };

    // Append outside JSON data
    let append_outside_data = {
        let import_outside_json = import_outside_json.clone();
        let outsides = outsides.clone();
        let json_content = json_content.clone();
        let show_import_outside_modal = show_import_outside_modal.clone();
        Callback::from(move |_| {
            let json_str = (*import_outside_json).clone();
            if !json_str.is_empty() {
                match storage::append_outside_from_json(&json_str) {
                    Ok(_) => {
                        let mut outside_data = storage::get_outsides();
                        outside_data.sort_by(|a, b| b.percentage.cmp(&a.percentage));
                        outsides.set(outside_data);
                        
                        json_content.set(storage::export_to_json());
                        show_import_outside_modal.set(false);
                        web_sys::console::log_1(&"Outside data appended successfully!".into());
                    }
                    Err(e) => {
                        web_sys::console::log_1(&format!("Append failed: {}", e).into());
                        // Show error to user
                        if let Some(window) = window() {
                            let _ = window.alert_with_message(&format!("Append failed: {}", e));
                        }
                    }
                }
            }
        })
    };

    // Import outside JSON data
    let import_outside_data = {
        let import_outside_json = import_outside_json.clone();
        let outsides = outsides.clone();
        let json_content = json_content.clone();
        let show_import_outside_modal = show_import_outside_modal.clone();
        Callback::from(move |_| {
            let json_str = (*import_outside_json).clone();
            if !json_str.is_empty() {
                match storage::import_outside_from_json(&json_str) {
                    Ok(_) => {
                        let mut outside_data = storage::get_outsides();
                        outside_data.sort_by(|a, b| b.percentage.cmp(&a.percentage));
                        outsides.set(outside_data);
                        
                        json_content.set(storage::export_to_json());
                        show_import_outside_modal.set(false);
                        web_sys::console::log_1(&"Outside data imported successfully!".into());
                    }
                    Err(e) => {
                        web_sys::console::log_1(&format!("Import failed: {}", e).into());
                        // Show error to user
                        if let Some(window) = window() {
                            let _ = window.alert_with_message(&format!("Import failed: {}", e));
                        }
                    }
                }
            }
        })
    };

    // Append inside JSON data
    let append_inside_data = {
        let import_inside_json = import_inside_json.clone();
        let insides = insides.clone();
        let json_content = json_content.clone();
        let show_import_inside_modal = show_import_inside_modal.clone();
        Callback::from(move |_| {
            let json_str = (*import_inside_json).clone();
            if !json_str.is_empty() {
                match storage::append_inside_from_json(&json_str) {
                    Ok(_) => {
                        let mut inside_data = storage::get_insides();
                        inside_data.sort_by(|a, b| b.date.cmp(&a.date));
                        insides.set(inside_data);
                        
                        json_content.set(storage::export_to_json());
                        show_import_inside_modal.set(false);
                        web_sys::console::log_1(&"Inside data appended successfully!".into());
                    }
                    Err(e) => {
                        web_sys::console::log_1(&format!("Append failed: {}", e).into());
                        // Show error to user
                        if let Some(window) = window() {
                            let _ = window.alert_with_message(&format!("Append failed: {}", e));
                        }
                    }
                }
            }
        })
    };

    // Import inside JSON data
    let import_inside_data = {
        let import_inside_json = import_inside_json.clone();
        let insides = insides.clone();
        let json_content = json_content.clone();
        let show_import_inside_modal = show_import_inside_modal.clone();
        Callback::from(move |_| {
            let json_str = (*import_inside_json).clone();
            if !json_str.is_empty() {
                match storage::import_inside_from_json(&json_str) {
                    Ok(_) => {
                        let mut inside_data = storage::get_insides();
                        inside_data.sort_by(|a, b| b.date.cmp(&a.date));
                        insides.set(inside_data);
                        
                        json_content.set(storage::export_to_json());
                        show_import_inside_modal.set(false);
                        web_sys::console::log_1(&"Inside data imported successfully!".into());
                    }
                    Err(e) => {
                        web_sys::console::log_1(&format!("Import failed: {}", e).into());
                        // Show error to user
                        if let Some(window) = window() {
                            let _ = window.alert_with_message(&format!("Import failed: {}", e));
                        }
                    }
                }
            }
        })
    };

    // Trigger file import
    let trigger_file_import = {
        let file_input_ref = file_input_ref.clone();
        Callback::from(move |_| {
            if let Some(input) = file_input_ref.cast::<HtmlInputElement>() {
                input.click();
            }
        })
    };

    // Handle file selection
    let on_file_change = {
        let outsides = outsides.clone();
        let insides = insides.clone();
        let json_content = json_content.clone();
        let show_import_modal = show_import_modal.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            if let Some(files) = input.files() {
                if let Some(file) = files.get(0) {
                    let outsides = outsides.clone();
                    let insides = insides.clone();
                    let json_content = json_content.clone();
                    let show_import_modal = show_import_modal.clone();
                    
                    let reader = web_sys::FileReader::new().unwrap();
                    let reader_clone = reader.clone();
                    
                    let _ = reader.read_as_text(&file);
                    
                    let closure = Closure::once(move || {
                        if let Ok(result) = reader_clone.result() {
                            if let Some(text_str) = result.as_string() {
                                match storage::import_from_json(&text_str) {
                                    Ok(_) => {
                                        let mut outside_data = storage::get_outsides();
                                        outside_data.sort_by(|a, b| b.percentage.cmp(&a.percentage));
                                        outsides.set(outside_data);
                                        
                                        let mut inside_data = storage::get_insides();
                                        inside_data.sort_by(|a, b| b.date.cmp(&a.date));
                                        insides.set(inside_data);
                                        
                                        json_content.set(storage::export_to_json());
                                        show_import_modal.set(false);
                                        web_sys::console::log_1(&"Data imported successfully from file!".into());
                                    }
                                    Err(e) => {
                                        web_sys::console::log_1(&format!("Import failed: {}", e).into());
                                        if let Some(window) = window() {
                                            let _ = window.alert_with_message(&format!("Import failed: {}", e));
                                        }
                                    }
                                }
                            }
                        }
                    });
                    
                    reader.set_onloadend(Some(closure.as_ref().unchecked_ref()));
                    closure.forget();
                }
            }
            // Reset the file input value to allow re-selecting the same file
            input.set_value("");
        })
    };

    html! {
        <>
            <Navigation title="Relf" />
            
            <div class="data-page-container">
                <div class="data-content">
                    <div class="markdown-display">
                        <div class="markdown-header">
                            <div class="button-group">
                                <button 
                                    class="refresh-button modern-button icon-only" 
                                    onclick={refresh_data}
                                    title="Reset all data to defaults"
                                >
                                    <span class="button-icon">{"🔄"}</span>
                                </button>
                                <button 
                                    class="copy-button modern-button icon-only" 
                                    onclick={copy_json}
                                    title="Copy data"
                                >
                                    <span class="button-icon">{"📋"}</span>
                                </button>
                                <button 
                                    class="paste-button modern-button icon-only" 
                                    onclick={show_modal}
                                    title="Paste data"
                                >
                                    <span class="button-icon">{"📄"}</span>
                                </button>
                                <button 
                                    class="paste-outside-button modern-button icon-only" 
                                    onclick={show_outside_modal}
                                    title="Paste outside data only"
                                >
                                    <span class="button-icon">{"📄"}</span>
                                </button>
                                <button 
                                    class="paste-inside-button modern-button icon-only" 
                                    onclick={show_inside_modal}
                                    title="Paste inside data only"
                                >
                                    <span class="button-icon">{"📄"}</span>
                                </button>
                                <button 
                                    class="save-button modern-button icon-only" 
                                    onclick={save_json}
                                    title="Save data to file"
                                >
                                    <span class="button-icon">{"💾"}</span>
                                </button>
                                <button 
                                    class="import-button modern-button icon-only" 
                                    onclick={trigger_file_import}
                                    title="Import from file"
                                >
                                    <span class="button-icon">{"📁"}</span>
                                </button>
                            </div>
                        </div>
                        <div class="markdown-content-wrapper">
                            <pre id="json-content" class="markdown-block">{&*json_content}</pre>
                        </div>
                    </div>
                </div>
            </div>
            
            // Import Modal
            if *show_import_modal {
                <div class="modal-overlay" onclick={close_modal.clone()}>
                    <div class="modal-content" onclick={|e: MouseEvent| e.stop_propagation()}>
                        <form class="modal-form" onsubmit={Callback::from(|e: SubmitEvent| e.prevent_default())}>
                            <label for="json-data">{"Data:"}</label>
                            <textarea 
                                ref={textarea_ref}
                                id="json-data"
                                placeholder="{\n  \"outside\": [...],\n  \"inside\": [...]\n}"
                                value={(*import_json).clone()}
                                onchange={on_textarea_change}
                                rows="15"
                            />
                            
                            <div class="button-row">
                                <button type="button" id="submit-btn" onclick={import_data}>{"Import"}</button>
                                <button type="button" id="append-btn" onclick={append_data}>{"Append"}</button>
                            </div>
                        </form>
                    </div>
                </div>
            }
            
            // Import Outside Modal
            if *show_import_outside_modal {
                <div class="modal-overlay" onclick={close_outside_modal.clone()}>
                    <div class="modal-content" onclick={|e: MouseEvent| e.stop_propagation()}>
                        <form class="modal-form" onsubmit={Callback::from(|e: SubmitEvent| e.prevent_default())}>
                            <label for="outside-json-data">{"Outside Data:"}</label>
                            <textarea 
                                ref={outside_textarea_ref}
                                id="outside-json-data"
                                placeholder="{\n  \"outside\": [...]\n}"
                                value={(*import_outside_json).clone()}
                                onchange={on_outside_textarea_change}
                                rows="15"
                            />
                            
                            <div class="button-row">
                                <button type="button" id="submit-btn" onclick={import_outside_data}>{"Import"}</button>
                                <button type="button" id="append-btn" onclick={append_outside_data}>{"Append"}</button>
                            </div>
                        </form>
                    </div>
                </div>
            }
            
            // Import Inside Modal
            if *show_import_inside_modal {
                <div class="modal-overlay" onclick={close_inside_modal.clone()}>
                    <div class="modal-content" onclick={|e: MouseEvent| e.stop_propagation()}>
                        <form class="modal-form" onsubmit={Callback::from(|e: SubmitEvent| e.prevent_default())}>
                            <label for="inside-json-data">{"Inside Data:"}</label>
                            <textarea 
                                ref={inside_textarea_ref}
                                id="inside-json-data"
                                placeholder="{\n  \"inside\": [...]\n}"
                                value={(*import_inside_json).clone()}
                                onchange={on_inside_textarea_change}
                                rows="15"
                            />
                            
                            <div class="button-row">
                                <button type="button" id="submit-btn" onclick={import_inside_data}>{"Import"}</button>
                                <button type="button" id="append-btn" onclick={append_inside_data}>{"Append"}</button>
                            </div>
                        </form>
                    </div>
                </div>
            }
            
            <input 
                type="file" 
                ref={file_input_ref}
                accept=".json"
                style="display: none;"
                onchange={on_file_change}
            />
        </>
    }
}