pub fn curriculum_metadata_model_to_proto(
  curriculum_metadata: CurriculumMetadata::Model,
) -> CurriculumMetadataData {
  CurriculumMetadataData {
      curriculum_id: curriculum_metadata.curriculum_id,
      create_time: Some(db_time_to_proto_time(curriculum_metadata.create_time)),
      author_id: curriculum_metadata.author_id,
      curriculum_title: curriculum_metadata.curriculum_title,
      curriculum_days: curriculum_metadata.curriculum_days,
      curriculum_type: curriculum_metadata.curriculum_type as i32,
      curriculum_level: curriculum_metadata.curriculum_level,
      total_num_enrolled: curriculum_metadata.total_num_enrolled,
      subject: curriculum_metadata.subject as i32,
      thumbnail: curriculum_metadata.thumbnail_path,
      curriculum_detail: Some(
          CurriculumDetail::decode(curriculum_metadata.curriculum_detail.as_slice()).unwrap(),
      ),
  }
}