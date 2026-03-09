# Paruman Development TODO

**Status:** Phase 0 Complete — Ready for Phase 1 Development  
**Last Updated:** 2026-03-09  
**Purpose:** Comprehensive checklist to guide end-to-end development from MVP to full ecosystem

---

## Phase 0 — Foundation ✅ COMPLETE

- [x] Architecture finalized
- [x] `balinese-calendar` crate published on crates.io
- [x] Climate & environmental data layer with Saka enrichment complete
- [x] Financial data layer operational (sentiment + on-chain live)
- [x] MandalaChain confirmed (Para ID 4818, active on Paseo)
- [x] IDCHAIN + PANDI identity infrastructure confirmed
- [x] ink! confirmed live on Paseo testnet
- [x] KPG as chain-native currency confirmed
- [x] Governance model designed from Musyawarah Mufakat principles
- [x] `lontar` crate in design phase
- [x] Create paruman-id GitHub org
- [x] Initial repo commit (README, ARCHITECTURE, governance, CONTRIBUTING, CODEOWNERS)
- [x] Set repo description and topics

---

## Phase 1 — MVP Forum

### 1.1 Project Setup & Infrastructure

- [x] Initialize monorepo structure
  - [x] Backend (Rust/Axum)
  - [x] Frontend (SvelteKit)
  - [x] Smart contracts (ink!)
  - [x] Shared types/utilities
- [x] Configure development environment
  - [x] Docker compose for local development
  - [x] PostgreSQL + PostGIS setup
  - [x] Redis for task queue
  - [x] S3-compatible object storage (MinIO for dev)
- [x] Set up CI/CD pipeline
  - [x] Automated testing
  - [x] Build verification
  - [x] Deployment automation
- [x] Set up MandalaChain Paseo testnet connection
  - [x] RPC endpoints configured
  - [x] Test wallet with KPGT tokens
  - [x] Explorer integration for debugging

### 1.2 Identity Layer — Mandala ID Integration

- [x] **CRITICAL: Founder conversation with MandalaChain team**
  - [x] Present mobile onboarding gap (no seed phrase path)
  - [x] Request embedded wallet roadmap discussion
  - [x] Propose Paruman as flagship use case
  - [x] Document outcome and integration path
- [x] Research Mandala ID / IDCHAIN documentation
  - [x] W3C DID implementation details
  - [x] KILT DIP pallet integration
  - [x] Verification flow for KK (Kartu Keluarga)
  - [x] Banjar attestation mechanism
- [x] Implement authentication service
  - [x] DID verification endpoint
  - [x] Session management with `pallet-proxy` for daily-use keys
  - [x] Identity tier detection (Tier 0, 1, 1D, 2, 3)
- [x] Build identity verification flows
  - [x] Tier 1: Bali residency/origin proof
  - [x] Tier 1D: Diaspora heritage proof + explicit marking
  - [x] Tier 2: Krama Mipil (KK verification + banjar attestation)
  - [x] Tier 3: Wakil Adat (role verification)
- [x] Implement 35-day Kawenang hold mechanism
  - [x] Track verification timestamp
  - [x] Calculate activation date (Gregorian + 1 Pawukon cycle)
  - [x] Prevent governance actions before activation

### 1.3 Smart Contracts (ink!)

- [x] Set up ink! development environment
  - [x] cargo-contract installation
  - [x] Local substrate node for testing
  - [x] Paseo testnet deployment scripts
- [x] Kawenang contract
  - [x] Non-transferable governance standing token
  - [x] Base weight: 1.0 for new Krama Mipil
  - [x] Seniority accrual logic (6mo→1.2, 1yr→1.5, 2yr→2.0, 5yr→2.5 cap)
  - [x] Role modifiers (Kelian +0.5, Moderator +0.3, Pekaseh +0.3, Wakil Adat +0.5)
  - [x] Diaspora weight: 0.5× base, no seniority accrual
  - [x] 35-day activation hold
  - [x] Banjar-scoped grants
  - [x] Query functions for effective weight calculation
