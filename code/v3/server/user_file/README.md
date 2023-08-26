# User_File

- 설명 : 사용자에게 노트, pdf에 필기, 열람, 공유 할 수 있도록 제공하는 서비스.

### get_user_file

사용자의 모든 파일(노트, pdf) 정보를 불러온다.
</br>
사용처는 다음과 같다.
</br>

1. "나의 문서"탭의 "최근 문서"에서 사용자가 최근 접근했던 순으로 모든 파일을 불러옴.
   </br>
2. 그룹에서 파일을 포함하여 글을 작성할 때, 사용자의 모든 파일을 불러옴.
   </br>

### move_user_file

파일을 현재 위치에서, 다른 위치로 이동 시킬 때 사용하는 api이다.
</br>
request 시에 다음과 같은 정보를 서버에 전송한다.

```sh
string shared_file_url = 1;
optional string parent_directory_file_url = 2;
```

</br>
parent_directory_file_url은 파일을 이동 시킬 폴더의 id를 변환시킨 shared_link(&str)이다.
</br>
server에서는 parent_directory_file_url를 역변환시켜서 다시 폴더 id를 얻어내고, 해당 파일의 위치를 폴더 id로 수정한다.
</br>
</br>
그런데 여기서 자그마한 문제가 발생했다.
</br>
파일 삭제 기능을 구현할 때에도 DB에서 삭제하는 것이 아닌, 파일 이동 api를 사용해서 휴지통이라는 폴더로 옮기도록 구현했는데
</br>
client에서 삭제 기능을 사용하면 변환시킨 shared_link가 아닌 "trash-can" 이라는 문자열을 server로 전달했기 때믄에
</br>
server에서 역변환 시킬 때, 제대로 된 휴지통 폴더 id를 얻을 수 없어 서버 에러가 발생했다.
</br>
이런 문제를 해결하기 위해 get_file_id_from_file_shared_link함수에 shared_link == "trash-can" 일 때는 folder_id = -1을 반환하는 조건을 추가함으로써 위 문제를 해결할 수 있었다.
