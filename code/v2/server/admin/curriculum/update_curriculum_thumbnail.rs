// 110 MB. 수능완성 한국지리가 106MB
const MAX_IMAGE_SIZE: usize = 10 * 1024 * 1024;

#[async_trait(?Send)]
impl MultipartHandler for UpdateCurriculumThumbnailRequest {
    type Response = UpdateCurriculumThumbnailResponse;

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
        let user_info = user_info.unwrap();
        if !user_info.is_admin {
            return Err(ServerError::Unauthorized(
                "User is not authorized.".to_owned(),
            ));
        }

        let curriculum_metadata =
            get_curriculum_metadata(&jogiyo_context.curriculum_context(), self.curriculum_id)
                .await?;

        let mut thumbnail_image_path = None;
        let mut thumbnail_gif_path = None;

        if self.update_thumbnail_image {
            let thumbnail_image_read = read_chunks_from_multipart_payload(
                &mut payload,
                &MAX_IMAGE_SIZE,
                Some(HashSet::from(["png", "webp"])),
            )
            .await?;

            if thumbnail_image_read.is_none() {
                return Err(ServerError::BadRequest(
                    "Thumbnail image is missing".to_owned(),
                ));
            }

            let (image_file_name, image_data) = thumbnail_image_read.unwrap();
            let image_ext = std::path::Path::new(&image_file_name)
                .extension()
                .and_then(std::ffi::OsStr::to_str)
                .unwrap_or("");

            thumbnail_image_path = Some(create_image(
                image_data,
                image_ext,
                &std::path::Path::new(
                    &jogiyo_context
                        .config_context()
                        .get_config()
                        .workbook_thumbnail_image_output_path,
                ),
            )?);

            println!("image : {:?}", thumbnail_image_path);
        }

        if self.update_thumbnail_gif {
            let thumbnail_gif_read = read_chunks_from_multipart_payload(
                &mut payload,
                &MAX_IMAGE_SIZE,
                Some(HashSet::from(["gif"])),
            )
            .await?;

            if thumbnail_gif_read.is_none() {
                return Err(ServerError::BadRequest(
                    "Thumbnail gif is missing".to_owned(),
                ));
            }

            let (gif_file_name, gif_data) = thumbnail_gif_read.unwrap();
            let gif_ext = std::path::Path::new(&gif_file_name)
                .extension()
                .and_then(std::ffi::OsStr::to_str)
                .unwrap_or("");

            thumbnail_gif_path = Some(create_image(
                gif_data,
                gif_ext,
                &std::path::Path::new(
                    &jogiyo_context
                        .config_context()
                        .get_config()
                        .workbook_thumbnail_image_output_path,
                ),
            )?);

            println!("gif : {:?}", thumbnail_gif_path);
        }

        let static_root_dir = jogiyo_context
            .config_context()
            .get_config()
            .static_root_dir
            .clone();

        if self.update_thumbnail_image && self.update_thumbnail_gif {
            let mut curriculum_metadata = curriculum_metadata.clone();

            if let Some(thumbnail_image_path) = thumbnail_image_path {
                curriculum_metadata.thumbnail =
                    to_serving_path(&static_root_dir, &thumbnail_image_path)?;
            }

            if let Some(thumbnail_gif_path) = thumbnail_gif_path {
                curriculum_metadata.thumbnail_gif =
                    to_serving_path(&static_root_dir, &thumbnail_gif_path)?;
            }

            jogiyo_context
                .curriculum_context()
                .update_curriculum_metadata(curriculum_metadata, &user_info.user_id)
                .await?;
        }

        Ok(UpdateCurriculumThumbnailResponse {
            error: "".to_owned(),
        })
    }

    fn get_request_field() -> &'static str {
        "update_curriculum_thumbnail_request"
    }
}

async fn update_curriculum_metadata(
  &self,
  curriculum_metadata: CurriculumMetadataData,
  user_id: &i32,
) -> Result<CurriculumMetadataData, ServerError> {
  let update_metadata = CurriculumMetadata::ActiveModel {
      curriculum_id: Set(curriculum_metadata.curriculum_id),
      create_time: Set(current_kst_time()),
      curriculum_type: Set(curriculum_metadata.curriculum_type.try_into().unwrap()),
      author_id: Set(*user_id),
      curriculum_title: Set(curriculum_metadata.curriculum_title),
      curriculum_days: Set(curriculum_metadata.curriculum_days),
      curriculum_level: Set(curriculum_metadata.curriculum_level),
      total_num_enrolled: Set(curriculum_metadata.total_num_enrolled),
      subject: Set(curriculum_metadata.subject.try_into().unwrap_or(0)),
      thumbnail_path: Set(curriculum_metadata.thumbnail),
      thumbnail_gif_path: Set(curriculum_metadata.thumbnail_gif),
      curriculum_detail: Set(proto_message_to_u8_vec(
          &curriculum_metadata.curriculum_detail.unwrap_or_default(),
      )?),
      ..Default::default()
  }
  .update(self.database.connection())
  .await?;

  Ok(curriculum_metadata_model_to_proto(update_metadata))
}
