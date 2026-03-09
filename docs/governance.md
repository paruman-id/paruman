# Tata Kelola Paruman — Musyawarah Mufakat

> *"Keputusan bukan hasil hitungan kepala, melainkan buah dari musyawarah yang bulat."*
> *"Decisions are not the result of counting heads, but the fruit of unanimous deliberation."*

**Dokumen ini** menjelaskan bagaimana komunitas Paruman membuat keputusan. Model ini tidak dirancang dari nol — model ini adalah implementasi digital dari proses Musyawarah Mufakat yang sudah lama mengatur kehidupan komunitas Bali.

*This document explains how the Paruman community makes decisions. This model was not designed from scratch — it is a digital implementation of the Musyawarah Mufakat process that has governed Balinese community life for generations.*

---

## Prinsip Dasar / Core Principle

Dalam setiap Paruman tradisional, tujuan utamanya adalah **mufakat** — kesepakatan bulat yang menjaga keharmonisan komunitas dan menghindari *wicara* (sengketa). Pemungutan suara formal adalah jalan terakhir, bukan titik awal.

*In every traditional Paruman, the primary goal is **mufakat** — unanimous consensus that preserves community harmony and avoids wicara (dispute). Formal voting is a last resort, not the starting point.*

```
JALUR UTAMA      Musyawarah → Mufakat
                 Deliberasi dibuka · komunitas berdiskusi ·
                 konsensus terbentuk · fasilitator menyatakan mufakat ·
                 hasil dicatat di blockchain dengan legitimasi tertinggi

HASIL YANG SAH   Ditunda
                 Tidak ada konsensus · masalah ditunda untuk
                 diskusi informal lebih lanjut · tidak ada stigma ·
                 dibuka kembali di siklus berikutnya

JALAN TERAKHIR   Pemungutan Suara
                 Hanya jika: tidak ada konsensus DAN masalah
                 tidak bisa ditunda tanpa menimbulkan kerugian
                 Tertimbang Kawenang · supermajoritas diperlukan
                 Dicatat sebagai legitimasi lebih rendah dalam rekam jejak
```

---

## Unit Partisipasi: Krama Mipil

Dalam adat Bali, hak untuk berpartisipasi dalam Paruman adalah hak **Krama Mipil** — kepala keluarga yang terdaftar. Satu keluarga, satu suara.

*In Balinese adat, the right to participate in a Paruman belongs to the **Krama Mipil** — the registered head of household. One household, one voice.*

Paruman digital mengikuti prinsip ini. **Kawenang** (bobot tata kelola) dipegang oleh perwakilan rumah tangga yang terverifikasi, bukan oleh individu yang tidak terhubung dengan struktur adat.

*The digital Paruman follows this principle. **Kawenang** (governance weight) is held by a verified household representative, not by an individual disconnected from adat structure.*

### Tingkatan Identitas / Identity Tiers

| Tingkatan | Deskripsi | Kawenang |
|---|---|---|
| **Tamu / Guest** | Akses baca saja | — |
| **Tier 1** | Orang Bali terverifikasi (pseudonim) | Bisa posting, belum punya Kawenang |
| **Tier 1D** | Diaspora Bali terverifikasi | Bisa posting · 0.5× Kawenang dasar · tanpa senioritas |
| **Tier 2 — Krama Mipil** | Perwakilan keluarga terverifikasi + afiliasi banjar | Kawenang penuh · aktif setelah 35 hari |
| **Tier 3 — Wakil Adat** | Krama Mipil + peran adat yang diakui | Kawenang + peran fasilitator Musyawarah |

Verifikasi Krama Mipil menggunakan **Kartu Keluarga (KK)** — dokumen registrasi keluarga yang sudah ada — melalui infrastruktur identitas Mandala ID / IDCHAIN. Tidak ada database nama asli yang disimpan oleh Paruman.

*Krama Mipil verification uses the **Kartu Keluarga (KK)** — the existing family registry document — via the Mandala ID / IDCHAIN identity infrastructure. No real-name database is stored by Paruman.*

---

## Kawenang — Hak Bersuara

**Kawenang** (kewenangan, hak, legitimasi dalam bahasa Bali) adalah bobot tata kelola yang diperoleh melalui kehadiran dan peran komunitas yang terverifikasi. Kawenang bukan token, bukan mata uang, tidak bisa dipindahtangankan, dan tidak memiliki nilai ekonomi.

*Kawenang is governance weight earned through verified community presence and role. It is not a token, not a currency, not transferable, and has no economic value.*

Kawenang tidak pernah ditampilkan sebagai angka di antarmuka pengguna. Yang terlihat adalah kedudukan, peran, dan senioritas komunitas.

*Kawenang is never displayed as a number in the user interface. What is visible is community standing, role, and seniority.*

### Bobot Kawenang / Weight Model

