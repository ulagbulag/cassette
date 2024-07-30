use cassette_core::{net::fetch::FetchState, prelude::*};
use cassette_plugin_kubernetes_core::user::UserSpec;
use yew::prelude::*;

#[function_component(UserAbout)]
pub fn user_about() -> Html {
    let title = "About Me";

    let body = match &*use_user_spec() {
        FetchState::Pending | FetchState::Fetching => html! { <Loading /> },
        FetchState::Collecting(spec) | FetchState::Completed(spec) => {
            html! {
                <>
                    <p>{ "Welcome, " }{ &spec.name }{ "!" }</p>
                </>
            }
        }
        FetchState::Error(msg) => html! { <Error msg={ msg.clone() } /> },
    };

    html! {
        <>
            <h2>{ title }</h2>
            { body }
        </>
    }
}

#[hook]
fn use_user_spec() -> UseStateHandle<FetchState<UserSpec>> {
    #[cfg(all(feature = "examples", not(feature = "mock-release")))]
    {
        use std::rc::Rc;

        use cassette_plugin_kubernetes_core::{
            net::DEFAULT_NAMESPACE,
            user::{UserMetadata, UserRoleSpec},
        };

        use_state_eq(|| {
            FetchState::Completed(Rc::new(UserSpec {
                metadata: UserMetadata {
                    email: "guest@example.com".into(),
                    preferred_username: "guest".into(),
                },
                name: "guest".into(),
                namespace: DEFAULT_NAMESPACE,
                role: UserRoleSpec { is_admin: true },
            }))
        })
    }

    #[cfg(not(all(feature = "examples", not(feature = "mock-release"))))]
    {
        use cassette_core::net::{
            fetch::{FetchRequestWithoutBody, Method},
            gateway::use_fetch,
        };

        use_fetch(move || FetchRequestWithoutBody {
            method: Method::GET,
            name: "user spec",
            url: "/user/me",
            body: None,
        })
    }
}
