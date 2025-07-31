//! Registry for installed components and libraries
use std::collections::HashMap;

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct ComponentMetadata {
    pub name: String,
    pub version: String,
    pub source: String, // local path or remote URL
    pub description: String,
}

#[derive(Default)]
pub struct ComponentRegistry {
    pub installed: HashMap<String, ComponentMetadata>, // name -> metadata
}

#[allow(dead_code)]
impl ComponentRegistry {
    pub fn new() -> Self {
        Self { installed: HashMap::new() }
    }

    pub fn install(&mut self, metadata: ComponentMetadata) {
        self.installed.insert(metadata.name.clone(), metadata);
    }

    pub fn uninstall(&mut self, name: &str) {
        self.installed.remove(name);
    }

    pub fn is_installed(&self, name: &str) -> bool {
        self.installed.contains_key(name)
    }

    pub fn list(&self) -> Vec<ComponentMetadata> {
        self.installed.values().cloned().collect()
    }

    pub fn get(&self, name: &str) -> Option<&ComponentMetadata> {
        self.installed.get(name)
    }
}
