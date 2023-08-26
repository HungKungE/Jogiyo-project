impl UserSessionChecker for GetUserFilesRequest {}

#[async_trait]
impl Handler for GetUserFilesRequest {
    type Response = GetUserFilesResponse;

    async fn handle_request(
        &self,
        jogiyo_context: web::Data<Arc<dyn JogiyoContext + Send + Sync>>,
        user_info: UserInfo,
    ) -> Result<Self::Response, ServerError> {
        let file_types = self
            .file_types
            .to_vec()
            .into_iter()
            .map(|file_type| file_type as i16)
            .collect();

        let files = jogiyo_context
            .user_file_context()
            .get_user_files(
                user_info.user_id,
                file_types,
                self.num_fetch,
                &self.latest_file_creation_time,
            )
            .await?;

        Ok(Self::Response {
            note_files: files
                .into_iter()
                .map(|file| user_file_to_renderable_user_file(file))
                .collect(),
            error: "".to_owned(),
        })
    }
}

async fn get_user_files(
  &self,
  author_id: i32,
  file_types: Vec<i16>,
  num_fetch: i32,
  latest_file_creation_time: &Option<Timestamp>,
) -> Result<Vec<UserFileData>, ServerError> {
  let latest_file_creation_time = match latest_file_creation_time {
      Some(latest_file_creation_time) => proto_time_to_db_time(latest_file_creation_time),
      None => current_kst_time(),
  };

  let user_files = UserFile::Entity::find()
      .filter(
          sea_orm::Condition::all()
              .add(UserFile::Column::AuthorId.eq(author_id))
              .add(UserFile::Column::FileType.is_in(file_types))
              .add(
                  sea_orm::Condition::any()
                      .add(UserFile::Column::ParentDirectoryFileId.ne(TRASH_CAN_ID))
                      .add(UserFile::Column::ParentDirectoryFileId.is_null()),
              )
              .add(UserFile::Column::UpdateTime.lte(latest_file_creation_time)),
      )
      .order_by_desc(UserFile::Column::UpdateTime)
      .limit(num_fetch as u64)
      .all(self.database.connection())
      .await?;

  Ok(user_files
      .into_iter()
      .map(|file| user_file_db_to_proto(file))
      .collect_vec())
}