/* Authorized Protocol Quality Assurance & Formal Verification Test Suite */

import test from "node:test";
import assert from "node:assert/strict";
import {
  getProvinces,
  getProvinceById,
  getProvinceByName,
  getDistricts,
  getDistrictById,
  getDistrictsByProvince,
  getLocalLevels,
  getLocalLevelById,
  getLocalLevelsByDistrict,
  getHierarchy,
  search,
} from "../src/index.ts";

test("National administrative total counts", () => {
  const provinces = getProvinces();
  assert.equal(provinces.length, 7);

  const districts = getDistricts();
  assert.equal(districts.length, 77);

  const localLevels = getLocalLevels();
  assert.equal(localLevels.length, 753);
});

test("Province queries and Devanagari lookups", () => {
  const p2 = getProvinceById(2);
  assert.ok(p2);
  assert.equal(p2.nameEn, "Madhesh");
  assert.equal(p2.nameNe, "मधेश");

  const pByNameEn = getProvinceByName("Madhesh");
  assert.deepEqual(pByNameEn, p2);

  const pByNameNe = getProvinceByName("मधेश");
  assert.deepEqual(pByNameNe, p2);
});

test("District and Local Level hierarchy navigation", () => {
  // Kathmandu district id 306
  const ktm = getDistrictById(306);
  assert.ok(ktm);
  assert.equal(ktm.nameEn, "Kathmandu");
  assert.equal(ktm.nameNe, "काठमाडौँ");
  assert.equal(ktm.provinceId, 3); // Bagmati

  const ktmLevels = getLocalLevelsByDistrict(306);
  assert.equal(ktmLevels.length, 11);

  // Kathmandu Metropolitan id 30608
  const ktmMetro = getLocalLevelById(30608);
  assert.ok(ktmMetro);
  assert.equal(ktmMetro.type, "metropolitan");

  const hierarchy = getHierarchy(30608);
  assert.ok(hierarchy);
  assert.equal(hierarchy.province.nameEn, "Bagmati");
  assert.equal(hierarchy.district.nameEn, "Kathmandu");
  assert.equal(hierarchy.localLevel.id, 30608);
});

test("Bilingual search capability", () => {
  const searchKtm = search("Kathmandu");
  assert.ok(searchKtm.length > 0);

  const searchRautahat = search("रौतहट");
  assert.ok(searchRautahat.length > 0);
  const foundDistrict = searchRautahat.some(
    (r) => r.type === "district" && r.data.nameEn === "Rautahat"
  );
  assert.ok(foundDistrict);
});
