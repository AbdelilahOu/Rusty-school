--
-- PostgreSQL database dump
--

-- Dumped from database version 15.3 (Debian 15.3-1.pgdg120+1)
-- Dumped by pg_dump version 15.3 (Debian 15.3-1.pgdg120+1)

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

--
-- Name: announcement_category_enum; Type: TYPE; Schema: public; Owner: root
--

CREATE TYPE public.announcement_category_enum AS ENUM (
    'event',
    'general',
    'academic'
);


ALTER TYPE public.announcement_category_enum OWNER TO root;

--
-- Name: audience_enum; Type: TYPE; Schema: public; Owner: root
--

CREATE TYPE public.audience_enum AS ENUM (
    'teachers',
    'parents',
    'students',
    'groups'
);


ALTER TYPE public.audience_enum OWNER TO root;

--
-- Name: day_of_week_enum; Type: TYPE; Schema: public; Owner: root
--

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

--
-- Name: performance_level_enum; Type: TYPE; Schema: public; Owner: root
--

CREATE TYPE public.performance_level_enum AS ENUM (
    'exceeds_expectations',
    'meets_expectations',
    'needs_improvement',
    'below_expectations',
    'not_yet_meeting_expectations'
);


ALTER TYPE public.performance_level_enum OWNER TO root;

--
-- Name: person_type_enum; Type: TYPE; Schema: public; Owner: root
--

CREATE TYPE public.person_type_enum AS ENUM (
    'teacher',
    'parent',
    'student',
    'not_defined'
);


ALTER TYPE public.person_type_enum OWNER TO root;

--
-- Name: time_table_item_type_enum; Type: TYPE; Schema: public; Owner: root
--

CREATE TYPE public.time_table_item_type_enum AS ENUM (
    'activity',
    'lecture',
    'event'
);


ALTER TYPE public.time_table_item_type_enum OWNER TO root;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: activities; Type: TABLE; Schema: public; Owner: root
--

CREATE TABLE public.activities (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    activity_title text,
    activity_description text,
    activity_type text,
    time_table_id uuid
);


ALTER TABLE public.activities OWNER TO root;

--
-- Name: announcements; Type: TABLE; Schema: public; Owner: root
--

CREATE TABLE public.announcements (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    title character varying NOT NULL,
    description text,
    created_at timestamp without time zone DEFAULT CURRENT_TIMESTAMP,
    start_date timestamp without time zone,
    end_date timestamp without time zone,
    category public.announcement_category_enum NOT NULL,
    targets uuid[],
    attachements timestamp without time zone,
    important boolean,
    audience public.audience_enum
);


ALTER TABLE public.announcements OWNER TO root;

--
-- Name: assignments; Type: TABLE; Schema: public; Owner: root
--

CREATE TABLE public.assignments (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    title character varying NOT NULL,
    description character varying NOT NULL,
    due_date timestamp without time zone NOT NULL,
    submission_type character varying NOT NULL,
    gradin_rubric_id uuid,
    file character varying,
    teacher_id uuid,
    subject_id uuid
);


ALTER TABLE public.assignments OWNER TO root;

--
-- Name: classes; Type: TABLE; Schema: public; Owner: root
--

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

--
-- Name: disciplinary_actions; Type: TABLE; Schema: public; Owner: root
--

CREATE TABLE public.disciplinary_actions (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    student_id uuid NOT NULL,
    created_at timestamp without time zone DEFAULT CURRENT_TIMESTAMP,
    issued_at timestamp without time zone DEFAULT CURRENT_TIMESTAMP,
    description text NOT NULL,
    consequences text NOT NULL
);


ALTER TABLE public.disciplinary_actions OWNER TO root;

--
-- Name: events; Type: TABLE; Schema: public; Owner: root
--

CREATE TABLE public.events (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    event_title text,
    event_description text,
    time_table_id uuid
);


ALTER TABLE public.events OWNER TO root;

--
-- Name: grades; Type: TABLE; Schema: public; Owner: root
--

CREATE TABLE public.grades (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    student_id uuid NOT NULL,
    assignment_id uuid NOT NULL,
    score real NOT NULL,
    feedback character varying
);


ALTER TABLE public.grades OWNER TO root;

--
-- Name: grading_criteria; Type: TABLE; Schema: public; Owner: root
--

