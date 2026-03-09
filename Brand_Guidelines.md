# Paruman Brand Guidelines

**Suara Bali — Rooted in the sacred thread.**

---

## Conceptual Foundation: Tri Datu

Tri Datu (तृदातु) is the sacred three-colored thread worn by Balinese Hindus — woven from red, black, and white — representing the Trimurti in constant balance:

| Color | Deity | Force | Direction | Element |
|---|---|---|---|---|
| **Brahma Red** | Brahma | Creator | South | Fire |
| **Wisnu Black** | Wisnu | Preserver | North | Water |
| **Iswara White** | Iswara / Siwa | Purifier | East | Ether |

The three strands are *inseparable*. No single color holds meaning alone — meaning emerges from their weaving. This is the visual metaphor for Paruman's Three Forces Model: environmental, financial, and community voice, held in balance, none dominant.

Every Krama Mipil in Bali has worn this thread. The brand does not need to explain itself — it is already known.

### What This Means for Design

Paruman is not a premium product. It is not a startup. It is a community assembly — the oldest institution in Bali, made digital. The visual language must feel:

- **Grounded, not aspirational** — worn, not polished
- **Communal, not exclusive** — the platform belongs to everyone
- **Sacred without being ceremonial** — everyday sacred, not temple sacred
- **Accessible before beautiful** — a Kelian Banjar on a 2G connection comes first

The contrast with Dedauh is intentional and total. Dedauh is gold on deep: premium, API-developer-facing, commercially positioned. Paruman is Tri Datu: community-rooted, sovereignty-focused, Balinese in its bones.

---

## Color Palette

The Tri Datu colors are not used literally (bright primary red / pure black / bright white). They are interpreted through the lens of Bali's natural environment — the red of temple brick, the black of volcanic stone, the white of sea foam and offering cloth.

### Primary Colors

| Name | Hex | Tri Datu | Usage |
|---|---|---|---|
| **Bata** (temple brick red) | `#8C3A2B` | Brahma · Red | Primary accent · CTAs · active states · urgency |
| **Watu** (volcanic stone) | `#1A1A18` | Wisnu · Black | Primary background · depth · grounding |
| **Kapas** (offering cloth) | `#F0EBE1` | Iswara · White | Primary text · foreground · purity |

### Secondary Colors

| Name | Hex | Derivation | Usage |
|---|---|---|---|
| **Bata Redup** (faded brick) | `#6B2D20` | Brahma · dark | Hover states · pressed buttons |
| **Bata Lembut** (soft brick) | `#A84535` | Brahma · light | Secondary accents |
| **Watu Pertengahan** (mid-stone) | `#2D2D2A` | Wisnu · lifted | Card surfaces · elevated backgrounds |
| **Watu Terang** (light stone) | `#3F3F3B` | Wisnu · bright | Borders · dividers · subtle surfaces |
| **Kapas Redup** (warm grey) | `#B8B0A4` | Iswara · muted | Secondary text · disabled states · metadata |

### Semantic Colors

| Name | Hex | Usage |
|---|---|---|
| **Mufakat** (consensus green) | `#4A7C59` | Mufakat outcome · success · agreement |
| **Ditunda** (deferral amber) | `#A07830` | Ditunda outcome · pending · in deliberation |
| **Pemungutan** (vote slate) | `#4A5568` | Pemungutan outcome · formal vote · lower legitimacy |
| **Darurat** (emergency) | `#8C3A2B` | Emergency actions · destructive · critical alerts |

### The Tri Datu Thread — UI Signature

The single most recognizable visual element: a thin horizontal band of three colors, woven together. Used as:
- Top border of the application shell
- Separator in community position documents
- Loading indicator (animated threaded sweep)
- Divider between the three forces in data visualizations

```css
/* Tri Datu thread */
background: linear-gradient(90deg,
  #8C3A2B 0%,        /* Brahma */
  #8C3A2B 33.33%,
  #1A1A18 33.33%,    /* Wisnu */
  #1A1A18 66.66%,
  #F0EBE1 66.66%,    /* Iswara */
  #F0EBE1 100%
);
height: 3px;
```

### CSS Variables

