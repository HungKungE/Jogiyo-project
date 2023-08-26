mpl UserSessionChecker for KickUserFromGroupRequest {}

const NUM_MEMBER_TO_SHOW: usize = 10;

#[async_trait]
impl Handler for KickUserFromGroupRequest {
    type Response = KickUserFromGroupResponse;

    async fn handle_request(
        &self,
        jogiyo_context: web::Data<Arc<dyn JogiyoContext + Send + Sync>>,
        user_info: UserInfo,
    ) -> Result<Self::Response, ServerError> {
        // groupUrl로 group이 존재하는지 확인하는 과정.
        let user_group_id = get_user_group_id_from_group_url(
            &self.user_group_url,
            &jogiyo_context.config_context().get_config(),
        )?;

        let user_group = jogiyo_context
            .community_context()
            .get_user_group(user_group_id)
            .await?;

        if user_group.is_none() {
            return Err(ServerError::BadRequest("Group does not exist.".to_owned()));
        }

        // user_group_memeber_info[]
        let group_member_infos = jogiyo_context
            .community_context()
            .get_members_in_user_group(user_group_id, NUM_MEMBER_TO_SHOW)
            .await?;

        // group_admin 정보
        let group_admin_info: Vec<UserGroupMemberInfo> = group_member_infos
            .clone()
            .into_iter()
            .filter(|group_member_info| group_member_info.is_admin)
            .collect();

        if group_admin_info.is_empty() {
            return Ok(Self::Response {
                error: "This Group has not admin user".to_owned(),
            });
        }

        // 그룹 admin이 보낸 요청이 아니면 error처리
        if user_info.user_id != group_admin_info[0].user_id {
            return Ok(Self::Response {
                error: "You are not Admin.".to_owned(),
            });
        }

        // userInfo[]
        let members_user_infos = jogiyo_context
            .user_context()
            .multi_get_user(
                &group_member_infos
                    .iter()
                    .map(|m| m.user_id)
                    .collect::<Vec<_>>(),
            )
            .await?;

        let mut kicked_user_info: Option<UserInfo> = None;

        for member_user_info in members_user_infos {
            let member_user_info = member_user_info.unwrap();
            if member_user_info.nickname == self.kicked_user_nickname {
                kicked_user_info = Some(member_user_info)
            }
        }

        if kicked_user_info.is_none() {
            return Ok(Self::Response {
                error: "User does not exist.".to_owned(),
            });
        }

        jogiyo_context
            .community_context()
            .remove_user_from_the_group(user_group_id, &kicked_user_info.unwrap())
            .await?;

        Ok(Self::Response {
            error: "".to_owned(),
        })
    }
}