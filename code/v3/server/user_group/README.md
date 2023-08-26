# User Group

- 설명 : 초대받은 사용자들끼리 피드형식으로 글을 쓰고, 댓글을 달면서 소통할 수 있는 group 서비스.

### update_user_group

그룹 정보를 수정하는 api이다.
</br>
if let 이라는 새로운 문법을 배울 수 있었다.
</br>

- user_group_url -> user_group_id -> user_group_data
- 그룹 배경 이미지를 지운다면, user_group의 image_path 경로를 비우고, 해당 파일을 로컬에서 삭제한다.
- 새 그룹 배경 이미지 파일이 있다면, 이를 multipart로 받아서 user_group image_path 및 변경사항을 update한다.
- 그리고 해당 이미지 파일을 로컬에 저장한다.
- 새 그룹 배경 이미지 파일이 없다면, user_group 변경사항을 수정하고 완료한다.

### if let

예를 들어 num_a가 Some(5) 라는 Optional 타입일 때
</br>

```sh
let num_b : String = if let Some(a) = num_a {
  if (a>=5){
    "big".to_owned()
  } else {
    "small".to_owned()
  }
} else {
    "none".to_owned()
}
```

위와 같은 문법을 사용하면 num_b는 num_a가 Some(5)이므로
</br>
num_b 값은 "big"이라는 String이 된다.
</br>
즉, if let Some문은 Optional 타입 값이 Some인지 None인지에 따라서 다른 결과를 도출하는 제어문이다.

### kick_user_from_group

특정 사용자를 그룹에서 추방하는 api이다.

### get_user_group_shared_link

user_group_id를 shared_link로 변환시킨다.
</br>
user_group_id는 유출에 민감한 데이터이기 때문에 공유해도 괜찮은 shared_link로 변환시켜 사용한다.
</br>
이는 초대링크를 만들 때, 해당 그룹의 정보를 얻기 위해 사용된다.

### get_user_group_from_shared_link

shared_link로 user_group 관련 정보를 받아온다.
</br>
해당 api는 초대링크에서 user_group의 정보를 보여주기 위해 사용한다.

### join_user_group

사용자를 그룹에 가입 시킬 때 사용한다.
</br>
원래는 가입 이후, 새로고침을 해야 해당 그룹을 볼 수 있었다.
</br>
이 점이 불편해서 가입 즉시 해당 그룹으로 이동할 수 있게
</br>
api 요청의 응답 데이터에 그룹 링크를 추가시켰다.
