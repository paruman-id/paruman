# Paruman — Architecture

> *Paruman: the traditional Balinese community assembly. Digital infrastructure for the same purpose.*

**Status:** Design Phase — Phase 0 Complete
**Last Updated:** 2026-03-09
**Authors:** Paruman Core Team

---

## Table of Contents

1. [Vision & Principles](#1-vision--principles)
2. [Platform Ecosystem](#2-platform-ecosystem)
3. [The Three Forces Model](#3-the-three-forces-model)
4. [System Overview](#4-system-overview)
5. [Layer Breakdown](#5-layer-breakdown)
   - 5.1 [Surface Layer — Forum](#51-surface-layer--forum)
   - 5.2 [Identity Layer — Mandala ID](#52-identity-layer--mandala-id)
   - 5.3 [Data Backbone](#53-data-backbone)
   - 5.4 [Sovereignty Layer — MandalaChain](#54-sovereignty-layer--mandalachain)
   - 5.5 [Cultural Runtime](#55-cultural-runtime)
   - 5.6 [Knowledge Output — Lontar](#56-knowledge-output--lontar)
6. [Data Architecture](#6-data-architecture)
7. [Data Integration Contracts](#7-data-integration-contracts)
8. [Governance Model](#8-governance-model)
9. [Technology Stack](#9-technology-stack)
10. [Security & Privacy](#10-security--privacy)
11. [Phased Rollout](#11-phased-rollout)
12. [Boundary Agreements](#12-boundary-agreements)
13. [Decisions Log](#13-decisions-log)

---

## 1. Vision & Principles

### What Paruman Is

Paruman is community-owned digital infrastructure for the Balinese people. It presents as a familiar public forum — a place to talk — but is backed by a robust technical backbone that transforms community voice into structured, durable, and actionable signal.

It is not a civic app. It is not a government portal. It is not a social media platform. It is the digital form of an institution that already exists: the *paruman*, the assembly where Balinese communities deliberate and reach consensus.

### What Paruman Is Not

- Not a replacement for banjar, subak, or desa adat
- Not a tool for fighting government — a tool for arriving already organized
- Not foreign-owned infrastructure with Balinese content on top
- Not a platform that can be sold, captured, or unilaterally changed by its founders
- Not a token economy — Paruman issues no economic token and invents no new currency

### Design Principles

**1. Built with the grain of Balinese social structure.**
The Krama Mipil household is the unit. The banjar is the cluster. The desa adat is the frame. The island is the whole. Technology follows this hierarchy — it does not impose a new one.

**2. Accessible surface, powerful backbone.**
What users see feels like a warung conversation. What runs underneath is sovereign data infrastructure. The power is invisible to the user, but real.

**3. Public ownership is architectural, not promised.**
Community ownership is enforced by on-chain governance, not by goodwill. No single entity — including the founders — can unilaterally alter the platform's rules or extract its data.

**4. Cultural context is first-class.**
Balinese time (Pawukon, Sasih), Balinese script (Aksara Bali), Balinese language — these are not features. They are the foundation. The governance calendar runs on Pawukon, not Gregorian months.

**5. Musyawarah Mufakat over majority rule.**
Consensus is the goal. Voting is the fallback. Deferral is a legitimate outcome, not a failure. The platform's governance mechanics mirror the actual Paruman process — not a Western DAO.

**6. Signal over noise.**
The platform is designed to produce high-quality community signal, not engagement metrics. Consensus matters more than virality.

**7. Privacy by default, accountability by design.**
Users can speak with appropriate anonymity. The system prevents capture by bad actors without creating a surveillance database.

**8. Evidence-backed advocacy.**
Community positions are not just anecdotes. They are correlated with environmental and financial ground truth — making them harder to dismiss in any policy room.

**9. Epistemic honesty.**
Data correlations are presented as context, not causation. The platform does not overclaim. Credibility is the long-term asset.

---

## 2. Platform Ecosystem

Paruman does not exist in isolation. It sits on top of a sovereign data stack — each piece purpose-built, Balinese in origin, and composable.

```
╔══════════════════════════════════════════════════════════════════════════╗
║                        PARUMAN ECOSYSTEM                                 ║
╠══════════════════════════════════════════════════════════════════════════╣
║                                                                          ║
║  ┌──────────────────────────────────────────────────────────────────┐   ║
║  │                   FOUNDATION CRATES (Rust)                       │   ║
║  │                                                                  │   ║
║  │   balinese-calendar               lontar                         │   ║
║  │   ─────────────────               ──────                         │   ║
║  │   Pawukon · Sasih · Rahinan       Document generation            │   ║
║  │   Saka year conversion            Aksara Bali (lontar-aksara)    │   ║
║  │   Pancaroba detection             DOCX · PDF · HTML · MD         │   ║
║  │   Zero-allocation · no_std        Universal script support       │   ║
║  │   Published on crates.io ✓        Design phase                   │   ║
║  └──────────────────────────────────────┬────────────────────────────┘  ║
║                                         │                               ║
║  ┌──────────────────────────────────────▼────────────────────────────┐  ║
║  │                       DATA LAYERS                                 │  ║
║  │                                                                   │  ║
║  │  Environmental Data Warehouse      Financial Data Warehouse       │  ║
║  │  ──────────────────────────        ───────────────────────        │  ║
║  │  DataFusion · Arrow · Parquet      TimescaleDB · Redis · Axum     │  ║
║  │  PostGIS                                                          │  ║
║  │                                                                   │  ║
║  │  Climate (BMKG/NOAA/OpenWeather)   Crypto on-chain metrics        │  ║
║  │  Finance (IDX)                     EVM on-chain flows             │  ║
║  │  Saka enrichment ✓                 Market sentiment               │  ║
║  │  Kabupaten GIS ✓                   DeFi indicators                │  ║
║  │  Biodiversity (TBD)                Capital flow proxies           │  ║
║  └──────────────────────────────────────┬────────────────────────────┘  ║
║                                         │ feeds ground truth            ║
║  ┌──────────────────────────────────────▼────────────────────────────┐  ║
║  │                          PARUMAN                                  │  ║
║  │               Community Sovereignty Platform                      │  ║
║  │                     (this document)                               │  ║
║  │                                                                   │  ║
║  │   Forum · Mandala ID · Musyawarah governance · lontar output      │  ║
║  └──────────────────────────────────────┬────────────────────────────┘  ║
║                                         │                               ║
║  ┌──────────────────────────────────────▼────────────────────────────┐  ║
║  │                        MANDALACHAIN                               │  ║
║  │     Polkadot Parachain · Para ID 4818 · Indonesia · KPG token     │  ║
║  │   Mandala ID (IDCHAIN + PANDI) · ink! + EVM · Immutable record    │  ║
║  └───────────────────────────────────────────────────────────────────┘  ║
╚══════════════════════════════════════════════════════════════════════════╝
```

### Component Summary

| Component | Type | Role |
|---|---|---|
| `balinese-calendar` | Rust crate | Temporal foundation — Balinese time as first-class concept |
| `lontar` | Rust crate | Knowledge output — Aksara Bali, multi-format documents |
| Environmental Data Warehouse | Data layer | Environmental forces — climate, ecology, IDX finance |
| Financial Data Warehouse | Data layer | Financial forces — crypto markets, on-chain flows, sentiment |
| MandalaChain | Blockchain infra | Sovereignty layer — identity, governance, immutable record |
| Paruman | Platform | Community voice — the assembly that synthesizes all of the above |

---

## 3. The Three Forces Model

Bali's sociocultural crisis is not caused by a single force. Three distinct pressures act on the island simultaneously. Paruman's architecture is designed to make all three visible — and to put that visibility in the hands of the community.

```
┌──────────────────────────────────────────────────────────────────────┐
│                THREE FORCES ACTING ON BALI                           │
│                                                                      │
│  🌿 ENVIRONMENTAL              💰 FINANCIAL                          │
│  ─────────────────             ──────────                            │
│  Water stress                  Foreign capital inflows               │
│  Land use change               Crypto-wealthy buyers                 │
│  Climate anomalies             Villa & development investment        │
│  Pancaroba disruption          Digital nomad economy                 │
│  Deforestation                 Token / DeFi speculation              │
│  Pollution                     IDX-linked development pressure       │
│                                                                      │
│  Measured by:                  Measured by:                          │
│  Environmental Data Layer      Financial Data Layer                  │
│  BMKG · NOAA · Saka ✓          On-chain metrics · CoinGecko ✓        │
│                                Sentiment signals · DeFi ✓            │
│                                                                      │
│                   ┌──────────────────────┐                           │
│                   │   🗣 COMMUNITY VOICE  │                           │
│                   │   ─────────────────  │                           │
│                   │   What people are    │                           │
│                   │   actually           │                           │
│                   │   experiencing       │                           │
│                   │                      │                           │
│                   │   Captured by:       │                           │
│                   │   PARUMAN            │                           │
│                   └──────────────────────┘                           │
│                                                                      │
│  Community voice correlated with environmental and financial         │
│  ground truth becomes evidence — not anecdote.                      │
│                                                                      │
│  Presented as context, not causation. Epistemic honesty is          │
│  the credibility that makes the evidence hard to dismiss.           │
│                                                                      │
└──────────────────────────────────────────────────────────────────────┘
```

### The Evidence Principle

Financial data in particular must be handled carefully. On-chain crypto metrics are a real but noisy proxy for foreign capital pressure on Bali. The correct framing:

> *"During this period, community concerns about development pressure increased. Simultaneously, these financial indicators showed elevated activity. We present this as context — not as a causal claim."*

Let the policy room draw their own conclusions. This is more persuasive than overclaiming, and it protects Paruman's credibility as an evidence platform.

---

## 4. System Overview

```
┌──────────────────────────────────────────────────────────────────────┐
│                         PARUMAN SYSTEM                               │
│                                                                      │
│  ┌────────────────────────────────────────────────────────────────┐  │
│  │                     SURFACE LAYER                              │  │
│  │           Forum UI — mobile-first, low-bandwidth               │  │
│  │      Bahasa Indonesia · Balinese language · Aksara Bali        │  │
│  └────────────────────────────┬───────────────────────────────────┘  │
│                               │                                      │
│  ┌────────────────────────────▼───────────────────────────────────┐  │
│  │                    IDENTITY LAYER                               │  │
│  │   Mandala ID (IDCHAIN + PANDI) · W3C DIDs · KILT DIP           │  │
│  │   Krama Mipil household unit · pseudonymous by default         │  │
│  └────────────────────────────┬───────────────────────────────────┘  │
│                               │                                      │
│  ┌────────────────────────────▼───────────────────────────────────┐  │
│  │                    DATA BACKBONE                                │  │
│  │   Content DB · NLP · Geospatial · Longitudinal tracking        │  │
│  │                                                                 │  │
│  │   ┌──────────────────────────┐   ┌───────────────────────────┐ │  │
│  │   │ Environmental Data API   │   │ Financial Data API        │ │  │
│  │   │ Climate · Saka ✓         │   │ On-chain · Sentiment      │ │  │
│  │   │ Kabupaten GIS ✓          │   │ Context layer only        │ │  │
│  │   └──────────────────────────┘   └───────────────────────────┘ │  │
│  └──────────────────┬──────────────────────┬────────────────────────┘ │
│                     │                      │                          │
│  ┌──────────────────▼───────┐  ┌───────────▼─────────────────────┐   │
│  │  SOVEREIGNTY LAYER       │  │  KNOWLEDGE OUTPUT LAYER         │   │
│  │  MandalaChain            │  │  lontar crate                   │   │
│  │  Musyawarah governance   │  │  Community position documents   │   │
│  │  Kawenang (ink!)         │  │  Aksara Bali rendering          │   │
│  │  On-chain agreements     │  │  Policy-ready exports           │   │
│  │  Immutable record        │  │  DOCX · PDF · HTML · MD         │   │
│  └──────────────────────────┘  └─────────────────────────────────┘   │
│                                                                       │
└───────────────────────────────────────────────────────────────────────┘
```

---

## 5. Layer Breakdown

### 5.1 Surface Layer — Forum

The entry point for all community members. Designed to feel familiar — closer to a group chat or community board than a civic platform.

**Core features (MVP):**
- Threaded discussion organized by topic category
- Categories: *Tanah & Tata Ruang · Adat & Budaya · Air & Lingkungan · Pariwisata · Pendidikan · Umum*
- Posts carry optional location tag (banjar / subak / desa / kecamatan)
- Posts carry Balinese calendar context (auto-populated via `balinese-calendar`)
- Endorsement mechanism — not likes; endorsements carry identity weight
- Moderation by elected community moderators, not platform admins

**Design constraints:**
- Mobile-first, works on entry-level Android
- Functional at 2G / low bandwidth — progressive enhancement only
- No dark patterns, no engagement optimization, no algorithmic feed
- Chronological and relevance-sorted views, user-chosen
- Bahasa Indonesia primary; Balinese language supported; Aksara Bali rendered correctly via `lontar-aksara`

**Deliberately absent:**
- Follower counts or public metrics
- Viral / trending mechanics
- Advertising surface
- External embeds or tracking pixels

---

### 5.2 Identity Layer — Mandala ID

**Foundation:** Mandala ID is built on IDCHAIN — MandalaChain's decentralized identity infrastructure developed in partnership with PANDI (Indonesia's `.id` domain authority). The identity layer implements W3C-compliant Decentralized Identifiers (DIDs) via the KILT Protocol DIP pallet, supporting verifiable credentials, selective disclosure, and cross-chain identity portability.

**Design principle:** pseudonymous by default, accountable by design.

#### The Participation Unit: Krama Mipil

In traditional Balinese adat governance, the right to participate in a Paruman belongs to the **Krama Mipil** — the registered household representative. A household speaks with one voice. Paruman's identity model reflects this: governance standing (Kawenang) is held by the household representative, not an atomized individual.

Practically: one Mandala ID per participating household representative. Verification uses the Kartu Keluarga (KK) — the Indonesian household registry card — which is already a household-level document. Multiple household members may hold Tier 1 identity for reading and posting; only the registered Krama Mipil holds Kawenang.

#### Identity Tiers

```
Tier 0 — Guest
  └── Read-only access · no posting · no voting

Tier 1 — Verified Balinese (pseudonymous)
  └── Mandala ID with Bali residency / origin proof
  └── Can post, endorse, flag
  └── Identity is a DID hash — not a name

Tier 1D — Verified Diaspora Balinese
  └── Mandala ID with Balinese heritage proof
      (family registry / prior KTP / community attestation)
  └── Explicitly marked as diaspora in profile
  └── Can post, endorse, read everything
  └── Kawenang weight: 0.5× base · no seniority accrual
  └── Cannot hold moderator positions or Krama Mipil standing
  └── Rationale: diaspora has voice; resident household has priority

Tier 2 — Krama Mipil (Verified Household Representative)
  └── Tier 1 + verified banjar affiliation as household representative
  └── Verification: valid KTP/KK with Bali address via Mandala ID,
      OR written attestation from recognized Kelian Banjar
  └── Holds Kawenang — governance standing in the Musyawarah
  └── 35-day hold before Kawenang becomes active
      (one Pawukon cycle — one full cycle of community presence)

Tier 3 — Wakil Adat (Verified Adat Representative)
  └── Tier 2 + recognized role in banjar / subak / desa adat structure
  └── Facilitator role in Musyawarah and Paruman Agung
  └── Can submit and sign community position documents
  └── Kawenang role modifier applies
```

#### Subak as Organizational Entity

Subak is recognized as a first-class organizational unit alongside banjar and desa adat. It is a *functional* unit — defined by shared irrigation systems, crossing banjar and desa adat boundaries — not a geographic one.

```
OrganizationalUnit (abstract)
  ├── Banjar         (geographic · social)
  ├── Subak          (functional · water · crosses boundaries)
  ├── DesaAdat       (geographic · adat authority)
  └── Kecamatan      (administrative · government)
```

#### Onboarding Gap & Founder Ask

The current Mandala ID / IDCHAIN documented onboarding path requires a Polkadot.js browser extension and seed phrase — unsuitable for entry-level Android users in rural Bali. Paruman needs an embedded wallet path: phone number + OTP flow on the surface, with keys generated and held on-device (non-custodial), and banjar-based social recovery via `pallet-recovery` for lost devices.

This is the primary technical ask for the MandalaChain founders: *"For Paruman's users, we need a no-seed-phrase mobile onboarding path. Is this on the roadmap, and can Paruman be the use case that drives it?"*

#### Identity Data Location

| Data | Location | Reason |
|---|---|---|
| Personal identity | Mandala ID / IDCHAIN / user only | Never on Paruman servers |
| DID + community affiliation proof | On-chain (MandalaChain) | Verifiable but not linkable to personal data |
| Posting history | Paruman DB | Linked to pseudonymous DID only |
| Real-name mapping | Nowhere | Platform does not hold this |

---

### 5.3 Data Backbone

The backbone transforms conversation into structured, evidence-correlated community signal — silently, without changing the user experience.

**Content storage:** PostgreSQL for structured content (posts, threads, endorsements, metadata). Object storage for media. All content tagged: topic · location · Balinese calendar date · author tier.

**NLP signal extraction pipeline:**
```
Raw post text
  └── Language detection (Bahasa Indonesia / Balinese / mixed)
  └── Topic classification → issue taxonomy
  └── Sentiment analysis (community concern level)
  └── Entity extraction (places, people, institutions)
  └── Urgency scoring
  └── Consensus detection (multiple voices on same concern?)
→ SignalRecord attached to post
→ Aggregated into community signal store
```

**NLP language strategy:**
- *Phase 1:* IndoBERT / IndoNLU for Indonesian text; Balinese content partially handled via code-switching coverage. Imperfect but functional.
- *Phase 2+:* Fine-tune on a dedicated Balinese language corpus. Built in partnership with Universitas Udayana and/or Akademi Bahasa Bali. Sources: ban.wikipedia.org, OPUS, digitized lontar texts.
- *Ownership:* The fine-tuned model is owned by the Paruman community — open-licensed with attribution to Balinese cultural sources, not extractable commercially without community approval.
- Corpus-building is a cultural preservation act — it belongs in the Paruman narrative, not hidden in a tech backlog.

**Geospatial layer:** Posts georeferenced to banjar / desa / subak polygon. Heat maps of concentrated concerns. Overlay with land use maps, development permits, water catchment areas. Kabupaten boundaries shared with the environmental data layer's GIS.

**Temporal layer:** Every post timestamped in Gregorian and full Balinese calendar (Wuku, Wewaran, Sasih, Rahinan). Saka enrichment fields (`saka_sasih_id`, `pancaroba_flag`, `season_tag`) sourced from the environmental data layer's pipeline — not rebuilt.

**Environmental correlation:** Community posts about water, flooding, drought, heat, or land correlated with BMKG/OpenWeather/NOAA measured data. Resolution: kabupaten-level (honest and defensible). Kecamatan-level via statistical interpolation where BMKG station density supports it. Banjar-level deferred until the environmental data layer's IoT sensor network is deployed. Precision is never fabricated.

**Financial pressure:** Community posts about development, land price spikes, or foreign buyer influx correlated with capital flow indicators sourced from the financial data layer. All financial data presented as context only — see Section 3 (Evidence Principle).

**Longitudinal tracking:** Issues tracked over time: when first raised, sentiment evolution, whether addressed. Periodic anchoring to MandalaChain — tamper-proof. Durable evidence base for policy engagement.

---

### 5.4 Sovereignty Layer — MandalaChain

MandalaChain provides the trust infrastructure that makes community ownership real rather than promised.

**Chain:** Polkadot parachain, Para ID 4818. Active on Paseo testnet. Native token: KPG (Kepeng Token, named after the traditional Balinese coin) on mainnet, KPGT on testnet. Both EVM and WASM (ink!) contract environments are live and documented.

**Gas:** KPG is the chain-native currency. Paruman issues no economic token of its own. KPG pays gas fees — Paruman community treasury sponsors gas for new users so they never see or think about fees.

**Contract layer:** ink! (WASM, Substrate-native) for Kawenang management, Musyawarah mechanics, data access agreements, and the immutable community record.

**What goes on-chain:**

| Data | Why |
|---|---|
| Identity proofs (Mandala ID / IDCHAIN DIDs) | Self-sovereign · not platform-controlled |
| Kawenang grants and weights | Community-controlled · tamper-proof |
| Musyawarah outcomes (mufakat / ditunda / pemungutan) | Immutable record of how decisions were reached |
| Platform rule changes | Cannot be made unilaterally |
| Data access permission grants | Community-controlled · auditable |
| Periodic signal anchors | Immutable community record |
| Verified community position documents | Timestamped · signed by Wakil Adat |
| Environmental + financial data access agreements | Terms anchored as on-chain law |

**What stays off-chain:** Post content · NLP results · user sessions · media attachments · raw data layer outputs.

---

### 5.5 Cultural Runtime

The layer that makes Paruman feel Balinese rather than just Balinese-themed.

**`balinese-calendar` integration:** Every content item carries full Balinese calendar context: Wuku, Wewaran, Sasih, Rahinan. Rahinan awareness on platform surface. Governance cycle anchored to Pawukon — Pasangkepan Rutin every 35 days on a community-configured Wuku day. Pancaroba flags in ecological discussions.

**Nampih Sasih (annual maintenance):** The intercalary month declared annually by PHDI is a Tier 0 maintenance action — factual, not policy. A designated *Calendar Keeper* monitors PHDI's declaration, submits a PR to `balinese-calendar`, and Paruman updates via CI. Community notified, not consulted. Long-term: formal relationship with PHDI for early access to annual declarations.

**Script and language:** Aksara Bali rendered via `lontar-aksara` (rustybuzz / HarfBuzz). Balinese language supported as its own language track, not an Indonesian dialect. Bahasa Indonesia primary in UI.

**Adat taxonomy:** Krama Mipil → Banjar → Subak → Desa Adat → Kecamatan → Kabupaten hierarchy throughout the data model. Subak is first-class, not a banjar subcategory.

---

### 5.6 Knowledge Output — Lontar

When community voice needs to become a formal document — a position paper, a government submission, a durable record — `lontar` provides the rendering layer.

**Use cases:** Community position documents with environmental and financial corroborating data; legislative submissions formatted for government intake; public HTML records of community stances; archive documents — the digital equivalent of lontar manuscripts.

**Lontar readiness and interim path:** `lontar` is in design phase. Phase 5 interim: well-structured HTML anchored on-chain, converted to PDF via headless Chromium (2-week implementation). Native `lontar` output replaces it once DOCX maturity is reached (estimated 6–9 months from focused start).

**Integration:** Musyawarah matters that reach mufakat trigger a render job. Output documents anchored to MandalaChain — immutable, timestamped, signed by Wakil Adat. Multiple formats from one source: PDF for government, HTML for web, MD for archive.

---

## 6. Data Architecture

### Core Entities

```
User
  ├── mandala_id_did               (W3C DID — pseudonymous)
  ├── identity_tier                (0, 1, 1D, 2, 3)
  ├── is_diaspora                  (bool)
  ├── is_krama_mipil               (bool — household representative)
  ├── kk_proof_hash                (on-chain — household registry)
  ├── banjar_proof_hash            (on-chain)
  ├── subak_membership[]           (→ SubakMembership)
  └── joined_at                    (Gregorian + Balinese date)

Kawenang
  ├── holder_id                    (→ User, must be Krama Mipil)
  ├── banjar_id
  ├── base_weight                  (1.0 at grant)
  ├── seniority_weight             (accrues over time, capped 2.5)
  ├── role_modifier                (see Section 8)
  ├── effective_weight             (computed)
  ├── granted_at                   (Gregorian + Balinese date)
  ├── active_from                  (granted_at + 35 days)
  └── chain_anchor_hash

Post
  ├── id · author_id · thread_id
  ├── content_text · content_language (id / ban / mixed)
  ├── location_banjar · location_subak · location_desa · location_kabupaten
  ├── gregorian_date · balinese_date  (BalineseDate from balinese-calendar)
  ├── topic_category
  ├── signal_record_id             (→ SignalRecord)
  └── endorsement_count            (weighted by Kawenang)

SubakMembership
  ├── user_id · subak_id · role    (anggota / pekaseh / etc.)

SignalRecord
  ├── post_id · topics[] · sentiment_score · urgency_score
  ├── entities[] · consensus_cluster_id
  ├── env_correlation_id           (→ environmental data snapshot)
  ├── fin_correlation_id           (→ financial data snapshot)
  └── processed_at

MusyawarahMatter
  ├── id · track                   (paruman_agung / pasangkepan_rutin)
  ├── title · raised_by · banjar_id
  ├── deliberation_opened_at · deliberation_closes_at
  ├── facilitator_id               (Wakil Adat for Agung · moderator for Rutin)
  ├── outcome                      (mufakat / ditunda / pemungutan_suara)
  ├── outcome_legitimacy           (consensus = high · pemungutan = standard)
  ├── supporting_post_ids[]
  └── chain_anchor_hash

CommunityPosition
  ├── id · title · issuing_entity_type · issuing_entity_id
  ├── musyawarah_matter_id         (must be mufakat outcome)
  ├── issue_category · content_summary
  ├── env_evidence_snapshot        (environmental data at time of position)
  ├── fin_evidence_snapshot        (financial data at time of position)
  ├── resolution_notes             (data resolution level disclosed explicitly)
  ├── signed_by_ids[]              (→ Wakil Adat / Tier 3)
  ├── chain_anchor_hash
  ├── render_url · render_method   (html_interim / lontar_native)
  └── issued_at
```

### Data Ownership

```
Paruman servers:           Post content · signal analysis · aggregates
MandalaChain:              DIDs · Kawenang · Musyawarah outcomes · position anchors
                           Data access agreements (on-chain law)
Users:                     Their identity (Mandala ID) · their posts (deletable)
Community:                 Aggregated signal · platform config · NLP model (Phase 2+)
Environmental data layer:  Climate · IDX finance · biodiversity data
Financial data layer:      Crypto · on-chain · sentiment data
Nobody:                    The governance rules — they are on-chain law
```

---

## 7. Data Integration Contracts

### Governing Principles

- Access terms anchored on MandalaChain as ink! storage
- Community must approve any change to how external data is consumed or restricted
- Paruman never stores raw external data — only derived snapshots at point of correlation
- Breaking API changes require 90-day notice minimum
- If either data layer ceases Bali-relevant data collection, a full historical export (Parquet / CSV) is provided within 90 days

### Environmental Data Access Agreement

```
Environmental Data Layer ↔ Paruman Community

1. PERPETUAL READ ACCESS
   Perpetual read access to Bali-relevant climate datasets at the
   v1 API surface. This right survives any change in the data
   layer's ownership or operational direction.

2. DATA CONTINUITY
   If the environmental data layer ceases operation, a full
   historical export is provided within 90 days in open format
   (Parquet / CSV).

3. NO UNILATERAL MONETIZATION
   Data access cannot be made paid without minimum 12-month notice
   and a Paruman community vote.

4. SCHEMA STABILITY
   Breaking API changes require 90-day notice and a migration path.

5. PARUMAN DATA STAYS WITH PARUMAN
   No community signal data flows to the environmental data layer
   without an explicit community vote.

6. AMENDMENT
   Both sides must consent: data layer operator approval + Paruman
   community supermajority (66%) via Paruman Agung track.
```

**What Paruman consumes from the environmental data layer:**

| Data | Frequency | Notes |
|---|---|---|
| Climate observations (Bali kabupaten) | On-demand | v1 API |
| Saka enrichment fields | Per-post | Reuse pipeline — don't rebuild |
| Kabupaten GIS boundaries | Static reference | Shared PostGIS or GeoJSON |
| Pancaroba and season tags | Per-post | Pipeline output |
| Historical climate | Community position evidence | DataFusion query |

**Resolution disclosure:** All community position documents disclose the data resolution level explicitly. Kabupaten-level is the current honest ceiling. Kecamatan interpolation where BMKG station density supports it. Banjar-level deferred.

**Example query — water shortage evidence:**
```sql
SELECT
    observation_date,
    saka_sasih_id,
    saka_wuku_id,
    season_tag,
    pancaroba_flag,
    rainfall_mm,
    temperature_avg,
    kabupaten
FROM climate.observations
WHERE kabupaten = 'Karangasem'
  AND observation_date >= NOW() - INTERVAL '18 months'
ORDER BY observation_date;
-- Presented as kabupaten-level context. Not banjar-level precision.
```

### Financial Data Access Agreement

Same structure as the environmental data access agreement. All financial data is context layer only — never causal.

**What Paruman consumes from the financial data layer:**

| Data | Use case | Source |
|---|---|---|
| Crypto market sentiment | Capital flow pressure indicator | Santiment · CryptoPanic |
| On-chain active addresses (EVM) | Foreign capital activity proxy | Etherscan · Chainbase |
| DeFi TVL | Investment pressure indicator | DeFiLlama |
| Stablecoin on-chain flows | Liquidity indicator | Chainbase |
| Market sentiment index | Macro indicator | CoinGecko · CMC |

**Required disclosure on all documents using financial data:**
> *"The following financial indicators are presented as contextual evidence of concurrent capital flow activity, not as direct causal claims."*

---

## 8. Governance Model

Paruman's governance is not a DAO. It is a digital implementation of the Musyawarah Mufakat process that already governs Balinese community life — with the chain providing tamper-proof record-keeping, not replacing the process itself.

### The Core Principle: Musyawarah Mufakat

Decisions are reached through collective deliberation, not head counts. The goal of every governance matter is **mufakat** — unanimous consensus that preserves community harmony (wicara). Formal voting exists as a last resort. Deferral (**ditunda**) is a legitimate and respected outcome.

```
PRIMARY PATH      Musyawarah → Mufakat
                  Deliberation opens · community discusses ·
                  consensus emerges · facilitator declares mufakat ·
                  outcome anchored on-chain at highest legitimacy

VALID OUTCOME     Ditunda (Deferred)
                  No consensus reached · matter deferred for
                  further informal discussion · reopened in next cycle
                  No stigma — this is culturally correct

LAST RESORT       Pemungutan Suara (Formal Vote)
                  Only when: no consensus AND deferral would cause harm
                  Weighted by Kawenang · supermajority required
                  Explicitly marked as lower-legitimacy in the record
```

### Kawenang — Governance Standing

**Kawenang** (authority, right, legitimacy in Balinese) is the earned governance standing of a verified Krama Mipil household representative within their community. It is not a token, not a currency, not transferable, and has no economic value. It is the digital form of the standing a household holds in their banjar — accumulated through presence, time, and recognized role.

Kawenang is not displayed as a number in the user interface. Users see community standing, role, and seniority — not a score.

**Weight model:**

| Basis | Weight |
|---|---|
| Kawenang inactive (within 35-day hold) | 0 |
| Active base (Krama Mipil, new) | 1.0 |
| + 6 months verified membership | 1.2 |
| + 1 year | 1.5 |
| + 2 years | 2.0 |
| + 5 years (cap) | 2.5 |
| Kelian Banjar attestor role | + 0.5 |
| Elected moderator (active term) | + 0.3 |
| Subak Pekaseh | + 0.3 |
| Wakil Adat / Tier 3 representative | + 0.5 |
| Diaspora (Tier 1D) | 0.5 × base · no seniority accrual |

Weight cannot be purchased, transferred, or delegated. It accrues only through verified community presence over time.

### Two Meeting Tracks

Traditional Balinese governance has a hierarchy of assemblies. Paruman mirrors this with two distinct deliberation tracks that differ in rhythm, scope, and legitimacy threshold.

#### Pasangkepan Rutin (Operational Assembly)

The regular community meeting. In traditional practice, held every 35 days on a specific Balinese calendar day. Handles the ongoing operational life of the platform and community.

```
Rhythm:          Every 35 days · on the Wuku day configured per banjar
                 (each banjar sets their Wuku anchor in their Awig-Awig)

Scope:           Feature additions · moderator elections
                 Treasury spending · routine operational decisions
                 Per-banjar configuration updates

Deliberation:    Agenda published at cycle open
                 Discussion throughout the 35-day window
                 Consensus check at cycle close

Facilitator:     Elected moderator

Fallback:        Pemungutan Suara · 66% Kawenang supermajority

On-chain:        Outcome anchored at cycle close
```

#### Paruman Agung (Strategic Assembly)

The high assembly. Reserved for strategic and constitutional matters.

```
Triggered by:    Proposed changes to platform rules or core principles
                 Data access agreement changes
                 Founding Council dissolution
                 Any matter with island-wide or multi-cycle implications

Deliberation:    Minimum 3 Pasangkepan cycles (105 days)
                 Cannot be shortened — not even by consensus

Facilitator:     Wakil Adat (Tier 3)

Fallback:        Pemungutan Suara · 80% Kawenang supermajority

On-chain:        Full deliberation transcript reference + outcome anchored
```

### Per-Banjar Awig-Awig Configuration

Balinese customary law varies by village. Each registered banjar configures their own governance parameters within the shared platform framework — anchored on-chain per banjar:

```
  └── Wuku day for Pasangkepan Rutin
  └── Minimum deliberation period (platform minimum is the floor)
  └── Local quorum expectations
  └── Custom role definitions
  └── Whether subak matters route through banjar or directly
```

### Bootstrap: Founding Council

A Founding Council (5–9 people, spanning multiple kabupaten) makes initial platform decisions. Explicitly temporary. **Dissolves automatically at 500 active Krama Mipil.** Authority limited to: initial Kawenang grant decisions, bootstrap moderator appointment, technical deployment. No veto over community decisions at any point.

**Bootstrap moderation:**
- Phase 1a: founder-appointed moderators, named publicly, all decisions logged
- Phase 1b: first community moderator election at 200 Krama Mipil
- After election, founder-appointed moderators expire automatically

### Governance Reference Table

| Matter | Track | Min. deliberation | Fallback threshold |
|---|---|---|---|
| Feature addition | Pasangkepan Rutin | 35 days | 66% |
| Moderator election | Pasangkepan Rutin | 35 days | 66% |
| Treasury spending | Pasangkepan Rutin | 35 days | 66% |
| Per-banjar config update | Pasangkepan Rutin | 35 days | 66% |
| Platform rule change | Paruman Agung | 105 days | 80% |
| Data access grant | Paruman Agung | 105 days | 80% |
| Data integration contract change | Paruman Agung | 105 days | 80% |
| Core principle change | Paruman Agung | 105 days | 80% |
| Emergency security action | Multisig 3-of-5 | Immediate · disclosed within 48h | — |
| Nampih Sasih update | Tier 0 maintenance | N/A | Community notified, not consulted |

### Outcome Legitimacy Record

The on-chain record distinguishes *how* a decision was reached. This distinction matters in a policy room.

```
Mufakat        → consensus · highest legitimacy
Ditunda        → deferred · valid outcome · no stigma
Pemungutan     → formal vote · fallback · lower legitimacy
                 marked as such in all downstream documents
```

A community position document backed by mufakat carries more weight than one backed by a 66% vote. The record preserves this distinction permanently and visibly.

---

## 9. Technology Stack

### Chain Infrastructure

| Component | Detail |
|---|---|
| Blockchain | MandalaChain · Polkadot parachain · Para ID 4818 |
| Testnet | Mandala Paseo testnet |
| Testnet RPC | `wss://rpc1.paseo.mandalachain.io` · `wss://rpc2.paseo.mandalachain.io` |
| Native token | KPG (mainnet) · KPGT (testnet) · 18 decimals |
| Gas | KPG — community treasury sponsors fees for users |
| Block time | ~12 seconds |
| Identity | Mandala ID (IDCHAIN + PANDI) · KILT Protocol DIP · W3C DIDs |
| Smart contracts | ink! (WASM) — primary · Solidity (EVM) — available |
| Explorer | Blockscout (EVM-compatible) |
| Hub | hub.mandalachain.io — Paruman to be registered at Phase 1 launch |

### Backend

| Component | Technology | Rationale |
|---|---|---|
| API server | Rust (Axum) | Performance · memory safety · consistent across Paruman's data stack |
| Database | PostgreSQL + PostGIS | Reliable · geospatial · shared geometry with environmental data layer |
| NLP pipeline | Python (spaCy / transformers) + Rust FFI | NLP ecosystem · Rust for perf-critical paths |
| Object storage | S3-compatible | Media attachments |
| Task queue | Tokio + Redis | Async signal processing |
| Calendar | `balinese-calendar` (Rust crate) | Native · zero-allocation · correct |

### Frontend

| Component | Technology | Rationale |
|---|---|---|
| Web app | SvelteKit | Lightweight · fast · SSR for low bandwidth |
| Mobile | PWA first · native later | Reach before polish |
| Script rendering | HarfBuzz via `lontar-aksara` | Correct Aksara Bali shaping |
| Offline support | Service workers + IndexedDB | Patchy connectivity tolerance |

### Blockchain Contracts & Pallets

| Component | Technology | Rationale |
|---|---|---|
| Kawenang contract | ink! (WASM) | Non-transferable · seniority-weighted · banjar-scoped |
| Musyawarah contract | ink! (WASM) | Two-track deliberation · outcome anchoring |
| Data access agreements | ink! storage | On-chain law — not a promise |
| Social recovery | `pallet-recovery` | Banjar as guardian set · no seed phrase needed |
| Posting proxy / session | `pallet-proxy` | Scoped daily-use key · root key unexposed |
| Gas sponsorship | KPG · community treasury relay | Users never see gas fees |

### Knowledge Output

| Component | Technology | Rationale |
|---|---|---|
| Phase 5 interim | HTML + headless Chromium → PDF | 2-week implementation · unblocks Phase 5 |
| Phase 5 production | `lontar` Rust crate | Native Aksara Bali · multi-format · in-house |
| Script shaping | `lontar-aksara` (rustybuzz) | Correct complex script rendering |

---

## 10. Security & Privacy

### Threat Model

| Threat | Mitigation |
|---|---|
| State surveillance | Pseudonymous DID identity · no real-name mapping stored |
| Capture by political group | Musyawarah governance · geographic distribution · consensus requirement |
| Sybil attacks | Mandala ID on-chain verification · Krama Mipil requires KK/banjar attestation · 35-day Kawenang hold |
| Data monetization | Community governance prevents unilateral extraction |
| Founder capture | No permanent special authority · multisig transfers at 500-member trigger |
| Infrastructure seizure | Decentralized governance · data portable · rules on-chain |
| Manipulation of community signal | NLP outlier detection · weighted by verified Kawenang |
| Data layer dependency capture | Integration contracts on-chain · community must approve changes |
| Overclaimed causality | Evidence principle enforced in document templates · financial data disclosure required |
| Governance manipulation by wealth | Kawenang not purchasable · KPG holdings have zero governance weight |
| Diaspora capture | Diaspora Kawenang 0.5× base · no seniority accrual · cannot hold moderator positions |

### Privacy Architecture

- No real name or personal identity stored on Paruman servers
- No third-party analytics, tracking pixels, or external scripts
- Posts deletable by author; pseudonymous DID can be abandoned
- Data access requests by researchers or government require community vote — transparent, auditable by all Krama Mipil
- Environmental and financial data accessed via defined contracts — raw data never replicated into Paruman

---

## 11. Phased Rollout

### Phase 0 — Foundation (complete)
- Architecture finalized ✓
- `balinese-calendar` crate published on crates.io ✓
- `lontar` crate in design phase
- Climate & environmental data layer · Saka enrichment complete ✓
- Financial data layer operational · sentiment + on-chain live ✓
- MandalaChain confirmed: Polkadot parachain Para ID 4818 · active on Paseo ✓
- IDCHAIN + PANDI identity infrastructure confirmed ✓
- ink! confirmed live on Paseo testnet ✓
- KPG as chain-native currency confirmed (no custom token needed) ✓
- Governance model designed from Musyawarah Mufakat principles ✓

### Phase 1 — MVP Forum
- Basic forum: threads, posts, endorsements
- Bahasa Indonesia + Balinese language support (IndoBERT NLP)
- Mandala ID authentication (Tier 1, Tier 1D, Tier 2 / Krama Mipil)
- Subak as first-class organizational unit in data model
- 6 core topic categories · mobile-first PWA
- Founding Council appointed (5–9 members, multiple kabupaten)
- Bootstrap moderators: named, public, all decisions logged
- Invite-only seed: 3–5 banjar communities
- Kawenang contract deployed (ink!) · 35-day hold on all grants
- Pasangkepan Rutin cadence configured per seed banjar
- Paruman registered in MandalaChain Hub
- Build in public: GitHub, architecture docs, design decisions open

### Phase 2 — Data Backbone + Environmental Layer
- NLP signal extraction pipeline live
- Balinese calendar context on all posts
- Geospatial tagging and heat maps (kabupaten resolution)
- Environmental data API integration live — climate correlation
- Community dashboard (read-only) for Krama Mipil
- Longitudinal issue tracking
- First community moderator election (triggered at 200 Krama Mipil)
- Balinese language corpus work begins (Universitas Udayana outreach)
- Per-banjar Awig-Awig configuration live

### Phase 3 — Financial Pressure Layer
- Financial data API integration live
- Capital flow context in community dashboards
- Cross-correlation view: community voice + environmental + financial
- Data integration contracts anchored on-chain
- Founding Council sunset (triggered at 500 Krama Mipil)
- Multisig keys transferred to community-elected representatives

### Phase 4 — Full Sovereignty Layer
- Paruman Agung and Pasangkepan Rutin fully on-chain
- All governance through Musyawarah process on MandalaChain
- On-chain anchoring of community positions
- Data access permission governance active
- Balinese NLP model fine-tuning begins (if corpus ready)

### Phase 5 — Knowledge Output
- Interim: HTML/PDF community position documents (headless Chromium)
- `lontar` integration when DOCX maturity reached (replaces interim)
- Aksara Bali rendering in all outputs
- Evidence-backed PDF / DOCX export for policy engagement
- Public archive with full data provenance and resolution disclosures

### Phase 6 — Scale & Ecosystem
- Open to all Balinese communities island-wide
- Inter-banjar and inter-subak coordination tools
- Researcher API (community-permissioned)
- Balinese NLP model published (community-owned, open license with attribution)
- Potential model: other Indonesian regions under cultural pressure
- zkML on-chain inference for community signal (MandalaChain AI Agent Layer)

---

## 12. Boundary Agreements

### Environmental Data Layer / Paruman

- The environmental data layer is external infrastructure; Paruman is a consumer via versioned API contract
- Paruman community signal data does not flow to the environmental data layer without a community vote
- On-chain integration contract terms (Section 7) are the protection if the data layer's direction changes

### Financial Data Layer / Paruman

- The financial data layer is external infrastructure; Paruman is a consumer via versioned API contract
- Financial data is context layer only — never causal
- Same on-chain contract structure as the environmental data layer

### MandalaChain / Paruman

- MandalaChain provides chain infrastructure; Paruman is an application on top
- KPG is the gas currency; Paruman introduces no competing token
- Mandala ID / IDCHAIN is the identity layer; Paruman does not build its own
- On-chain agreements and data portability provisions protect the community if MandalaChain changes direction
- Paruman is a flagship community use case for MandalaChain's Digital Nations thesis — this alignment is mutual, not one-directional

---

## 13. Decisions Log

| Date | Decision | Rationale |
|---|---|---|
| 2026-03-09 | Platform name: Paruman | Traditional Balinese community assembly — carries institutional legitimacy |
| 2026-03-09 | Blockchain: MandalaChain | Indonesian sovereign infra · Polkadot parachain Para ID 4818 · PANDI identity integration |
| 2026-03-09 | Smart contracts: ink! (WASM) | Live on Paseo · Rust-native · consistent with Paruman's data stack |
| 2026-03-09 | No custom token economy | Paruman issues no economic token · KPG is the chain currency · governance standing is not tradeable |
| 2026-03-09 | Governance: Musyawarah Mufakat | Mirrors actual Balinese deliberation · consensus primary · voting is last resort |
| 2026-03-09 | Participation unit: Krama Mipil | Balinese adat governance is household-based · KK document already in verification path |
| 2026-03-09 | Governance standing: Kawenang | Earned through presence and role · not purchasable · no crypto connotations in user-facing surfaces |
| 2026-03-09 | Two meeting tracks | Paruman Agung (strategic · 105-day min) and Pasangkepan Rutin (operational · 35-day cycle) mirror traditional hierarchy |
| 2026-03-09 | Governance calendar: Pawukon | 35-day Pasangkepan cycle anchored to Wuku · not Gregorian months |
| 2026-03-09 | Per-banjar Awig-Awig config | Customary law varies by village — platform must respect this |
| 2026-03-09 | Deferral as valid outcome | Ditunda is culturally correct · not a system failure |
| 2026-03-09 | Outcome legitimacy record | Mufakat > pemungutan in the on-chain record — the how matters as much as the what |
| 2026-03-09 | Kawenang hold: 35 days | One Pawukon cycle — one full cycle of community presence before governance weight activates |
| 2026-03-09 | Founding Council sunset: 500 Krama Mipil | Concrete, measurable trigger — not a date that can be deferred |
| 2026-03-09 | Moderation election trigger: 200 Krama Mipil | Community-driven · earlier than full governance maturity |
| 2026-03-09 | Diaspora: 0.5× base Kawenang | Voice without overriding resident household priority · no seniority accrual |
| 2026-03-09 | Subak: first-class entity | Subak is functional, not geographic — crosses banjar boundaries |
| 2026-03-09 | Environmental data resolution: kabupaten now | Epistemic honesty — fabricating precision destroys credibility |
| 2026-03-09 | Financial data: context only, not causal | Overclaiming causality is a credibility risk in policy rooms |
| 2026-03-09 | NLP Phase 1: IndoBERT | Pragmatic MVP — Balinese fine-tuning is Phase 2+ with community-owned model |
| 2026-03-09 | lontar Phase 5 interim: HTML/Chromium PDF | Unblocks Phase 5 without 6-month lontar dependency |
| 2026-03-09 | Data integration contracts on-chain | Protects community if any external data layer changes direction |
| 2026-03-09 | PHDI calendar: Tier 0 maintenance | Factual update · not policy · community notified not consulted |
| 2026-03-09 | Build in public from day one | Trust is architectural · community follows the build · early adopters arrive invested |
| 2026-03-09 | AA gap: founder ask for Phase 1 | No mobile embedded wallet path exists today · primary integration ask to MandalaChain |

---

*"Paruman — Suara Bali"*

---

**Document status:** Living document. Updated as architecture evolves.
**Next review:** After MandalaChain founder conversation and Phase 1 technical spike.
