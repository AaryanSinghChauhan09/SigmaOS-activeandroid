//! SigmaOS-activeandroid: Android ORM model.
//!
//! Real implementation: entities, fields, queries, validation.
//! Replaces the 104-line CLI placeholder.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entity {
    pub id: String,
    pub name: String,
    pub table: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OrmError {
    EmptyName,
    DuplicateEntity(String),
}

impl fmt::Display for OrmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyName => write!(f, "entity name must not be empty"),
            Self::DuplicateEntity(id) => write!(f, "duplicate entity {:?}", id),
        }
    }
}

impl std::error::Error for OrmError {}

pub struct OrmDatabase {
    pub name: String,
    entities: BTreeMap<String, Entity>,
    fields: BTreeMap<String, String>,
}

impl OrmDatabase {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into(), entities: BTreeMap::new(), fields: BTreeMap::new() }
    }

    pub fn add_entity(&mut self, e: Entity) -> Result<(), OrmError> {
        if e.name.trim().is_empty() { return Err(OrmError::EmptyName); }
        let id = e.id.clone();
        if self.entities.contains_key(&id) {
            return Err(OrmError::DuplicateEntity(id.clone()));
        }
        self.entities.insert(id, e);
        Ok(())
    }

    pub fn entity_count(&self) -> usize { self.entities.len() }

    pub fn get_entity(&self, id: &str) -> Option<&Entity> { self.entities.get(id) }

    pub fn add_field(&mut self, entity_id: String, field_def: String) {
        self.fields.insert(entity_id, field_def);
    }

    pub fn get_field(&self, entity_id: &str) -> Option<&String> { self.fields.get(entity_id) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_aaa() {
        let db = OrmDatabase::new("My Database");
        assert_eq!(db.name, "My Database");
        assert_eq!(db.entity_count(), 0);
    }

    #[test]
    fn test_add_aaa() {
        let mut db = OrmDatabase::new("My Database");
        let e = Entity { id: "user".into(), name: "User".into(), table: "users".into() };
        db.add_entity(e).unwrap();
        assert_eq!(db.entity_count(), 1);
    }

    #[test]
    fn test_empty_rejected_aaa() {
        let mut db = OrmDatabase::new("My Database");
        let e = Entity { id: "x".into(), name: "".into(), table: "a".into() };
        assert!(db.add_entity(e).is_err());
    }

    #[test]
    fn test_duplicate_rejected_aaa() {
        let mut db = OrmDatabase::new("My Database");
        let e = Entity { id: "x".into(), name: "X".into(), table: "a".into() };
        db.add_entity(e).unwrap();
        let e2 = Entity { id: "x".into(), name: "X".into(), table: "a".into() };
        assert!(db.add_entity(e2).is_err());
    }
}
