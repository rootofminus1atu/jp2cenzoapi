use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct Storage {
    pub bucket_id: String,
    pub supabase_url: String,
    // future props for api key etc
}

impl Storage {
    pub fn new(bucket_id: String, supabase_url: String) -> Self {
        Storage { bucket_id, supabase_url }
    }
}


#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
pub struct Object {
    name: String,
    bucket_id: String
}

impl Object {
    pub fn to_link(&self, storage_data: &Storage) -> String {
        format!("{}/storage/v1/object/public/{}/{}", storage_data.supabase_url, storage_data.bucket_id, self.name)
    }
}