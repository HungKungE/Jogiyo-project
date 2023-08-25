# school

-설명 : [전국학교 리스트](https://open.neis.go.kr/portal/data/service/selectServicePage.do?page=1&rows=10&sortColumn=&sortDirection=&infId=OPEN17020190531110010104913&infSeq=3)(공공데이터)이다.
사용자가 자신의 소속학교를 고를 때 사용하는 데이터셋이다.

csv파일 -> json으로 변환하여 사용했다.
</br>
그러나 학교 검색 관련 page를 개발할 때 client폴더에 json파일을 그대로 포함시켰더니
</br>
사용자가 학교 검색 기능을 사용하지 않아도, 웹 서비스에 접속할 때마다 json파일 데이터를 모두 로딩하므로
로딩 시간과 성능이 감소할 여지가 있었다.
</br>
이러한 문제를 해결하기 위해서 db table을 새로 만들고 학교 데이터 업로드, 학교 검색 api를 구현했다.

### get_school_metadata

입력받은 search_keyword가 이름에 포함된 학교 리스트를 최대 10개까지 전송하는 api이다.
</br>
여기서 paginate, fetch_page를 처음 사용했다.
</br>
paginate, fetch_page는 select 결과 데이터들을 특정 단위의 페이지로 나누고 특정 순서의 페이지를 가져오는 기능이다.
</br>
이를 사용하면 모든 데이터를 불러오지 않으므로 초기 로딩 속도를 향샹시킬 수 있다.

### upload_school_json

학교 리스트 json을 db에 업로드하는 api이다.
