SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;


CREATE TYPE public.day_of_week_enum AS ENUM (
    'monday',
    'tuesday',
    'wednesday',
    'thursday',
    'friday',
    'saturday',
    'sunday'
);


ALTER TYPE public.day_of_week_enum OWNER TO root;


CREATE TYPE public.performance_level_type AS ENUM (
    'exceeds_expectations',
    'meets_expectations',
    'needs_improvement',
    'below_expectations',
    'not_yet_meeting_expectations'
);


ALTER TYPE public.performance_level_type OWNER TO root;


CREATE TYPE public.time_table_item_type AS ENUM (
    'activity',
    'lecture',
    'event'
);


ALTER TYPE public.time_table_item_type OWNER TO root;

SET default_tablespace = '';

SET default_table_access_method = heap;


CREATE TABLE public.activities (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    activity_title text,
    activity_description text,
    activity_type text,
    time_table_id uuid
);


ALTER TABLE public.activities OWNER TO root;


CREATE TABLE public.assignments (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    title character varying NOT NULL,
    description character varying NOT NULL,
    due_date timestamp without time zone NOT NULL,
    submission_type character varying NOT NULL,
    gradin_rubric_id uuid,
    file character varying,
    teacher_id uuid
);


ALTER TABLE public.assignments OWNER TO root;


CREATE TABLE public.class_assignments (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    class_id uuid,
    assignment_id uuid
);


ALTER TABLE public.class_assignments OWNER TO root;


CREATE TABLE public.classes (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    created_at timestamp without time zone DEFAULT CURRENT_TIMESTAMP,
    subject_id uuid,
    teacher_id uuid,
    group_id uuid,
    room_id uuid,
    md_idx character varying GENERATED ALWAYS AS (md5(((((((((subject_id)::character varying)::text || '-'::text) || ((teacher_id)::character varying)::text) || '-'::text) || ((group_id)::character varying)::text) || '-'::text) || ((room_id)::character varying)::text))) STORED
);


ALTER TABLE public.classes OWNER TO root;


CREATE TABLE public.disciplinary_actions (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    student_id uuid NOT NULL,
    created_at timestamp without time zone DEFAULT CURRENT_TIMESTAMP,
    issued_at timestamp without time zone DEFAULT CURRENT_TIMESTAMP,
    description text NOT NULL,
    consequences text NOT NULL
);


ALTER TABLE public.disciplinary_actions OWNER TO root;


CREATE TABLE public.events (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    event_title text,
    event_description text,
    time_table_id uuid
);


ALTER TABLE public.events OWNER TO root;


CREATE TABLE public.grades (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    student_id uuid NOT NULL,
    assignment_id uuid NOT NULL,
    score real NOT NULL,
    feedback character varying
);


ALTER TABLE public.grades OWNER TO root;


CREATE TABLE public.grading_criteria (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    grading_rubric_id uuid NOT NULL,
    description character varying,
    points numeric(4,2)
);


ALTER TABLE public.grading_criteria OWNER TO root;


CREATE TABLE public.grading_rubrics (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    title character varying NOT NULL,
    description character varying
);


ALTER TABLE public.grading_rubrics OWNER TO root;


CREATE TABLE public.groups (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    group_name character varying NOT NULL,
    group_description character varying,
    created_at timestamp without time zone DEFAULT CURRENT_TIMESTAMP,
    level_id uuid
);


ALTER TABLE public.groups OWNER TO root;


CREATE TABLE public.lectures (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    class_id uuid,
    time_table_id uuid
);


ALTER TABLE public.lectures OWNER TO root;


CREATE TABLE public.levels (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    level_name character varying NOT NULL,
    level_description character varying,
    created_at timestamp without time zone DEFAULT CURRENT_TIMESTAMP
);


ALTER TABLE public.levels OWNER TO root;


CREATE TABLE public.parents (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    first_name character varying NOT NULL,
    last_name character varying NOT NULL,
    created_at timestamp without time zone DEFAULT CURRENT_TIMESTAMP,
    full_name character varying GENERATED ALWAYS AS ((((first_name)::text || ' '::text) || (last_name)::text)) STORED,
    person_id uuid
);


ALTER TABLE public.parents OWNER TO root;


