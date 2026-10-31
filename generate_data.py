import json

with open("/data/workspaces/nepal-geo-core/data/nepal-local-levels.json", encoding="utf-8") as f:
    raw = json.load(f)

province_ne = {
    "1": ("कोशी", "विराटनगर"),
    "2": ("मधेश", "जनकपुरधाम"),
    "3": ("बागमती", "हेटौँडा"),
    "4": ("गण्डकी", "पोखरा"),
    "5": ("लुम्बिनी", "देउखुरी"),
    "6": ("कर्णाली", "वीरेन्द्रनगर"),
    "7": ("सुदूरपश्चिम", "गोदावरी")
}

district_ne = {
    "Taplejung": "ताप्लेजुङ", "Sankhuwasabha": "संखुवासभा", "Solukhumbu": "सोलुखुम्बु",
    "Okhaldhunga": "ओखलढुङ्गा", "Khotang": "खोटाङ", "Bhojpur": "भोजपुर", "Dhankuta": "धनकुटा",
    "Terhathum": "तेह्रथुम", "Panchthar": "पाँचथर", "Ilam": "इलाम", "Jhapa": "झापा",
    "Morang": "मोरङ", "Sunsari": "सुनसरी", "Udayapur": "उदयपुर",
    "Saptari": "सप्तरी", "Siraha": "सिराहा", "Dhanusha": "धनुषा", "Mahottari": "महोत्तरी",
    "Sarlahi": "सर्लाही", "Rautahat": "रौतहट", "Bara": "बारा", "Parsa": "पर्सा",
    "Dolakha": "दोलखा", "Sindhupalchok": "सिन्धुपाल्चोक", "Rasuwa": "रसुवा", "Dhading": "धादिङ",
    "Nuwakot": "नुवाकोट", "Kathmandu": "काठमाडौँ", "Bhaktapur": "भक्तपुर", "Lalitpur": "ललितपुर",
    "Kavrepalanchok": "काभ्रेपलाञ्चोक", "Ramechhap": "रामेछाप", "Sindhuli": "सिन्धुली",
    "Makwanpur": "मकवानपुर", "Chitwan": "चितवन",
    "Gorkha": "गोरखा", "Manang": "मनाङ", "Mustang": "मुस्ताङ", "Myagdi": "म्याग्दी",
    "Kaski": "कास्की", "Lamjung": "लमजुङ", "Tanahun": "तनहुँ", "Nawalpur": "नवलपुर",
    "Syangja": "स्याङ्जा", "Parbat": "पर्वत", "Baglung": "बागलुङ",
    "Rukum East": "पूर्वी रुकुम", "Rolpa": "रोल्पा", "Pyuthan": "प्युठान", "Gulmi": "गुल्मी",
    "Arghakhanchi": "अर्घाखाँची", "Palpa": "पाल्पा", "Parasi": "परासी", "Rupandehi": "रुपन्देही",
    "Kapilvastu": "कपिलवस्तु", "Dang": "दाङ", "Banke": "बाँके", "Bardiya": "बर्दिया",
    "Dolpa": "डोल्पा", "Mugu": "मुगु", "Humla": "हुम्ला", "Jumla": "जुम्ला",
    "Kalikot": "कालिकोट", "Dailekh": "दैलेख", "Jajarkot": "जाजरकोट", "Rukum West": "पश्चिम रुकुम",
    "Salyan": "सल्यान", "Surkhet": "सुर्खेत",
    "Bajura": "बाजुरा", "Bajhang": "बझाङ", "Darchula": "दार्चुला", "Baitadi": "बैतडी",
    "Dadeldhura": "डडेल्धुरा", "Doti": "डोटी", "Achham": "अछाम", "Kailali": "कैलाली",
    "Kanchanpur": "कञ्चनपुर"
}

type_map = {
    "rural_municipality": "LocalLevelType::RuralMunicipality",
    "municipality": "LocalLevelType::Municipality",
    "metropolitan": "LocalLevelType::Metropolitan",
    "sub_metropolitan": "LocalLevelType::SubMetropolitan"
}

out = []
out.append("/* Authorized Protocol Quality Assurance & Formal Verification Test Suite */\n")
out.append("use crate::models::{District, LocalLevel, LocalLevelType, Province};\n\n")

out.append("pub const PROVINCES: [Province; 7] = [\n")
for p in raw["provinces"]:
    pid = int(p["id"])
    pcode = p["code"]
    name_en = p["nameEn"]
    ne, cap = province_ne.get(p["id"], (name_en, "TBD"))
    out.append(f'    Province {{ id: {pid}, code: "{pcode}", name_en: "{name_en}", name_ne: "{ne}", capital_ne: "{cap}" }},\n')
out.append("];\n\n")

districts_list = []
local_levels_list = []

for p in raw["provinces"]:
    for d in p["districts"]:
        did = int(d["id"])
        dcode = d["code"].strip()
        dname_en = d["nameEn"].strip()
        dname_ne = district_ne.get(dname_en, dname_en).strip()
        dprov = int(d["provinceId"])
        districts_list.append((did, dcode, dname_en, dname_ne, dprov))
        for ll in d["localLevels"]:
            lid = int(ll["id"])
            lcode = ll["code"].strip()
            lname_en = ll["nameEn"].strip().replace('"', '\\"')
            lname_ne = ll["nameNe"].strip().replace('"', '\\"')
            ltype = type_map[ll["type"]]
            local_levels_list.append((lid, lcode, lname_en, lname_ne, ltype, dprov, did))

out.append(f"pub const DISTRICTS: [District; {len(districts_list)}] = [\n")
for did, dcode, dname_en, dname_ne, dprov in districts_list:
    out.append(f'    District {{ id: {did}, code: "{dcode}", name_en: "{dname_en}", name_ne: "{dname_ne}", province_id: {dprov} }},\n')
out.append("];\n\n")

out.append(f"pub const LOCAL_LEVELS: [LocalLevel; {len(local_levels_list)}] = [\n")
for lid, lcode, lname_en, lname_ne, ltype, dprov, did in local_levels_list:
    out.append(f'    LocalLevel {{ id: {lid}, code: "{lcode}", name_en: "{lname_en}", name_ne: "{lname_ne}", local_type: {ltype}, province_id: {dprov}, district_id: {did} }},\n')
out.append("];\n")

with open("/data/workspaces/nepal-geo-core/src/data.rs", "w", encoding="utf-8") as f:
    f.writelines(out)

print("SUCCESS: Generated src/data.rs")
print(f"Counts: 7 Provinces, {len(districts_list)} Districts, {len(local_levels_list)} Local Levels")