| Dasar | Bobot |
|---|---|
| Masa tunggu (35 hari pertama) | Kawenang belum aktif |
| Krama Mipil baru (aktif) | 1.0 |
| + 6 bulan keanggotaan terverifikasi | 1.2 |
| + 1 tahun | 1.5 |
| + 2 tahun | 2.0 |
| + 5 tahun (batas maksimum) | 2.5 |
| Peran: Kelian Banjar attestor | + 0.5 |
| Peran: Moderator terpilih (masa jabatan aktif) | + 0.3 |
| Peran: Subak Pekaseh | + 0.3 |
| Peran: Wakil Adat / Tier 3 | + 0.5 |
| Diaspora (Tier 1D) | 0.5 × dasar · tanpa akumulasi senioritas |

Bobot tidak bisa dibeli, dipindahtangankan, atau didelegasikan. Bobot hanya tumbuh melalui kehadiran komunitas yang terverifikasi seiring waktu — mengikuti prinsip *Ulu Apad* (senioritas dihormati).

*Weight cannot be purchased, transferred, or delegated. It grows only through verified community presence over time — following the Ulu Apad principle (seniority is respected).*

### Masa Tunggu 35 Hari / 35-Day Hold

Kawenang baru aktif setelah **35 hari** sejak verifikasi Krama Mipil — satu siklus Pawukon penuh. Satu siklus kehadiran komunitas sebelum bobot tata kelola aktif.

*Kawenang only activates after **35 days** from Krama Mipil verification — one full Pawukon cycle. One cycle of community presence before governance weight activates.*

---

## Dua Jalur Sidang / Two Meeting Tracks

Tata kelola tradisional Bali memiliki hierarki majelis. Paruman mencerminkan ini dengan dua jalur deliberasi yang berbeda dalam ritme, cakupan, dan ambang legitimasi.

*Traditional Balinese governance has a hierarchy of assemblies. Paruman mirrors this with two distinct deliberation tracks differing in rhythm, scope, and legitimacy threshold.*

### Pasangkepan Rutin (Sidang Operasional)

Sidang komunitas rutin. Dalam praktik tradisional, diadakan setiap **35 hari** pada hari Wuku tertentu dalam kalender Bali. Menangani urusan operasional harian platform dan komunitas.

*The regular community meeting. In traditional practice, held every **35 days** on a specific Wuku day in the Balinese calendar. Handles the ongoing operational life of the platform and community.*

```
Ritme:        Setiap 35 hari · pada hari Wuku yang dikonfigurasi per banjar
Cakupan:      Penambahan fitur · pemilihan moderator
              Pengeluaran kas · keputusan operasional rutin
              Pembaruan konfigurasi per-banjar
Deliberasi:   Agenda dibuka di awal siklus · diskusi sepanjang 35 hari
              Pemeriksaan konsensus di akhir siklus
Fasilitator:  Moderator terpilih
Pemungutan:   Supermajoritas 66% Kawenang (jalan terakhir)
On-chain:     Hasil dicatat di akhir siklus
```

### Paruman Agung (Sidang Strategis)

Majelis tinggi. Dikhususkan untuk urusan strategis dan konstitusional — *Awig-Awig* platform itu sendiri.

*The high assembly. Reserved for strategic and constitutional matters — the platform's own Awig-Awig.*

```
Dipicu oleh:  Perubahan aturan platform atau prinsip dasar
              Perubahan perjanjian akses data
              Pembubaran Dewan Pendiri
              Hal-hal dengan implikasi seluruh pulau
Deliberasi:   Minimum 3 siklus Pasangkepan (105 hari)
              Tidak bisa diperpendek — bahkan dengan konsensus
Fasilitator:  Wakil Adat (Tier 3)
Pemungutan:   Supermajoritas 80% Kawenang (jalan terakhir)
On-chain:     Referensi transkrip deliberasi + hasil dicatat
```

---

## Konfigurasi Awig-Awig per Banjar

Adat Bali berbeda-beda di setiap desa. Setiap banjar yang terdaftar mengonfigurasi parameter tata kelola mereka sendiri dalam kerangka platform bersama — dan konfigurasi ini disimpan di blockchain:

*Balinese customary law varies by village. Each registered banjar configures their own governance parameters within the shared platform framework — anchored on-chain:*

- Hari Wuku untuk Pasangkepan Rutin
- Periode deliberasi minimum (minimum platform adalah lantai)
- Ekspektasi kuorum lokal
- Definisi peran khusus jika diperlukan
- Apakah urusan subak dirutekan melalui banjar atau langsung

---

## Rekam Hasil / Outcome Legitimacy Record

Rekam on-chain membedakan **bagaimana** keputusan dicapai, bukan hanya apa yang diputuskan. Perbedaan ini penting di ruang kebijakan.

