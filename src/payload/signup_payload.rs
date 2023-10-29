use chrono::{self, Utc};
use serde::{Deserialize, Serialize};
use validator::ValidationError;
use validator_derive::Validate;

#[derive(Clone, Debug, Serialize, Deserialize, Validate)]
pub struct SignupPayload {
    #[validate(length(min = 2))]
    pub name: String,

    #[validate(email)]
    pub email: String,

    #[validate(length(min = 2))]
    pub residency: String,

    #[serde(rename = "dateOfBirth")]
    #[validate(custom = "validate_birthdate")]
    pub date_of_birth: chrono::DateTime<Utc>,
}

fn validate_birthdate(
    date_of_birth: &chrono::DateTime<chrono::Utc>,
) -> Result<(), ValidationError> {
    let now = chrono::Utc::now();
    if *date_of_birth > now {
        Err(ValidationError::new("date_of_birth must be in the past"))
    } else {
        Ok(())
    }
}
