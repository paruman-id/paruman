-- Seed sample organizational units
-- Kabupaten
INSERT INTO organizational_units (unit_type, name, code, geom)
VALUES
  ('kabupaten', 'Kabupaten Badung', 'KAB_BADUNG', NULL),
  ('kabupaten', 'Kabupaten Gianyar', 'KAB_GIANYAR', NULL)
ON CONFLICT (code) DO NOTHING;

-- Desa Adat
INSERT INTO organizational_units (unit_type, name, code, parent_id, geom)
SELECT 'desa_adat', 'Desa Adat Kuta', 'DESA_KUTA', id, NULL
FROM organizational_units WHERE code = 'KAB_BADUNG'
ON CONFLICT (code) DO NOTHING;

INSERT INTO organizational_units (unit_type, name, code, parent_id, geom)
SELECT 'desa_adat', 'Desa Adat Ubud', 'DESA_UBUD', id, NULL
FROM organizational_units WHERE code = 'KAB_GIANYAR'
ON CONFLICT (code) DO NOTHING;

-- Banjar
INSERT INTO organizational_units (unit_type, name, code, parent_id, geom)
SELECT 'banjar', 'Banjar Kuta', 'BANJAR_KUTA', id, NULL
FROM organizational_units WHERE code = 'DESA_KUTA'
ON CONFLICT (code) DO NOTHING;

INSERT INTO organizational_units (unit_type, name, code, parent_id, geom)
SELECT 'banjar', 'Banjar Padangtegal', 'BANJAR_PADANGTEGAL', id, NULL
FROM organizational_units WHERE code = 'DESA_UBUD'
ON CONFLICT (code) DO NOTHING;

-- Subak (functional; no parent)
INSERT INTO organizational_units (unit_type, name, code, geom)
VALUES
  ('subak', 'Subak Sayan', 'SUBAK_SAYAN', NULL),
  ('subak', 'Subak Kedewatan', 'SUBAK_KEDEWATAN', NULL)
ON CONFLICT (code) DO NOTHING;
