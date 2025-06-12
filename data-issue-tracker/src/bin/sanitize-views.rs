// generate-default-views.rs
// Generates default view and form definitions for each entity in entity-model, if not already present.

use data_issue_tracker::*;
use serde_yaml;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

fn main() {
    let forms_dir = "forms";
    let views_dir = "view";
    fs::create_dir_all(forms_dir).unwrap();
    fs::create_dir_all(views_dir).unwrap();
    let ems = load_entity_models();
    for model in ems.values() {
        let entity = &model.entity;
        generate_form_if_not_exists(&model, format!("{}/{}.yaml", forms_dir, entity));
        generate_view_if_not_exists(&model, format!("{}/{}.yaml", views_dir, entity));
    }
}

fn generate_view_if_not_exists(model: &EntityModel, view_path: String) {
    if !Path::new(&view_path).exists() {
        let columns: Vec<_> = model
            .attributes
            .as_ref()
            .map(|attrs| {
                attrs
                    .iter()
                    .map(|a| {
                        let mut m = HashMap::new();
                        m.insert("name", a.name.clone());
                        m.insert("label", a.name.clone());
                        m
                    })
                    .collect()
            })
            .unwrap_or_default();
        let mut view_def_map = serde_yaml::Mapping::new();
        view_def_map.insert(
            serde_yaml::to_value("title").unwrap(),
            serde_yaml::to_value(format!("List of {}", model.entity)).unwrap(),
        );
        view_def_map.insert(
            serde_yaml::to_value("columns").unwrap(),
            serde_yaml::to_value(columns).unwrap(),
        );
        let view_def = serde_yaml::to_string(&view_def_map).unwrap();
        fs::write(&view_path, view_def).unwrap();
        println!("Created default view: {}", view_path);
    }
}

fn generate_form_if_not_exists(model: &EntityModel, form_path: String) {
    if !Path::new(&form_path).exists() {
        let fields: Vec<_> = model
            .attributes
            .as_ref()
            .map(|attrs| {
                attrs
                    .iter()
                    .map(|a| {
                        let mut m = HashMap::new();
                        m.insert("name", a.name.clone());
                        m.insert("label", a.name.clone());
                        m
                    })
                    .collect()
            })
            .unwrap_or_default();
        let relations: Vec<_> = model
            .relations
            .as_ref()
            .map(|rels| {
                rels.iter()
                    .map(|r| {
                        let mut m = HashMap::new();
                        m.insert("name", r.name.clone());
                        m.insert("label", r.name.clone());
                        m
                    })
                    .collect()
            })
            .unwrap_or_default();
        let mut form_def_map = serde_yaml::Mapping::new();
        form_def_map.insert(
            serde_yaml::to_value("title").unwrap(),
            serde_yaml::to_value(format!("{}", model.entity)).unwrap(),
        );
        form_def_map.insert(
            serde_yaml::to_value("fields").unwrap(),
            serde_yaml::to_value(fields).unwrap(),
        );
        form_def_map.insert(
            serde_yaml::to_value("relations").unwrap(),
            serde_yaml::to_value(relations).unwrap(),
        );

        fs::write(&form_path, serde_yaml::to_string(&form_def_map).unwrap()).unwrap();
        println!("Created default form: {}", form_path);
    }
}
