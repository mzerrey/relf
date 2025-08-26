use gloo::storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};
use crate::models::{Outside, Inside};
use web_sys::window;
use wasm_bindgen::{JsValue, JsCast};
use js_sys;

const OUTSIDE_KEY: &str = "relf_outside_data";
const INSIDE_KEY: &str = "relf_inside_data";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StorageData {
    pub outside: Vec<Outside>,
    pub inside: Vec<Inside>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExportData {
    pub outside: Vec<ExportOutside>,
    pub inside: Vec<ExportInside>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExportOutside {
    pub name: String,
    pub context: String,
    pub url: String,
    pub percentage: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExportInside {
    pub date: String,
    pub context: String,
}

impl Default for StorageData {
    fn default() -> Self {
        Self {
            outside: vec![
                Outside {
                    uuid: uuid::Uuid::new_v4().to_string(),
                    name: "Rust Programming Language".to_string(),
                    context: "A language empowering everyone to build reliable and efficient software".to_string(),
                    url: "https://www.rust-lang.org/".to_string(),
                    percentage: 90,
                },
            ],
            inside: vec![
                Inside {
                    uuid: uuid::Uuid::new_v4().to_string(),
                    context: "cargo new my_project - Create a new Rust project".to_string(),
                    date: "2024-01-01 10:00:00".to_string(),
                },
            ],
        }
    }
}

pub fn get_outsides() -> Vec<Outside> {
    LocalStorage::get(OUTSIDE_KEY).unwrap_or_else(|_| {
        let default_data = StorageData::default();
        let _ = LocalStorage::set(OUTSIDE_KEY, &default_data.outside);
        default_data.outside
    })
}

pub fn get_insides() -> Vec<Inside> {
    LocalStorage::get(INSIDE_KEY).unwrap_or_else(|_| {
        let default_data = StorageData::default();
        let _ = LocalStorage::set(INSIDE_KEY, &default_data.inside);
        default_data.inside
    })
}

pub fn save_outsides(outsides: &Vec<Outside>) -> Result<(), String> {
    LocalStorage::set(OUTSIDE_KEY, outsides)
        .map_err(|e| format!("Failed to save outside data: {:?}", e))
}

pub fn save_insides(insides: &Vec<Inside>) -> Result<(), String> {
    LocalStorage::set(INSIDE_KEY, insides)
        .map_err(|e| format!("Failed to save inside data: {:?}", e))
}

pub fn add_outside(outside: Outside) -> Result<(), String> {
    let mut outsides = get_outsides();
    outsides.push(outside);
    save_outsides(&outsides)
}

pub fn update_outside(uuid: &str, updated: Outside) -> Result<(), String> {
    let mut outsides = get_outsides();
    if let Some(index) = outsides.iter().position(|o| o.uuid == uuid) {
        outsides[index] = updated;
        save_outsides(&outsides)
    } else {
        Err("Outside item not found".to_string())
    }
}

pub fn delete_outside(uuid: &str) -> Result<(), String> {
    let mut outsides = get_outsides();
    outsides.retain(|o| o.uuid != uuid);
    save_outsides(&outsides)
}

pub fn add_inside(inside: Inside) -> Result<(), String> {
    let mut insides = get_insides();
    insides.push(inside);
    save_insides(&insides)
}

pub fn update_inside(uuid: &str, updated: Inside) -> Result<(), String> {
    let mut insides = get_insides();
    if let Some(index) = insides.iter().position(|i| i.uuid == uuid) {
        insides[index] = updated;
        save_insides(&insides)
    } else {
        Err("Inside item not found".to_string())
    }
}

pub fn delete_inside(uuid: &str) -> Result<(), String> {
    let mut insides = get_insides();
    insides.retain(|i| i.uuid != uuid);
    save_insides(&insides)
}

pub fn export_to_json() -> String {
    let outsides = get_outsides();
    let insides = get_insides();
    
    let export_outsides: Vec<ExportOutside> = outsides.into_iter().map(|o| ExportOutside {
        name: o.name,
        context: o.context,
        url: o.url,
        percentage: o.percentage,
    }).collect();
    
    let export_insides: Vec<ExportInside> = insides.into_iter().map(|i| ExportInside {
        date: i.date,
        context: i.context,
    }).collect();
    
    let data = ExportData {
        outside: export_outsides,
        inside: export_insides,
    };
    
    serde_json::to_string_pretty(&data).unwrap_or_else(|_| "{}".to_string())
}

pub fn import_from_json(json_str: &str) -> Result<(), String> {
    let data: ExportData = serde_json::from_str(json_str)
        .map_err(|e| format!("Invalid JSON format: {:?}", e))?;
    
    // Convert ExportData to StorageData with new UUIDs
    let outsides: Vec<Outside> = data.outside.into_iter().map(|o| Outside {
        uuid: uuid::Uuid::new_v4().to_string(),
        name: o.name,
        context: o.context,
        url: o.url,
        percentage: o.percentage,
    }).collect();
    
    let insides: Vec<Inside> = data.inside.into_iter().map(|i| Inside {
        uuid: uuid::Uuid::new_v4().to_string(),
        date: i.date,
        context: i.context,
    }).collect();
    
    save_outsides(&outsides)?;
    save_insides(&insides)?;
    
    Ok(())
}


pub fn download_json() {
    if let Some(window) = window() {
        let json_content = export_to_json();
        
        if let Some(document) = window.document() {
            let blob_parts = js_sys::Array::new();
            blob_parts.push(&JsValue::from_str(&json_content));
            
            let opts = web_sys::BlobPropertyBag::new();
            opts.set_type("application/json");
            
            if let Ok(blob) = web_sys::Blob::new_with_str_sequence_and_options(&blob_parts, &opts) {
                if let Ok(url) = web_sys::Url::create_object_url_with_blob(&blob) {
                    if let Ok(a) = document.create_element("a") {
                        let _ = a.set_attribute("href", &url);
                        let _ = a.set_attribute("download", "relf_data.json");
                        let _ = a.set_attribute("style", "display: none");
                        
                        if let Some(body) = document.body() {
                            let _ = body.append_child(&a);
                            
                            if let Some(html_element) = a.dyn_ref::<web_sys::HtmlElement>() {
                                html_element.click();
                            }
                            
                            let _ = body.remove_child(&a);
                        }
                        
                        let _ = web_sys::Url::revoke_object_url(&url);
                        web_sys::console::log_1(&"File download initiated!".into());
                    }
                }
            }
        }
    }
}

pub fn reset_to_defaults() -> Result<(), String> {
    let default_data = StorageData::default();
    save_outsides(&default_data.outside)?;
    save_insides(&default_data.inside)?;
    Ok(())
}