- [x] Musyawarah contract (basic)
  - [x] Matter creation (title, track, banjar_id, raised_by)
  - [x] Deliberation window tracking
  - [x] Outcome recording (mufakat / ditunda / pemungutan_suara)
  - [x] Legitimacy level storage
  - [x] Chain anchor hash storage
- [ ] Deploy contracts to Paseo testnet
  - [ ] Test with multiple accounts
  - [ ] Verify weight calculations
  - [ ] Test governance flows
- [ ] Document contract addresses and ABIs

### 1.4 Database Schema

- [ ] Design PostgreSQL schema
  - [ ] Users table (mandala_id_did, identity_tier, is_diaspora, is_krama_mipil, joined_at)
  - [ ] Kawenang table (holder_id, banjar_id, weights, granted_at, active_from, chain_anchor_hash)
  - [ ] OrganizationalUnits (abstract: Banjar, Subak, DesaAdat, Kecamatan)
  - [ ] SubakMembership (user_id, subak_id, role)
  - [ ] Posts table (author_id, thread_id, content, location tags, calendar context)
  - [ ] Threads table (category, title, created_at)
  - [ ] Endorsements table (post_id, endorser_id, kawenang_weight_snapshot)
  - [ ] MusyawarahMatters table (track, title, outcome, facilitator_id, timestamps)
- [ ] Implement PostGIS extensions
  - [ ] Banjar polygon geometries
  - [ ] Subak polygon geometries (functional boundaries)
  - [ ] Desa Adat boundaries
  - [ ] Kabupaten boundaries (shared with environmental data layer)
- [ ] Create migration scripts
- [ ] Seed development data
  - [ ] Test banjar entries
  - [ ] Test subak entries
  - [ ] Sample organizational hierarchy

### 1.5 Backend API (Rust/Axum)

- [ ] Set up Axum server structure
  - [ ] Router configuration
  - [ ] Middleware (auth, logging, CORS)
  - [ ] Error handling
  - [ ] Database connection pool
- [ ] Authentication endpoints
  - [ ] POST /auth/verify-did
  - [ ] POST /auth/session
  - [ ] GET /auth/me
  - [ ] POST /auth/logout
- [ ] Forum endpoints
  - [ ] GET /threads (with filtering by category, location)
  - [ ] POST /threads
  - [ ] GET /threads/:id/posts
  - [ ] POST /posts
  - [ ] DELETE /posts/:id (author only)
  - [ ] POST /posts/:id/endorse
  - [ ] DELETE /posts/:id/endorse
- [ ] User profile endpoints
  - [ ] GET /users/me
  - [ ] GET /users/:did/public-profile
- [ ] Organizational unit endpoints
  - [ ] GET /banjar
  - [ ] GET /subak
  - [ ] GET /desa-adat
- [ ] Integrate `balinese-calendar` crate
  - [ ] Auto-populate Balinese date on post creation
  - [ ] Expose calendar conversion utilities
  - [ ] Calculate Pasangkepan Rutin dates per banjar

### 1.6 Frontend (SvelteKit)

- [ ] Set up SvelteKit project
  - [ ] SSR configuration for low bandwidth
  - [ ] Progressive enhancement strategy
  - [ ] Mobile-first responsive design
- [ ] Design system
  - [ ] Implement Tri Datu color palette per Brand_Guidelines.md
  - [ ] Typography (Fraunces, Source Serif 4, Noto Sans per Brand_Guidelines.md)
  - [ ] Component library (buttons, forms, cards per Brand_Guidelines.md)
  - [ ] No dark patterns, no engagement optimization
- [ ] Authentication UI
  - [ ] Login/verification flow
  - [ ] Identity tier display (not as numbers)
  - [ ] Session management
- [ ] Forum UI
  - [ ] Thread list view (chronological + relevance-sorted toggle)
  - [ ] Thread detail view with posts
  - [ ] Post creation form
  - [ ] Category filtering (Tanah & Tata Ruang, Adat & Budaya, Air & Lingkungan, Pariwisata, Pendidikan, Umum)
  - [ ] Location tagging UI (banjar/subak/desa/kecamatan)
  - [ ] Balinese calendar context display
  - [ ] Endorsement mechanism (not "likes")
- [ ] User profile UI
  - [ ] Community standing display (not Kawenang numbers)
  - [ ] Role badges
  - [ ] Seniority indication