CREATE TABLE public.grading_criteria (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    grading_rubric_id uuid NOT NULL,
    description character varying,
    performance public.performance_level_enum NOT NULL
);


ALTER TABLE public.grading_criteria OWNER TO root;

--
-- Name: grading_rubrics; Type: TABLE; Schema: public; Owner: root
--

CREATE TABLE public.grading_rubrics (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    title character varying NOT NULL,
    description character varying
);


ALTER TABLE public.grading_rubrics OWNER TO root;

--
-- Name: group_assignments; Type: TABLE; Schema: public; Owner: root
--

CREATE TABLE public.group_assignments (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    group_id uuid,
    assignment_id uuid
);


ALTER TABLE public.group_assignments OWNER TO root;

--
-- Name: groups; Type: TABLE; Schema: public; Owner: root
--

CREATE TABLE public.groups (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    group_name character varying NOT NULL,
    group_description character varying,
    created_at timestamp without time zone DEFAULT CURRENT_TIMESTAMP,
    level_id uuid
);


ALTER TABLE public.groups OWNER TO root;

--
-- Name: lectures; Type: TABLE; Schema: public; Owner: root
--

CREATE TABLE public.lectures (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    class_id uuid,
    time_table_id uuid
);


ALTER TABLE public.lectures OWNER TO root;

--
-- Name: levels; Type: TABLE; Schema: public; Owner: root
--

CREATE TABLE public.levels (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    level_name character varying NOT NULL,
    level_description character varying,
    created_at timestamp without time zone DEFAULT CURRENT_TIMESTAMP
);


ALTER TABLE public.levels OWNER TO root;

--
-- Name: parents; Type: TABLE; Schema: public; Owner: root
--

CREATE TABLE public.parents (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    first_name character varying NOT NULL,
    last_name character varying NOT NULL,
    created_at timestamp without time zone DEFAULT CURRENT_TIMESTAMP,
    full_name character varying GENERATED ALWAYS AS ((((first_name)::text || ' '::text) || (last_name)::text)) STORED,
    person_id uuid
);


ALTER TABLE public.parents OWNER TO root;

--
-- Name: persons; Type: TABLE; Schema: public; Owner: root
--

CREATE TABLE public.persons (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    person_type public.person_type_enum NOT NULL
);


ALTER TABLE public.persons OWNER TO root;

--
-- Name: pickups; Type: TABLE; Schema: public; Owner: root
--

CREATE TABLE public.pickups (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    student_id uuid NOT NULL,
    parent_id uuid NOT NULL,
    created_at timestamp without time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
);


ALTER TABLE public.pickups OWNER TO root;

--
-- Name: rooms; Type: TABLE; Schema: public; Owner: root
--

CREATE TABLE public.rooms (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    room_name character varying NOT NULL,
    room_description character varying
);


ALTER TABLE public.rooms OWNER TO root;

--
-- Name: scans; Type: TABLE; Schema: public; Owner: root
--

CREATE TABLE public.scans (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    scan_date timestamp without time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    person_id uuid NOT NULL
);


ALTER TABLE public.scans OWNER TO root;

--
-- Name: seaql_migrations; Type: TABLE; Schema: public; Owner: root
--

CREATE TABLE public.seaql_migrations (
    version character varying NOT NULL,
    applied_at bigint NOT NULL
);


ALTER TABLE public.seaql_migrations OWNER TO root;

--
-- Name: sessions; Type: TABLE; Schema: public; Owner: root
--

CREATE TABLE public.sessions (
    id uuid NOT NULL,
    user_id uuid NOT NULL,
    user_agent character varying NOT NULL,
    client_ip character varying NOT NULL,
    is_blocked boolean DEFAULT false NOT NULL,
    refresh_token character varying NOT NULL,
    expires_at timestamp without time zone NOT NULL,
    created_at timestamp without time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
);


ALTER TABLE public.sessions OWNER TO root;

--
-- Name: students; Type: TABLE; Schema: public; Owner: root
--

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

--
-- Name: subjects; Type: TABLE; Schema: public; Owner: root
--

CREATE TABLE public.subjects (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    subject_name character varying NOT NULL,
    subject_description character varying,
    created_at timestamp without time zone DEFAULT CURRENT_TIMESTAMP,
    level_id uuid
);


ALTER TABLE public.subjects OWNER TO root;