```css
:root {
  /* Tri Datu primaries */
  --brahma:        #8C3A2B;   /* Bata — temple brick red */
  --wisnu:         #1A1A18;   /* Watu — volcanic stone */
  --iswara:        #F0EBE1;   /* Kapas — offering cloth */

  /* Brahma variants */
  --brahma-dark:   #6B2D20;
  --brahma-light:  #A84535;

  /* Wisnu variants */
  --wisnu-mid:     #2D2D2A;
  --wisnu-light:   #3F3F3B;

  /* Iswara variants */
  --iswara-muted:  #B8B0A4;

  /* Semantic */
  --mufakat:       #4A7C59;
  --ditunda:       #A07830;
  --pemungutan:    #4A5568;

  /* Functional */
  --surface:       #2D2D2A;
  --border:        rgba(240, 235, 225, 0.1);
  --border-accent: rgba(140, 58, 43, 0.35);
}
```

---

## Typography

### Design Philosophy

Paruman's typography holds two things simultaneously:
- **The weight of tradition** — the font families should feel rooted, not contemporary
- **The clarity of a public record** — legible at 2G on entry-level Android, at any size

No decorative display fonts. No thin weights that fail on low-resolution screens. No language that centers the technology.

### Font Families

#### Display Font: Fraunces
- **Weights**: 400, 600, 700
- **Usage**: Section titles, community position headers, major headings
- **Character**: Optical-size variable serif — designed for display use, has warmth and age without being archaic. Carries the weight of a lontar manuscript without being literal about it.
- **Rationale**: Feels like it could have been carved into stone or written with a pen. Not cold, not corporate.

#### Body Font: Source Serif 4
- **Weights**: 300, 400
- **Usage**: Post content, descriptions, long-form community text
- **Character**: A reading font. Optimized for text at body sizes. Works across scripts — important for Bahasa Indonesia and Balinese language content.
- **Rationale**: Community voice is the product. The text that carries that voice deserves a reading font, not a UI font.

#### UI Font: Noto Sans (Indonesian + Balinese subsets)
- **Weights**: 400, 500
- **Usage**: UI chrome, navigation, labels, metadata, form elements
- **Character**: Neutral, functional, specifically designed to support Indonesia's many scripts. Includes Aksara Bali support via the Noto Sans Balinese subset.
- **Rationale**: The only font in this stack explicitly designed for Indonesian language contexts. Never use a Latin-only font for UI elements that will carry Bahasa Indonesia or Balinese text.

#### Monospace Font: Noto Sans Mono
- **Weight**: 400
- **Usage**: On-chain hashes, DIDs, technical references, timestamps, code
- **Character**: Consistent with the Noto family; not a design statement, just correct
- **Rationale**: Wallet addresses, DID hashes, and Kawenang contract references should look like what they are.

### Typography Scale

| Element | Font | Size | Weight | Notes |
|---|---|---|---|---|
| Platform name | Fraunces | 1.5rem | 600 | "Paruman" wordmark |
| Page title | Fraunces | 2.2rem | 600 | Major section headings |
| Section title | Fraunces | 1.6rem | 400 | Sub-section headings |
| Community position title | Fraunces | 1.8rem | 600 | Formal document heading |
| Body text | Source Serif 4 | 1rem | 400 | Post content, descriptions |
| Body emphasis | Source Serif 4 | 1rem | 400 italic | Emphasis within body |
| UI label | Noto Sans | 0.8rem | 500 | Navigation, tabs, buttons |
| Metadata | Noto Sans | 0.75rem | 400 | Timestamps, location tags, tiers |
| Balinese calendar | Noto Sans | 0.75rem | 400 | Wuku, Sasih, Rahinan |
| Aksara Bali | Noto Sans Balinese | variable | 400 | Script rendering via lontar-aksara |
| Hash / DID | Noto Sans Mono | 0.72rem | 400 | Truncated on-chain identifiers |

### Font Import

```html
<link rel="preconnect" href="https://fonts.googleapis.com">
<link href="https://fonts.googleapis.com/css2?
  family=Fraunces:opsz,wght@9..144,400;9..144,600;9..144,700
  &family=Source+Serif+4:ital,wght@0,300;0,400;1,400
  &family=Noto+Sans:wght@400;500
  &family=Noto+Sans+Mono:wght@400
  &display=swap" rel="stylesheet">
```

*Note: Noto Sans Balinese is loaded separately on-demand when Aksara Bali rendering is needed — not in the critical path.*

---

## Component Styles

### Application Shell