CREATE TABLE public.performance_level (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    level_name public.performance_level_type NOT NULL,
    grading_criteria_id uuid NOT NULL,
    description character varying
);


ALTER TABLE public.performance_level OWNER TO root;


CREATE TABLE public.persons (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    person_type character varying NOT NULL
);


ALTER TABLE public.persons OWNER TO root;


CREATE TABLE public.pickups (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    student_id uuid NOT NULL,
    parent_id uuid NOT NULL,
    created_at timestamp without time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
);


ALTER TABLE public.pickups OWNER TO root;


CREATE TABLE public.rooms (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    room_name character varying NOT NULL,
    room_description character varying
);


ALTER TABLE public.rooms OWNER TO root;


CREATE TABLE public.scans (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    scan_date timestamp without time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    person_id uuid NOT NULL
);


ALTER TABLE public.scans OWNER TO root;


CREATE TABLE public.seaql_migrations (
    version character varying NOT NULL,
    applied_at bigint NOT NULL
);


ALTER TABLE public.seaql_migrations OWNER TO root;


CREATE TABLE public.students (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    first_name character varying NOT NULL,
    last_name character varying NOT NULL,
    created_at timestamp without time zone DEFAULT CURRENT_TIMESTAMP,
    full_name character varying GENERATED ALWAYS AS ((((first_name)::text || ' '::text) || (last_name)::text)) STORED,
    person_id uuid,
    group_id uuid
);


ALTER TABLE public.students OWNER TO root;


CREATE TABLE public.subjects (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    subject_name character varying NOT NULL,
    subject_description character varying,
    created_at timestamp without time zone DEFAULT CURRENT_TIMESTAMP,
    level_id uuid
);


ALTER TABLE public.subjects OWNER TO root;


CREATE TABLE public.teacher_subjects (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    subject_id uuid,
    teacher_id uuid,
    md_idx character varying GENERATED ALWAYS AS (md5(((((subject_id)::character varying)::text || '-'::text) || ((teacher_id)::character varying)::text))) STORED
);


ALTER TABLE public.teacher_subjects OWNER TO root;


CREATE TABLE public.teachers (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    first_name character varying NOT NULL,
    last_name character varying NOT NULL,
    created_at timestamp without time zone DEFAULT CURRENT_TIMESTAMP,
    full_name character varying GENERATED ALWAYS AS ((((first_name)::text || ' '::text) || (last_name)::text)) STORED,
    person_id uuid,
    level_id uuid
);


ALTER TABLE public.teachers OWNER TO root;


CREATE TABLE public.time_table (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    item_type public.time_table_item_type NOT NULL,
    day_of_week public.day_of_week_enum,
    full_date date,
    start_time time without time zone,
    end_time time without time zone
);


ALTER TABLE public.time_table OWNER TO root;


CREATE TABLE public.users (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    email character varying NOT NULL,
    first_name character varying NOT NULL,
    last_name character varying NOT NULL,
    full_name character varying GENERATED ALWAYS AS ((((first_name)::text || ' '::text) || (last_name)::text)) STORED,
    person_id uuid NOT NULL,
    picture character varying
);


ALTER TABLE public.users OWNER TO root;


ALTER TABLE ONLY public.activities
    ADD CONSTRAINT activities_pkey PRIMARY KEY (id);



ALTER TABLE ONLY public.assignments
    ADD CONSTRAINT assignments_pkey PRIMARY KEY (id);



ALTER TABLE ONLY public.class_assignments
    ADD CONSTRAINT class_assignments_pkey PRIMARY KEY (id);



ALTER TABLE ONLY public.classes
    ADD CONSTRAINT classes_pkey PRIMARY KEY (id);



ALTER TABLE ONLY public.disciplinary_actions
    ADD CONSTRAINT disciplinary_actions_pkey PRIMARY KEY (id);



ALTER TABLE ONLY public.events
    ADD CONSTRAINT events_pkey PRIMARY KEY (id);



ALTER TABLE ONLY public.grades
    ADD CONSTRAINT grades_pkey PRIMARY KEY (id);



ALTER TABLE ONLY public.grading_criteria
    ADD CONSTRAINT grading_criteria_pkey PRIMARY KEY (id);



