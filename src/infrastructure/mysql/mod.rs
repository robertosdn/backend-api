use async_trait::async_trait;
use sqlx::mysql::MySqlPool;
use uuid::Uuid;
use crate::{
	domain::access_user::AccessUser,
	outbox::outbox_record::OutboxRecord,
	repositories::access_user_write_repository::{AccessUserWriteRepository, WriteRepositoryError},
};
pub struct MySqlAccessUserRepository {
	pool: MySqlPool,
}
impl MySqlAccessUserRepository {
	pub fn new(pool: MySqlPool) -> Self {
		Self { pool }
	}
}
fn uuid_bytes(value: &str) -> Result<[u8; 16], WriteRepositoryError> {
	Uuid::parse_str(value)
		.map(|uuid| *uuid.as_bytes())
		.map_err(|_| WriteRepositoryError::Storage)
}
#[async_trait]
impl AccessUserWriteRepository for MySqlAccessUserRepository {
	async fn save_user_and_event(&self, user: AccessUser, event: OutboxRecord) -> Result<(), WriteRepositoryError> {
		let user_id = uuid_bytes(&user.id)?;
		let event_id = uuid_bytes(&event.id)?;
		let aggregate_id = uuid_bytes(&event.aggregate_id)?;
		let payload = serde_json::to_string(&event.payload).map_err(|_| WriteRepositoryError::Storage)?;
		let mut transaction = self.pool.begin().await.map_err(|_| WriteRepositoryError::Storage)?;

		let user_result = sqlx::query(
			"INSERT INTO access_users (id, email, name, password_hash, status, version, created_at, updated_at) VALUES (?, ?, ?, ?, 'active', ?, ?, ?)",
		)
		.bind(user_id.as_slice())
		.bind(user.email.as_str())
		.bind(user.name.as_str())
		.bind(&user.password_hash)
		.bind(user.version)
		.bind(&user.created_at)
		.bind(&user.updated_at)
		.execute(&mut *transaction)
		.await;

		if let Err(error) = user_result {
			if error.as_database_error().and_then(|database| database.code()).as_deref() == Some("1062") {
				return Err(WriteRepositoryError::DuplicateEmail);
			}
			return Err(WriteRepositoryError::Storage);
		}

		sqlx::query(
			"INSERT INTO access_users_outbox (id, aggregate_id, event_type, payload, status, attempts, available_at, created_at) VALUES (?, ?, ?, ?, 'pending', 0, ?, ?)",
		)
		.bind(event_id.as_slice())
		.bind(aggregate_id.as_slice())
		.bind(&event.event_type)
		.bind(payload)
		.bind(&user.created_at)
		.bind(&user.created_at)
		.execute(&mut *transaction)
		.await
		.map_err(|_| WriteRepositoryError::Storage)?;

		transaction.commit().await.map_err(|_| WriteRepositoryError::Storage)
	}
}