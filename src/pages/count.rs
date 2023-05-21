use crate::components::Counter;
use leptos::*;
use leptos_meta::*;

#[component]
pub fn Count(cx: Scope) -> impl IntoView {
    view! {cx,
        <Title text= "Count" />
        <div class="flex justify-center">
            <div class="flex flex-col">
                <Counter />
            </div>
        </div>
    }
}
