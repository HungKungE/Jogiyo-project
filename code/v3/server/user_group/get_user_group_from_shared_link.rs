impl UserSessionChecker for GetUserGroupFromSharedLinkRequest {}

const NUM_MEMBER_TO_SHOW: usize = 10;

#[async_trait]
impl Handler for GetUserGroupFromSharedLinkRequest {
    type Response = GetUserGroupFromSharedLinkResponse;

    async fn handle_request(
        &self,
        jogiyo_context: web::Data<Arc<dyn JogiyoContext + Send + Sync>>,
        _user_info: UserInfo,
    ) -> Result<Self::Response, ServerError> {
        let user_group_id = get_user_group_id_from_shared_link(
            &self.share_link,
            &jogiyo_context
                .config_context()
                .get_config()
                .user_group_shared_link_key,
        )?;

        let user_group = jogiyo_context
            .community_context()
            .get_user_group(user_group_id)
            .await?;
        if user_group.is_none() {
            return Err(ServerError::BadRequest("Not existing group.".to_owned()));
        }

        let user_group = user_group.unwrap();
        let group_members = jogiyo_context
            .community_context()
            .get_members_in_user_group(user_group_id, NUM_MEMBER_TO_SHOW)
            .await?;

        let group_member_infos = jogiyo_context
            .user_context()
            .multi_get_user(&group_members.iter().map(|m| m.user_id).collect::<Vec<_>>())
            .await?;

        let user_group_members: Vec<_> = group_members
            .into_iter()
            .zip(group_member_infos.into_iter())
            .filter_map(|(member_info, member_user_info)| {
                if let Some(member_user_info) = member_user_info {
                    Some(UserGroupMember {
                        user_nickname: member_user_info.nickname,
                        user_icon: member_user_info.user_icon,
                        is_admin: member_info.is_admin,
                    })
                } else {
                    None
                }
            })
            .collect();

        Ok(Self::Response {
            group_name: user_group.group_name,
            group_intro: user_group.group_intro,
            group_icon: user_group.group_icon,
            member_list: user_group_members,
            group_header_image_info: user_group.group_header_image_info,
            error: "".to_owned(),
        })
    }
}