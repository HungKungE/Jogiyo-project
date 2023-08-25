# v2 : 태블릿 전용 문제집 웹 사이트

## 프로젝트 소개

![image](https://github.com/HungKungE/Jogiyo-v2/assets/84065412/33ff7a22-a540-4540-817a-5b3786429cb9)

스타트업 "모두의 코드"에서 진행한
</br>
태블릿의 터치펜을 사용해서 문제집을 풀고 채점 하는 사이트 개발 프로젝트입니다.
</br>
전반적으로 학습에 도움을 줄 수 있는 기능을 담은 것이 특징입니다.
</br>
</br>
진행 기간 : 2022.08 ~ 2022.12
</br>

---

## 프로젝트 자료

- [서비스 디자인 문서](https://docs.google.com/document/d/1sjdeP5pTJ3hGAOYfTTb8Vf2y_o755gf8aqhGPMYWu-M/edit#heading=h.7fet0jbsxwh6)

</br>

- [페이지 디자인](https://www.figma.com/file/Uj6PNk5oF0ZyyrbG5a6KxG/Jogiyo-v2-UI?type=design&node-id=0-1&mode=design&t=0DCZ4sSJPMo2RI8u-0)
</br>

## 주요 기능

<details>
<summary><b>나의 문제집</b></summary>
<div markdown="1">
  </br>
  내가 풀었던 문제집들을 확인하고 이어서 풀거나 오답노트 확인할 수 있다.
  </br>
  
  ![myStudyRoom1](https://github.com/HungKungE/Jogiyo-v2/assets/84065412/26154547-6306-4c09-a1f2-41a15a75cb7c)
![myStudyRoom2](https://github.com/HungKungE/Jogiyo-v2/assets/84065412/86104f46-5aaa-4199-954e-9ede14f43a47)

</div>
</details>
</br>
<details>
<summary><b>문제집 검색</b></summary>
<div markdown="1">
  </br>
  여러 조건을 통해서 문제집을 검색할 수 있다.
  </br>
  
  ![믄제집찾기](https://github.com/HungKungE/Jogiyo-v2/assets/84065412/de75c94c-bc7a-4735-b3ed-c0457e96f82e)

</div>
</details>
</br>
<details>
<summary><b>문제 풀기</b></summary>
<div markdown="1">
  </br>
  온라인으로 문제집을 풀면서 채점할 수 있다.
  </br>
  
  ![풀기](https://github.com/HungKungE/Jogiyo-v2/assets/84065412/ff15dc56-833e-4786-b4a4-ed6edf837369)
![풀기2](https://github.com/HungKungE/Jogiyo-v2/assets/84065412/e73a4d5a-2631-44ed-a176-f6b2c87e45bf)

</div>
</details>
</br>

## 나의 개발 기능

### client

<details>
<summary><b>사용자 정보 입력 페이지</b></summary>
<div markdown="1">
  </br>
  회원 가입 시, 사용자의 정보를 입력하는 페이지 디자인과 구현을 맡았다.
  </br>
  사용자의 닉네임, 학년, 학교 정보를 입력받는다.
  </br>
  
### 사용 skills
  <div>
    <img src="https://img.shields.io/badge/react-61DAFB?style=for-the-badge&logo=react&logoColor=black">
    <img src="https://img.shields.io/badge/Typescript-3178C6?style=for-the-badge&logo=typescript&logoColor=white">
    <img src="https://img.shields.io/badge/tailwindcss-F7DF1E?style=for-the-badge&logo=tailwindcss&logoColor=white">
  </div>

### 페이지 디자인

![정보입력](https://github.com/HungKungE/Jogiyo-v2/assets/84065412/fec1bed9-2f70-403c-b3a6-f403d5a9499a)

</div>
</details>

<details>
<summary><b>문제집 검색 기능 구현</b></summary>
<div markdown="1">
  </br>
  문제집 이름, 문제집 태그를 통해서 문제집을 검색하는 기능을 구현했다.
  </br>
  
### 사용 skills
  <div>
    <img src="https://img.shields.io/badge/react-61DAFB?style=for-the-badge&logo=react&logoColor=black">
    <img src="https://img.shields.io/badge/Typescript-3178C6?style=for-the-badge&logo=typescript&logoColor=white">
    <img src="https://img.shields.io/badge/tailwindcss-F7DF1E?style=for-the-badge&logo=tailwindcss&logoColor=white">
  </div>

### 페이지 디자인

![믄제집찾기](https://github.com/HungKungE/Jogiyo-v2/assets/84065412/2f6fb1cd-b74a-4e4c-b4ef-f806866dce49)

</div>
</details>

---

### server

<details>
<summary><b>curriculum api</b></summary>
<div markdown="1">
  </br>
    영어단어 암기 커리큘럼 서비스 관련 admin api를 구현했다.
  </br>
  
  - [curriculum 코드 확인](https://github.com/HungKungE/Jogiyo-v3/tree/main/code/v2/server/admin/curriculum)
  
### 사용 skills
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
    curriculum에서 사용하는 1일 치 단어들의 정보를 가져오는 api를 구현했다. 
  </br>

   - [vocabulary 코드 확인](https://github.com/HungKungE/Jogiyo-v3/tree/main/code/v2/server/admin/voca)
  
### 사용 skills
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
    사용자에게 제공하는 문제집 관련 데이터와 사용자의 정보를 가져오는 api를 구현했다. 
  </br>

  - [workbook 코드 확인](https://github.com/HungKungE/Jogiyo-v3/tree/main/code/v2/server/admin/workbook)
  - [user 코드 확인](https://github.com/HungKungE/Jogiyo-v3/tree/main/code/v2/server/admin/user)
  
### 사용 skills
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
    공공데이터인 전국 학교 리스트 관련 api를 구현했다. 
  </br>
  
  - [school 코드 확인](https://github.com/HungKungE/Jogiyo-v3/tree/main/code/v2/server/school)
  
### 사용 skills
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

처음으로 웹 사이트를 개발하는 프로젝트를 진행하게 되서 좋은 경험이었다.
</br>
client : react, typescript, tailwind-css 를 활용하여 동적 폼 client 개발 경험을 할 수 있었다.
</br>
server : 한국에서는 생소한 rust 언어를 사용하여 api 개발을 경험 할 수 있었다.
</br>
처음으로 back-end 개발을 시도했기 때문에, 사용자들에게 제공되는 api보다는 admin이 사용할 수 있는 간단한 api 위주로 구현했었는데
이런 점은 좋은 기회기도 했으나, 스스로 서비스를 만들어보지 못해 아쉬움을 느꼈다.
