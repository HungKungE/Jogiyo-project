// 그룹에 들어올 수 있는 최대 인원수
const MAX_NUM_GROUP_MEMBER: i32 = 10;

impl UserSessionChecker for JoinUserGroupRequest {}

#[async_trait]
impl Handler for JoinUserGroupRequest {
    type Response = JoinUserGroupResponse;

    async fn handle_request(
        &self,
        jogiyo_context: web::Data<Arc<dyn JogiyoContext + Send + Sync>>,
        user_info: UserInfo,
    ) -> Result<Self::Response, ServerError> {
        let user_group_id = get_user_group_id_from_shared_link(
            &self.user_group_shared_link,
            &jogiyo_context
                .config_context()
                .get_config()
                .user_group_shared_link_key,
        )?;

        let group_to_join = jogiyo_context
            .community_context()
            .get_user_group(user_group_id)
            .await?;

        if group_to_join.is_none() {
            return Err(ServerError::BadRequest("Not existing group.".to_owned()));
        }

        let group_to_join = group_to_join.unwrap();

        let group_url = user_group_id_to_group_url(
            user_group_id,
            &jogiyo_context.config_context().get_config(),
        )?;

        if jogiyo_context
            .community_context()
            .check_user_joined_group(group_to_join.user_group_id, user_info.user_id)
            .await?
        {
            return Ok(Self::Response {
                result: true,
                group_url,
                error: "".to_owned(),
            });
        }

        if jogiyo_context
            .community_context()
            .count_num_group_member_in_the_group(user_group_id)
            .await?
            >= MAX_NUM_GROUP_MEMBER
        {
            return Err(ServerError::BadRequest(
                "Exceeding 10 group members limit.".to_owned(),
            ));
        }

        jogiyo_context
            .community_context()
            .add_user_to_the_group(user_group_id, &user_info)
            .await?;

        Ok(Self::Response {
            result: true,
            group_url,
            error: "".to_owned(),
        })
    }
}