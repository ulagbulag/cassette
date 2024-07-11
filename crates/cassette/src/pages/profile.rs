use cassette_core::net::gateway::{use_gateway, use_gateway_status};
use patternfly_yew::prelude::*;
use tracing::info;
use yew::prelude::*;

use crate::build_info::*;

#[function_component(Profile)]
pub fn profile() -> Html {
    info!("Beginning loading profile...");

    // Application

    let title = "Profile";
    let subtitle = "This page can be used to check the system or share the problem situation with experts when there is a problem.";

    // Dependencies

    let dependencies = DEPENDENCIES.into_iter().map(|(name, version)| {
        html! {
            <tr>
                <td class="profile-table-key">{ name }</td>
                <td class="profile-table-value">{ version }</td>
            </tr>
        }
    });

    // Runtime

    let gateway_url = use_gateway();
    let gateway_status = use_gateway_status();

    info!("Completed loading profile");

    html! {
        <super::PageBody {title} {subtitle} >
            <Content>
                <h1>{ "System Profile" }</h1>

                <h2>{ "Application Information" }</h2>
                <table class="profile">
                    <tr>
                        <th class="profile-table-key">{ "Key" }</th>
                        <th class="profile-table-value">{ "Value" }</th>
                    </tr>
                    <tr>
                        <td class="profile-table-key">{ "Name" }</td>
                        <td class="profile-table-value">{ PKG_NAME }</td>
                    </tr>
                    <tr>
                        <td class="profile-table-key">{ "Description" }</td>
                        <td class="profile-table-value">{ PKG_DESCRIPTION }</td>
                    </tr>
                    <tr>
                        <td class="profile-table-key">{ "Authors" }</td>
                        <td class="profile-table-value">{ PKG_AUTHORS }</td>
                    </tr>
                    <tr>
                        <td class="profile-table-key">{ "Homepage" }</td>
                        <td class="profile-table-value"><a href={ PKG_HOMEPAGE }>{ PKG_HOMEPAGE }</a></td>
                    </tr>
                    <tr>
                        <td class="profile-table-key">{ "Repository" }</td>
                        <td class="profile-table-value"><a href={ PKG_REPOSITORY }>{ PKG_REPOSITORY }</a></td>
                    </tr>
                    <tr>
                        <td class="profile-table-key">{ "License" }</td>
                        <td class="profile-table-value"><a href="/license">{ PKG_LICENSE }</a></td>
                    </tr>
                </table>

                <h2>{ "Build Information" }</h2>
                <table class="profile">
                    <tr>
                        <td class="profile-table-key">{ "Build Features" }</td>
                        <td class="profile-table-value">{ FEATURES_LOWERCASE_STR }</td>
                    </tr>
                    <tr>
                        <td class="profile-table-key">{ "Build Jobs" }</td>
                        <td class="profile-table-value">{ NUM_JOBS }</td>
                    </tr>
                    <tr>
                        <td class="profile-table-key">{ "Build Profile" }</td>
                        <td class="profile-table-value">{ PROFILE }</td>
                    </tr>
                    <tr>
                        <td class="profile-table-key">{ "Build Time" }</td>
                        <td class="profile-table-value">{ BUILT_TIME_UTC }</td>
                    </tr>
                    <tr>
                        <td class="profile-table-key">{ "Build Version" }</td>
                        <td class="profile-table-value">{ PKG_VERSION }</td>
                    </tr>
                    <tr>
                        <td class="profile-table-key">{ "Debug Mode" }</td>
                        <td class="profile-table-value">{ DEBUG }</td>
                    </tr>
                    <tr>
                        <td class="profile-table-key">{ "Pointer Width" }</td>
                        <td class="profile-table-value">{ CFG_POINTER_WIDTH }</td>
                    </tr>
                    <tr>
                        <td class="profile-table-key">{ "Optimization Level" }</td>
                        <td class="profile-table-value">{ OPT_LEVEL }</td>
                    </tr>
                    <tr>
                        <td class="profile-table-key">{ "Rustc" }</td>
                        <td class="profile-table-value">{ RUSTC }</td>
                    </tr>
                    <tr>
                        <td class="profile-table-key">{ "Rustc Version" }</td>
                        <td class="profile-table-value">{ RUSTC_VERSION }</td>
                    </tr>
                    <tr>
                        <td class="profile-table-key">{ "Target Arch" }</td>
                        <td class="profile-table-value">{ CFG_TARGET_ARCH }</td>
                    </tr>
                    <tr>
                        <td class="profile-table-key">{ "Target Endian" }</td>
                        <td class="profile-table-value">{ CFG_ENDIAN }</td>
                    </tr>
                    <tr>
                        <td class="profile-table-key">{ "Target Environment" }</td>
                        <td class="profile-table-value">{ CFG_ENV }</td>
                    </tr>
                    <tr>
                        <td class="profile-table-key">{ "Target OS" }</td>
                        <td class="profile-table-value">{ CFG_OS }</td>
                    </tr>
                    <tr>
                        <td class="profile-table-key">{ "Target OS Family" }</td>
                        <td class="profile-table-value">{ CFG_FAMILY }</td>
                    </tr>
                </table>

                <h2>{ "Dependency Information" }</h2>
                <table class="profile">
                    <tr>
                        <th class="profile-table-key">{ "Name" }</th>
                        <th class="profile-table-value">{ "Version" }</th>
                    </tr>
                    { for dependencies }
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
            </Content>
        </super::PageBody>
    }
}
