use leptos::*;

#[component]
pub fn SimpleLink(
    link: &'static str,
    #[prop(optional)] text: &'static str,
    #[prop(optional)] children: MaybeProp<ChildrenFn>,
) -> impl IntoView {
    view! {
        <a href=link rel="external" target="_blank" class="text-blue-500 hover:underline">
            {text}
            {children}
        </a>
    }
}