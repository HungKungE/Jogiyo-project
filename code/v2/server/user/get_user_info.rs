impl UserSessionChecker for GetUserInfoRequest {}

#[async_trait]
impl Handler for GetUserInfoRequest {
    type Response = GetUserInfoResponse;

    async fn handle_request(
        &self,
        jogiyo_context: web::Data<Arc<dyn JogiyoContext + Send + Sync>>,
        user_info: UserInfo,
    ) -> Result<Self::Response, ServerError> {
        if !user_info.is_admin {
            return Err(ServerError::Unauthorized("Not an admin".to_owned()));
        }

        let user_infos = UserInfoMetadatas::Entity::find()
            .all(jogiyo_context.database_context().connection())
            .await?;

        let user_infos = user_infos
            .into_iter()
            .map(|user_info| user_db_to_proto(user_info))
            .collect();

        Ok(GetUserInfoResponse {
            user_infos,
            error: "".to_owned(),
        })
    }
}