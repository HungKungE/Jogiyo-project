impl UserSessionChecker for GetUserWorkbookPageMetadatasRequest {}

#[async_trait]
impl Handler for GetUserWorkbookPageMetadatasRequest {
    type Response = GetUserWorkbookPageMetadatasResponse;

    async fn handle_request(
        &self,
        jogiyo_context: web::Data<Arc<dyn JogiyoContext + Send + Sync>>,
        user_info: UserInfo,
    ) -> Result<Self::Response, ServerError> {
        if !user_info.is_admin {
            return Err(ServerError::Unauthorized("Not an admin".to_owned()));
        }

        let user_workbook_pages = UserWorkbookPage::Entity::find()
            .filter(UserWorkbookPage::Column::UserId.eq(self.user_id))
            .all(jogiyo_context.database_context().connection())
            .await?;

        let user_workbook_pages = user_workbook_pages
            .into_iter()
            .map(|user_workbook_page| user_workbook_page_model_to_proto(user_workbook_page))
            .collect();

        Ok(GetUserWorkbookPageMetadatasResponse {
            user_workbook_pages,
            error: "".to_owned(),
        })
    }
}