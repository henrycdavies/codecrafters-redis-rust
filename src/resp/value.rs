use chrono::{DateTime, Utc};

pub struct StoredValue {
    pub value: String,
    pub ttl: Option<i64>,
    pub created_at: DateTime<Utc>,
}

impl StoredValue {
    pub fn new(v: &str) -> Self {
        Self::new_with_ttl(v, 0)
    }

    pub fn new_with_ttl(v: &str, ttl: i64) -> Self {
        let created_at = Utc::now();
        StoredValue { value: v.to_string(), ttl: Some(ttl), created_at }
    }

    pub fn is_expired(&self) -> bool {
        match &self.ttl {
            Some(ttl) => {
                if *ttl == 0 {
                    return false
                }
                let now_millis = Utc::now().timestamp_millis();
                let expiry_date_millis = self.created_at.timestamp_millis() + ttl;
                if now_millis > expiry_date_millis {
                    return true;
                }
                false
            },
            None => return false,
        }
    }
}