/**
 * Get Version
 * 
 * Retrieves the version from the `Cargo.toml` file.
 * 
 * NOTE: Package version is passed as an environment variable to the compiler.
 */
pub fn get_version() -> String {
    /* Retrieve app version from toml. */
    let version: &str = env!("CARGO_PKG_VERSION");

    /* Return formatted app version. */
    format!("v{} (alpha)", version)
}

/**
 * Validator
 */
pub trait Validator {
    fn get_id(&self) -> String;
}

/**
 * Subnet
 */
pub trait Subnet {
    fn get_id(&self) -> String;
}

pub struct FederationNode {
    pub id: String,
    pub owner: String,
    pub title: String,
    pub created_at: String,
}

impl Validator for FederationNode {
    fn get_id(&self) -> String {
        format!("id is {}", self.id)
    }
}
