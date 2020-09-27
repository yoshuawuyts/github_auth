use serde::{Serialize, Deserialize};

/// GitHub OAuth scope definitions.
///
/// ## Further Reading
/// - [Understanding scopes for Oauth apps](https://developer.github.com/apps/building-oauth-apps/understanding-scopes-for-oauth-apps)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Scope {
    /// Grants read/write access to code, commit statuses, invitations,
    /// collaborators, adding team memberships, and deployment statuses for public
    /// and private repositories and organizations.
    #[serde(rename = "repo")]
    Repo,
    /// Grants read/write access to public and private repository commit statuses.
    /// This scope is only necessary to grant other users or services access to
    /// private repository commit statuses without granting access to the code.
    #[serde(rename = "repo:status")]
    RepoStatus,
    /// Grants access to deployment statuses for public and private repositories.
    /// This scope is only necessary to grant other users or services access to
    /// deployment statuses, without granting access to the code.
    #[serde(rename = "repo_deployment")]
    RepoDeployment,
    /// Grants read/write access to code, commit statuses, collaborators, and
    /// deployment statuses for public repositories and organizations. Also
    /// required for starring public repositories.
    #[serde(rename = "public_repo")]
    PublicRepo,
    /// Grants accept/decline abilities for invitations to collaborate on a
    /// repository. This scope is only necessary to grant other users or services
    /// access to invites without granting access to the code.
    #[serde(rename = "repo:invite")]
    RepoInvite,
    /// Fully manage organization, teams, and memberships.
    #[serde(rename = "admin:org")]
    AdminOrg,
    /// Publicize and unpublicize organization membership.
    #[serde(rename = "write:org")]
    WriteOrg,
    /// Read-only access to organization, teams, and membership.
    #[serde(rename = "read:org")]
    ReadOrg,
    /// Fully manage public keys.
    #[serde(rename = "admin:public_key")]
    AdminPublicKey,
    /// Create, list, and view details for public keys.
    #[serde(rename = "write:public_key")]
    WritePublicKey,
    /// List and view details for public keys.
    #[serde(rename = "read:public_key")]
    ReadPublicKey,
    /// Grants read, write, ping, and delete access to hooks in public or private
    /// repositories.
    #[serde(rename = "admin:repo_hook")]
    AdminRepoHook,
    /// Grants read, write, and ping access to hooks in public or private repositories.
    #[serde(rename = "write:repo_hook")]
    WriteRepoHook,
    /// Grants read and ping access to hooks in public or private repositories.
    #[serde(rename = "read:repo_hook")]
    ReadRepoHook,
    /// Grants read, write, ping, and delete access to organization hooks. Note:
    /// OAuth tokens will only be able to perform these actions on organization
    /// hooks which were created by the OAuth App. Personal access tokens will
    /// only be able to perform these actions on organization hooks created by a
    /// user.
    #[serde(rename = "admin:org_hook")]
    AdminOrgHook,
    /// Grants write access to gists.
    #[serde(rename = "gist")]
    Gist,
    /// Grants read access to a user's notifications. repo also provides this
    /// access.
    #[serde(rename = "notifications")]
    Notifications,
    /// Grants read/write access to profile info only. Note that this scope
    /// includes `user:email` and `user:follow`.
    #[serde(rename = "user")]
    User,
    /// Grants access to read a user's profile data.
    #[serde(rename = "read:user")]
    ReadUser,
    /// Grants read access to a user's email addresses.
    #[serde(rename = "user:email")]
    UserEmail,
    /// Grants access to follow or unfollow other users.
    #[serde(rename = "user:follow")]
    UserFollow,
    /// Grants access to delete adminable repositories.
    #[serde(rename = "delete_repo")]
    DeleteRepo,
    /// Allows read and write access for team discussions.
    #[serde(rename = "write:discussion")]
    WriteDiscussion,
    /// Allows read access for team discussions.
    #[serde(rename = "read:discussion")]
    ReadDiscussion,
    ///	Fully manage GPG keys.
    #[serde(rename = "admin:gpg_key")]
    AdminGpgKey,
    ///	Create, list, and view details for GPG keys.
    #[serde(rename = "write:gpg_key")]
    WriteGpgKey,
    ///	List and view details for GPG keys.
    #[serde(rename = "read:gpg_key")]
    ReadGpgKey,
}
