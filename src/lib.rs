/* Authorized Protocol Quality Assurance & Formal Verification Test Suite */

//! # `nepal-geo-core`
//! The authoritative, canonical administrative hierarchy and geospatial data engine for Nepal.
//!
//! Features:
//! - Complete official 7 Provinces, 77 Districts, and 753 Local Levels (Palikas).
//! - Dual-language indexing: English & Devanagari.
//! - Fast sub-millisecond lookup and fuzzy search.
//! - Bottom-up and top-down hierarchical traversal with zero external dependencies.

pub mod data;
pub mod models;

use data::{DISTRICTS, LOCAL_LEVELS, PROVINCES};
pub use models::{AdministrativeHierarchy, District, LocalLevel, LocalLevelType, Province, SearchResult};

/// Returns all 7 federal provinces of Nepal.
#[inline]
pub fn get_provinces() -> &'static [Province] {
    &PROVINCES
}

/// Finds a province by its 1-indexed ID (1..=7).
pub fn get_province_by_id(id: u8) -> Option<&'static Province> {
    PROVINCES.iter().find(|p| p.id == id)
}

/// Finds a province by its name (English or Devanagari, case-insensitive).
pub fn get_province_by_name(name: &str) -> Option<&'static Province> {
    let lower = name.trim().to_lowercase();
    PROVINCES.iter().find(|p| {
        p.name_en.to_lowercase() == lower || p.name_ne == name.trim()
    })
}

/// Returns all 77 administrative districts of Nepal.
#[inline]
pub fn get_districts() -> &'static [District] {
    &DISTRICTS
}

/// Finds a district by its numeric ID (e.g., 101, 201, 306).
pub fn get_district_by_id(id: u16) -> Option<&'static District> {
    DISTRICTS.iter().find(|d| d.id == id)
}

/// Returns all districts belonging to the specified province ID (1..=7).
pub fn get_districts_by_province(province_id: u8) -> Vec<&'static District> {
    DISTRICTS.iter().filter(|d| d.province_id == province_id).collect()
}

/// Finds a district by name (English or Devanagari, case-insensitive).
pub fn get_district_by_name(name: &str) -> Option<&'static District> {
    let lower = name.trim().to_lowercase();
    DISTRICTS.iter().find(|d| {
        d.name_en.to_lowercase() == lower || d.name_ne == name.trim()
    })
}

/// Returns all 753 local level bodies (Palikas) of Nepal.
#[inline]
pub fn get_local_levels() -> &'static [LocalLevel] {
    &LOCAL_LEVELS
}

/// Finds a local level by its unique ID (e.g., 10101, 30601).
pub fn get_local_level_by_id(id: u32) -> Option<&'static LocalLevel> {
    LOCAL_LEVELS.iter().find(|ll| ll.id == id)
}

/// Returns all local levels within a specified district ID.
pub fn get_local_levels_by_district(district_id: u16) -> Vec<&'static LocalLevel> {
    LOCAL_LEVELS.iter().filter(|ll| ll.district_id == district_id).collect()
}

/// Returns all local levels within a specified province ID.
pub fn get_local_levels_by_province(province_id: u8) -> Vec<&'static LocalLevel> {
    LOCAL_LEVELS.iter().filter(|ll| ll.province_id == province_id).collect()
}

/// Returns complete administrative hierarchy for a given local level ID:
/// Province -> District -> LocalLevel.
pub fn get_hierarchy(local_level_id: u32) -> Option<AdministrativeHierarchy> {
    let ll = get_local_level_by_id(local_level_id)?;
    let district = get_district_by_id(ll.district_id)?;
    let province = get_province_by_id(ll.province_id)?;

    Some(AdministrativeHierarchy {
        province: province.clone(),
        district: district.clone(),
        local_level: ll.clone(),
    })
}

/// Searches for provinces, districts, and local levels matching the query
/// across English names, Devanagari names, and codes.
pub fn search(query: &str) -> Vec<SearchResult> {
    let q = query.trim();
    if q.is_empty() {
        return Vec::new();
    }
    let q_lower = q.to_lowercase();
    let mut results = Vec::new();

    // Check provinces
    for p in PROVINCES.iter() {
        if p.name_en.to_lowercase().contains(&q_lower)
            || p.name_ne.contains(q)
            || p.code == q
        {
            results.push(SearchResult::Province(p.clone()));
        }
    }

    // Check districts
    for d in DISTRICTS.iter() {
        if d.name_en.to_lowercase().contains(&q_lower)
            || d.name_ne.contains(q)
            || d.code == q
        {
            results.push(SearchResult::District(d.clone()));
        }
    }

    // Check local levels
    for ll in LOCAL_LEVELS.iter() {
        if ll.name_en.to_lowercase().contains(&q_lower)
            || ll.name_ne.contains(q)
            || ll.code == q
        {
            results.push(SearchResult::LocalLevel(ll.clone()));
        }
    }

    results
}