--
-- Name: teacher_subjects; Type: TABLE; Schema: public; Owner: root
--

CREATE TABLE public.teacher_subjects (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    subject_id uuid,
    teacher_id uuid,
    md_idx character varying GENERATED ALWAYS AS (md5(((((subject_id)::character varying)::text || '-'::text) || ((teacher_id)::character varying)::text))) STORED
);


ALTER TABLE public.teacher_subjects OWNER TO root;

--
-- Name: teachers; Type: TABLE; Schema: public; Owner: root
--

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

--
-- Name: time_table; Type: TABLE; Schema: public; Owner: root
--

CREATE TABLE public.time_table (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    item_type public.time_table_item_type_enum NOT NULL,
    day_of_week public.day_of_week_enum,
    full_date date,
    start_time time without time zone,
    end_time time without time zone
);


ALTER TABLE public.time_table OWNER TO root;

--
-- Name: users; Type: TABLE; Schema: public; Owner: root
--

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

--
-- Name: activities activities_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.activities
    ADD CONSTRAINT activities_pkey PRIMARY KEY (id);


--
-- Name: announcements announcements_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.announcements
    ADD CONSTRAINT announcements_pkey PRIMARY KEY (id);


--
-- Name: assignments assignments_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.assignments
    ADD CONSTRAINT assignments_pkey PRIMARY KEY (id);


--
-- Name: classes classes_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.classes
    ADD CONSTRAINT classes_pkey PRIMARY KEY (id);


--
-- Name: disciplinary_actions disciplinary_actions_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.disciplinary_actions
    ADD CONSTRAINT disciplinary_actions_pkey PRIMARY KEY (id);


--
-- Name: events events_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.events
    ADD CONSTRAINT events_pkey PRIMARY KEY (id);


--
-- Name: grades grades_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.grades
    ADD CONSTRAINT grades_pkey PRIMARY KEY (id);


--
-- Name: grading_criteria grading_criteria_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.grading_criteria
    ADD CONSTRAINT grading_criteria_pkey PRIMARY KEY (id);


--
-- Name: grading_rubrics grading_rubrics_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.grading_rubrics
    ADD CONSTRAINT grading_rubrics_pkey PRIMARY KEY (id);


--
-- Name: group_assignments group_assignments_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.group_assignments
    ADD CONSTRAINT group_assignments_pkey PRIMARY KEY (id);


--
-- Name: groups groups_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.groups
    ADD CONSTRAINT groups_pkey PRIMARY KEY (id);


--
-- Name: lectures lectures_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.lectures
    ADD CONSTRAINT lectures_pkey PRIMARY KEY (id);


--
-- Name: levels levels_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.levels
    ADD CONSTRAINT levels_pkey PRIMARY KEY (id);


--
-- Name: parents parents_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.parents
    ADD CONSTRAINT parents_pkey PRIMARY KEY (id);


--
-- Name: persons persons_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.persons
    ADD CONSTRAINT persons_pkey PRIMARY KEY (id);


--
-- Name: pickups pickups_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.pickups
    ADD CONSTRAINT pickups_pkey PRIMARY KEY (id);


--
-- Name: rooms rooms_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.rooms
    ADD CONSTRAINT rooms_pkey PRIMARY KEY (id);


--
-- Name: scans scans_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.scans
    ADD CONSTRAINT scans_pkey PRIMARY KEY (id);


--
-- Name: seaql_migrations seaql_migrations_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.seaql_migrations
    ADD CONSTRAINT seaql_migrations_pkey PRIMARY KEY (version);


--
-- Name: sessions sessions_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.sessions
    ADD CONSTRAINT sessions_pkey PRIMARY KEY (id);


--
-- Name: students students_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.students
    ADD CONSTRAINT students_pkey PRIMARY KEY (id);


--
-- Name: subjects subjects_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.subjects
    ADD CONSTRAINT subjects_pkey PRIMARY KEY (id);


--
-- Name: teacher_subjects teacher_subjects_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.teacher_subjects
    ADD CONSTRAINT teacher_subjects_pkey PRIMARY KEY (id);


--
-- Name: teachers teachers_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.teachers
    ADD CONSTRAINT teachers_pkey PRIMARY KEY (id);


--
-- Name: time_table time_table_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.time_table
    ADD CONSTRAINT time_table_pkey PRIMARY KEY (id);