*The on-chain record distinguishes **how** a decision was reached, not just what was decided. This distinction matters in a policy room.*

| Hasil | Legitimasi | Catatan |
|---|---|---|
| **Mufakat** | Tertinggi | Konsensus dicapai melalui musyawarah |
| **Ditunda** | Valid | Sah secara adat · tidak ada stigma · dibuka kembali |
| **Pemungutan Suara** | Standar (lebih rendah) | Jalan terakhir · ditandai demikian dalam semua dokumen |

Dokumen posisi komunitas yang didukung mufakat membawa bobot lebih besar daripada yang didukung pemungutan suara 66%. Rekam ini dipertahankan secara permanen.

*A community position document backed by mufakat carries more weight than one backed by a 66% vote. This record is preserved permanently.*

---

## Tabel Referensi Tata Kelola / Governance Reference

| Hal / Matter | Jalur / Track | Min. deliberasi | Ambang pemungutan |
|---|---|---|---|
| Penambahan fitur | Pasangkepan Rutin | 35 hari | 66% |
| Pemilihan moderator | Pasangkepan Rutin | 35 hari | 66% |
| Pengeluaran kas | Pasangkepan Rutin | 35 hari | 66% |
| Pembaruan konfigurasi banjar | Pasangkepan Rutin | 35 hari | 66% |
| Perubahan aturan platform | Paruman Agung | 105 hari | 80% |
| Izin akses data | Paruman Agung | 105 hari | 80% |
| Perubahan kontrak warehouse | Paruman Agung | 105 hari | 80% |
| Perubahan prinsip dasar | Paruman Agung | 105 hari | 80% |
| Tindakan keamanan darurat | Multisig 3-dari-5 | Segera · diungkap dalam 48 jam | — |
| Pembaruan kalender Nampih Sasih | Perawatan Tier 0 | N/A | Komunitas diberitahu, tidak dikonsultasikan |

---

## Bootstrap: Dewan Pendiri / Founding Council

Sebelum tata kelola komunitas matang, **Dewan Pendiri** (5–9 orang, mewakili beberapa kabupaten) membuat keputusan platform awal. Dewan ini bersifat sementara secara eksplisit.

*Before community governance matures, a **Founding Council** (5–9 people, spanning multiple kabupaten) makes initial platform decisions. This council is explicitly temporary.*

**Dewan Pendiri bubar otomatis saat Paruman mencapai 500 Krama Mipil aktif.**

*The Founding Council dissolves automatically when Paruman reaches 500 active Krama Mipil.*

Kewenangan Dewan Pendiri terbatas pada:
- Keputusan pemberian Kawenang awal
- Penunjukan moderator bootstrap
- Keputusan teknis deployment

Tidak ada hak veto atas keputusan komunitas. Tidak ada otoritas khusus permanen.

*No veto over community decisions. No permanent special authority.*

### Moderasi Bootstrap

- **Fase 1a:** Moderator yang ditunjuk pendiri, bernama publik, semua keputusan tercatat
- **Fase 1b:** Pemilihan moderator komunitas pertama dipicu saat 200 Krama Mipil
- Setelah pemilihan, otoritas moderator yang ditunjuk pendiri **kadaluarsa secara otomatis** — tidak diserahkan, kadaluarsa

---

## Implementasi Teknis / Technical Implementation

Mekanisme tata kelola diimplementasikan sebagai kontrak **ink!** (WASM) di [MandalaChain](https://mandalachain.io) — blockchain sovereign Indonesia (Polkadot parachain, Para ID 4818).

*Governance mechanics are implemented as **ink!** (WASM) contracts on [MandalaChain](https://mandalachain.io) — Indonesia's sovereign blockchain (Polkadot parachain, Para ID 4818).*

- **KPG** adalah mata uang gas chain — Paruman tidak menerbitkan token ekonomi sendiri
- Biaya gas disponsori oleh kas komunitas Paruman — pengguna tidak pernah melihat atau memikirkan biaya
- Pemulihan akun menggunakan `pallet-recovery` dengan anggota banjar sebagai guardian — tidak perlu seed phrase
- Semua hasil Musyawarah dicatat permanen dan dapat diaudit oleh seluruh Krama Mipil

*KPG is the chain's gas currency — Paruman issues no economic token of its own. Gas fees are sponsored by the Paruman community treasury — users never see or think about fees. Account recovery uses `pallet-recovery` with banjar members as guardians — no seed phrase required. All Musyawarah outcomes are permanently recorded and auditable by all Krama Mipil.*

---

*Dokumen ini adalah bagian dari [`ARCHITECTURE.md`](../ARCHITECTURE.md) yang dipisahkan untuk keterbacaan.*
*This document is extracted from [`ARCHITECTURE.md`](../ARCHITECTURE.md) for readability.*
