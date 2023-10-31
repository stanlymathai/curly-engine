use chrono::{DateTime, Utc};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SignupData {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,

    pub status: Status,

    pub identifier: String,

    #[serde(rename = "userDetails")]
    pub user_details: UserDetails,

    #[serde(rename = "miraclData")]
    pub miracl_data: MiraclData,

    #[serde(rename = "locationDetails")]
    pub location_details: LocationData,

    #[serde(rename = "affiliateData")]
    pub affiliate_data: AffiliateData,

    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,

    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,

}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Status {
    Pending,
    Verified,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserDetails {
    pub name: String,

    #[serde(rename = "countryCode")]
    pub country_code: String,

    #[serde(rename = "emailAddress")]
    pub email_address: String,

    #[serde(rename = "dateOfBirth")]
    pub date_of_birth: DateTime<Utc>,

}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MiraclData {
    code: String,

    #[serde(rename = "userId")]
    user_id : String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LocationData {
    city: String,
    latitude: f32,
    longitude: f32,
    postal: String,
    state: String,

    #[serde(rename = "ipV4")]
    ip_v4: String,

    #[serde(rename = "countryCode")]
    country_code: String,

    #[serde(rename = "countryName")]
    country_name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AffiliateData {
    #[serde(rename = "affiliateId")]
    pub affiliate_id: String,

    #[serde(rename = "offerId")]
    pub offer_id: String,

    #[serde(rename = "transactionId")]
    pub transaction_id: String,

    #[serde(rename = "sourceUrl")]
    pub source_url: String,

    #[serde(rename = "trafficType")]
    pub traffic_type: String,

    #[serde(rename = "searchEngine")]
    pub search_engine: String,

    #[serde(rename = "cookiePresent")]
    pub cookie_present: String,

    #[serde(rename = "cookieTimestamp")]
    pub cookie_timestamp: String,

    #[serde(rename = "cookieAgeDays")]
    pub cookie_age_days: String,

    #[serde(rename = "expiredAffiliateId")]
    pub expired_affiliate_id: String,
}