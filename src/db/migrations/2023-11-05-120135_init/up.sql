-- Create the teachers table
CREATE TABLE teachers (
    id UUID DEFAULT UUID_GENERATE_V4() PRIMARY KEY
    name VARCHAR(255),
    contact_details VARCHAR(255),
);

-- Create the students table
CREATE TABLE students (
    id UUID DEFAULT UUID_GENERATE_V4() PRIMARY KEY
    name VARCHAR(255),
    date_of_birth TIMESTAMP,
    contact_details VARCHAR(255),
    grade_level VARCHAR(255)
);

-- Create the attendance table
CREATE TABLE attendance (
    id UUID DEFAULT UUID_GENERATE_V4() PRIMARY KEY
    student_id UUID NOT NULL,
    attendance_date  TIMESTAMP,
    status VARCHAR(20), -- present, absent
    reason_for_absence VARCHAR(255),
    FOREIGN KEY (student_id) REFERENCES students(student_id),
);

-- Create the parents table
CREATE TABLE parents (
    id UUID DEFAULT UUID_GENERATE_V4() PRIMARY KEY
    name VARCHAR(255),
    contact_details VARCHAR(255)
);

-- Create the schedule table
CREATE TABLE schedule (
    id UUID DEFAULT UUID_GENERATE_V4() PRIMARY KEY
    class_id UUID NOT NULL,
    subject_id UUID NOT NULL,
    day VARCHAR(20),
    start_time TIME,
    end_time  TIME,
    FOREIGN KEY (class_id) REFERENCES classes(class_id),
    FOREIGN KEY (subject_id) REFERENCES subjects(subject_id),
);

-- Create the classes table
CREATE TABLE classes (
    id UUID DEFAULT UUID_GENERATE_V4() PRIMARY KEY
    name VARCHAR(255),
    grade_level VARCHAR(20),
    teacher_id UUID NOT NULL,
    FOREIGN KEY (teacher_id) REFERENCES staff(staff_id)
);

-- Create the subjects table
CREATE TABLE subjects (
    id UUID DEFAULT UUID_GENERATE_V4() PRIMARY KEY
    name VARCHAR(255),
    description TEXT,
    grade_level VARCHAR(20)
);

-- Create the exams table
CREATE TABLE exams (
    id UUID DEFAULT UUID_GENERATE_V4() PRIMARY KEY
    subject_id UUID NOT NULL,
    date TIMESTAMP,
    type VARCHAR(20),
    FOREIGN KEY (subject_id) REFERENCES subjects(subject_id)
);

-- Create the results table
CREATE TABLE results (
    id UUID DEFAULT UUID_GENERATE_V4() PRIMARY KEY
    exam_id UUID NOT NULL,
    student_id UUID NOT NULL,
    score DECIMAL(10, 2),
    grade VARCHAR(20),
    FOREIGN KEY (exam_id) REFERENCES exams(exam_id),
    FOREIGN KEY (student_id) REFERENCES students(student_id)
);

-- Create the fees table
CREATE TABLE fees (
    id UUID DEFAULT UUID_GENERATE_V4() PRIMARY KEY
    student_id UUID NOT NULL,
    invoice_id UUID,
    description VARCHAR(255),
    amount DECIMAL(10, 2),
    due_date TIMESTAMP,
    FOREIGN KEY (student_id) REFERENCES students(student_id)
    FOREIGN KEY (invoice_id) REFERENCES invoices(invoice_id)
);

-- Create the invoices table
CREATE TABLE invoices (
    id UUID DEFAULT UUID_GENERATE_V4() PRIMARY KEY
    student_id UUID NOT NULL,
    issue_date TIMESTAMP,
    due_date TIMESTAMP,
    status VARCHAR(20),
    FOREIGN KEY (student_id) REFERENCES students(student_id)
);

-- Create the payments table
CREATE TABLE payments (
    id UUID DEFAULT UUID_GENERATE_V4() PRIMARY KEY
    invoice_id UUID NOT NULL,
    payment_date TIMESTAMP,
    amount DECIMAL(10, 2),
    payment_method VARCHAR(20),
    transaction_reference VARCHAR(255),
    FOREIGN KEY (invoice_id) REFERENCES invoices(invoice_id)
);

-- Create the parent_pickup table
CREATE TABLE pickups (
    id UUID DEFAULT UUID_GENERATE_V4() PRIMARY KEY
    parent_id UUID NOT NULL,
    student_id UUID NOT NULL,
    pickup_date TIMESTAMP,
    FOREIGN KEY (parent_id) REFERENCES parents(parent_id),
    FOREIGN KEY (student_id) REFERENCES students(student_id)
);

-- Create the scans table
CREATE TABLE scans (
    id UUID DEFAULT UUID_GENERATE_V4() PRIMARY KEY
    student_id UUID,
    parent_id UUID,
    scan_date TIMESTAMP,
    FOREIGN KEY (student_id) REFERENCES students(student_id)
    FOREIGN KEY (parent_id) REFERENCES parents(parent_id)
);
