// 학교 정보 Vec 최대 크기
static MAX_NUM_SEARCHED_SCHOOL : usize = 10;

impl UserSessionChecker for GetSchoolMetadatasRequest {}

#[async_trait]
impl Handler for GetSchoolMetadatasRequest {
    type Response = GetSchoolMetadatasResponse;

    async fn handle_request(
        &self,
        jogiyo_context: web::Data<Arc<dyn JogiyoContext + Send + Sync>>,
        _user_info: UserInfo,
    ) -> Result<Self::Response, ServerError> {
        let school_metadatas = SchoolInfo::Entity::find()
            .filter(SchoolInfo::Column::SchoolName.contains(&self.search_keyword))
            .order_by_asc(SchoolInfo::Column::SchoolName)
            .paginate(jogiyo_context.database_context().connection(), MAX_NUM_SEARCHED_SCHOOL)
            .fetch_page(0)
            .await?;

        let school_metadatas = school_metadatas
            .into_iter()
            .map(|school_metadata| school_db_to_proto(school_metadata))
            .collect();

        Ok(GetSchoolMetadatasResponse {
            school_metadatas,
            error: "".to_owned(),
        })
    }
}