# `nepal-geo-core`
*The Authoritative, Canonical Administrative Hierarchy and Geospatial Data Engine for Nepal*

[![Rust CI](https://img.shields.io/badge/Rust-100%25_Verified-brightgreen)](https://github.com/Ranjeet2063/nepal-geo-core)
[![TypeScript](https://img.shields.io/badge/TypeScript-Zero_Dependencies-blue)](https://github.com/Ranjeet2063/nepal-geo-core)
[![License](https://img.shields.io/badge/License-MIT_OR_Apache--2.0-blue.svg)](LICENSE)
[![Data Integrity](https://img.shields.io/badge/Administrative_Integrity-7_Provinces_|_77_Districts_|_753_Palikas-success)](https://github.com/Ranjeet2063/nepal-geo-core)

---

## 🌟 Why `nepal-geo-core`?

Almost every single web and mobile application built in Nepal—from e-commerce (Daraz), delivery (Pathao, InDrive), banking KYC (NIC Asia, Nabil), to government forms (Lok Sewa, Nagarik App)—requires dependent dropdowns:
$$\text{Province} \longrightarrow \text{District} \longrightarrow \text{Local Level (Palika)} \longrightarrow \text{Ward}$$

Historically, developers copy-pasted obsolete GitHub gists with misspelled district names ("Madhesh" vs "Province 2", "Rautahat " with trailing spaces), missing local levels, and zero Devanagari translation support.

`nepal-geo-core` is the definitive solution:
* **Canonical National Dataset**: Exactly **7 Provinces**, **77 Districts**, and **753 Local Levels** (6 Metropolitan, 11 Sub-Metropolitan, 276 Municipalities, 460 Rural Municipalities).
* **Dual-Language Native**: Complete English and Devanagari names for every single administrative unit.
* **Zero Dependencies**: Pure, self-contained data structures compiled directly into memory.
* **Instant Bilingual Search**: Search by English, Devanagari, or code in $<1\text{ ms}$.
* **Bottom-Up & Top-Down Traversal**: Instantly resolve `getHierarchy(localLevelId)` to receive the complete path from Province to Palika.

---

## 📦 Installation

### Rust Crate
```toml
[dependencies]
nepal-geo-core = "0.1.0"
```

### TypeScript / JavaScript (Node.js, Next.js, React, React Native)
```bash
npm install @ranjeet2063/nepal-geo-core
# or
pnpm add @ranjeet2063/nepal-geo-core
# or
bun add @ranjeet2063/nepal-geo-core
```

---

## 🚀 Quick Start

### 1. TypeScript / Node.js Example

```typescript
import {
  getProvinces,
  getDistrictsByProvince,
  getLocalLevelsByDistrict,
  getHierarchy,
  search
} from "@ranjeet2063/nepal-geo-core";

// 1. Dependent Dropdown: Province -> District -> Local Level
const provinces = getProvinces(); // 7 provinces
const madheshDistricts = getDistrictsByProvince(2); // 8 districts in Madhesh Province
const ktmLocalLevels = getLocalLevelsByDistrict(306); // 11 local levels in Kathmandu

// 2. Bottom-Up Hierarchy Resolution
const hierarchy = getHierarchy(30608); // Kathmandu Metropolitan City
console.log(hierarchy.province.nameEn); // "Bagmati"
console.log(hierarchy.district.nameEn); // "Kathmandu"
console.log(hierarchy.localLevel.nameNe); // "काठमाडौँ महानगरपालिका"

// 3. Bilingual Fuzzy Search
const results = search("रौतहट");
console.log(results[0].data.nameEn); // "Rautahat"
```

### 2. Rust Example

```rust
use nepal_geo_core::{get_provinces, get_districts_by_province, get_hierarchy, search};

fn main() {
    // 1. Get all provinces
    for p in get_provinces() {
        println!("{}: {} ({})", p.id, p.name_en, p.name_ne);
    }

    // 2. Get districts in Bagmati Province (id 3)
    let bagmati_districts = get_districts_by_province(3);
    println!("Bagmati has {} districts", bagmati_districts.len()); // 13

    // 3. Resolve complete administrative hierarchy
    if let Some(h) = get_hierarchy(30608) {
        println!("{} -> {} -> {}", h.province.name_en, h.district.name_en, h.local_level.name_ne);
    }

    // 4. Search in English or Devanagari
    let results = search("Pokhara");
    println!("Found {} matches", results.len());
}
```

---

## 📊 Administrative Data Integrity

| Level | Total Units | Breakdown |
|---|---|---|
| **Provinces** | **7** | Koshi (1), Madhesh (2), Bagmati (3), Gandaki (4), Lumbini (5), Karnali (6), Sudurpashchim (7) |
| **Districts** | **77** | 14 in Koshi, 8 in Madhesh, 13 in Bagmati, 11 in Gandaki, 12 in Lumbini, 10 in Karnali, 9 in Sudurpashchim |
| **Local Levels** | **753** | **6** Metropolitan Cities, **11** Sub-Metropolitan Cities, **276** Municipalities, **460** Rural Municipalities |

---

## 👤 Author & Organization
* **Author**: Ranjeet Gupta ([@Ranjeet2063](https://github.com/Ranjeet2063))
* **Organization**: Open-Source Nepal Infrastructure Project
* **License**: Dual-licensed under MIT or Apache 2.0.
