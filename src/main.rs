use leptos::prelude::*;

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = signal(0_u64);

    view! {
        <main>
            <p class="eyebrow">"Un primo esperimento con Rust e WebAssembly"</p>
            <h1>"Rust nel browser"</h1>
            <p class="description">
                "Questa pagina è scritta in Rust: premi il pulsante e guarda il contatore aggiornarsi."
            </p>
            <section aria-label="Contatore interattivo" class="counter">
                <p>"Il tuo contatore"</p>
                <output aria-live="polite" aria-atomic="true">{move || count.get()}</output>
                <button type="button" on:click=move |_| set_count.update(|value| *value += 1)>
                    "Incrementa"
                </button>
            </section>
        </main>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}
