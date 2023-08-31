## v2 : 태블릿 전용 문제집 웹 사이트

![캡처1](https://github.com/HungKungE/Jogiyo-project/assets/84065412/b547c66e-f9b2-4abc-8ee0-4da0c7089dc2)

스타트업 "모두의 코드"에서 진행한
</br>
태블릿의 터치펜을 사용해서 문제집을 풀고 채점 하는 사이트 개발 프로젝트입니다.
</br>
전반적으로 학습에 도움을 줄 수 있는 기능을 담은 것이 특징입니다.
</br>
</br>
진행 기간 : 2022.08 ~ 2022.12
</br>

## 프로젝트 자료 :books:

:notebook_with_decorative_cover: [서비스 디자인 문서](https://docs.google.com/document/d/1sjdeP5pTJ3hGAOYfTTb8Vf2y_o755gf8aqhGPMYWu-M/edit#heading=h.7fet0jbsxwh6)

:pencil: [페이지 디자인](https://www.figma.com/file/Uj6PNk5oF0ZyyrbG5a6KxG/Jogiyo-v2-UI?type=design&node-id=0-1&mode=design&t=0DCZ4sSJPMo2RI8u-0)
</br>

## 주요 기능

<details>
<summary><b>나의 문제집</b></summary>
<div markdown="1">
  </br>
  
  > 내가 풀었던 문제집들을 확인하고 이어서 풀거나 오답노트 확인할 수 있다.
  
  </br>
  
![myStudyRoom1](https://github.com/HungKungE/Jogiyo-project/assets/84065412/c7a5975f-d7b3-484d-be6e-6e3e1ad56c3d)
![myStudyRoom2](https://github.com/HungKungE/Jogiyo-project/assets/84065412/77a8d68a-d57c-405e-a570-d3c22ba775fb)

</div>
</details>

<details>
<summary><b>문제집 검색</b></summary>
<div markdown="1">
  </br>
  
  > 여러 조건을 통해서 문제집을 검색할 수 있다.
  
  </br>

![믄제집찾기](https://github.com/HungKungE/Jogiyo-project/assets/84065412/0827d638-1840-46c3-aad0-64f7ce5e8186)

</div>
</details>

<details>
<summary><b>문제 풀기</b></summary>
<div markdown="1">
  </br>
  
  > 온라인으로 문제집을 풀면서 채점할 수 있다.
  
  </br>
  
![풀기](https://github.com/HungKungE/Jogiyo-project/assets/84065412/3e4792ec-5cba-4543-959a-1c54d26f4442)
![풀기2](https://github.com/HungKungE/Jogiyo-project/assets/84065412/ec26c63d-8be4-42fa-aab7-ce8f2d284935)


</div>
</details>

## 나의 개발 기능

### ◽client

<details>
<summary><b>사용자 정보 입력 페이지</b></summary>
<div markdown="1">
  </br>
  회원 가입 시, 사용자의 정보를 입력하는 페이지 디자인과 구현을 맡았다.
  </br>
  사용자의 닉네임, 학년, 학교 정보를 입력받는다.
  </br>
  

  </br>
  
  > 사용 skills
  <div>
    <img src="https://img.shields.io/badge/react-61DAFB?style=for-the-badge&logo=react&logoColor=black">
    <img src="https://img.shields.io/badge/Typescript-3178C6?style=for-the-badge&logo=typescript&logoColor=white">
    <img src="https://img.shields.io/badge/tailwindcss-F7DF1E?style=for-the-badge&logo=tailwindcss&logoColor=white">
  </div>

> 페이지 디자인

![정보입력](https://github.com/HungKungE/Jogiyo-project/assets/84065412/073e97c9-7237-4826-b716-dbbe8785ff23)

</div>
</details>

<details>
<summary><b>문제집 검색 기능 구현</b></summary>
<div markdown="1">
  </br>
  
  문제집 이름, 문제집 태그를 통해서 문제집을 검색하는 기능을 구현했다.
  
  

  </br>
  
  > 사용 skills
  <div>
    <img src="https://img.shields.io/badge/react-61DAFB?style=for-the-badge&logo=react&logoColor=black">
    <img src="https://img.shields.io/badge/Typescript-3178C6?style=for-the-badge&logo=typescript&logoColor=white">
    <img src="https://img.shields.io/badge/tailwindcss-F7DF1E?style=for-the-badge&logo=tailwindcss&logoColor=white">
  </div>

> 페이지 디자인

![믄제집찾기](https://github.com/HungKungE/Jogiyo-project/assets/84065412/e36833ba-e7dc-4195-a822-968ba95a61c2)

</div>
</details>

### ◽server

<details>
<summary><b>curriculum api</b></summary>
<div markdown="1">
  </br>
  
  > 영어단어 암기 커리큘럼 서비스 관련 admin api를 구현했다.
  
  
  - [curriculum 코드 확인](https://github.com/HungKungE/Jogiyo-v3/tree/main/code/v2/server/curriculum)

  | 종류 | api | 개발 내용 |
  | ----- | ----- | ----- |
  | 구현 | get_curriculum_metadata  | 직렬화 된 curriculum data를 역직렬화하여 admin page에서 열람하는 api를 구현했다. |
  | 구현 | update_curriculum_thumbnail  | curriculum thumbnail image를 업데이트하는 api이다. |
  

  </br>
  
  > 사용 skills
  <div>
    <img src="https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white">
    <img src="https://img.shields.io/badge/sea_orm-000000?style=for-the-badge&logo=rust&logoColor=white">
    <img src="https://img.shields.io/badge/Multipart-000000?style=for-the-badge&logo=rust&logoColor=white">
  </div>
</div>
</details>

<details>
<summary><b>vocabulary api</b></summary>
<div markdown="1">
  </br>
  
  > curriculum에서 사용하는 1일 치 단어들의 정보를 가져오는 api를 구현했다. 
  

   - [vocabulary 코드 확인](https://github.com/HungKungE/Jogiyo-v3/tree/main/code/v2/server/voca)
  
  | 종류 | api | 개발 내용 |
  | ----- | ----- | ----- |
  | 구현 | update_vocabulary_set | 단어장 내용을 수정하는 api를 구현했다. |
  </br>
  
  > 사용 skills
  <div>
    <img src="https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white">
    <img src="https://img.shields.io/badge/sea_orm-000000?style=for-the-badge&logo=rust&logoColor=white">
    <img src="https://img.shields.io/badge/ActiveModel-000000?style=for-the-badge&logo=rust&logoColor=white">
  </div>
</div>
</details>

<details>
<summary><b>workbook & user api</b></summary>
<div markdown="1">
  </br>
  
  > 사용자에게 제공하는 문제집 관련 데이터와 사용자의 정보를 가져오는 api를 구현했다. 
  

  - [workbook 코드 확인](https://github.com/HungKungE/Jogiyo-v3/tree/main/code/v2/server/workbook)
  - [user 코드 확인](https://github.com/HungKungE/Jogiyo-v3/tree/main/code/v2/server/user)

  | 종류 | api | 개발 내용 |
  | ----- | ----- | ----- |
  | 구현 | get_user_info | 모든 user_info data를 가져오는 api를 구현했다. |
  | 구현 | get_user_data  | 문제집 데이터인 workbook data와, 사용자의 문제집 진도 정보 user_generated_workbook data를 가져오는 api를 구현했다. |
  | 구현 | get_user_workbook_page | 특정 사용자의 문제집 특정 페이지 필기 데이터를 가져오는 api를 구현했다. |

  </br>
  
  > 사용 skills
  <div>
    <img src="https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white">
    <img src="https://img.shields.io/badge/sea_orm-000000?style=for-the-badge&logo=rust&logoColor=white">
  </div>
</div>
</details>

<details>
<summary><b>school api</b></summary>
<div markdown="1">
  </br>
  
  > 공공데이터인 전국 학교 리스트 관련 api를 구현했다. 
  
  
  - [school 코드 확인](https://github.com/HungKungE/Jogiyo-v3/tree/main/code/v2/server/school)

  | 종류 | api | 개발 내용 |
  | ----- | ----- | ----- |
  | 구현 | get_school_metadata  | 입력받은 search_keyword가 포함된 학교 리스트를 전송하는 api를 구현했다. |
  | 구현 | upload_school_json  | 학교 리스트 json을 db에 업로드하는 api이다. |
  
  </br>
  
  > 사용 skills
  <div>
    <img src="https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white">
    <img src="https://img.shields.io/badge/sea_orm-000000?style=for-the-badge&logo=rust&logoColor=white">
    <img src="https://img.shields.io/badge/paginate-000000?style=for-the-badge&logo=rust&logoColor=white">
    <img src="https://img.shields.io/badge/ActiveModel-000000?style=for-the-badge&logo=rust&logoColor=white">
  </div>
</div>
</details>
</br>

## 프로젝트 후기

3학년 1학기가 끝난 여름방학에 운 좋게 인연이 닿아 2학년 때 팀플을 같이 했던 선배가 창업한 스타트업 프로젝트에 참여하게 되었다.
</br>
그 동안, 학교에서는 c, c++, c#, bash 등을 사용한 로컬 프로그램 개발 프로젝트만 경험해봐서 웹 서비스 개발 프로젝트는 처음이었다. 
</br>
프로젝트에 참여할 때는 Front로 참여해서 회원 정보 입력 page 개발을 맡았었는데, 페이지 디자인에서 시간을 많이 잡아먹어서
</br>
거의 한 달동안은 디자인, 페이지 개발에 주력했던 것 같다.
</br>
이 이후로도 여러 번 페이지 디자인을 해봤으나, 사이트 스타일에 어울리는 페이지 디자인에 큰 어려움을 느꼈고
</br>
디자인 센스가 크게 모자르다는 점을 알게 되었다. 이 부분은 단기간에 상승시킬 수 없는 능력이므로 9월 달 쯤부터 Back-End로 눈길을 돌리게 되었다.
</br>
Back-End는 api의 기능의 구현 능력이 필요함을 느꼈고, 이는 그 동안 해왔던 학교 과제들과 유사성을 느낄 수 있어서 좀더 마음이 갔던 것 같다.
</br>
프로젝트 진행할 때는 Front-End와 Back-End를 병행하면서 참여했지만, 마음 속으로는 점점 Back-End 개발자로 진로를 결정하게 되었다.
</br>
그런데, 우리 서버 코드가 조금 특이했다. 한국에서 많이 사용하는 java, js, python이 아닌 rust라는 생소한 언어였다.
</br>
네이버에 검색해도 게임 rust가 먼저 검색되는 이상한 언어. 그것이 rust의 첫 인상이였다.
</br>
</br>
이후로 rust server에 api 코드를 추가하기 위해 공부를 하면서 보통 이상한 언어가 아님을 점점 알게 되었다.
</br>
가장 놀랐던 건 소유권 개념이었다.
</br>
let A : i16 = 3 이라는 변수가 있을 때, let B = A 라고 하면
</br>
변수 A는 더 이상 유효하지 않은 변수가 되고, 그 값들이 변수 B로 이동한다.
</br>
그리고 if, for, match, some 등의 문법도 알고 있는 문법과 조금 달라서 익숙해지기 까지 시간이 걸렸던 것 같다.
</br>
</br>
어느 정도 rust에 익숙해졌을 때 쯤, server api를 추가하기 위해 server코드를 보았다.
</br>
놀랍게도 코드들이 이해되지 않았다. 문법을 안다고 해서 서버 코드들이 어떻게 동작하는지 파악할 수가 없었다.
</br>
그래서 rust server 개발자분께 속성과외를 받아 api추가 방법을 배우게 되었다.
</br>
여러 시행착오가 끝나고 처음으로 만든 api가 동작했을 때 달성감을 느낄 수 있었다.
</br>
비록 프로젝트 결과가 좋지 못했지만, 웹 서비스를 다른 개발자들과 함께 개발하는 경험을 할 수 있어서 좋았다.

## Problem & Solution
| 문제점 | 해결책 |
| ----- | ----- |
| 학교 목록 검색 기능을 구현하기 위해서 전국 학교 목록 json을 client폴더에 두었더니, 빌드 파일의 용량이 크게 늘어났다. | 학교 목록 json 데이터를 db로 옮기는 api와 학교 검색 api를 구현하여 해결했다. 이를 통해서 웹 페이지의 초기 로딩시간을 줄일 수 있었다. |