ALTER TABLE ONLY public.grading_rubrics
    ADD CONSTRAINT grading_rubrics_pkey PRIMARY KEY (id);



ALTER TABLE ONLY public.groups
    ADD CONSTRAINT groups_pkey PRIMARY KEY (id);



ALTER TABLE ONLY public.lectures
    ADD CONSTRAINT lectures_pkey PRIMARY KEY (id);



ALTER TABLE ONLY public.levels
    ADD CONSTRAINT levels_pkey PRIMARY KEY (id);



ALTER TABLE ONLY public.parents
    ADD CONSTRAINT parents_pkey PRIMARY KEY (id);



ALTER TABLE ONLY public.performance_level
    ADD CONSTRAINT performance_level_pkey PRIMARY KEY (id);



ALTER TABLE ONLY public.persons
    ADD CONSTRAINT persons_pkey PRIMARY KEY (id);



ALTER TABLE ONLY public.pickups
    ADD CONSTRAINT pickups_pkey PRIMARY KEY (id);



ALTER TABLE ONLY public.rooms
    ADD CONSTRAINT rooms_pkey PRIMARY KEY (id);



ALTER TABLE ONLY public.scans
    ADD CONSTRAINT scans_pkey PRIMARY KEY (id);



ALTER TABLE ONLY public.seaql_migrations
    ADD CONSTRAINT seaql_migrations_pkey PRIMARY KEY (version);



ALTER TABLE ONLY public.students
    ADD CONSTRAINT students_pkey PRIMARY KEY (id);



ALTER TABLE ONLY public.subjects
    ADD CONSTRAINT subjects_pkey PRIMARY KEY (id);



ALTER TABLE ONLY public.teacher_subjects
    ADD CONSTRAINT teacher_subjects_pkey PRIMARY KEY (id);



ALTER TABLE ONLY public.teachers
    ADD CONSTRAINT teachers_pkey PRIMARY KEY (id);



ALTER TABLE ONLY public.time_table
    ADD CONSTRAINT time_table_pkey PRIMARY KEY (id);



ALTER TABLE ONLY public.users
    ADD CONSTRAINT users_email_key UNIQUE (email);



ALTER TABLE ONLY public.users
    ADD CONSTRAINT users_pkey PRIMARY KEY (id);



CREATE UNIQUE INDEX idx_class_md_idx ON public.classes USING btree (md_idx);



CREATE INDEX idx_parents_full_name ON public.parents USING btree (full_name);



CREATE INDEX idx_students_full_name ON public.students USING btree (full_name);



CREATE INDEX idx_teachers_full_name ON public.teachers USING btree (full_name);



ALTER TABLE ONLY public.activities
    ADD CONSTRAINT fk_activities_time_table_id FOREIGN KEY (time_table_id) REFERENCES public.time_table(id) ON DELETE CASCADE;



ALTER TABLE ONLY public.assignments
    ADD CONSTRAINT fk_assignment_rubric_id FOREIGN KEY (gradin_rubric_id) REFERENCES public.grading_rubrics(id) ON DELETE CASCADE;



ALTER TABLE ONLY public.assignments
    ADD CONSTRAINT fk_assignment_teacher_id FOREIGN KEY (gradin_rubric_id) REFERENCES public.teachers(id) ON DELETE SET NULL;



ALTER TABLE ONLY public.class_assignments
    ADD CONSTRAINT fk_class_assignments_assignment_id FOREIGN KEY (class_id) REFERENCES public.classes(id) ON DELETE SET NULL;



ALTER TABLE ONLY public.class_assignments
    ADD CONSTRAINT fk_class_assignments_class_id FOREIGN KEY (class_id) REFERENCES public.classes(id) ON DELETE SET NULL;



ALTER TABLE ONLY public.classes
    ADD CONSTRAINT fk_classes_group_id FOREIGN KEY (group_id) REFERENCES public.groups(id) ON DELETE CASCADE;



ALTER TABLE ONLY public.classes
    ADD CONSTRAINT fk_classes_room_id FOREIGN KEY (room_id) REFERENCES public.rooms(id) ON DELETE CASCADE;



ALTER TABLE ONLY public.classes
    ADD CONSTRAINT fk_classes_subject_id FOREIGN KEY (subject_id) REFERENCES public.subjects(id) ON DELETE SET NULL;



