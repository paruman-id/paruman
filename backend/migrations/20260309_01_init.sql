-- Enable extensions
CREATE EXTENSION IF NOT EXISTS postgis;
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS pgcrypto;

-- Enums
CREATE TYPE identity_tier AS ENUM ('guest', 'tier1', 'tier1d', 'tier2', 'tier3');
CREATE TYPE content_language AS ENUM ('id', 'ban', 'mixed');
CREATE TYPE topic_category AS ENUM (
  'land_tata_ruang',
  'adat_budaya',
  'air_lingkungan',
  'pariwisata',
  'pendidikan',
  'umum'
);
CREATE TYPE musyawarah_track AS ENUM ('pasangkepan_rutin', 'paruman_agung');
CREATE TYPE musyawarah_outcome AS ENUM ('mufakat', 'ditunda', 'pemungutan_suara');

-- Organizational Units (banjar, subak, desa adat, kecamatan, kabupaten)
CREATE TABLE organizational_units (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  unit_type TEXT NOT NULL CHECK (unit_type IN ('banjar', 'subak', 'desa_adat', 'kecamatan', 'kabupaten')),
  name TEXT NOT NULL,
  code TEXT UNIQUE,
  parent_id UUID REFERENCES organizational_units(id) ON DELETE SET NULL,
  geom geometry(MultiPolygon, 4326)
);
CREATE INDEX organizational_units_geom_idx ON organizational_units USING GIST (geom);

-- Users
CREATE TABLE users (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  mandala_id_did TEXT NOT NULL UNIQUE,
  identity_tier identity_tier NOT NULL DEFAULT 'guest',
  is_diaspora BOOLEAN NOT NULL DEFAULT FALSE,
  is_krama_mipil BOOLEAN NOT NULL DEFAULT FALSE,
  joined_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Kawenang (governance standing)
CREATE TABLE kawenang (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  holder_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  banjar_id UUID NOT NULL REFERENCES organizational_units(id) ON DELETE CASCADE,
  base_weight NUMERIC(6,2) NOT NULL DEFAULT 1.0,
  seniority_weight NUMERIC(6,2) NOT NULL DEFAULT 1.0,
  role_modifier NUMERIC(6,2) NOT NULL DEFAULT 0.0,
  granted_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  active_from TIMESTAMPTZ NOT NULL,
  chain_anchor_hash TEXT
);
CREATE INDEX kawenang_holder_idx ON kawenang(holder_id);

-- Threads
CREATE TABLE threads (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  category topic_category NOT NULL,
  title TEXT NOT NULL,
  created_by UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  location_unit_id UUID REFERENCES organizational_units(id) ON DELETE SET NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Posts
CREATE TABLE posts (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  thread_id UUID NOT NULL REFERENCES threads(id) ON DELETE CASCADE,
  author_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  content TEXT NOT NULL,
  content_language content_language NOT NULL DEFAULT 'id',
  location_banjar UUID REFERENCES organizational_units(id) ON DELETE SET NULL,
  location_subak UUID REFERENCES organizational_units(id) ON DELETE SET NULL,
  location_desa UUID REFERENCES organizational_units(id) ON DELETE SET NULL,
  location_kabupaten UUID REFERENCES organizational_units(id) ON DELETE SET NULL,
  balinese_date JSONB NOT NULL DEFAULT '{}'::jsonb,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  topic_category topic_category NOT NULL,
  endorsement_count INTEGER NOT NULL DEFAULT 0
);
CREATE INDEX posts_thread_idx ON posts(thread_id);
CREATE INDEX posts_location_idx ON posts(location_banjar, location_subak, location_desa, location_kabupaten);

-- Endorsements
CREATE TABLE endorsements (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  post_id UUID NOT NULL REFERENCES posts(id) ON DELETE CASCADE,
  endorser_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  kawenang_weight_snapshot NUMERIC(6,2) NOT NULL DEFAULT 0.0,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  UNIQUE(post_id, endorser_id)
);

-- Musyawarah Matters
CREATE TABLE musyawarah_matters (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  track musyawarah_track NOT NULL,
  title TEXT NOT NULL,
  raised_by UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  banjar_id UUID NOT NULL REFERENCES organizational_units(id) ON DELETE CASCADE,
  deliberation_opened_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  deliberation_closes_at TIMESTAMPTZ NOT NULL,
  outcome musyawarah_outcome,
  chain_anchor_hash TEXT,
  facilitator_id UUID REFERENCES users(id) ON DELETE SET NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX musyawarah_track_idx ON musyawarah_matters(track);
CREATE INDEX musyawarah_banjar_idx ON musyawarah_matters(banjar_id);

-- Subak Membership
CREATE TABLE subak_memberships (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  subak_id UUID NOT NULL REFERENCES organizational_units(id) ON DELETE CASCADE,
  role TEXT NOT NULL DEFAULT 'anggota'
);
CREATE INDEX subak_memberships_user_idx ON subak_memberships(user_id);
CREATE INDEX subak_memberships_subak_idx ON subak_memberships(subak_id);
