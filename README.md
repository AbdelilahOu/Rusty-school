# RustThingy

School management api build using:

- actix-web as backend server
- postgres as database
- sea-orm and sea-migration to connect and run migrations

This is how the schema looks like:

![database schema](school-management-db.svg)

You can also find the schema in the schema.sql file

Im using docker to run my database if you wanna use an other database you can here are the env variables that you need to run the project :

- DATABASE_URL
- OAUTH_CLIENT_ID
- OAUTH_SECRET
- REDIRECT_URL
- JWT_SECRET

## API

Note for api with search params other than page and limit, other params arent required, they are for filtering

### Auth

- login using google account :

  ```
    curl -X GET http://0.0.0.0:8080/auth/login
  ```

- renew access token :

  ```
    curl -X POST http://0.0.0.0:8080/auth/renew-access \
      -H "Content-Type: application/json" \
      -H "Authorization: Bearer YOUR_ACCESS_TOKEN" \
      -d '{ "refresh_token": "YOUR_REFRESH_TOKEN" }'
  ```

### Students

- create new student:

  ```
    curl -X POST http://0.0.0.0:8080/students/ \
      -H "Content-Type: application/json" \
      -H "Authorization: Bearer YOUR_ACCESS_TOKEN" \
      -d '{"first_name": "FIRST_NAME", "last_name": "LAST_NAME", "group_id": "GROUP_ID", "person_id": "PERSON_ID"}"
  ```

- list all students :

  ```
    curl -X GET http://0.0.0.0:8080/students/?page={PAGE}&limit={LIMIT}&full_name={FULL_NAME} \
      -H "Authorization: Bearer YOUR_ACCESS_TOKEN"
  ```

- delete a student :

  ```
    curl -X DELETE http://0.0.0.0:8080/students/{your_student_id} \
      -H "Authorization: Bearer YOUR_ACCESS_TOKEN"
  ```

- update a student :

  ```
    curl -X PUT http://0.0.0.0:8080/students/{your_student_id} \
      -H "Content-Type: application/json" \
      -H "Authorization: Bearer YOUR_ACCESS_TOKEN"
      -d '{"first_name": "FIRST_NAME", "last_name": "LAST_NAME", "group_id": "GROUP_ID", "person_id": "PERSON_ID"}'
  ```

### Parents

- create new parent :

  ```
    curl -X POST http://0.0.0.0:8080/parents/ \
      -H "Content-Type: application/json" \
      -H "Authorization: Bearer YOUR_ACCESS_TOKEN"
      -d '{"first_name": "FIRST_NAME", "last_name": "LAST_NAME", "person_id": "PERSON_ID"}'
  ```

- list all parents :

  ```
    curl -X GET http://0.0.0.0:8080/parents/?page={PAGE}&limit={LIMIT}&full_name={FULL_NAME} \
      -H "Authorization: Bearer YOUR_ACCESS_TOKEN"
  ```

- delete a parent :

  ```
    curl -X DELETE http://0.0.0.0:8080/parents/{id} \
      -H "Authorization: Bearer YOUR_ACCESS_TOKEN"
  ```

- update a parent :

  ```
    curl -X PUT http://0.0.0.0:8080/parents/{id} \
      -H "Content-Type: application/json" \
      -H "Authorization: Bearer YOUR_ACCESS_TOKEN" \
      -d '{"first_name": "FIRST_NAME", "last_name": "LAST_NAME", "person_id": "PERSON_ID"}'
  ```

### Teachers

- create new teacher :

  ```
    curl -X POST http://0.0.0.0:8080/teachers/ \
      -H "Content-Type: application/json" \
      -H "Authorization: Bearer YOUR_ACCESS_TOKEN" \
      -d '{"first_name": "FIRST_NAME", "last_name": "LAST_NAME", "person_id": "PERSON_ID"}'
  ```

- list all teachers :

  ```
    curl -X GET http://0.0.0.0:8080/teachers/?page={PAGE}&limit={LIMIT}&full_name={FULL_NAME} \
      -H "Authorization: Bearer YOUR_ACCESS_TOKEN"
  ```

- delete a teacher :

  ```
    curl -X DELETE http://0.0.0.0:8080/teachers/{id} \
      -H "Authorization: Bearer YOUR_ACCESS_TOKEN"
  ```