--
-- Name: users users_email_key; Type: CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.users
    ADD CONSTRAINT users_email_key UNIQUE (email);


--
-- Name: users users_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.users
    ADD CONSTRAINT users_pkey PRIMARY KEY (id);


--
-- Name: idx_class_md_idx; Type: INDEX; Schema: public; Owner: root
--

CREATE UNIQUE INDEX idx_class_md_idx ON public.classes USING btree (md_idx);


--
-- Name: idx_parents_full_name; Type: INDEX; Schema: public; Owner: root
--

CREATE INDEX idx_parents_full_name ON public.parents USING btree (full_name);


--
-- Name: idx_students_full_name; Type: INDEX; Schema: public; Owner: root
--

CREATE INDEX idx_students_full_name ON public.students USING btree (full_name);


--
-- Name: idx_teachers_full_name; Type: INDEX; Schema: public; Owner: root
--

CREATE INDEX idx_teachers_full_name ON public.teachers USING btree (full_name);


--
-- Name: activities fk_activities_time_table_id; Type: FK CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.activities
    ADD CONSTRAINT fk_activities_time_table_id FOREIGN KEY (time_table_id) REFERENCES public.time_table(id) ON DELETE CASCADE;


--
-- Name: assignments fk_assignment_rubric_id; Type: FK CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.assignments
    ADD CONSTRAINT fk_assignment_rubric_id FOREIGN KEY (gradin_rubric_id) REFERENCES public.grading_rubrics(id) ON DELETE CASCADE;


--
-- Name: assignments fk_assignment_subject_id; Type: FK CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.assignments
    ADD CONSTRAINT fk_assignment_subject_id FOREIGN KEY (gradin_rubric_id) REFERENCES public.subjects(id) ON DELETE SET NULL;


--
-- Name: assignments fk_assignment_teacher_id; Type: FK CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.assignments
    ADD CONSTRAINT fk_assignment_teacher_id FOREIGN KEY (gradin_rubric_id) REFERENCES public.teachers(id) ON DELETE SET NULL;


--
-- Name: classes fk_classes_group_id; Type: FK CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.classes
    ADD CONSTRAINT fk_classes_group_id FOREIGN KEY (group_id) REFERENCES public.groups(id) ON DELETE CASCADE;


--
-- Name: classes fk_classes_room_id; Type: FK CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.classes
    ADD CONSTRAINT fk_classes_room_id FOREIGN KEY (room_id) REFERENCES public.rooms(id) ON DELETE CASCADE;


--
-- Name: classes fk_classes_subject_id; Type: FK CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.classes
    ADD CONSTRAINT fk_classes_subject_id FOREIGN KEY (subject_id) REFERENCES public.subjects(id) ON DELETE SET NULL;


--
-- Name: classes fk_classes_teacher_id; Type: FK CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.classes
    ADD CONSTRAINT fk_classes_teacher_id FOREIGN KEY (teacher_id) REFERENCES public.teachers(id) ON DELETE CASCADE;


--
-- Name: groups fk_details_person_id; Type: FK CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.groups
    ADD CONSTRAINT fk_details_person_id FOREIGN KEY (level_id) REFERENCES public.levels(id) ON DELETE CASCADE;


--
-- Name: disciplinary_actions fk_disciplinary_actions_student_id; Type: FK CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.disciplinary_actions
    ADD CONSTRAINT fk_disciplinary_actions_student_id FOREIGN KEY (student_id) REFERENCES public.students(id) ON DELETE RESTRICT;


--
-- Name: events fk_events_time_table_id; Type: FK CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.events
    ADD CONSTRAINT fk_events_time_table_id FOREIGN KEY (time_table_id) REFERENCES public.time_table(id) ON DELETE CASCADE;


--
-- Name: grades fk_grade_assignment_id; Type: FK CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.grades
    ADD CONSTRAINT fk_grade_assignment_id FOREIGN KEY (assignment_id) REFERENCES public.assignments(id) ON DELETE RESTRICT;


--
-- Name: grades fk_grade_student_id; Type: FK CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.grades
    ADD CONSTRAINT fk_grade_student_id FOREIGN KEY (student_id) REFERENCES public.students(id) ON DELETE RESTRICT;