```css
/* Top border — Tri Datu thread, always visible */
.app-shell::before {
  content: '';
  display: block;
  width: 100%;
  height: 3px;
  background: linear-gradient(90deg,
    var(--brahma)  0%,   var(--brahma)  33.33%,
    var(--wisnu)   33.33%, var(--wisnu)  66.66%,
    var(--iswara)  66.66%, var(--iswara) 100%
  );
  position: fixed;
  top: 0;
  z-index: 1000;
}

body {
  background: var(--wisnu);
  color: var(--iswara);
  font-family: 'Noto Sans', sans-serif;
}
```

### Buttons

#### Primary (Brahma — action)
```css
.btn-primary {
  background: var(--brahma);
  color: var(--iswara);
  border: 1px solid var(--brahma);
  padding: 0.65rem 1.5rem;
  font-family: 'Noto Sans', sans-serif;
  font-size: 0.85rem;
  font-weight: 500;
  border-radius: 2px;
  transition: background 0.2s ease;
}

.btn-primary:hover {
  background: var(--brahma-light);
  border-color: var(--brahma-light);
}
```

#### Secondary (outlined — deliberation)
```css
.btn-secondary {
  background: transparent;
  color: var(--iswara);
  border: 1px solid var(--border);
  padding: 0.65rem 1.5rem;
  font-family: 'Noto Sans', sans-serif;
  font-size: 0.85rem;
  font-weight: 500;
  border-radius: 2px;
  transition: border-color 0.2s ease, background 0.2s ease;
}

.btn-secondary:hover {
  border-color: rgba(240, 235, 225, 0.3);
  background: rgba(240, 235, 225, 0.04);
}
```

#### Destructive (emergency / delete)
```css
.btn-destructive {
  background: transparent;
  color: var(--brahma-light);
  border: 1px solid var(--border-accent);
  padding: 0.65rem 1.5rem;
  font-size: 0.85rem;
  font-weight: 500;
  border-radius: 2px;
}
```

### Cards

#### Post Card
```css
.post-card {
  background: var(--surface);
  border: 1px solid var(--border);
  border-radius: 2px;
  padding: 1.25rem 1.5rem;
  transition: border-color 0.2s ease;
}

.post-card:hover {
  border-color: rgba(240, 235, 225, 0.2);
}

/* Post with active community signal */
.post-card--signal {
  border-left: 2px solid var(--brahma);
}
```

#### Community Position Document Card
```css
.position-card {
  background: var(--surface);
  border: 1px solid rgba(140, 58, 43, 0.25);
  border-radius: 2px;
  padding: 2rem;
}

/* Tri Datu thread at top of position cards */
.position-card::before {
  content: '';
  display: block;
  width: 100%;
  height: 2px;
  background: linear-gradient(90deg,
    var(--brahma)  0%,   var(--brahma)  33.33%,
    var(--wisnu-light) 33.33%, var(--wisnu-light) 66.66%,
    var(--iswara)  66.66%, var(--iswara) 100%
  );
  margin-bottom: 1.5rem;
}
```

#### Governance Matter Card
```css
.matter-card {
  background: var(--surface);
  border: 1px solid var(--border);
  border-radius: 2px;
  padding: 1.5rem;
}

/* Track indicator */
.matter-card--agung {
  border-left: 3px solid var(--brahma);
}

.matter-card--rutin {
  border-left: 3px solid var(--wisnu-light);
}
```

### Outcome Badges

```css
/* Mufakat — consensus, highest legitimacy */
.badge--mufakat {
  background: rgba(74, 124, 89, 0.15);
  color: #6BAF80;
  border: 1px solid rgba(74, 124, 89, 0.35);
  font-size: 0.72rem;
  font-weight: 500;
  padding: 0.2rem 0.6rem;
  border-radius: 2px;
  font-family: 'Noto Sans', sans-serif;
}

/* Ditunda — deferred, valid */
.badge--ditunda {
  background: rgba(160, 120, 48, 0.15);
  color: #C49A3C;
  border: 1px solid rgba(160, 120, 48, 0.35);
  font-size: 0.72rem;
  font-weight: 500;
  padding: 0.2rem 0.6rem;
  border-radius: 2px;
}

/* Pemungutan — formal vote, lower legitimacy */
.badge--pemungutan {
  background: rgba(74, 85, 104, 0.15);
  color: #8896AA;
  border: 1px solid rgba(74, 85, 104, 0.35);
  font-size: 0.72rem;
  font-weight: 500;
  padding: 0.2rem 0.6rem;
  border-radius: 2px;
}
```

