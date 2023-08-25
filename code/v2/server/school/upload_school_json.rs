impl UserSessionChecker for UploadSchoolJsonRequest {}

#[async_trait]
impl Handler for UploadSchoolJsonRequest {
    type Response = UploadSchoolJsonResponse;

    fn validate_request(&self) -> Result<(), ServerError> {
        if self.school_metadatas.is_empty() {
            return Err(ServerError::BadRequest(
                "Please Check Json File Again.".to_owned(),
            ));
        }

        Ok(())
    }

    async fn handle_request(
        &self,
        jogiyo_context: web::Data<Arc<dyn JogiyoContext + Send + Sync>>,
        user_info: UserInfo,
    ) -> Result<Self::Response, ServerError> {
        if !user_info.is_admin {
            return Err(ServerError::Unauthorized("Not an admin".to_owned()));
        }

        let upload_school_metadatas = self
            .school_metadatas
            .iter()
            .map(|school_metadata| SchoolInfo::ActiveModel {
                school_id: Set(school_metadata.school_id.to_owned()),
                school_name: Set(school_metadata.school_name.to_owned()),
                old_address: Set(school_metadata.old_address.to_owned()),
                new_address: Set(school_metadata.new_address.to_owned()),
                ..Default::default()
            })
            .collect::<Vec<SchoolInfo::ActiveModel>>();

        SchoolInfo::Entity::insert_many(upload_school_metadatas)
            .exec(jogiyo_context.database_context().connection())
            .await?;

        Ok(Self::Response {
            error: "".to_owned(),
        })
    }
}