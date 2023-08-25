# workbook

-설명 : 사용자에게 제공하는 문제집 서비스.

### get_user_data

rust를 사용해서 처음 만들었던 db select api이다.
</br>
문제집 데이터인 workbook data와, 사용자의 문제집 진도 정보 user_generated_workbook data를 가져온다.
</br>
이를 통해서 admin page에서 사용자들의 문제집 진도를 한번에 열람할 수 있다.

### get_user_info

모든 사용자 db data를 가져오는 api이다.
</br>
이 api는 밑의 get_user_workbook_page api와 함께 사용하여
<br>
사용자 별 문제집 각 page에 필기한 내용을 확인할 수 있다.

### get_user_workbook_page

특정 사용자의 문제집 특정 페이지 필기 데이터를 불러온다.
</br>
이를 통해서 사용자 별 문제집 각 page에 필기한 내용을 확인할 수 있다.

# curriculum

-설명 : jogiyo v2 - 태블릿 문제집 사이트에서 제공하던 영어단어 암기 커리큘럼 서비스.

### get_curriculum_metadata

curriculum data는 직렬화되어 bytea형태로 db에 저장되어 있기 때문에 터미널에서 db데이터를 확인하기 힘들다.
</br>
따라서 curriculum data를 눈으로 확인할 수 있는 형으로 변환시켜 admin page에서 확인할 수 있도록 해주는 api가 필요했다.
</br>

- 요청을 보낸 유저가 admin인지 확인한다.
- 커리큘럼 DB data를 모두 select한다.
- 해당 data를 proto로 변환한다. 이 때 bytea형 데이터가 눈으로 확인가능한 데이터로 역직렬화된다.
- proto로 변환한 data를 client에 전송한다.

### curriculum_model_to_proto

curriculum db data model을 proto형으로 바꿔주는 함수이다.

### update_curriculum_thumbnail

curriculum에 썸네일 이미지가 추가되었기 때문에 만든 이미지 update api이다.
</br>
문자열 data를 통한 db data update는 해봤지만 image file 전송은 처음이었다.
</br>
file 전송은 multipart를 사용해서 구현했다.
</br>
multipart란 문자열, 파일 및 여러 타입의 데이터를 담은 form을 가진 http 프로토콜 body를 여러부분으로 나눠서 전송하는 방법이다.
</br>
multipart는 한 body에 여러 데이터가 있을 때, 이를 구분하기 위해 사용한다.
</br>

# vocabulary

-설명 : curriculum에서 사용하는 1일 치 단어들의 정보.

### update_vocabulary_set

단어장 내용을 수정하는 api이다.
</br>
이 때 처음으로 activeModel을 사용해서 update하는 방법을 배웠다.
</br>

### activeModel?

- update는 특정 조건의 db data model을 select하고, 그 model data를 수정하고, db에 수정된 model data를 업데이트하는 3개의 과정을 거친다.
- 그러나 activeModel은 임의로 특정 조건을 만족하는 db model을 생성한다. 생성된 model은 이미 변경된 데이터를 가지고 있다.
- 그리고 이런 activeModel을 db에 업데이트하면 특정 조건에 만족하는 db data가 바뀐 데이터로 수정된다.
