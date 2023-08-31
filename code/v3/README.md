## v3 : 노트 기반 질의응답 플랫폼

![캡처1](https://github.com/HungKungE/Jogiyo-project/assets/84065412/e0a4229a-8d07-4d5e-9005-2a8458541a23)

스타트업 "모두의 코드"에서 진행한
</br>
IOS의 GoodNotes 같은 노트 필기 서비스를 무료로 제공하는 웹 애플리케이션 개발 프로젝트입니다.
</br>
그 외에도 노트 필기를 QR코드로 공유, 다른 사용자들과 질의응답 및 소통을 할 수 있는 커뮤니티 서비스를 제공합니다.
</br>
</br>
진행 기간 : 2023.01 ~ 2023.03

---

## 프로젝트 자료

:pencil: [페이지 디자인](https://www.figma.com/file/XkFjQCfJ5hLQQbOjh7gqru/Jogiyo-v2-New-design?type=design&mode=design&t=0DCZ4sSJPMo2RI8u-0)

## 주요 기능

<details>
<summary><b>나의 문서</b></summary>
<div markdown="1">
  </br>
  
  > 필기노트 파일을 폴더탐색기 형식으로 관리할 수 있는 페이지입니다. 문서 생성 및 삭제, 공유가 가능합니다.

  </br>
  
  ![aaaa](https://github.com/HungKungE/Jogiyo-v3/assets/84065412/acb63efd-6630-44d6-ae2a-4e58b3ecaf79)

</div>
</details>

<details>
<summary><b>그룹 커뮤니티</b></summary>
<div markdown="1">
  </br>
  
  > 다른 사용자들과 소통할 수 있는 그룹 커뮤니티 페이지입니다. "그룹"단위로 그룹내 사용자들과 피드 형식으로 소통 할 수 있습니다.

  </br>
  
  ![12122](https://github.com/HungKungE/Jogiyo-v3/assets/84065412/2f3c01a5-8a11-4723-96bd-72715b1b8c4b)


</div>
</details>

<details>
<summary><b>노트 필기</b></summary>
<div markdown="1">
  </br>
  
  > PDF를 불러오거나, 새 노트를 만들어서 그 위에 필기할 수 있습니다.
  
  </br>
  
![캡처](https://github.com/HungKungE/Jogiyo-v3/assets/84065412/18325dfa-243c-4338-bab7-5b5529109291)

</div>
</details>

## 나의 개발 기능

### ◽client

<details>
<summary><b>Hd note</b></summary>
<div markdown="1">
  </br>
  
  - [코드 확인](https://github.com/HungKungE/Jogiyo-project/blob/main/code/v3/client/DownloadNoteAsHighQualityPDF.ts)
  
  > pdf-lib 모듈을 사용하여 업로드했던 pdf를 다시 다운로드했을 때, 확대하면 필기 내용의 해상도가 깨지는 문제를 해결했다.
  
  </br>

  pdf를 다운 받을 때 다음과 같은 로직을 사용했다.

  </br>

 1. pdf와, pdf 페이지 별 필기 정보를 불러온다.
 2. pdf를 페이지 별로 쪼개서 해당 페이지의 필기 정보와 함께 html tag에 그린다.
 3. 해당 html tag를 img로 변환시킨다.
 4. 변환시킨 img들을 합쳐 하나의 pdf로 만든다.
 5. 해당 pdf를 다운로드한다.
  
  </br>
  일반적으로 pdf의 데이터는 svg같은 벡터 그래픽 형식이므로 확대해도 고해상도가 유지된다.
  </br>
  그러나 해당 페이지 데이터 자체를 백터 그래픽 형식이 아닌 img로 변환시켜버려서 확대 시 화질이 떨어지는 현상이 발생했다.
  </br>
  </br>
  이를 해결하기 위한 방법을 찾던 중에, 고화질 pdf 관련 모듈인 pdf-lib을 알게 되었다.
  </br>
  
  [pdf-lib 공식문서](https://pdf-lib.js.org/)의 Embed PNG and JPEG Images를 참고해서  
  필기 내용을 pdf 위에 직접 추가함으로써 필기 내용을 PDF에 벡터 데이터로 저장할 수 있게 되어 해당 화질 문제를 해결 할 수 있었다.

  
### 사용 skills
  <div>
    <img src="https://img.shields.io/badge/react-61DAFB?style=for-the-badge&logo=react&logoColor=black">
    <img src="https://img.shields.io/badge/Typescript-3178C6?style=for-the-badge&logo=typescript&logoColor=white">
    <img src="https://img.shields.io/badge/pdf_lib-F7DF1E?style=for-the-badge&logo=react&logoColor=white">
  </div>

</div>
</details>

### ◽server

<details>
<summary><b>user file api</b></summary>
<div markdown="1">
  </br>
  
  > 사용자의 필기 노트 관련 api를 개발하거나, 불편한 점을 수정했다.

  - [user_file 코드 및 세부 내용](https://github.com/HungKungE/Jogiyo-v3/tree/main/code/v3/server/user_file)

  | 종류 | api | 개발 내용 |
  | ----- | ----- | ----- |
  | 구현 | get_user_file  | 사용자의 모든 파일 정보를 불러오는 api를 구현했다. |
  | 수정 | move_user_file  | 삭제 기능으로 사용할 때, 발생한 버그를 수정했다. |

  
  </br>
  
### 사용 skills
  <div>
    <img src="https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white">
    <img src="https://img.shields.io/badge/sea_orm-000000?style=for-the-badge&logo=rust&logoColor=white">  
  </div>

</div>
</details>

<details>
<summary><b>user group api</b></summary>
<div markdown="1">
  </br>
  
  > 다른 사용자들과 소통할 수 있는 그룹 커뮤니티 관련 api를 개발하거나, 문제점을 해결했다.
  
  - [user group 코드 및 세부 내용](https://github.com/HungKungE/Jogiyo-v3/tree/main/code/v3/server/user_group)

  | 종류 | api | 개발 내용 |
  | ----- | ----- | ----- |
  | 구현 | update_user_group  | 그룹 정보를 수정하는 api를 구현했다. |
  | 구현 | kick_user_from_group | 그룹 내의 특정 사용자를 추방하는 api를 구현했다.|
  | 구현 | get_user_group_shared_link  | 그룹 id를 변환시킨 shared_link를 받아오는 api를 구현했다. |
  | 구현 | get_user_group_from_shared_link  | shared_link를 통해서 user_group data를 받아오는 api를 구현했다. |
  | 수정 | join_user_group ( 그룹 가입 api )  | 그룹 가입이 성공했을 때, 새로고침 없이 해당 그룹으로 바로 이동할 수 있게 수정했다. |
  </br>
  
### 사용 skills
  <div>
    <img src="https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white">
    <img src="https://img.shields.io/badge/sea_orm-000000?style=for-the-badge&logo=rust&logoColor=white">
    <img src="https://img.shields.io/badge/ActiveModel-000000?style=for-the-badge&logo=rust&logoColor=white">
  </div>

</div>
</details>


## 프로젝트 후기

[이전 프로젝트](https://github.com/HungKungE/Jogiyo-v3/tree/main/code/v2)의 핵심 기술인 브라우저 화면에서 그림을 그릴 수 있는 Note 기능을 활용하여
</br>
[GoodNotes](https://www.goodnotes.com/kr) 같은 필기 기능을 무료로 제공하고 노트 필기를 다른 사용자들과 쉽게 공유하는 커뮤니티 웹 애플리케이션 프로젝트를 진행했다.
</br>
이전 프로젝트에서는 처음 Back-End개발을 하는 것이었기 때문에 사용자들에게 서비스를 제공하는 api가 아닌, admin들이 사용하는 테스트 성 api를 개발했었으나,
</br>
이번 프로젝트에서는 사용자들에게 서비스를 제공하는 api를 개발할 수 있게 되어 개인적으로 뿌듯함을 느낄 수 있었다.
</br>
또한, 다른 개발자가 개발한 코드에서 발생한 문제나 불편했던 점을 해결하는 등의 성취를 통해 이전 프로젝트를 통해서 성장했음을 알 수 있었다.
</br>
그런데 내가 보기에는 잘 만든 웹 서비스인데, 무슨 이유에서인지 사업은 잘 되지 않아서 가슴 아팠다.
</br>
역시 사업은 공대생들끼리 하기에는 쉽지 않은 것 같다. 창업은 나중에 경험과 인맥이 어느 정도 쌓이고 나서 해야할 것 같다고 느꼈다.

## Problemb & Solution
| 문제점 | 해결책 |
| ----- | ----- |
| 그룹 가입 성공 시, 사용자가 새로 고침을 해야 해당 그룹을 확인할 수 있었다. | api를 수정하여 가입 성공 시, 바로 해당 그룹으로 넘어갈 수 있게 만들었다. |
| 파일 이동 api를 삭제 기능으로 사용할 때, 제대로 삭제 되지 않는 문제가 있었다. | shared_link 역변환 함수에 추가 조건을 부여하여 해결했다. |
| 사이트에서 다운로드 받은 노트 필기 pdf를 확대하면 화질이 깨지는 현상이 있었다. | pdf-lib 모듈을 사용하여 pdf위에 직접 img를 그림으로써 해결했다. |
