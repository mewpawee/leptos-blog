use leptos::*;

#[component]
pub fn SectionContainer(cx: Scope, children: Children) -> impl IntoView {
    // Fragment has `nodes` field that contains a Vec<View>
    let children = children(cx)
        .nodes
        .into_iter()
        .map(|child| view! { cx, <>{child}</>})
        .collect_view(cx);

    view! { cx,
        <div class="mx-auto max-w-3xl px-4 sm:px-6 xl:max-w-5xl xl:px-0">{children}</div>
    }
}
