impl UserSessionChecker for MoveUserFileRequest {}

#[async_trait]
impl Handler for MoveUserFileRequest {
    type Response = MoveUserFileResponse;

    async fn handle_request(
        &self,
        jogiyo_context: web::Data<Arc<dyn JogiyoContext + Send + Sync>>,
        user_info: UserInfo,
    ) -> Result<Self::Response, ServerError> {
        let file_id = get_file_id_from_file_shared_link(
            &self.shared_file_url,
            &jogiyo_context.config_context().get_config(),
        )?;

        let user_file = get_user_file(
            &jogiyo_context.user_file_context(),
            file_id,
            Some(user_info.user_id),
        )
        .await?;

        let mut parent_directory_file_id = None;
        if let Some(parent_directory_file_url) = &self.parent_directory_file_url {
            parent_directory_file_id = Some(get_file_id_from_file_shared_link(
                parent_directory_file_url,
                &jogiyo_context.config_context().get_config(),
            )?);

            // 부모 dir_id == trash_can_id 면 검사하지 않고 넘어가기
            if parent_directory_file_id != Some(TRASH_CAN_ID) {
                let dir = get_user_file(
                    &jogiyo_context.user_file_context(),
                    parent_directory_file_id.unwrap(),
                    Some(user_info.user_id),
                )
                .await?;

                if dir.author_id != user_info.user_id {
                    return Err(ServerError::BadRequest("Unknown directory.".to_owned()));
                }

                if dir.file_type != (FileType::Folder as i32) {
                    return Err(ServerError::BadRequest(
                        "Must move under the directory.".to_owned(),
                    ));
                }
            }
        }

        jogiyo_context
            .user_file_context()
            .change_file_directory(
                file_id,
                user_file.parent_directory_file_id,
                parent_directory_file_id,
            )
            .await?;

        Ok(Self::Response {
            error: "".to_owned(),
        })
    }
}