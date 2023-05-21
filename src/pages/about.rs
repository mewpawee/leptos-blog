use leptos::*;
use leptos_meta::*;

#[component]
pub fn About(cx: Scope) -> impl IntoView {
    view! {cx,
        <Title text= "About" />
        <div class="flex justify-center">
            <div class="flex flex-col">
                <h1 class="text-center">"Hi!"</h1>
            </div>
        </div>
    }
}
