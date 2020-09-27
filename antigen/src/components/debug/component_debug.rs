use std::{collections::HashMap, fmt::Debug};

use crate::entity_component_system::{ComponentDebugTrait, ComponentID};

#[derive(Clone)]
pub struct ComponentDebugInfo {
    labels: HashMap<ComponentID, String>,
    descriptions: HashMap<ComponentID, String>,
}

impl ComponentDebugInfo {
    pub fn new() -> Self {
        ComponentDebugInfo {
            labels: HashMap::new(),
            descriptions: HashMap::new(),
        }
    }

    pub fn register_component(
        &mut self,
        component_id: ComponentID,
        label: String,
        description: String,
    ) -> &mut Self {
        self.labels.insert(component_id, label);
        self.descriptions.insert(component_id, description);
        self
    }

    pub fn get_label(&self, component_id: &ComponentID) -> String {
        self.labels
            .get(component_id)
            .cloned()
            .unwrap_or(format!("Component {}", component_id))
    }
}

impl Debug for ComponentDebugInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ComponentDebugComponent").finish()
    }
}

impl Default for ComponentDebugInfo {
    fn default() -> Self {
        ComponentDebugInfo::new()
    }
}

impl ComponentDebugTrait for ComponentDebugInfo {
    fn get_name() -> String {
        "Component Debug".into()
    }

    fn get_description() -> String {
        "Container for component debug data".into()
    }
}
