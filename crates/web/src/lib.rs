//! Project Baca — Leptos 0.7 WASM Single Page Application
//! Estetika: Vintage Literary (1900–1950)

use leptos::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    let (lang, set_lang) = signal("ID".to_string());

    let toggle_lang = move |_| {
        set_lang.update(|l| {
            if l == "ID" {
                *l = "EN".to_string();
            } else {
                *l = "ID".to_string();
            }
        });
    };

    view! {
        <div class="app-container">
            // Header / Navigation
            <header class="header-vintage">
                <a href="/" class="brand-title">
                    <span class="brand-ornament">"❖"</span>
                    <span>"Project Baca"</span>
                </a>
                <nav class="nav-links">
                    <a href="#katalog" class="nav-link">"Katalog"</a>
                    <a href="#semantik" class="nav-link">"Pencarian Kutipan"</a>
                    <a href="#tentang" class="nav-link">"Tentang"</a>
                    <button class="lang-switch" on:click=toggle_lang>
                        {move || format!("[ {} ]", lang.get())}
                    </button>
                </nav>
            </header>

            // Hero Banner
            <section class="hero-vintage">
                <div class="hero-subtitle">"Koleksi Naskah Sastra Klasik Ranah Publik"</div>
                <h1 class="hero-heading">"Keabadian Kata dalam Estetika Cetak Klasik"</h1>
                <p class="hero-desc">
                    "Membaca karya agung sastra dunia bebas royalti dengan tipografi berkelas, kenyamanan reflowable reader, dan penemuan kutipan berdaya semantik."
                </p>
            </section>

            // Catalog Section
            <main>
                <div class="catalog-section-title">
                    <span>"Buku Pilihan Pembaca"</span>
                </div>

                <div class="book-grid">
                    <div class="book-card">
                        <div>
                            <div class="book-tag">"Sastra Melayu Modern"</div>
                            <h2 class="book-title">"Siti Nurbaya"</h2>
                            <div class="book-author">"Marah Rusli (1922)"</div>
                            <p class="book-synopsis">
                                "Kisah kasih tak sampai antara Samsulbahri dan Siti Nurbaya yang terhalang oleh adat dan jeratan rentenir Datuk Maringgih."
                            </p>
                        </div>
                        <div class="book-card-footer">
                            <span>"~78.000 kata"</span>
                            <a href="#read/siti-nurbaya" class="btn-read">
                                <span>"Baca"</span>
                                <span>"→"</span>
                            </a>
                        </div>
                    </div>

                    <div class="book-card">
                        <div>
                            <div class="book-tag">"Klasik Dunia"</div>
                            <h2 class="book-title">"Pride and Prejudice"</h2>
                            <div class="book-author">"Jane Austen (1813)"</div>
                            <p class="book-synopsis">
                                "Perjalanan Elizabeth Bennet menghadapi dinamika kelas sosial, pernikahan, dan prasangka terhadap Mr. Darcy yang memikat."
                            </p>
                        </div>
                        <div class="book-card-footer">
                            <span>"~120.000 kata"</span>
                            <a href="#read/pride-and-prejudice" class="btn-read">
                                <span>"Baca"</span>
                                <span>"→"</span>
                            </a>
                        </div>
                    </div>

                    <div class="book-card">
                        <div>
                            <div class="book-tag">"Fiksi Ilmiah Klasik"</div>
                            <h2 class="book-title">"The Time Machine"</h2>
                            <div class="book-author">"H.G. Wells (1895)"</div>
                            <p class="book-synopsis">
                                "Eksplorasi penjelajah waktu ke masa depan bumi tahun 802.701 Masehi, menyaksikan percabangan ras manusia Eloi dan Morlock."
                            </p>
                        </div>
                        <div class="book-card-footer">
                            <span>"~32.000 kata"</span>
                            <a href="#read/the-time-machine" class="btn-read">
                                <span>"Baca"</span>
                                <span>"→"</span>
                            </a>
                        </div>
                    </div>
                </div>
            </main>

            // Footer
            <footer class="footer-vintage">
                <div class="footer-fleuron">"❖"</div>
                <p>"Project Baca — Hak cipta naskah buku berada di domain publik. Kode sumber berlisensi MIT / Apache-2.0."</p>
            </footer>
        </div>
    }
}
