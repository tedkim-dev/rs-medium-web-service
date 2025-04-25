use uuid::Uuid;

use crate::{Error, User, UserService};

impl UserService {
    pub async fn get_user_by_id(&self, id: Uuid) -> Result<User, Error> {
        let user = self
            .repo
            .get_user_by_id(id)
            .await
            .map_err(Error::GetUserByIdFailed)?;
        Ok(user)
    }
}
