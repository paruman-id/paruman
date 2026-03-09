# Berkontribusi pada Paruman / Contributing to Paruman

Paruman dibangun secara terbuka sejak hari pertama. Kontribusi disambut pada semua tingkatan — dari diskusi desain hingga kode, dari koreksi bahasa Bali hingga pengujian koneksi rendah.

*Paruman is built in public from day one. Contributions are welcome at all levels — from design discussion to code, from Balinese language correction to low-bandwidth testing.*

---

## Fase Saat Ini / Current Phase

Paruman saat ini dalam **Fase Desain (Phase 0)**. Belum ada kode produksi. Yang ada sekarang adalah arsitektur, keputusan desain, dan model tata kelola — semua terbuka untuk diskusi.

*Paruman is currently in **Design Phase (Phase 0)**. There is no production code yet. What exists now is architecture, design decisions, and the governance model — all open for discussion.*

Kontribusi yang paling berharga saat ini:

*The most valuable contributions right now:*

- **Umpan balik arsitektur** — baca [`ARCHITECTURE.md`](./ARCHITECTURE.md), buka Issue jika ada yang tidak masuk akal atau bisa diperbaiki
- **Umpan balik tata kelola** — baca [`docs/governance.md`](./docs/governance.md), koreksi jika ada yang tidak mencerminkan praktik Musyawarah Mufakat yang sesungguhnya
- **Koreksi bahasa** — Bahasa Indonesia dan Bali di dokumen ini harus tepat dan terhormat; koreksi selalu disambut
- **Koneksi komunitas** — mengenal banjar atau subak yang mungkin tertarik menjadi komunitas benih? Buka Issue untuk memperkenalkan diri

---

## Cara Berkontribusi / How to Contribute

### Issues

Gunakan GitHub Issues untuk:

*Use GitHub Issues for:*

- Pertanyaan tentang keputusan desain / design decision questions
- Saran perubahan arsitektur / architecture change suggestions
- Koreksi bahasa atau terminologi adat / language or adat terminology corrections
- Memperkenalkan diri sebagai calon mitra komunitas / introducing yourself as a potential community partner

Beri label Issue dengan tepat / label Issues appropriately:

- `desain` / `design` — pertanyaan atau saran arsitektur
- `tata-kelola` / `governance` — hal-hal yang berkaitan dengan model Musyawarah
- `bahasa` / `language` — koreksi Bahasa Indonesia atau Bali
- `komunitas` / `community` — keterlibatan komunitas, kemitraan banjar

### Pull Requests

Untuk fase desain ini, PR diterima untuk:

*For this design phase, PRs are accepted for:*

- Koreksi dokumentasi / documentation corrections
- Klarifikasi yang meningkatkan keterbacaan / clarifications that improve readability
- Penambahan konteks adat yang hilang atau kurang tepat / missing or incorrect adat context

**Catatan tooling frontend:** gunakan `pnpm` untuk instalasi dan perintah frontend (lint/build/test). Jalankan `pnpm install` di folder `frontend/` sebelum menjalankan `pnpm run check` atau perintah lainnya.

Setiap perubahan substantif pada arsitektur atau model tata kelola harus dimulai sebagai Issue terlebih dahulu.

*Every substantive change to architecture or governance model must start as an Issue first.*

### Diskusi / Discussions

GitHub Discussions digunakan untuk percakapan yang lebih panjang — misalnya perdebatan desain tata kelola, pertanyaan tentang visi, atau topik yang belum siap menjadi Issue spesifik.

*GitHub Discussions is for longer conversations — governance design debates, vision questions, or topics not yet ready for a specific Issue.*

---

## Prinsip Kontribusi / Contribution Principles

**1. Konteks adat diutamakan.**
Jika ada tegangan antara kemudahan teknis dan kesesuaian dengan adat Bali, adat yang menang. Diskusikan dulu.

*If there is tension between technical convenience and Balinese adat appropriateness, adat wins. Discuss first.*

**2. Epistemic honesty.**
Jangan overclaim. Ini berlaku untuk PR juga. Jika kamu tidak yakin apakah sesuatu sudah benar, katakan demikian.

*Don't overclaim. This applies to PRs too. If you're not sure whether something is correct, say so.*

**3. Aksesibilitas bahasa.**
Dokumentasi teknis dalam Bahasa Inggris itu wajar. Dokumen yang menghadap komunitas harus selalu bilingual Bahasa Indonesia dan Inggris. Bahasa Bali disambut di mana pun relevan.

*Technical documentation in English is fine. Community-facing documents must always be bilingual Indonesian and English. Balinese language is welcome wherever relevant.*

**4. Tidak ada kontribusi yang terlalu kecil.**
Memperbaiki satu kata dalam bahasa Bali di dokumen tata kelola bisa lebih berharga dari menambahkan ratusan baris kode.

*Fixing one word in Balinese in the governance document can be more valuable than adding hundreds of lines of code.*

---

## Siapa di Balik Paruman / Who Is Behind Paruman

Paruman adalah proyek komunitas yang diinisiasi oleh pengembang teknologi Bali sebagai kontributor teknis pendiri. Kepemilikan platform dirancang untuk berpindah ke komunitas Bali yang lebih luas seiring pertumbuhan platform.

*Paruman is a community project initiated by Balinese technology practitioners as founding technical contributors. Platform ownership is designed to transfer to the broader Balinese community as the platform grows.*

Dewan Pendiri bubar secara otomatis saat platform mencapai 500 Krama Mipil aktif. Lihat [`docs/governance.md`](./docs/governance.md) untuk detailnya.

*The Founding Council dissolves automatically when the platform reaches 500 active Krama Mipil. See [`docs/governance.md`](./docs/governance.md) for details.*

---

*Satu pertanyaan yang belum terjawab? Buka Issue. Kami membangun ini bersama.*
*One question unanswered? Open an Issue. We're building this together.*
