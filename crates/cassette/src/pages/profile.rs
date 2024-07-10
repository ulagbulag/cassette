use cassette_core::net::gateway::{use_gateway, use_gateway_status};
use yew::prelude::*;
use yew_router::prelude::*;

use crate::route::Route;

#[function_component(Profile)]
pub fn profile() -> Html {
    let gateway_url = use_gateway();
    let gateway_status = use_gateway_status();

    html! {
        <main class="profile">
            <h1>{ "System Profile" }</h1>

            <h2>{ "Build Information" }</h2>
            <table class="profile">
                <tr>
                    <th class="profile-table-key">{ "Key" }</th>
                    <th class="profile-table-value">{ "Value" }</th>
                </tr>
                <tr>
                    <td class="profile-table-key">{ "Name" }</td>
                    <td class="profile-table-value">{ env!("CARGO_PKG_NAME") }</td>
                </tr>
                <tr>
                    <td class="profile-table-key">{ "Description" }</td>
                    <td class="profile-table-value">{ env!("CARGO_PKG_DESCRIPTION") }</td>
                </tr>
                <tr>
                    <td class="profile-table-key">{ "Authors" }</td>
                    <td class="profile-table-value">{ env!("CARGO_PKG_AUTHORS") }</td>
                </tr>
                <tr>
                    <td class="profile-table-key">{ "License" }</td>
                    <td class="profile-table-value"><a href="/license">{ env!("CARGO_PKG_LICENSE") }</a></td>
                </tr>
                <tr>
                    <td class="profile-table-key">{ "Version" }</td>
                    <td class="profile-table-value">{ env!("CARGO_PKG_VERSION") }</td>
                </tr>
                <tr>
                    <td class="profile-table-key">{ "Homepage" }</td>
                    <td class="profile-table-value"><a href={ env!("CARGO_PKG_HOMEPAGE") }>{ env!("CARGO_PKG_HOMEPAGE") }</a></td>
                </tr>
                <tr>
                    <td class="profile-table-key">{ "Repository" }</td>
                    <td class="profile-table-value"><a href={ env!("CARGO_PKG_REPOSITORY") }>{ env!("CARGO_PKG_REPOSITORY") }</a></td>
                </tr>
            </table>

            <h2>{ "Runtime Information" }</h2>
            <table class="profile">
                <tr>
                    <th class="profile-table-key">{ "Key" }</th>
                    <th class="profile-table-value">{ "Value" }</th>
                </tr>
                <tr>
                    <td class="profile-table-key">{ "Gateway URL" }</td>
                    <td class="profile-table-value">{ gateway_url }</td>
                </tr>
                <tr>
                    <td class="profile-table-key">{ "Gateway Status" }</td>
                    <td class="profile-table-value">{ gateway_status }</td>
                </tr>
            </table>
        </main>
    }
}
