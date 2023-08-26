impl UserSessionChecker for GetUserGroupSharedLinkRequest {}

#[async_trait]
impl Handler for GetUserGroupSharedLinkRequest {
    type Response = GetUserGroupSharedLinkResponse;

    async fn handle_request(
        &self,
        jogiyo_context: web::Data<Arc<dyn JogiyoContext + Send + Sync>>,
        user_info: UserInfo,
    ) -> Result<Self::Response, ServerError> {
        let user_group_id = get_user_group_id_from_group_url(
            &self.group_url,
            &jogiyo_context.config_context().get_config(),
        )?;

        if !jogiyo_context
            .community_context()
            .check_user_joined_group(user_group_id, user_info.user_id)
            .await?
        {
            return Err(ServerError::BadRequest("Unjoined user.".to_owned()));
        }

        let share_link = user_group_id_to_shared_link(
            user_group_id,
            &jogiyo_context
                .config_context()
                .get_config()
                .user_group_shared_link_key,
        )?;

        Ok(Self::Response {
            share_link,
            error: "".to_owned(),
        })
    }
}

async fn check_user_joined_group(
  &self,
  user_group_id: i32,
  user_id: i32,
) -> Result<bool, ServerError> {
  let user_group_member_info = UserGroupMemberInfo::Entity::find()
      .filter(
          sea_orm::Condition::all()
              .add(UserGroupMemberInfo::Column::UserId.eq(user_id))
              .add(UserGroupMemberInfo::Column::UserGroupId.eq(user_group_id)),
      )
      .one(self.database.connection())
      .await?;

  Ok(user_group_member_info.is_some())
}