- update a teacher :

  ```
    curl -X PUT http://0.0.0.0:8080/teachers/{id} \
      -H "Content-Type: application/json" \
      -H "Authorization: Bearer YOUR_ACCESS_TOKEN" \
      -d '{"first_name": "FIRST_NAME", "last_name": "LAST_NAME", "person_id": "PERSON_ID"}'
  ```

- Assign subject to teacher :

  ```
    curl -X POST http://0.0.0.0:8080/teachers/{teacher_id}/subject/{subject_id} \
      -H "Authorization: Bearer YOUR_ACCESS_TOKEN"
  ```

- delete teacher subject id using id of the column in teacher_subjects table :

  ```
    curl -X DELETE http://0.0.0.0:8080/teachers/subject/{id} \
      -H "Authorization: Bearer YOUR_ACCESS_TOKEN"
  ```

### Levels

- create new level :

  ```
    curl -X POST http://0.0.0.0:8080/levels/ \
      -H "Content-Type: application/json" \
      -H "Authorization: Bearer YOUR_ACCESS_TOKEN" \
      -d '{"name": "LEVEL_NAME", "description": "LEVEL_DESCRIPTION"}'
  ```

- list all levels :

  ```
    curl -X GET http://0.0.0.0:8080/levels/?page={PAGE}&limit={LIMIT}&name={NAME} \
      -H "Authorization: Bearer YOUR_ACCESS_TOKEN"
  ```

- delete a level :

  ```
    curl -X DELETE http://0.0.0.0:8080/levels/{id} \
      -H "Authorization: Bearer YOUR_ACCESS_TOKEN"
  ```

- update a level :

  ```
    curl -X PUT http://0.0.0.0:8080/levels/{id} \
      -H "Content-Type: application/json" \
      -H "Authorization: Bearer YOUR_ACCESS_TOKEN" \
      -d '{"name": "LEVEL_NAME", "description": "LEVEL_DESCRIPTION"}'
  ```

### Groups

- create new group :

  ```
    curl -X POST http://0.0.0.0:8080/groups/ \
      -H "Content-Type: application/json" \
      -H "Authorization: Bearer YOUR_ACCESS_TOKEN" \
      -d '{"name": "GROUP_NAME", "description": "GROUP_DESCRIPTION", "level_id": "LEVEL_ID"}'
  ```

- list all groups :

  ```
    curl -X GET http://0.0.0.0:8080/groups/?page={PAGE}&limit={LIMIT}&name={NAME} \
      -H "Authorization: Bearer YOUR_ACCESS_TOKEN"
  ```

- delete a group :

  ```
    curl -X DELETE http://0.0.0.0:8080/groups/{id} \
      -H "Authorization: Bearer YOUR_ACCESS_TOKEN"
  ```

- update a group :

  ```
    curl -X PUT http://0.0.0.0:8080/groups/{id} \
      -H "Content-Type: application/json" \
      -H "Authorization: Bearer YOUR_ACCESS_TOKEN" \
      -d '{"name": "GROUP_NAME", "description": "GROUP_DESCRIPTION", "level_id": "LEVEL_ID"}'
  ```

- select groups by level :

  ```
    curl -X GET http://0.0.0.0:8080/groups/by-level-id/{id} \
      -H "Authorization: Bearer YOUR_ACCESS_TOKEN"
  ```

### Subjects

- create new subject :

  ```
    curl -X POST http://0.0.0.0:8080/subjects/ \
      -H "Content-Type: application/json" \
      -H "Authorization: Bearer YOUR_ ACCESS_TOKEN" \
      -d '{"name": "SUBJECT_NAME", "description": "SUBJECT_DESCRIPTION", "level_id": "LEVEL_ID"}'
  ```

- list all subjects :

  ```
    curl -X GET http://0.0.0.0:8080/subjects/?page={PAGE}&limit={LIMIT}&name={NAME} \
      -H "Authorization: Bearer YOUR_ACCESS_TOKEN"
  ```

- delete a subject :

  ```
    curl -X DELETE http://0.0.0.0:8080/subjects/{id} \
      -H "Authorization: Bearer YOUR_ACCESS_TOKEN"
  ```

- update a subject :

  ```
    curl -X PUT http://0.0.0.0:8080/subjects/{id} \
      -H "Content-Type: application/json" \
      -H "Authorization: Bearer YOUR_ACCESS_TOKEN" \
      -d '{"name": "SUBJECT_NAME", "description": "SUBJECT_DESCRIPTION", "level_id": "LEVEL_ID"}'
  ```

