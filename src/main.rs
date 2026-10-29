mod api;
mod model;

use leptos::prelude::*;
use model::Actor;

#[component]
fn ActorCard(actor: Actor) -> impl IntoView {
    let data = actor.data;
    let (image_failed, set_image_failed) = signal(false);
    let image = data
        .image
        .filter(|url| url.starts_with("https://") || url.starts_with("http://"));
    let name = data.name.clone();
    view! {
        <article class="card">
            <div class="portrait">
                {image.map(|url| view! {
                    <img src=url alt=format!("Ritratto di {name}") loading="lazy"
                        hidden=move || image_failed.get()
                        on:error=move |_| set_image_failed.set(true)/>
                })}
                <span class="image-placeholder" aria-hidden="true">"Ritratto non disponibile"</span>
                <span class="badge">{actor.category}</span>
            </div>
            <div class="card-body">
                <h2>{data.name}</h2>
                <p class="metadata">{data.nationality.filter(|s| !s.trim().is_empty()).unwrap_or_else(|| "Nazionalità non disponibile".into())}
                    {data.birth_year.map(|year| format!(" · {year}{}", data.death_year.map(|end| format!("–{end}")).unwrap_or_default()))}
                </p>
                <p class="biography">{if data.biography.trim().is_empty() { "Biografia non disponibile".into() } else { data.biography }}</p>
                {details("Film", data.known_for)}
                {details("Premi", data.awards)}
            </div>
        </article>
    }
}

fn details(label: &'static str, items: Option<Vec<String>>) -> impl IntoView {
    let text = items
        .unwrap_or_default()
        .into_iter()
        .filter(|s| !s.trim().is_empty())
        .collect::<Vec<_>>()
        .join(" · ");
    (!text.is_empty()).then(|| view! { <p class="details"><strong>{label}": "</strong>{text}</p> })
}

#[component]
fn Catalog(actors: Vec<Actor>) -> impl IntoView {
    let (category, set_category) = signal(String::from("tutti"));
    let (search, set_search) = signal(String::new());
    let (order, set_order) = signal(String::from("originale"));
    let visible = Memo::new(move |_| {
        let query = search.get().trim().to_lowercase();
        let mut result: Vec<_> = actors
            .iter()
            .filter(|actor| {
                (category.get() == "tutti" || actor.category == category.get())
                    && actor.data.name.to_lowercase().contains(&query)
            })
            .cloned()
            .collect();
        if order.get() != "originale" {
            result.sort_by_key(|actor| actor.data.name.to_lowercase());
            if order.get() == "za" {
                result.reverse();
            }
        }
        result
    });
    view! {
        <section class="filters" aria-label="Filtri del catalogo">
            <label>"Categoria"
                <select prop:value=move || category.get() on:change=move |ev| set_category.set(event_target_value(&ev))>
                    <option value="tutti">"Tutti"</option><option value="attori">"Attori"</option><option value="attrici">"Attrici"</option>
                </select>
            </label>
            <label class="search">"Cerca per nome"
                <input type="search" placeholder="Es. Meryl Streep" prop:value=move || search.get()
                    on:input=move |ev| set_search.set(event_target_value(&ev))/>
            </label>
            <label>"Ordinamento"
                <select prop:value=move || order.get() on:change=move |ev| set_order.set(event_target_value(&ev))>
                    <option value="originale">"Ordine originale"</option><option value="az">"Nome A–Z"</option><option value="za">"Nome Z–A"</option>
                </select>
            </label>
            <button class="secondary" on:click=move |_| { set_category.set("tutti".into()); set_search.set(String::new()); set_order.set("originale".into()); }>"Reset filtri"</button>
        </section>
        <p class="results" role="status">{move || format!("{} persone nel catalogo", visible.get().len())}</p>
        <Show when=move || visible.get().is_empty()>
            <p class="notice">"Nessun risultato. Modifica la ricerca o usa Reset filtri."</p>
        </Show>
        <section class="grid" aria-label="Persone del cinema">
            <For each=move || visible.get() key=|actor| actor.uid.clone() children=|actor| view! { <ActorCard actor/> }/>
        </section>
    }
}

#[component]
fn App() -> impl IntoView {
    let (loading, set_loading) = signal(true);
    let catalog = LocalResource::new(move || async move {
        set_loading.set(true);
        let result = api::load_catalog().await;
        set_loading.set(false);
        result
    });
    view! {
        <main>
            <header>
                <p class="eyebrow">"RUST + WEBASSEMBLY / LEPTOS"</p>
                <h1>"Volti del cinema"</h1>
                <p class="description">"Storie, film e riconoscimenti di attori e attrici che hanno segnato il grande schermo."</p>
                <p class="source">"Dati dalle API pubbliche Lanciweb · Biografie nella lingua originale"</p>
            </header>
            {move || {
                if loading.get() {
                    return view! { <p class="notice" role="status">"Caricamento del catalogo…"</p> }.into_any();
                }
                match catalog.get() {
                    None => view! { <p class="notice" role="status">"Caricamento del catalogo…"</p> }.into_any(),
                    Some(Err(error)) => view! {
                        <section class="notice error" role="alert">
                            <h2>"Non è stato possibile caricare il catalogo completo"</h2>
                            <p>{error}</p>
                            <button on:click=move |_| { set_loading.set(true); catalog.refetch(); }>"Riprova"</button>
                        </section>
                    }.into_any(),
                    Some(Ok(actors)) if actors.is_empty() => view! { <p class="notice" role="status">"Il catalogo è vuoto. Le sorgenti non contengono persone."</p> }.into_any(),
                    Some(Ok(actors)) => view! { <Catalog actors/> }.into_any(),
                }
            }}
            <footer>"Un catalogo scritto in Rust, eseguito nel tuo browser."</footer>
        </main>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}
