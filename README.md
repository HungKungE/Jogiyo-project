# 노트 필기 및 공유 커뮤니티

## 프로젝트 소개
![image](https://github.com/HungKungE/Jogiyo-v2/assets/84065412/33ff7a22-a540-4540-817a-5b3786429cb9)

스타트업 "모두의 코드"에서 진행한
</br>
IOS의 GoodNotes 같은 노트 필기 서비스를 제공하는 웹 사이트 개발 프로젝트입니다.
</br>
그 외에도 노트 필기를 QR코드로 공유, 다른 사용자들과 소통할 수 있는 커뮤니티 서비스를 제공합니다.
</br>
</br>
진행 기간 : 2022.10 ~ 2023.12

---

## 프로젝트 자료

- [페이지 디자인](https://www.figma.com/file/XkFjQCfJ5hLQQbOjh7gqru/Jogiyo-v2-New-design?type=design&mode=design&t=0DCZ4sSJPMo2RI8u-0)

## 주요 기능

<details>
<summary><b>나의 문서</b></summary>
<div markdown="1">
  </br>
  필기노트 파일을 폴더탐색기 형식으로 관리할 수 있는 페이지입니다.
  </br>
  문서 생성 및 삭제, 공유가 가능합니다.
  </br>
  </br>
  
  ![aaaa](https://github.com/HungKungE/Jogiyo-v3/assets/84065412/acb63efd-6630-44d6-ae2a-4e58b3ecaf79)

</div>
</details>

<details>
<summary><b>그룹 커뮤니티</b></summary>
<div markdown="1">
  </br>
  다른 사용자들과 소통할 수 있는 그룹 커뮤니티 페이지입니다.
  </br>
  "그룹"단위로 그룹내 사용자들과 피드 형식으로 소통 할 수 있습니다.
  </br>
  </br>
  
  ![12122](https://github.com/HungKungE/Jogiyo-v3/assets/84065412/2f3c01a5-8a11-4723-96bd-72715b1b8c4b)


</div>
</details>

<details>
<summary><b>노트 필기</b></summary>
<div markdown="1">
  </br>
  PDF를 불러오거나, 새 노트를 만들어서 그 위에 필기할 수 있습니다.
  </br>
  </br>
  
![캡처](https://github.com/HungKungE/Jogiyo-v3/assets/84065412/18325dfa-243c-4338-bab7-5b5529109291)

</div>
</details>

## 나의 개발 기능

<details>
<summary><b>나의 문서</b></summary>
<div markdown="1">
  </br>
  노트 파일을 정리, 확인, 생성, 수정, 삭제할수 있는 페이지입니다.
  </br>
  이 중에서 노트 추가 폼 디자인, PDF/Note 구분 기능, 노트 삭제(api)를 개발했습니다.
  </br>
  
### 사용 skills
  <div>
    <img src="https://img.shields.io/badge/react-61DAFB?style=for-the-badge&logo=react&logoColor=black">
    <img src="https://img.shields.io/badge/Typescript-3178C6?style=for-the-badge&logo=typescript&logoColor=white">
    <img src="https://img.shields.io/badge/tailwindcss-F7DF1E?style=for-the-badge&logo=tailwindcss&logoColor=white">
        <img src="https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white">
  </div>

### 페이지 디자인
  ![aaaa](https://github.com/HungKungE/Jogiyo-v3/assets/84065412/acb63efd-6630-44d6-ae2a-4e58b3ecaf79)

</div>
</details>

<details>
<summary><b>그룹 커뮤니티</b></summary>
<div markdown="1">
  </br>
  다른 사용자들과 소통할 수 있는 그룹 커뮤니티 페이지입니다.
  </br>
  이 중에서 그룹 초대 , 추방, 정보 수정 기능을 구현했습니다.
  </br>
  
### 사용 skills
  <div>
    <img src="https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white">
  </div>

</div>
</details>

<details>
<summary><b>admin 페이지</b></summary>
<div markdown="1">
  </br>
    전반적인 admin 페이지 개발을 맡았습니다.
  </br>
    최근 로그인한 사용자 목록 확인, 사용자 별 필기 노트 내용 확인 기능을 구현했습니다.
  </br>
  
  
### 사용 skills
  <div>
    <img src="https://img.shields.io/badge/react-61DAFB?style=for-the-badge&logo=react&logoColor=black">
    <img src="https://img.shields.io/badge/Typescript-3178C6?style=for-the-badge&logo=typescript&logoColor=white">
    <img src="https://img.shields.io/badge/tailwindcss-F7DF1E?style=for-the-badge&logo=tailwindcss&logoColor=white">
    <img src="https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white">
  </div>
</div>
</details>

## 프로젝트 후기
이번 프로젝트를 진행하면서 react 사용법과 더불어
</br>
여러 api와 DB table을 직접 개발하면서 rust 라는 프로그래밍 언어와 친숙해질 수 있었다.
</br>
처음 사용했을 때는 Ownership 개념, match, some, none등의 문법이 내가 알고 있던 c, c++, python, js등의 문법과 달라서 개발 도중에 많이 질문했다.
</br>
이전 프로젝트에서는 단순하게 DB에 접근해서 바로 그정보를 client로 전송하는 api들을 개발했었는데,
이번 프로젝트에서는 여러 조건문을 통해서 필요한 정보만 골라내서 전송, 수정, 삭제하는 api를 개발할 수 있어 좋은 경험이 되었던 것 같다.
