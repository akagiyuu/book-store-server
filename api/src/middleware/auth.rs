use uuid::Uuid;

pub enum Role {
    User,
    Admin,
}

pub struct AuthContext {
    pub id: Uuid,
    pub role: Role,
}