- [ ] Offline support
  - [ ] Service worker for caching
  - [ ] IndexedDB for offline reading
  - [ ] Sync queue for offline posts
- [ ] Language support
  - [ ] Bahasa Indonesia (primary)
  - [ ] Balinese language support
  - [ ] Language switcher
- [ ] Aksara Bali rendering
  - [ ] Integrate `lontar-aksara` (rustybuzz/HarfBuzz)
  - [ ] Test with complex script shaping
  - [ ] Fallback for unsupported browsers

### 1.7 Governance Bootstrap

- **Dependency:** Founding Council must be formed before Kawenang contracts are deployed (1.3) and before community seeding (1.8).
- [ ] Form Founding Council
  - [ ] Recruit 5-9 members spanning multiple kabupaten
  - [ ] Document members publicly
  - [ ] Set up multisig wallet (3-of-5 for emergency actions)
- [ ] Appoint bootstrap moderators
  - [ ] Name publicly
  - [ ] Document all moderation decisions
  - [ ] Set up moderation logging system
- [ ] Configure initial banjar Awig-Awig
  - [ ] Wuku day for Pasangkepan Rutin per banjar
  - [ ] Minimum deliberation periods
  - [ ] Local quorum expectations
  - [ ] Store on-chain

### 1.8 Community Seeding

- [ ] Identify 3-5 seed banjar communities
  - [ ] Geographic distribution
  - [ ] Mix of urban/rural
  - [ ] Existing community organization strength
- [ ] Community outreach
  - [ ] Present Paruman vision to Kelian Banjar
  - [ ] Explain verification process
  - [ ] Address concerns and questions
- [ ] Onboarding support
  - [ ] In-person verification assistance
  - [ ] Mobile device setup help
  - [ ] Low-bandwidth testing
- [ ] Initial Kawenang grants
  - [ ] Verify Krama Mipil status
  - [ ] Deploy Kawenang contracts per banjar
  - [ ] 35-day hold begins

### 1.9 Registration & Launch

- [ ] Register Paruman in MandalaChain Hub
  - [ ] hub.mandalachain.io registration
  - [ ] Project metadata
  - [ ] Links to documentation
- [ ] Deploy to production
  - [ ] Backend infrastructure
  - [ ] Frontend hosting
  - [ ] Database backups
  - [ ] Monitoring and alerting
- [ ] Announce Phase 1 launch
  - [ ] GitHub announcement
  - [ ] Community notification
  - [ ] Build-in-public update

### 1.10 Testing & Quality Assurance

- [ ] Unit tests
  - [ ] Backend API endpoints
  - [ ] Smart contract functions
  - [ ] Calendar integration
- [ ] Integration tests
  - [ ] End-to-end user flows
  - [ ] Chain interaction tests
  - [ ] Database operations
- [ ] Low-bandwidth testing
  - [ ] 2G connection simulation
  - [ ] Entry-level Android device testing
  - [ ] Progressive enhancement verification
- [ ] Security audit
  - [ ] Smart contract audit
  - [ ] API security review
  - [ ] Privacy architecture review

---

## Phase 2 — Data Backbone + Environmental Layer

### 2.1 NLP Signal Extraction Pipeline

- [ ] Set up Python NLP service
  - [ ] spaCy / transformers environment
  - [ ] Rust FFI bindings for performance-critical paths
  - [ ] Redis task queue integration
- [ ] Language detection
  - [ ] Bahasa Indonesia detection
  - [ ] Balinese language detection
  - [ ] Mixed language handling
- [ ] Topic classification
  - [ ] Map to issue taxonomy
  - [ ] 6 core categories + subtopics
- [ ] Sentiment analysis
  - [ ] Community concern level scoring
  - [ ] IndoBERT model integration
- [ ] Entity extraction
  - [ ] Places (banjar, desa, landmarks)
  - [ ] People (roles, not personal names)
  - [ ] Institutions
- [ ] Urgency scoring
  - [ ] Keyword-based initial implementation
  - [ ] Refinement based on community feedback
- [ ] Consensus detection
  - [ ] Multiple voices on same concern
  - [ ] Cluster similar posts
