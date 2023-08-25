impl UserSessionChecker for GetUserDataRequest {}

#[async_trait]
impl Handler for GetUserDataRequest {
    type Response = GetUserDataResponse;

    async fn handle_request(
        &self,
        jogiyo_context: web::Data<Arc<dyn JogiyoContext + Send + Sync>>,
        user_info: UserInfo,
    ) -> Result<Self::Response, ServerError> {
        if !user_info.is_admin {
            return Err(ServerError::Unauthorized("Not an admin".to_owned()));
        }

        let user_workbooks = UserWorkbookMetadata::Entity::find()
            .filter(UserWorkbookMetadata::Column::UserId.eq(self.user_id))
            .all(jogiyo_context.database_context().connection())
            .await?;

        let user_workbook_metadatas = user_workbooks
            .into_iter()
            .map(|user_workbook| user_workbook_model_to_proto(user_workbook))
            .collect();

        // author_id가 user_id인 document를 가져옴
        let document_metadatas = DocumentMetadata::Entity::find()
            .filter(DocumentMetadata::Column::AuthorId.eq(self.user_id))
            .all(jogiyo_context.database_context().connection())
            .await?;

        let document_ids = document_metadatas
            .iter()
            .map(|doc| doc.document_id)
            .collect::<Vec<i32>>();

        // document_id가 위에서 긁어온 document_metadata와 같은 user_generated_workbook를 긁어옴
        let user_generated_workbook = UserGeneratedWorkbookMetadata::Entity::find()
            .filter(UserGeneratedWorkbookMetadata::Column::DocumentId.is_in(document_ids))
            .all(jogiyo_context.database_context().connection())
            .await?;

        let user_generated_workbook_metadatas = user_generated_workbook
            .into_iter()
            .map(|user_generated| user_generated_workbook_model_to_proto(user_generated))
            .collect();

        Ok(GetUserDataResponse {
            user_workbook_metadatas,
            user_generated_workbook_metadatas,
            error: "".to_owned(),
        })
    }
}