### Balinese Calendar Context Block

```css
.calendar-context {
  display: flex;
  gap: 0.75rem;
  flex-wrap: wrap;
  padding: 0.5rem 0;
  border-top: 1px solid var(--border);
  margin-top: 0.75rem;
}

.calendar-tag {
  font-family: 'Noto Sans', sans-serif;
  font-size: 0.72rem;
  color: var(--iswara-muted);
  background: rgba(240, 235, 225, 0.05);
  padding: 0.15rem 0.5rem;
  border-radius: 2px;
}

/* Pancaroba flag — elevated visibility */
.calendar-tag--pancaroba {
  color: var(--brahma-light);
  background: rgba(140, 58, 43, 0.1);
  border: 1px solid rgba(140, 58, 43, 0.2);
}
```

### Identity Tier Display

```css
/* Never show Kawenang as a number. Show standing. */
.identity-tier {
  display: inline-flex;
  align-items: center;
  gap: 0.4rem;
  font-size: 0.75rem;
  font-family: 'Noto Sans', sans-serif;
  color: var(--iswara-muted);
}

/* Tri Datu dot indicator for active Krama Mipil */
.identity-tier--krama::before {
  content: '';
  display: inline-block;
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background: conic-gradient(
    var(--brahma) 0deg 120deg,
    var(--wisnu-light) 120deg 240deg,
    var(--iswara) 240deg 360deg
  );
}

.identity-tier--diaspora {
  opacity: 0.65;
}
```

### The Tri Datu Divider

```css
.tridatu-divider {
  width: 100%;
  height: 1px;
  background: linear-gradient(90deg,
    transparent,
    var(--brahma) 20%,
    var(--wisnu-light) 50%,
    var(--iswara-muted) 80%,
    transparent
  );
  margin: 2rem 0;
}
```

---

## Animations & Motion

### Philosophy

This is a 2G-tolerant, entry-level Android platform. Animation must earn its existence. No animation for decoration. No animation that delays content. Animation is used only when it communicates:

- **State change** (post submitted, endorsement registered)
- **Process** (deliberation in progress, calendar loading)
- **Significance** (mufakat reached — this deserves a moment)

All animations respect `prefers-reduced-motion`. Transitions are 0.15s–0.25s maximum unless the animation IS the communication.

### Allowed Animations

#### Fade in (content load)
```css
@keyframes fadeIn {
  from { opacity: 0; transform: translateY(8px); }
  to   { opacity: 1; transform: translateY(0); }
}

/* Short, purposeful */
.post-card { animation: fadeIn 0.2s ease-out; }
```

#### Mufakat moment
```css
/* When a matter reaches mufakat — one of very few celebratory moments */
@keyframes mufakatPulse {
  0%   { box-shadow: 0 0 0 0 rgba(74, 124, 89, 0.4); }
  70%  { box-shadow: 0 0 0 12px rgba(74, 124, 89, 0); }
  100% { box-shadow: 0 0 0 0 rgba(74, 124, 89, 0); }
}

.matter-card--mufakat {
  animation: mufakatPulse 1s ease-out 1;
}
```

#### Tri Datu loading thread
```css
/* Used for loading states — the thread being woven */
@keyframes threadWeave {
  0%   { background-position: -200% center; }
  100% { background-position: 200% center; }
}

.loading-thread {
  height: 2px;
  background: linear-gradient(90deg,
    var(--brahma), var(--wisnu-light), var(--iswara),
    var(--brahma), var(--wisnu-light), var(--iswara)
  );
  background-size: 200% auto;
  animation: threadWeave 1.5s linear infinite;
}
```

#### Deliberation progress
```css
/* Visual rhythm for active deliberation period */
@keyframes deliberate {
  0%, 100% { opacity: 0.5; }
  50%       { opacity: 1; }
}

.deliberation-active {
  animation: deliberate 3s ease-in-out infinite;
}
```

### Explicitly Forbidden