--
-- Name: grading_criteria fk_grading_criteria_rubrics_id; Type: FK CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.grading_criteria
    ADD CONSTRAINT fk_grading_criteria_rubrics_id FOREIGN KEY (grading_rubric_id) REFERENCES public.grading_rubrics(id) ON DELETE CASCADE;


--
-- Name: group_assignments fk_group_assignments_assignment_id; Type: FK CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.group_assignments
    ADD CONSTRAINT fk_group_assignments_assignment_id FOREIGN KEY (group_id) REFERENCES public.assignments(id) ON DELETE SET NULL;


--
-- Name: group_assignments fk_group_assignments_group_id; Type: FK CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.group_assignments
    ADD CONSTRAINT fk_group_assignments_group_id FOREIGN KEY (group_id) REFERENCES public.groups(id) ON DELETE SET NULL;


--
-- Name: lectures fk_lectures_class_id; Type: FK CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.lectures
    ADD CONSTRAINT fk_lectures_class_id FOREIGN KEY (class_id) REFERENCES public.classes(id) ON DELETE CASCADE;


--
-- Name: lectures fk_lectures_time_table_id; Type: FK CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.lectures
    ADD CONSTRAINT fk_lectures_time_table_id FOREIGN KEY (time_table_id) REFERENCES public.time_table(id) ON DELETE CASCADE;


--
-- Name: parents fk_parent_person_id; Type: FK CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.parents
    ADD CONSTRAINT fk_parent_person_id FOREIGN KEY (person_id) REFERENCES public.persons(id);


--
-- Name: pickups fk_pickup_parent_id; Type: FK CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.pickups
    ADD CONSTRAINT fk_pickup_parent_id FOREIGN KEY (parent_id) REFERENCES public.parents(id) ON DELETE CASCADE;


--
-- Name: pickups fk_pickup_student_id; Type: FK CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.pickups
    ADD CONSTRAINT fk_pickup_student_id FOREIGN KEY (student_id) REFERENCES public.students(id) ON DELETE CASCADE;


--
-- Name: scans fk_scan_person_id; Type: FK CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.scans
    ADD CONSTRAINT fk_scan_person_id FOREIGN KEY (person_id) REFERENCES public.persons(id);


--
-- Name: sessions fk_sessions_user_id; Type: FK CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.sessions
    ADD CONSTRAINT fk_sessions_user_id FOREIGN KEY (user_id) REFERENCES public.users(id) ON DELETE RESTRICT;


--
-- Name: students fk_student_group_id; Type: FK CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.students
    ADD CONSTRAINT fk_student_group_id FOREIGN KEY (group_id) REFERENCES public.groups(id);


--
-- Name: students fk_student_person_id; Type: FK CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.students
    ADD CONSTRAINT fk_student_person_id FOREIGN KEY (person_id) REFERENCES public.persons(id);


--
-- Name: subjects fk_subject_level_id; Type: FK CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.subjects
    ADD CONSTRAINT fk_subject_level_id FOREIGN KEY (level_id) REFERENCES public.levels(id) ON DELETE CASCADE;


--
-- Name: teachers fk_teacher_level_id; Type: FK CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.teachers
    ADD CONSTRAINT fk_teacher_level_id FOREIGN KEY (level_id) REFERENCES public.levels(id);


--
-- Name: teachers fk_teacher_person_id; Type: FK CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.teachers
    ADD CONSTRAINT fk_teacher_person_id FOREIGN KEY (person_id) REFERENCES public.persons(id);


--
-- Name: teacher_subjects fk_teacher_subjects_subject_id; Type: FK CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.teacher_subjects
    ADD CONSTRAINT fk_teacher_subjects_subject_id FOREIGN KEY (subject_id) REFERENCES public.subjects(id) ON DELETE SET NULL;


--
-- Name: teacher_subjects fk_teacher_subjects_teacher_id; Type: FK CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.teacher_subjects
    ADD CONSTRAINT fk_teacher_subjects_teacher_id FOREIGN KEY (teacher_id) REFERENCES public.teachers(id) ON DELETE CASCADE;


--
-- Name: users fk_user_person_id; Type: FK CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.users
    ADD CONSTRAINT fk_user_person_id FOREIGN KEY (person_id) REFERENCES public.persons(id) ON DELETE CASCADE;


--
-- PostgreSQL database dump complete
--

