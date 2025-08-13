use crate::common::*;

use indexmap::IndexMap;
use slotmap::SlotMap;
use std::collections::HashMap;

pub struct IfStore {
    pub id_with_key:        SlotMap<IfKey, Interface>,
    pub id_with_name:       IndexMap<String, IfKey>,
    pub id_with_ifindex:    HashMap<IfIndex, IfKey>,
}

impl IfStore {
    pub fn new() -> Self {
        Self {
            id_with_key:        SlotMap::with_key(),
            id_with_name:       IndexMap::new(),
            id_with_ifindex:    HashMap::new(),
        }
    }

    pub fn insert(&mut self, mut ifp: Interface) -> IfKey {
        let key = self.id_with_key.insert_with_key(|k| { ifp.key = k; ifp.clone() });
        self.id_with_name.insert(ifp.name.0.clone(), key);
        self.id_with_ifindex.insert(ifp.ifindex.0, key);
        key
    }

    pub fn get(&self, k: IfKey) -> Option<&Interface> {
        self.id_with_key.get(k)
    }
    pub fn get_mut(&self, k: IfKey) -> Option<&mut Interface> {
        self.id_with_key.get_mut(k)
    }

    pub fn id_with_name(&self, name: &str) -> Option<IfKey> {
        self.id_with_name.get(name).copied()
    }

    pub fn id_with_ifindex(&self, index: i32) -> Option<IfKey> {
        self.id_with_ifindex.get(&index).copied()
    }

    pub fn rename_index(&mut self, k: IfKey, new_name: String) {
        if let Some(ifp) = self.id_with_key.get(k).cloned() {
            self.id_with_name.swap_remove(&ifp.name.0);
            self.id_with_name.insert(new_name.clone(), k);
            if let Some(m) = self.id_with_key.get_mut(k) {
                m.name = IfName(new_name);
            }
        }
    }

    pub fn update_ifindex(&mut self, k: IfKey, ifindex: i32) {
        if let Some(old) = self.id_with_key.get(k) {
            self.id_with_ifindex.remove(&old.ifindex.0);
        }
        self.id_with_ifindex.insert(ifindex, k);
        if let Some(m) = self.id_with_key.get_mut(k) {
            m.ifindex = IfIndex(ifindex);
        }
    }
}
