use crate::Result;

#[derive(Debug)]
pub struct ApiState {}

impl ApiState {
    pub async fn new() -> Result<Self> {
        Ok(Self {})
    }
}