- [ ] SignalRecord storage
  - [ ] Attach to posts
  - [ ] Aggregate into community signal store

### 2.2 Balinese Calendar Integration

- [ ] Full calendar context on all posts
  - [ ] Wuku, Wewaran, Sasih, Rahinan
  - [ ] Saka year
  - [ ] Pancaroba flags
  - [ ] Season tags
- [ ] Governance calendar
  - [ ] Calculate Pasangkepan Rutin dates per banjar
  - [ ] 35-day cycle tracking
  - [ ] Paruman Agung 105-day minimum enforcement
- [ ] Rahinan awareness in UI
  - [ ] Display important calendar days
  - [ ] Cultural context tooltips
- [ ] Nampih Sasih maintenance process
  - [ ] Designate Calendar Keeper role
  - [ ] PHDI declaration monitoring
  - [ ] PR submission to `balinese-calendar`
  - [ ] CI update and community notification

### 2.3 Geospatial Layer

- [ ] PostGIS integration
  - [ ] Banjar polygon storage
  - [ ] Subak polygon storage (functional boundaries)
  - [ ] Desa Adat boundaries
  - [ ] Kabupaten boundaries (shared with env data layer)
- [ ] Post georeferencing
  - [ ] Location tagging to polygons
  - [ ] Spatial queries
- [ ] Heat maps
  - [ ] Concentrated concern visualization
  - [ ] Kabupaten-level resolution
- [ ] Overlay capabilities
  - [ ] Land use maps
  - [ ] Development permits (if available)
  - [ ] Water catchment areas

### 2.4 Environmental Data Integration

- [ ] Environmental data API client
  - [ ] v1 API integration
  - [ ] Authentication and rate limiting
  - [ ] Error handling and retries
- [ ] Climate correlation
  - [ ] Match posts about water/flooding/drought/heat
  - [ ] Query BMKG/OpenWeather/NOAA data
  - [ ] Kabupaten-level resolution (honest ceiling)
  - [ ] Kecamatan interpolation where station density supports
- [ ] Saka enrichment reuse
  - [ ] Consume pipeline output from env data layer
  - [ ] Don't rebuild — integrate
  - [ ] Fields: saka_sasih_id, pancaroba_flag, season_tag
- [ ] Historical climate queries
  - [ ] DataFusion integration
  - [ ] Time-series analysis
  - [ ] Evidence snapshots for community positions
- [ ] Resolution disclosure
  - [ ] Explicit data resolution in all outputs
  - [ ] Never fabricate precision
  - [ ] Kabupaten-level clearly stated

### 2.5 Longitudinal Tracking

- [ ] Issue tracking over time
  - [ ] When first raised
  - [ ] Sentiment evolution
  - [ ] Whether addressed
  - [ ] Related posts clustering
- [ ] Periodic chain anchoring
  - [ ] Tamper-proof timestamps
  - [ ] Signal snapshots
  - [ ] Immutable evidence base

### 2.6 Community Dashboard

- [ ] Read-only dashboard for Krama Mipil
  - [ ] Community signal aggregates
  - [ ] Issue trends over time
  - [ ] Geographic heat maps
  - [ ] Environmental correlation views
- [ ] Data export
  - [ ] CSV/JSON exports for transparency
  - [ ] Community-permissioned access

### 2.7 Moderation & Community Management

- [ ] First community moderator election
  - [ ] Triggered at 200 Krama Mipil
  - [ ] Pasangkepan Rutin track
  - [ ] 35-day deliberation
  - [ ] Kawenang-weighted voting (fallback)
- [ ] Moderator tools
  - [ ] Flag review interface
  - [ ] Post moderation actions (hide, remove)
  - [ ] All actions logged publicly
- [ ] Bootstrap moderator expiration
  - [ ] Automatic after community election
  - [ ] No manual handoff needed

### 2.8 Balinese Language Corpus Work

- [ ] Universitas Udayana outreach
  - [ ] Present corpus-building as cultural preservation
  - [ ] Discuss partnership structure
  - [ ] Community ownership of resulting model
