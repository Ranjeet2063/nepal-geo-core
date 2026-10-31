/* Authorized Protocol Quality Assurance & Formal Verification Test Suite */

use core::fmt;

/// Type of local government administrative body in Nepal.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LocalLevelType {
    /// महानगरपालिका (6 in Nepal: Kathmandu, Lalitpur, Pokhara, Bharatpur, Biratnagar, Birgunj)
    Metropolitan,
    /// उपमहानगरपालिका (11 in Nepal: Dharan, Itahari, Janakpur, Hetauda, Butwal, Tulsipur, Ghorahi, Nepalgunj, Dhangadhi, Kalaiya, Jitpur Simara)
    SubMetropolitan,
    /// नगरपालिका (276 in Nepal)
    Municipality,
    /// गाउँपालिका (460 in Nepal)
    RuralMunicipality,
}

impl LocalLevelType {
    pub fn name_en(&self) -> &'static str {
        match self {
            LocalLevelType::Metropolitan => "Metropolitan City",
            LocalLevelType::SubMetropolitan => "Sub-Metropolitan City",
            LocalLevelType::Municipality => "Municipality",
            LocalLevelType::RuralMunicipality => "Rural Municipality",
        }
    }

    pub fn name_ne(&self) -> &'static str {
        match self {
            LocalLevelType::Metropolitan => "महानगरपालिका",
            LocalLevelType::SubMetropolitan => "उपमहानगरपालिका",
            LocalLevelType::Municipality => "नगरपालिका",
            LocalLevelType::RuralMunicipality => "गाउँपालिका",
        }
    }
}

impl fmt::Display for LocalLevelType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name_en())
    }
}

/// Represents one of the 7 federal provinces of Nepal.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Province {
    pub id: u8,
    pub code: &'static str,
    pub name_en: &'static str,
    pub name_ne: &'static str,
    pub capital_ne: &'static str,
}

/// Represents one of the 77 administrative districts of Nepal.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct District {
    pub id: u16,
    pub code: &'static str,
    pub name_en: &'static str,
    pub name_ne: &'static str,
    pub province_id: u8,
}

/// Represents one of the 753 local level bodies (Palikas) of Nepal.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LocalLevel {
    pub id: u32,
    pub code: &'static str,
    pub name_en: &'static str,
    pub name_ne: &'static str,
    pub local_type: LocalLevelType,
    pub province_id: u8,
    pub district_id: u16,
}

/// Complete hierarchical administrative structure for a location.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdministrativeHierarchy {
    pub province: Province,
    pub district: District,
    pub local_level: LocalLevel,
}

/// Search result returned by fuzzy/prefix search.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SearchResult {
    Province(Province),
    District(District),
    LocalLevel(LocalLevel),
}
