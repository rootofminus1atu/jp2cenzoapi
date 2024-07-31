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