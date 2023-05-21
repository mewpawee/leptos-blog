use crate::pages::{About, Count, Index};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! {
        cx,

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        // <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>
        <Stylesheet id="leptos" href="https://unpkg.com/tailwindcss@2.2.19/dist/tailwind.min.css"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                <Route path="/" view=|cx| view! { cx, <Index /> }/>
                    <Route path="/about" view=|cx| view! { cx, <About /> }/>
                    <Route path="/count" view=|cx| view! { cx, <Count /> }/>
                </Routes>
            </main>
        </Router>
    }
}
