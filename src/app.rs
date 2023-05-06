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

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=|cx| view! { cx, <HomePage/> }/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage(cx: Scope) -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(cx, 0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! { cx,
        <div class="flex justify-center">
            <div class="flex flex-col">
                <h1 class="text-center">"Welcome to Leptos with tailwindcss!"</h1>
                <button class="bg-blue-500 hover:bg-blue-700 rounded text-white font-bold py-2 px-4" on:click=on_click>"Click Me: " {count}</button>
            </div>
        </div>
    }
}