- [ ] Corpus sources
  - [ ] ban.wikipedia.org
  - [ ] OPUS multilingual corpus
  - [ ] Digitized lontar texts (with permission)
  - [ ] Community-contributed content
- [ ] Licensing framework
  - [ ] Open license with attribution
  - [ ] Community approval required for commercial use
  - [ ] Cultural source acknowledgment

### 2.9 Per-Banjar Awig-Awig Configuration

- [ ] Configuration UI for banjar leaders
  - [ ] Wuku day selection
  - [ ] Deliberation period settings
  - [ ] Quorum expectations
  - [ ] Custom role definitions
  - [ ] Subak routing preferences
- [ ] On-chain storage
  - [ ] ink! contract for Awig-Awig
  - [ ] Per-banjar configuration anchoring
  - [ ] Version history

---

## Phase 3 — Financial Pressure Layer

### 3.1 Financial Data Integration

- [ ] Financial data API client
  - [ ] Sentiment API (Santiment, CryptoPanic)
  - [ ] On-chain API (Etherscan, Chainbase)
  - [ ] DeFi API (DeFiLlama)
  - [ ] Market API (CoinGecko, CMC)
- [ ] Data consumption
  - [ ] Crypto market sentiment
  - [ ] EVM active addresses
  - [ ] DeFi TVL indicators
  - [ ] Stablecoin flows
  - [ ] Market sentiment index
- [ ] Context-only framing
  - [ ] Never causal claims
  - [ ] Always disclosed as concurrent indicators
  - [ ] Evidence Principle enforcement

### 3.2 Cross-Correlation Views

- [ ] Dashboard integration
  - [ ] Community voice + environmental + financial
  - [ ] Three Forces Model visualization
  - [ ] Time-series correlation charts
  - [ ] Kabupaten-level aggregation
- [ ] Evidence snapshots
  - [ ] Capture data state at time of community position
  - [ ] Store with resolution disclosure
  - [ ] Immutable anchoring

### 3.3 Data Integration Contracts

- [ ] Environmental data access agreement
  - [ ] Perpetual read access terms
  - [ ] Data continuity provisions
  - [ ] No unilateral monetization clause
  - [ ] Schema stability requirements
  - [ ] Community data protection
  - [ ] Amendment process (supermajority)
- [ ] Financial data access agreement
  - [ ] Same structure as environmental
  - [ ] Context-only disclosure requirement
- [ ] On-chain anchoring
  - [ ] ink! storage for contract terms
  - [ ] Community vote required for changes
  - [ ] Transparent and auditable

### 3.4 Founding Council Sunset

- [ ] Monitor Krama Mipil count
  - [ ] Automated trigger at 500 active
  - [ ] No manual override possible
- [ ] Multisig key transfer
  - [ ] Community-elected representatives
  - [ ] 3-of-5 multisig for emergency actions
  - [ ] Transparent handoff process
- [ ] Founding Council dissolution
  - [ ] Automatic authority expiration
  - [ ] Public announcement
  - [ ] Historical record preservation

---

## Phase 4 — Full Sovereignty Layer

### 4.1 Musyawarah Contract (Full Implementation)

- [ ] Pasangkepan Rutin track
  - [ ] 35-day cycle enforcement
  - [ ] Agenda publication
  - [ ] Discussion period tracking
  - [ ] Consensus check mechanism
  - [ ] Fallback to pemungutan suara (66% threshold)
  - [ ] Outcome anchoring
- [ ] Paruman Agung track
  - [ ] 105-day minimum (3 Pawukon cycles)
  - [ ] Cannot be shortened
  - [ ] Strategic matter triggers
  - [ ] Wakil Adat facilitation
  - [ ] Fallback to pemungutan suara (80% threshold)
  - [ ] Full deliberation transcript reference
- [ ] Outcome legitimacy recording
  - [ ] Mufakat (highest legitimacy)
  - [ ] Ditunda (valid, no stigma)
  - [ ] Pemungutan (lower legitimacy, marked)
- [ ] Matter lifecycle
  - [ ] Creation
  - [ ] Deliberation
  - [ ] Consensus check
  - [ ] Outcome recording
  - [ ] Chain anchoring

### 4.2 Community Position Documents

