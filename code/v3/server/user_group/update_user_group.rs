// 이미지 최대 크기 3MB
const MAX_IMAGE_SIZE: usize = 3 * 1024 * 1024;

#[async_trait(?Send)]
impl MultipartHandler for UpdateUserGroupRequest {
    type Response = UpdateUserGroupResponse;

    async fn handle_request(
        &self,
        mut payload: Multipart,
        jogiyo_context: web::Data<Arc<dyn JogiyoContext + Send + Sync>>,
        user_info: Option<UserInfo>,
    ) -> Result<Self::Response, ServerError> {
        if user_info.is_none() {
            return Err(ServerError::Unauthorized(
                "Not authenticated user".to_owned(),
            ));
        }

        let user_group_id = get_user_group_id_from_group_url(
            &self.user_group_url,
            &jogiyo_context.config_context().get_config(),
        )?;

        let user_group =
            get_user_group_by_group_id(&jogiyo_context.community_context(), user_group_id).await?;

        let update_group_header_image_info = if let Some(ref group_header_image_info) =
            self.group_header_image_info
        {
            let group_header_image_info = group_header_image_info.to_owned();

            // 이미지 파일 삭제
            if self.is_image_file_delete {
                jogiyo_context
                    .community_context()
                    .update_user_group(
                        user_group_id,
                        self.group_name.clone(),
                        self.group_intro.clone(),
                        Some(group_header_image_info),
                    )
                    .await?;

                if let Some(prev_group_header_image_info) = user_group.group_header_image_info {
                    remove_file(
                        Path::new(&jogiyo_context.config_context().get_config().static_root_dir)
                            // image path가 "/static/..." 으로 시작해서 맨 앞을 떼어버림
                            .join(&prev_group_header_image_info.image_path[1..]),
                    )
                    .map_err(|_| ServerError::BadRequest("Image not found".to_owned()))?;
                }

                return Ok(UpdateUserGroupResponse {
                    error: "".to_owned(),
                });
            }

            let group_header_image_read = read_chunks_from_multipart_payload(
                &mut payload,
                &MAX_IMAGE_SIZE,
                Some(HashSet::from(["png"])),
            )
            .await?;

            // image_file이 없으면 이미지 위치 변경
            if group_header_image_read.is_none() {
                jogiyo_context
                    .community_context()
                    .update_user_group(
                        user_group_id,
                        self.group_name.clone(),
                        self.group_intro.clone(),
                        Some(group_header_image_info),
                    )
                    .await?;

                return Ok(UpdateUserGroupResponse {
                    error: "".to_owned(),
                });
            }

            let (image_file_name, image_data) = group_header_image_read.unwrap();
            let image_ext = std::path::Path::new(&image_file_name)
                .extension()
                .and_then(std::ffi::OsStr::to_str)
                .unwrap_or("");

            if image_ext == "" {
                return Err(ServerError::BadRequest(
                    "Please Check Image File.".to_owned(),
                ));
            }

            let group_header_image_path = create_image(
                image_data,
                image_ext,
                &std::path::Path::new(
                    &jogiyo_context
                        .config_context()
                        .get_config()
                        .user_uploaded_image_save_path,
                ),
            )?;

            let static_root_dir = jogiyo_context
                .config_context()
                .get_config()
                .static_root_dir
                .clone();

            let mut update_group_header_image_info = group_header_image_info.to_owned();
            update_group_header_image_info.image_path =
                to_serving_path(&static_root_dir, &group_header_image_path)?;

            Some(update_group_header_image_info)
        } else {
            None
        };

        // 이미지 업데이트
        jogiyo_context
            .community_context()
            .update_user_group(
                user_group_id,
                self.group_name.clone(),
                self.group_intro.clone(),
                update_group_header_image_info,
            )
            .await?;

        Ok(UpdateUserGroupResponse {
            error: "".to_owned(),
        })
    }

    fn get_request_field() -> &'static str {
        "update_user_group_request"
    }
}

// 전달 받은 정보만 업데이트한다.
async fn update_user_group(
  &self,
  group_id: i32,
  group_name: Option<String>,
  group_intro: Option<String>,
  group_header_image_info: Option<GroupHeaderImageInfo>,
) -> Result<(), ServerError> {
  let mut update_user_group = UserGroup::ActiveModel {
      user_group_id: Set(group_id),
      ..Default::default()
  };

  if let Some(group_name) = group_name {
      update_user_group.group_name = Set(group_name);
  }

  if let Some(group_intro) = group_intro {
      update_user_group.group_intro = Set(group_intro);
  }

  if let Some(group_header_image_info) = group_header_image_info {
      update_user_group.group_header_image_info =
          Set(Some(proto_message_to_u8_vec(&group_header_image_info)?));
  }

  update_user_group.update(self.database.connection()).await?;

  Ok(())
}