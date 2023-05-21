use crate::components::SectionContainer;
use leptos::*;

struct Header {
    href: String,
    title: String,
}

#[component]
pub fn NavBar(cx: Scope) -> impl IntoView {
    let headers: Vec<Header> = vec![
        Header {
            href: String::from("/blog"),
            title: String::from("Blog"),
        },
        Header {
            href: String::from("/tags"),
            title: String::from("Tags"),
        },
        Header {
            href: String::from("/about"),
            title: String::from("About"),
        },
    ];

    view! {cx,
        <SectionContainer>
          <div class="flex flex-col justify-between">
            <div class="flex items-center text-base leading-5">
                <div class="hidden sm:block">
                    <div class="flex flex-row justify-between">
                        {headers.into_iter()
                                .map(|header| view! {cx, <a href={header.href}>{header.title}</a>})
                                .collect::<Vec<_>>()
                                }
                    </div>
                </div>
            </div>
          </div>
        </SectionContainer>
    }
}
