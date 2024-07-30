use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserSpec {
    /// User metadata
    #[serde(default)]
    pub metadata: UserMetadata,

    /// User name
    pub name: String,

    /// User namespace
    pub namespace: String,

    /// User role
    pub role: UserRoleSpec,
}

impl UserSpec {
    #[cfg(feature = "vine")]
    pub fn from_vine_session(
        metadata: UserMetadata,
        session: ::vine_api::user_session::UserSession,
    ) -> ::anyhow::Result<(Self, ::kube::Client)> {
        let ::vine_api::user_session::UserSession {
            box_bindings: _,
            box_name: _,
            box_quota_bindings: _,
            kube,
            namespace,
            role:
                ::vine_api::user_role::UserRoleSpec {
                    is_admin,
                    is_dev: _,
                    is_ops: _,
                },
            user: _,
            user_name,
        } = session;

        let spec = Self {
            metadata,
            name: user_name,
            namespace,
            role: UserRoleSpec { is_admin },
        };

        let client = kube.ok_or_else(|| ::anyhow::anyhow!("No kubernetes client"))?;

        Ok((spec, client))
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct UserMetadata {
    /// User e-mail address
    #[serde(default)]
    pub email: String,

    /// Preferred user name
    #[serde(default)]
    pub preferred_username: String,
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserRoleSpec {
    #[serde(default)]
    pub is_admin: bool,
}
