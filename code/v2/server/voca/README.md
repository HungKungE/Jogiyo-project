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
