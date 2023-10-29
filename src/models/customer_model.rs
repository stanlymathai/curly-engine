use chrono::{DateTime, Utc};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Customer {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,

    pub name: String,

    pub residency: String,

    pub status: Status,

    #[serde(rename = "email")]
    pub email: String,

    #[serde(rename = "dateOfBirth")]
    pub date_of_birth: DateTime<Utc>,

    #[serde(rename = "securitySecret")]
    pub security_secret: SecuritySecret,

    #[serde(rename = "secretOrKey")]
    pub secret_or_key: String,

    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,

    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SecuritySecret {
    pub key: String,
    pub value: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Status {
    Pending,
    Active,
}
