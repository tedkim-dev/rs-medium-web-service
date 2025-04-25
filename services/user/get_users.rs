use crate::{model::User, Error, UserService};

impl UserService {
    pub async fn get_users(&self) -> Result<Vec<User>, Error> {
        let users = self.repo.get_users().await.map_err(Error::GetUsersFailed)?;
        Ok(users)
    }
}