- [ ] CommunityPosition entity
  - [ ] Title, issuing entity, Musyawarah matter reference
  - [ ] Must be mufakat outcome
  - [ ] Issue category and content summary
  - [ ] Environmental evidence snapshot
  - [ ] Financial evidence snapshot
  - [ ] Resolution disclosure
  - [ ] Wakil Adat signatures
  - [ ] Chain anchor hash
- [ ] Position creation workflow
  - [ ] Triggered by mufakat outcome
  - [ ] Evidence correlation
  - [ ] Document generation
  - [ ] Wakil Adat review and signing
  - [ ] On-chain anchoring
- [ ] Position storage and retrieval
  - [ ] Database storage
  - [ ] Public archive
  - [ ] Search and filtering

### 4.3 Data Access Permission Governance

- [ ] Permission request workflow
  - [ ] Researcher/government data access requests
  - [ ] Paruman Agung track (105-day deliberation)
  - [ ] Community vote (80% supermajority)
  - [ ] Transparent and auditable
- [ ] Permission grants
  - [ ] Scoped access (time-limited, purpose-limited)
  - [ ] On-chain record
  - [ ] Revocation mechanism
- [ ] Access logging
  - [ ] All data access logged
  - [ ] Visible to all Krama Mipil
  - [ ] Audit trail

### 4.4 Balinese NLP Model Fine-Tuning

- [ ] Corpus readiness check
  - [ ] Sufficient Balinese language data
  - [ ] Quality validation
  - [ ] Licensing confirmed
- [ ] Fine-tuning process
  - [ ] Base model selection
  - [ ] Training infrastructure
  - [ ] Evaluation metrics
  - [ ] Community validation
- [ ] Model ownership
  - [ ] Community-owned
  - [ ] Open license with attribution
  - [ ] Commercial use requires community approval
- [ ] Deployment
  - [ ] Replace IndoBERT for Balinese content
  - [ ] A/B testing
  - [ ] Performance monitoring

---

## Phase 5 — Knowledge Output

### 5.1 Interim Solution (HTML/PDF)

- [ ] HTML template design
  - [ ] Community position document structure
  - [ ] Aksara Bali rendering via lontar-aksara
  - [ ] Evidence sections (environmental + financial)
  - [ ] Resolution disclosure
  - [ ] Signature section
- [ ] Headless Chromium integration
  - [ ] HTML to PDF conversion
  - [ ] Styling for print
  - [ ] Page breaks and formatting
- [ ] Render pipeline
  - [ ] Triggered by mufakat outcome
  - [ ] Evidence correlation
  - [ ] HTML generation
  - [ ] PDF conversion
  - [ ] On-chain anchoring
- [ ] 2-week implementation target
  - [ ] Unblocks Phase 5
  - [ ] Temporary until lontar ready

### 5.2 Lontar Integration (Production)

- [ ] Monitor `lontar` crate maturity
  - [ ] DOCX generation capability
  - [ ] Aksara Bali rendering
  - [ ] Multi-format output (DOCX, PDF, HTML, MD)
- [ ] Integration when ready
  - [ ] Replace interim HTML/Chromium solution
  - [ ] Native Rust implementation
  - [ ] Better performance and control
- [ ] Document templates
  - [ ] Community position documents
  - [ ] Legislative submissions
  - [ ] Public HTML records
  - [ ] Archive documents (lontar manuscript style)
- [ ] Multi-format export
  - [ ] PDF for government
  - [ ] DOCX for editing
  - [ ] HTML for web
  - [ ] Markdown for archive

### 5.3 Evidence-Backed Export

- [ ] Policy-ready formatting
  - [ ] Government submission templates
  - [ ] Legislative format compliance
  - [ ] Professional presentation
- [ ] Data provenance
  - [ ] Full source attribution
  - [ ] Resolution disclosure
  - [ ] Methodology transparency
- [ ] Public archive
  - [ ] All community positions published
  - [ ] Searchable and filterable
  - [ ] Permanent URLs
  - [ ] On-chain anchoring

---

## Phase 6 — Scale & Ecosystem

### 6.1 Island-Wide Expansion