- Floating ornaments (decorative, bandwidth-heavy)
- Shimmer text effects (Dedauh's territory — wrong brand)
- Page transitions that delay navigation
- Any animation autoplay longer than 3 seconds
- Parallax scrolling

---

## Layout & Spacing

### Philosophy

Mobile-first. Content-first. Nothing between the community and their voice.

No sidebars on mobile. No complex grid layouts that break on small screens. The layout should feel like a well-organized community noticeboard — clear sections, clear hierarchy, nothing hidden.

### Spacing Scale

```css
:root {
  --space-xs:   0.25rem;   /*  4px */
  --space-sm:   0.5rem;    /*  8px */
  --space-md:   1rem;      /* 16px */
  --space-lg:   1.5rem;    /* 24px */
  --space-xl:   2rem;      /* 32px */
  --space-2xl:  3rem;      /* 48px */
  --space-3xl:  4rem;      /* 64px */
}
```

### Container Widths

```css
:root {
  --container-sm:  480px;   /* Single-column content */
  --container-md:  720px;   /* Reading column */
  --container-lg:  960px;   /* Standard layout */
  --container-xl:  1100px;  /* Dashboard / data views */
}
```

### Breakpoints

```css
/* Mobile-first — these are minimums, not maximums */
--bp-sm:   480px;
--bp-md:   768px;
--bp-lg:   1024px;
--bp-xl:   1280px;
```

### Border Radius

```css
--radius: 2px;  /* Consistent, sharp. Not rounded-xl. Not zero. */
```

Sharp corners signal seriousness — this is not a playful consumer app. The 2px radius keeps edges from feeling harsh while maintaining a grounded, document-like quality.

---

## Visual Elements

### Sacred Geometry — Tri Datu Only

No floating mandala ornaments. No lotus motifs. No tourist-Bali visual clichés.

The one permitted decorative element is the **Tri Datu thread** — used sparingly:
- Application shell top border (always present, 3px)
- Community position document separator (2px)
- Loading state (animated)
- Deliberation status indicator

When used as a decorative background element (rare), render it as parallel horizontal threads at very low opacity:

```css
.tridatu-bg {
  background-image: repeating-linear-gradient(
    180deg,
    transparent,
    transparent 12px,
    rgba(140, 58, 43, 0.03) 12px,
    rgba(140, 58, 43, 0.03) 13px,
    rgba(26, 26, 24, 0) 13px,
    rgba(26, 26, 24, 0) 25px,
    rgba(240, 235, 225, 0.02) 25px,
    rgba(240, 235, 225, 0.02) 26px,
    transparent 26px,
    transparent 38px
  );
}
```

### Borders & Dividers

```css
/* Standard border — barely visible, present */
border: 1px solid rgba(240, 235, 225, 0.1);

/* Accent border — attention, not alarm */
border: 1px solid rgba(140, 58, 43, 0.25);

/* Active / selected */
border: 1px solid rgba(240, 235, 225, 0.35);

/* Brahma accent line — left border for signal/urgency */
border-left: 2px solid var(--brahma);
```

---

## Accessibility

### Color Contrast

| Pair | Ratio | Level |
|---|---|---|
| Kapas (#F0EBE1) on Watu (#1A1A18) | 13.8:1 | ✓ WCAG AAA |
| Kapas on Watu Pertengahan (#2D2D2A) | 11.2:1 | ✓ WCAG AAA |
| Bata (#8C3A2B) on Watu (#1A1A18) | 4.6:1 | ✓ WCAG AA |
| Kapas Redup (#B8B0A4) on Watu (#1A1A18) | 7.2:1 | ✓ WCAG AAA |
| Mufakat (#6BAF80) on Watu (#1A1A18) | 5.1:1 | ✓ WCAG AA |

### Typography Accessibility

- Minimum body text: 1rem (16px) — never smaller for reading content
- Line height: 1.65–1.8 for body text
- Maximum line length: 70ch for reading columns
- Balinese calendar metadata may go to 0.72rem — always accompanied by full text on tap/hover

### Low-Bandwidth Considerations

- Google Fonts loaded with `display=swap` — text visible immediately with fallback
- No web fonts in the critical render path
- Images lazy-loaded below the fold
- No background videos or GIF animations
- CSS animations disabled when `prefers-reduced-motion: reduce`

### Fallback Font Stack

```css
font-family: 'Fraunces', 'Georgia', 'Times New Roman', serif;
font-family: 'Source Serif 4', 'Georgia', serif;
font-family: 'Noto Sans', 'Helvetica Neue', 'Arial', sans-serif;
font-family: 'Noto Sans Mono', 'Courier New', monospace;
```

---

## Language & Script

### Text Rendering Priority

1. **Bahasa Indonesia** — primary UI language, always present
2. **Balinese language** — supported, never suppressed
3. **Aksara Bali** — rendered via `lontar-aksara` (rustybuzz/HarfBuzz), falls back to Latin transliteration if rendering fails

### Bilingual UI Patterns

```css
/* When Indonesian and Balinese language appear together */
.bilingual-label {
  display: flex;
  flex-direction: column;
  gap: 0.1rem;
}

.bilingual-label__id   { font-size: 0.85rem; font-weight: 500; }
.bilingual-label__ban  { font-size: 0.75rem; color: var(--iswara-muted); }
```

### Aksara Bali Rendering

```css
/* Applied to elements requiring proper script shaping */
.aksara-bali {
  font-family: 'Noto Sans Balinese', 'Noto Serif Balinese', serif;
  font-size: 1.1em;  /* Balinese script needs slightly larger size */
  line-height: 2;    /* Descenders require more line height */
}
```

---

## Brand Voice

### Tone

**Deliberate.** Every word in the UI is chosen with care. No filler text. No startup enthusiasm. No calls to action that sound like marketing.

**Community-first.** The platform speaks as the community, not to the community. "Suara Bali" — not "Your Voice, Amplified."

**Honest about weight.** This platform exists because Bali is under pressure. The tone doesn't pretend otherwise, but it doesn't catastrophize either. Matter-of-fact.

**Formally informal.** The register should feel like how a thoughtful Kelian Banjar speaks at a sangkep — respected, clear, not bureaucratic, not casual.

### UI Copy Principles

| Instead of | Use |
|---|---|
| "Sign up" | "Daftar sebagai Krama Mipil" |
| "Your vote matters" | "Musyawarah dibuka" |
| "Earn rewards" | — (never) |
| "Like" | "Dukung" (endorse) |
| "Trending" | — (never) |
| "Notifications" | "Pemberitahuan" |
| "Post" | "Sampaikan" (convey / bring forward) |
| "DAO" | — (never in user-facing UI) |
| "Token" | — (never in user-facing UI) |
| "Wallet" | — (minimize; "akun" when necessary) |

---

## Paruman vs. Dedauh — Side by Side

| Dimension | Dedauh | Paruman |
|---|---|---|
| **Palette** | Gold on deep black | Tri Datu: brick red, volcanic black, cloth white |
| **Typography** | Cormorant Garamond (elegant serif display) | Fraunces + Source Serif 4 (grounded, readable) |
| **Audience** | Developers, technical API consumers | Krama Mipil, community leaders, rural Bali |
| **Tone** | Premium, sophisticated, aspirational | Grounded, communal, serious |
| **Animation** | Floating ornaments, shimmer, elaborate entrance | Minimal — only when it communicates |
| **Border radius** | 2px (shared) | 2px |
| **Primary CTA** | Gold button | Brahma red button |
| **Decorative elements** | Mandala ornaments, SVG patterns | Tri Datu thread only |
| **Bandwidth target** | Desktop broadband | 2G mobile, entry-level Android |
| **Orientation** | Commercial product | Community infrastructure |

---

## Implementation Notes

### Dark Mode

Paruman is dark-first. The primary experience is Watu (#1A1A18) background with Kapas (#F0EBE1) text. There is no light mode — the visual language of volcanic stone and temple brick does not invert gracefully, and the community using this platform at dusk on a mobile device benefits from the dark default.

### Icon System

Use a single-weight icon set with 1.5px strokes. Recommended: Phosphor Icons (MIT license, available as web font or SVG). Line icons only — no filled icons, no gradient icons.

Icons accompany text — never replace it. A Krama Mipil who has never used a smartphone should still understand the UI from text alone.

### Print / PDF Considerations

Community position documents are exported as PDF. Print styles apply:

```css
@media print {
  body { background: white; color: #1A1A18; }
  .tridatu-thread { background: #000; }  /* Full black for print */
  .app-shell::before { position: relative; }
  .btn-primary, .btn-secondary { display: none; }
}
```

---

*Tri Datu — tiga warna, satu benang. Three colors, one thread.*

**Last Updated:** 2026-03-09  
**Version:** 1.0  
**Maintained by:** Paruman Core Team