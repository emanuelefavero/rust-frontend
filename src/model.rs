use serde::Deserialize;

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct ActorData {
    pub id: u64,
    pub name: String,
    pub biography: String,
    #[serde(default)]
    pub image: Option<String>,
    pub birth_year: Option<u16>,
    pub death_year: Option<u16>,
    pub nationality: Option<String>,
    #[serde(default)]
    pub known_for: Option<Vec<String>>,
    #[serde(default)]
    pub awards: Option<Vec<String>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Actor {
    pub uid: String,
    pub category: &'static str,
    pub data: ActorData,
}

pub fn normalize(data: Vec<ActorData>, category: &'static str) -> Result<Vec<Actor>, String> {
    let mut ids = std::collections::HashSet::new();
    data.into_iter()
        .map(|data| {
            if data.name.trim().is_empty() || !ids.insert(data.id) {
                return Err(format!(
                    "Dati non validi nella sorgente {category}: nome vuoto o ID duplicato."
                ));
            }
            Ok(Actor {
                uid: format!("{category}-{}", data.id),
                category,
                data,
            })
        })
        .collect()
}
