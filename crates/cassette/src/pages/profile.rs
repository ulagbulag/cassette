use yew::prelude::*;

use crate::hooks::gateway::use_gateway;

#[function_component(Profile)]
pub fn profile() -> Html {
    let gateway = use_gateway();

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
                    <td class="profile-table-value">{ env!("CARGO_PKG_LICENSE") }</td>
                </tr>
                <tr>
                    <td class="profile-table-key">{ "Version" }</td>
                    <td class="profile-table-value">{ env!("CARGO_PKG_VERSION") }</td>
                </tr>
                <tr>
                    <td class="profile-table-key">{ "Homepage" }</td>
                    <td class="profile-table-value"><a href={ env!("CARGO_PKG_HOMEPAGE") } /></td>
                </tr>
                <tr>
                    <td class="profile-table-key">{ "Repository" }</td>
                    <td class="profile-table-value"><a href={ env!("CARGO_PKG_REPOSITORY") } /></td>
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
                    <td class="profile-table-value">{ gateway }</td>
                </tr>
            </table>
        </main>
    }
}