- [ ] Open to all Balinese communities
  - [ ] Remove invite-only restriction
  - [ ] Self-service banjar registration
  - [ ] Verification scaling
- [ ] Onboarding optimization
  - [ ] Streamlined verification flow
  - [ ] Mobile-first improvements
  - [ ] Multi-language support
- [ ] Infrastructure scaling
  - [ ] Database sharding
  - [ ] CDN for static assets
  - [ ] Load balancing
  - [ ] Monitoring and alerting

### 6.2 Inter-Community Coordination

- [ ] Inter-banjar tools
  - [ ] Cross-banjar discussions
  - [ ] Shared concerns aggregation
  - [ ] Coordinated positions
- [ ] Inter-subak tools
  - [ ] Water management coordination
  - [ ] Irrigation system discussions
  - [ ] Functional boundary respect
- [ ] Desa Adat level
  - [ ] Village-wide deliberations
  - [ ] Adat authority integration
  - [ ] Custom configurations

### 6.3 Researcher API

- [ ] API design
  - [ ] Community-permissioned access
  - [ ] Scoped queries
  - [ ] Rate limiting
  - [ ] Authentication
- [ ] Permission governance
  - [ ] Paruman Agung approval required
  - [ ] Time-limited grants
  - [ ] Purpose restrictions
  - [ ] Revocation mechanism
- [ ] Documentation
  - [ ] API reference
  - [ ] Usage examples
  - [ ] Ethical guidelines
  - [ ] Attribution requirements

### 6.4 Balinese NLP Model Publication

- [ ] Model packaging
  - [ ] Hugging Face Hub publication
  - [ ] Model card with attribution
  - [ ] Training data documentation
  - [ ] Evaluation metrics
- [ ] Licensing
  - [ ] Community-owned
  - [ ] Open license with attribution
  - [ ] Commercial use restrictions
  - [ ] Cultural source acknowledgment
- [ ] Community announcement
  - [ ] Cultural preservation narrative
  - [ ] Balinese language technology milestone
  - [ ] Open for research use

### 6.5 Ecosystem Expansion

- [ ] Other Indonesian regions
  - [ ] Adapt model for regions under cultural pressure
  - [ ] Respect local adat structures
  - [ ] Community ownership principles
  - [ ] Not extraction, partnership
- [ ] zkML on-chain inference
  - [ ] MandalaChain AI Agent Layer integration
  - [ ] On-chain community signal verification
  - [ ] Privacy-preserving analytics
  - [ ] Trustless evidence generation

---

## Cross-Cutting Concerns

### Security & Privacy

- [ ] Ongoing security audits
  - [ ] Smart contract audits
  - [ ] API security reviews
  - [ ] Penetration testing
- [ ] Privacy architecture maintenance
  - [ ] No real-name storage
  - [ ] No third-party tracking
  - [ ] Pseudonymous DID protection
  - [ ] Post deletion capability
- [ ] Threat monitoring
  - [ ] Sybil attack detection
  - [ ] Manipulation attempts
  - [ ] Data extraction attempts
- [ ] Incident response
  - [ ] Emergency multisig actions
  - [ ] 48-hour disclosure requirement
  - [ ] Community notification

### Documentation

- [ ] Keep ARCHITECTURE.md updated
  - [ ] Living document
  - [ ] Review after major milestones
  - [ ] Community feedback integration
- [ ] Keep governance.md updated
  - [ ] Review after each Musyawarah model change
- [ ] Keep Brand_Guidelines.md updated
  - [ ] Review after visual identity changes
  - [ ] Ensure consistency with implementation
- [ ] API documentation
  - [ ] OpenAPI/Swagger specs
  - [ ] Code examples
  - [ ] Integration guides
- [ ] Smart contract documentation
  - [ ] ink! contract docs
  - [ ] ABI documentation
  - [ ] Integration examples
- [ ] User guides
  - [ ] Bahasa Indonesia + English
  - [ ] Mobile-friendly
  - [ ] Video tutorials for low-literacy users
- [ ] Governance documentation
  - [ ] Keep governance.md updated
  - [ ] Musyawarah process guides
  - [ ] Kawenang explanation
  - [ ] Awig-Awig configuration guides

### Community Engagement

