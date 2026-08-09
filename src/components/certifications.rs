// src/components/certifications.rs — Sakthi Harish Portfolio × Leptos 0.7
//
// ── UI COMPONENT ─────────────────────────────────────────────────────────────
// Fully encapsulated component. Data is injected statically at compile time.
// ─────────────────────────────────────────────────────────────────────────────

use leptos::prelude::*;
use crate::content;

#[component]
pub fn Certifications() -> impl IntoView {
    let items = content::load().certifications;
    view! {
        <section class="certifications-section grid grid-cols-1 md:grid-cols-12 border-b">
            <div class="md:col-span-4 p-[clamp(32px,5vw,80px)] flex flex-col justify-center md:border-r">
                <h2 class="section-heading font-sans text-[0.7rem] font-semibold tracking-[0.25em] uppercase text-muted">"CERTIFICATIONS"</h2>
                <p class="text-[0.85rem] text-muted leading-[1.7] mt-4 max-w-[360px]">
                    "Industry-recognized credentials validating expertise in cloud, Kubernetes, and infrastructure automation."
                </p>
            </div>
            <div class="md:col-span-8 p-[clamp(32px,5vw,80px)] flex flex-col justify-center">
                <div class="flex flex-col">
                    {items.into_iter().map(|item| {
                        view! {
                            <a
                                href=item.href
                                target="_blank"
                                class="cert-item flex justify-between py-4.5 border-b last:border-b-0 hover-target"
                                style="text-decoration:none;color:inherit;"
                            >
                                <div class="flex flex-col">
                                    <span class="font-serif text-[0.9rem] font-bold">{item.name}</span>
                                    <span class="text-[0.65rem] font-semibold tracking-[0.2em] text-muted mt-1">{item.issuer} | {item.year}</span>
                                </div>
                                <span class="text-[0.6rem] tracking-[0.15em] text-muted">"VIEW"</span>
                            </a>
                        }
                    }).collect_view()}
                </div>
            </div>
        </section>
    }
}