ALTER TABLE ONLY public.classes
    ADD CONSTRAINT fk_classes_teacher_id FOREIGN KEY (teacher_id) REFERENCES public.teachers(id) ON DELETE CASCADE;



ALTER TABLE ONLY public.groups
    ADD CONSTRAINT fk_details_person_id FOREIGN KEY (level_id) REFERENCES public.levels(id) ON DELETE CASCADE;



ALTER TABLE ONLY public.disciplinary_actions
    ADD CONSTRAINT fk_disciplinary_actions_student_id FOREIGN KEY (student_id) REFERENCES public.students(id) ON DELETE RESTRICT;



ALTER TABLE ONLY public.events
    ADD CONSTRAINT fk_events_time_table_id FOREIGN KEY (time_table_id) REFERENCES public.time_table(id) ON DELETE CASCADE;



ALTER TABLE ONLY public.grades
    ADD CONSTRAINT fk_grade_assignment_id FOREIGN KEY (assignment_id) REFERENCES public.assignments(id) ON DELETE RESTRICT;



ALTER TABLE ONLY public.grades
    ADD CONSTRAINT fk_grade_student_id FOREIGN KEY (student_id) REFERENCES public.students(id) ON DELETE RESTRICT;



ALTER TABLE ONLY public.lectures
    ADD CONSTRAINT fk_lectures_class_id FOREIGN KEY (class_id) REFERENCES public.classes(id) ON DELETE CASCADE;



ALTER TABLE ONLY public.lectures
    ADD CONSTRAINT fk_lectures_time_table_id FOREIGN KEY (time_table_id) REFERENCES public.time_table(id) ON DELETE CASCADE;



ALTER TABLE ONLY public.parents
    ADD CONSTRAINT fk_parent_person_id FOREIGN KEY (person_id) REFERENCES public.persons(id);



ALTER TABLE ONLY public.performance_level
    ADD CONSTRAINT fk_performance_criteria_id FOREIGN KEY (grading_criteria_id) REFERENCES public.grading_criteria(id) ON DELETE CASCADE;



ALTER TABLE ONLY public.pickups
    ADD CONSTRAINT fk_pickup_parent_id FOREIGN KEY (parent_id) REFERENCES public.parents(id) ON DELETE CASCADE;



ALTER TABLE ONLY public.pickups
    ADD CONSTRAINT fk_pickup_student_id FOREIGN KEY (student_id) REFERENCES public.students(id) ON DELETE CASCADE;



ALTER TABLE ONLY public.scans
    ADD CONSTRAINT fk_scan_person_id FOREIGN KEY (person_id) REFERENCES public.persons(id);



ALTER TABLE ONLY public.students
    ADD CONSTRAINT fk_student_group_id FOREIGN KEY (group_id) REFERENCES public.groups(id);



ALTER TABLE ONLY public.students
    ADD CONSTRAINT fk_student_person_id FOREIGN KEY (person_id) REFERENCES public.persons(id);



ALTER TABLE ONLY public.subjects
    ADD CONSTRAINT fk_subject_level_id FOREIGN KEY (level_id) REFERENCES public.levels(id) ON DELETE CASCADE;



ALTER TABLE ONLY public.teachers
    ADD CONSTRAINT fk_teacher_level_id FOREIGN KEY (level_id) REFERENCES public.levels(id);



ALTER TABLE ONLY public.teachers
    ADD CONSTRAINT fk_teacher_person_id FOREIGN KEY (person_id) REFERENCES public.persons(id);



ALTER TABLE ONLY public.teacher_subjects
    ADD CONSTRAINT fk_teacher_subjects_subject_id FOREIGN KEY (subject_id) REFERENCES public.subjects(id) ON DELETE SET NULL;



ALTER TABLE ONLY public.teacher_subjects
    ADD CONSTRAINT fk_teacher_subjects_teacher_id FOREIGN KEY (teacher_id) REFERENCES public.teachers(id) ON DELETE CASCADE;



ALTER TABLE ONLY public.users
    ADD CONSTRAINT fk_user_person_id FOREIGN KEY (person_id) REFERENCES public.persons(id) ON DELETE CASCADE;



