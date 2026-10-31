/* Authorized Protocol Quality Assurance & Formal Verification Test Suite */

import {
  PROVINCES,
  DISTRICTS,
  LOCAL_LEVELS,
} from "./data.ts";
import type {
  Province,
  District,
  LocalLevel,
  LocalLevelType,
} from "./data.ts";

export type { Province, District, LocalLevel, LocalLevelType };

export interface AdministrativeHierarchy {
  province: Province;
  district: District;
  localLevel: LocalLevel;
}

export type SearchResult =
  | { type: "province"; data: Province }
  | { type: "district"; data: District }
  | { type: "localLevel"; data: LocalLevel };

/**
 * Returns all 7 federal provinces of Nepal.
 */
export function getProvinces(): readonly Province[] {
  return PROVINCES;
}

/**
 * Finds a province by its numeric ID (1..7).
 */
export function getProvinceById(id: number): Province | undefined {
  return PROVINCES.find((p) => p.id === id);
}

/**
 * Finds a province by name (English or Devanagari, case-insensitive).
 */
export function getProvinceByName(name: string): Province | undefined {
  const q = name.trim().toLowerCase();
  return PROVINCES.find(
    (p) => p.nameEn.toLowerCase() === q || p.nameNe === name.trim()
  );
}

/**
 * Returns all 77 districts of Nepal.
 */
export function getDistricts(): readonly District[] {
  return DISTRICTS;
}

/**
 * Finds a district by its numeric ID (e.g. 101, 206, 306).
 */
export function getDistrictById(id: number): District | undefined {
  return DISTRICTS.find((d) => d.id === id);
}

/**
 * Returns all districts belonging to a given province ID.
 */
export function getDistrictsByProvince(provinceId: number): District[] {
  return DISTRICTS.filter((d) => d.provinceId === provinceId);
}

/**
 * Finds a district by name (English or Devanagari, case-insensitive).
 */
export function getDistrictByName(name: string): District | undefined {
  const q = name.trim().toLowerCase();
  return DISTRICTS.find(
    (d) => d.nameEn.toLowerCase() === q || d.nameNe === name.trim()
  );
}

/**
 * Returns all 753 local level bodies (Palikas) of Nepal.
 */
export function getLocalLevels(): readonly LocalLevel[] {
  return LOCAL_LEVELS;
}

/**
 * Finds a local level by its unique ID (e.g., 10101, 30601).
 */
export function getLocalLevelById(id: number): LocalLevel | undefined {
  return LOCAL_LEVELS.find((ll) => ll.id === id);
}

/**
 * Returns all local levels within a given district ID.
 */
export function getLocalLevelsByDistrict(districtId: number): LocalLevel[] {
  return LOCAL_LEVELS.filter((ll) => ll.districtId === districtId);
}

/**
 * Returns all local levels within a given province ID.
 */
export function getLocalLevelsByProvince(provinceId: number): LocalLevel[] {
  return LOCAL_LEVELS.filter((ll) => ll.provinceId === provinceId);
}

/**
 * Returns the complete bottom-up administrative hierarchy for a given local level ID:
 * Province -> District -> LocalLevel.
 */
export function getHierarchy(localLevelId: number): AdministrativeHierarchy | undefined {
  const localLevel = getLocalLevelById(localLevelId);
  if (!localLevel) return undefined;
  const district = getDistrictById(localLevel.districtId);
  if (!district) return undefined;
  const province = getProvinceById(localLevel.provinceId);
  if (!province) return undefined;

  return { province, district, localLevel };
}

/**
 * High-speed bilingual search matching English names, Devanagari names, or codes.
 */
export function search(query: string): SearchResult[] {
  const q = query.trim();
  if (!q) return [];
  const qLower = q.toLowerCase();
  const results: SearchResult[] = [];

  // Provinces
  for (const p of PROVINCES) {
    if (
      p.nameEn.toLowerCase().includes(qLower) ||
      p.nameNe.includes(q) ||
      p.code === q
    ) {
      results.push({ type: "province", data: p });
    }
  }

  // Districts
  for (const d of DISTRICTS) {
    if (
      d.nameEn.toLowerCase().includes(qLower) ||
      d.nameNe.includes(q) ||
      d.code === q
    ) {
      results.push({ type: "district", data: d });
    }
  }

  // Local Levels
  for (const ll of LOCAL_LEVELS) {
    if (
      ll.nameEn.toLowerCase().includes(qLower) ||
      ll.nameNe.includes(q) ||
      ll.code === q
    ) {
      results.push({ type: "localLevel", data: ll });
    }
  }

  return results;
}
