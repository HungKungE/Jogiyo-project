impl UserSessionChecker for GetCurriculumDataRequest {}

#[async_trait]
impl Handler for GetCurriculumDataRequest {
    type Response = GetCurriculumDataResponse;

    async fn handle_request(
        &self,
        jogiyo_context: web::Data<Arc<dyn JogiyoContext + Send + Sync>>,
        user_info: UserInfo,
    ) -> Result<Self::Response, ServerError> {
        if !user_info.is_admin {
            return Err(ServerError::Unauthorized("Not an admin".to_owned()));
        }

        let curriculums = CurriculumMetadata::Entity::find()
            .all(jogiyo_context.database_context().connection())
            .await?;

        let curriculum_metadatas = curriculums
            .into_iter()
            .map(|curriculum| curriculum_metadata_model_to_proto(curriculum))
            .collect();

        Ok(GetCurriculumDataResponse {
            curriculum_metadatas,
            error: "".to_owned(),
        })
    }
}