impl UserSessionChecker for UpdateVocabularySetRequest {}

#[async_trait]
impl Handler for UpdateVocabularySetRequest {
    type Response = UpdateVocabularySetResponse;

    async fn handle_request(
        &self,
        jogiyo_context: web::Data<Arc<dyn JogiyoContext + Send + Sync>>,
        user_info: UserInfo,
    ) -> Result<Self::Response, ServerError> {
        if !user_info.is_admin {
            return Err(ServerError::Unauthorized("Not an admin".to_owned()));
        }

        let voca_set = self.update_vocabulary_set.clone().unwrap();

        jogiyo_context
            .vocab_context()
            .update_vocabulary_set(voca_set)
            .await?;

        Ok(UpdateVocabularySetResponse {
            error: "".to_owned(),
        })
    }
}

// activeModel로 vocabulary_set을 update한다.
async fn update_vocabulary_set(
  &self,
  vocabulary_set: VocabularySetData,
) -> Result<(), ServerError> {
  // 특정 voca_id인 db data의 voca_set 값을 변경한 activeModel을 update함.
  VocabularySet::ActiveModel {
      vocabulary_set_id: Set(vocabulary_set.vocabulary_set_id),
      vocabulary_set: Set(proto_message_to_u8_vec(&vocabulary_set)?),
  }
  .update(self.database.connection())
  .await?;

  Ok(())
}