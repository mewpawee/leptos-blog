use leptos::*;
use leptos_meta::*;

#[component]
pub fn Index(cx: Scope) -> impl IntoView {
    view! { cx,
        <Title text= "ayo" />
        <div class="flex justify-center">
            <div class="flex flex-col">
                <h1 class="text-center">"Welcome to Leptos with tailwindcss!"</h1>
            </div>
        </div>
    }
}