- select subjects by level :

  ```
    curl -X GET http://0.0.0.0:8080/subjects/by-level-id/{id} \
      -H "Authorization: Bearer YOUR_ACCESS_TOKEN"
  ```

### Classes

- create new class :

  ```
    curl -X POST http://0.0.0.0:8080/classes/ \
      -H "Content-Type: application/json" \
      -H "Authorization: Bearer YOUR_ACCESS_TOKEN" \
      -d '{"subject_id": "SUBJECT_ID", "teacher_id": "TEACHER_ID", "group_id": "GROUP_ID", "room_id": "ROOM_ID"}'
  ```

- list all classes :

  ```
    curl -X GET http://0.0.0.0:8080/classes/?page={PAGE}&limit={LIMIT}&subject_id={SUBJECT_ID}&teacher_id={TEACHER_ID}&group_id={GROUP_ID} \
      -H "Authorization: Bearer YOUR_ACCESS_TOKEN"
  ```

- delete a class :

  ```
    curl -X DELETE http://0.0.0.0:8080/classes/{id} \
      -H "Authorization: Bearer YOUR_ACCESS_TOKEN"
  ```

- update a class :

  ```
    curl -X PUT http://0.0.0.0:8080/classes/{id} \
      -H "Content-Type: application/json" \
      -H "Authorization: Bearer YOUR_ACCESS_TOKEN" \
      -d '{"subject_id": "SUBJECT_ID", "teacher_id": "TEACHER_ID", "group_id": "GROUP_ID", "room_id": "ROOM_ID"}'
  ```

### Scans

- list all scans (students, parents, teachers) :

  ```
    curl -X GET http://0.0.0.0:8080/scans/?page={PAGE}&limit={LIMIT}&scan_time_end={SCAN_TIME_END}&scan_time_start={SCAN_TIME_START}&full_name={FULL_NAME}&person_type={PERSON_TYPE}\
      -H "Authorization: Bearer YOUR_ACCESS_TOKEN"
  ```

- create a scans :

  ```
    curl -X POST http://0.0.0.0:8080/scans/ \
      -H "Content-Type: application/json" \
      -H "Authorization: Bearer YOUR_ACCESS_TOKEN" \
      -d '{"person_id": "{PERSON_ID}"}'
  ```

### Attendance

- list attendance of students :

  ```
    curl -X GET http://0.0.0.0:8080/attendance/?page={PAGE}&limit={LIMIT}&scan_time_end={SCAN_TIME_END}&scan_time_start={SCAN_TIME_START}&full_name={FULL_NAME}&group_id={GROUP_ID} \
      -H "Authorization: Bearer YOUR_ACCESS_TOKEN"
  ```

### Assignments

- create new assignment :

  ```
    curl -X POST http://0.0.0.0:8080/assignments/ \
      -H "Content-Type: application/json" \
      -H "Authorization: Bearer YOUR_ACCESS_TOKEN" \
      -d '{"title": "TITLE", "description": "DESCRIPTION", "due_date": "DUE_DATE", "submission_type": "SUBMISSION_TYPE", "grading_rubric_id": "GRADING_RUBRIC_ID", "file": "FILE", "teacher_id": "TEACHER_ID", "subject_id": "SUBJECT_ID"}'

  ```

- list all assignments :

  ```
    curl -X GET http://0.0.0.0:8080/assignments/?page={PAGE}&limit={LIMIT}&title={TITLE}&due_date={DUE_DATE}&teacher_id={TEACHER_ID} \
        -H "Authorization: Bearer YOUR_ACCESS_TOKEN"

  ```

- delete a assignment :

  ```
    curl -X DELETE http://0.0.0.0:8080/assignments/{id} \
      -H "Authorization: Bearer YOUR_ACCESS_TOKEN"
  ```

- update a assignment :

  ```
    curl -X PUT http://0.0.0.0:8080/assignments/{id} \
      -H "Content-Type: application/json" \
      -H "Authorization: Bearer YOUR_ACCESS_TOKEN" \
      -d '{"title": "TITLE", "description": "DESCRIPTION", "due_date": "DUE_DATE", "submission_type": "SUBMISSION_TYPE", "grading_rubric_id": "GRADING_RUBRIC_ID", "file": "FILE", "teacher_id": "TEACHER_ID", "subject_id": "SUBJECT_ID"}'

  ```

### disciplinary actions

### grades

### grading rubrics

### rooms
