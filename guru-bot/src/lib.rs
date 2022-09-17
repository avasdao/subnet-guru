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
