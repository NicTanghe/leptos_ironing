
use leptos::*;
use leptos_router::*;

#[component]
pub fn ContactList(contacts: ReadSignal<Vec<String>>) -> impl IntoView {
    view! {
        <div class="contact-list">
            <h3>"Contacts"</h3>
            <div class="contact-list-contacts">
                {move || contacts.get().into_iter().map(|name| {
                    let href = format!("/contacts/{}", name.to_lowercase());
                    view! {
                        <A href={href}>{name}</A>
                    }
                }).collect_view()}
            </div>
             <Outlet/>
        </div>
    }
}

 #[component]
pub fn ContactInfo(contacts: ReadSignal<Vec<String>>) -> impl IntoView {
    let params = use_params_map();
    let id = create_memo(move |_| params.with(|params| params.get("id").cloned().unwrap_or_default()));
    // Reactively find the contact with the matching name (id)
    let name = move || {
        let lowercase_id = id().to_lowercase();
        contacts.get().iter()
            .find(|&contact| contact.to_lowercase() == lowercase_id)
            .cloned()
            .unwrap_or("User not found.".to_string())
    };

    view! {
        <h4 key={id()}>{name}</h4>
        <div class="contact-info">
            <div class="tabs">
                <A href="" exact=true>"Contact Info"</A>
                <A href="conversations">"Conversations"</A>
            </div>
            <Outlet/>
        </div>
    }
}

