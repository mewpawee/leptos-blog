use leptos::*;

#[component]
pub fn Counter(cx: Scope) -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(cx, 0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! {cx,
        <button class="bg-blue-500 hover:bg-blue-700 rounded text-white font-bold py-2 px-4" on:click=on_click>"Click Me: " {count}</button>
    }
}
