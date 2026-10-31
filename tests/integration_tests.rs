/* Authorized Protocol Quality Assurance & Formal Verification Test Suite */

use nepal_geo_core::{
    get_district_by_id, get_district_by_name, get_districts, get_districts_by_province,
    get_hierarchy, get_local_levels, get_local_levels_by_district,
    get_province_by_id, get_province_by_name, get_provinces,
    search, LocalLevelType, SearchResult,
};

#[test]
fn test_exact_national_administrative_counts() {
    let provinces = get_provinces();
    assert_eq!(provinces.len(), 7, "Nepal must have exactly 7 provinces");

    let districts = get_districts();
    assert_eq!(districts.len(), 77, "Nepal must have exactly 77 districts");

    let local_levels = get_local_levels();
    assert_eq!(
        local_levels.len(),
        753,
        "Nepal must have exactly 753 local level bodies (Palikas)"
    );

    // Verify breakdown by type
    let mut metro = 0;
    let mut sub_metro = 0;
    let mut muni = 0;
    let mut rural = 0;

    for ll in local_levels {
        match ll.local_type {
            LocalLevelType::Metropolitan => metro += 1,
            LocalLevelType::SubMetropolitan => sub_metro += 1,
            LocalLevelType::Municipality => muni += 1,
            LocalLevelType::RuralMunicipality => rural += 1,
        }
    }

    assert_eq!(metro, 6, "Must have exactly 6 Metropolitan Cities");
    assert_eq!(sub_metro, 11, "Must have exactly 11 Sub-Metropolitan Cities");
    assert_eq!(muni, 276, "Must have exactly 276 Municipalities");
    assert_eq!(rural, 460, "Must have exactly 460 Rural Municipalities");
}

#[test]
fn test_relational_hierarchy_integrity() {
    let local_levels = get_local_levels();

    for ll in local_levels {
        // Must have valid province
        let p = get_province_by_id(ll.province_id);
        assert!(
            p.is_some(),
            "Local level {} refers to non-existent province {}",
            ll.name_en,
            ll.province_id
        );

        // Must have valid district
        let d = get_district_by_id(ll.district_id);
        assert!(
            d.is_some(),
            "Local level {} refers to non-existent district {}",
            ll.name_en,
            ll.district_id
        );

        // District province must match local level province
        assert_eq!(
            d.unwrap().province_id,
            ll.province_id,
            "Hierarchy mismatch: district province != local level province"
        );

        // Hierarchy resolver check
        let h = get_hierarchy(ll.id).expect("Hierarchy must resolve");
        assert_eq!(h.province.id, ll.province_id);
        assert_eq!(h.district.id, ll.district_id);
        assert_eq!(h.local_level.id, ll.id);
    }
}

#[test]
fn test_filtering_apis() {
    // Madhesh Province (id 2) has 8 districts
    let madhesh_districts = get_districts_by_province(2);
    assert_eq!(madhesh_districts.len(), 8);

    // Bagmati Province (id 3) has 13 districts
    let bagmati_districts = get_districts_by_province(3);
    assert_eq!(bagmati_districts.len(), 13);

    // Kathmandu district (id 306) local levels
    let ktm_levels = get_local_levels_by_district(306);
    assert_eq!(ktm_levels.len(), 11); // 1 Metro + 10 Municipalities
}

#[test]
fn test_bilingual_lookups_and_search() {
    // English lookup
    let p = get_province_by_name("Madhesh").expect("Must find Madhesh");
    assert_eq!(p.id, 2);

    // Devanagari lookup
    let p_ne = get_province_by_name("मधेश").expect("Must find मधेश");
    assert_eq!(p_ne.id, 2);

    // District lookup
    let d = get_district_by_name("Kathmandu").expect("Must find Kathmandu");
    assert_eq!(d.id, 306);
    assert_eq!(d.name_ne, "काठमाडौँ");

    // Search English
    let search_ktm = search("Kathmandu");
    assert!(!search_ktm.is_empty());

    // Search Devanagari
    let search_rautahat = search("रौतहट");
    assert!(!search_rautahat.is_empty());
    let has_district = search_rautahat.iter().any(|r| match r {
        SearchResult::District(d) => d.name_en == "Rautahat",
        _ => false,
    });
    assert!(has_district, "Search for 'रौतहट' must return Rautahat district");
}
