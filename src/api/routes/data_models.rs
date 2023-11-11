
#[derive(serde::Serialize, Clone, serde::Deserialize)]
pub struct NewVault {
    pub name: String,
    pub replicas: usize,
}