- [ ] Build in public
  - [ ] Regular GitHub updates
  - [ ] Design decision documentation
  - [ ] Community feedback loops
- [ ] Transparency
  - [ ] All decisions logged
  - [ ] Founding Council actions public
  - [ ] Moderation decisions visible
- [ ] Cultural appropriateness
  - [ ] Ongoing adat consultation
  - [ ] Language accuracy
  - [ ] Respect for tradition
- [ ] Feedback mechanisms
  - [ ] GitHub Issues
  - [ ] GitHub Discussions
  - [ ] In-person community meetings

### Testing & Quality

- [ ] Continuous testing
  - [ ] Unit tests (>80% coverage target)
  - [ ] Integration tests
  - [ ] End-to-end tests
- [ ] Performance testing
  - [ ] Load testing
  - [ ] Low-bandwidth simulation
  - [ ] Mobile device testing
- [ ] Accessibility testing
  - [ ] Screen reader compatibility
  - [ ] Keyboard navigation
  - [ ] Color contrast
  - [ ] Font size flexibility
- [ ] Cultural testing
  - [ ] Balinese language accuracy
  - [ ] Aksara Bali rendering
  - [ ] Calendar correctness
  - [ ] Adat appropriateness

---

## Critical Path Items

These items are blockers or high-priority dependencies:

1. **MandalaChain founder conversation** (Phase 1.2) — Mobile onboarding path is critical for user adoption
2. **Kawenang contract deployment** (Phase 1.3) — Core governance infrastructure
3. **Mandala ID integration** (Phase 1.2) — Identity layer is foundational
4. **3-5 seed banjar recruitment** (Phase 1.8) — Real community testing required
5. **Environmental data API integration** (Phase 2.4) — Evidence backbone
6. **First moderator election** (Phase 2.7) — Community governance maturity
7. **Founding Council sunset** (Phase 3.4) — Ownership transfer milestone
8. **Full Musyawarah implementation** (Phase 4.1) — Complete sovereignty

---

## Success Metrics

### Phase 1 Success
- [ ] 3-5 seed banjar communities onboarded
- [ ] 50+ verified Krama Mipil
- [ ] 100+ posts created
- [ ] Forum accessible on 2G connection
- [ ] Zero security incidents
- [ ] All bootstrap decisions logged publicly

### Phase 2 Success
- [ ] 200+ Krama Mipil (triggers first moderator election)
- [ ] NLP pipeline processing 100% of posts
- [ ] Environmental correlation live
- [ ] Community dashboard in use
- [ ] First community moderator elected
- [ ] Per-banjar Awig-Awig configured

### Phase 3 Success
- [ ] 500+ Krama Mipil (triggers Founding Council sunset)
- [ ] Financial data correlation live
- [ ] Data integration contracts on-chain
- [ ] Founding Council dissolved
- [ ] Community-elected multisig active

### Phase 4 Success
- [ ] Full Musyawarah governance on-chain
- [ ] First community position document issued
- [ ] Balinese NLP model deployed (if corpus ready)
- [ ] Data access governance active
- [ ] Zero founder special authority

### Phase 5 Success
- [ ] Community position documents in policy use
- [ ] lontar integration complete
- [ ] Multi-format export working
- [ ] Public archive established
- [ ] Evidence-backed advocacy demonstrated

### Phase 6 Success
- [ ] Island-wide availability
- [ ] Inter-community coordination active
- [ ] Researcher API in use (community-approved)
- [ ] Balinese NLP model published
- [ ] Ecosystem expansion begun

---

## Notes

- **Epistemic honesty:** Never fabricate precision. Kabupaten-level is the honest ceiling until better data exists.
- **Cultural primacy:** When technical convenience conflicts with adat appropriateness, adat wins.
- **Community ownership:** All major decisions require community approval via Musyawarah process.
- **Build in public:** All architecture, decisions, and progress visible from day one.
- **No token economy:** Paruman issues no economic token. KPG is the chain currency.
- **Governance is not a DAO:** This is Musyawarah Mufakat, not Western voting mechanics.

---

*"Paruman — Suara Bali"*

**Last Updated:** 2026-03-09  
**Living Document:** Update as development progresses
