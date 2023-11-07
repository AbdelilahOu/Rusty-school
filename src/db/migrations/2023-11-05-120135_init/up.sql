-- Create the Staff table
CREATE TABLE Staff (
    StaffID UUID DEFAULT UUID_GENERATE_V4() PRIMARY KEY,
    Name VARCHAR(255),
    Position VARCHAR(255),
    ContactDetails VARCHAR(255),
    Qualifications VARCHAR(255)
);

-- Create the Students table
CREATE TABLE Students (
    StudentID UUID DEFAULT UUID_GENERATE_V4() PRIMARY KEY,
    Name VARCHAR(255),
    DateOfBirth DATE,
    ContactDetails VARCHAR(255),
    GradeLevel VARCHAR(255)
);

-- Create the Attendance table
CREATE TABLE Attendance (
    AttendanceID UUID DEFAULT UUID_GENERATE_V4() PRIMARY KEY,
    StudentID UUID NOT NULL,
    StaffID UUID NOT NULL,
    Date DATE,
    Status VARCHAR(20),
    ReasonForAbsence VARCHAR(255),
    FOREIGN KEY (StudentID) REFERENCES Students(StudentID),
    FOREIGN KEY (StaffID) REFERENCES Staff(StaffID)
);

-- Create the Performance table
CREATE TABLE Performance (
    PerformanceID UUID DEFAULT UUID_GENERATE_V4() PRIMARY KEY,
    StudentID UUID NOT NULL,
    Subject VARCHAR(255),
    Date DATE,
    Grade VARCHAR(10),
    TeacherID UUID NOT NULL,
    FOREIGN KEY (StudentID) REFERENCES Students(StudentID),
    FOREIGN KEY (TeacherID) REFERENCES Staff(StaffID)
);

-- Create the Parents table
CREATE TABLE Parents (
    ParentID UUID DEFAULT UUID_GENERATE_V4() PRIMARY KEY,
    Name VARCHAR(255),
    ContactDetails VARCHAR(255)
);

-- Create the Schedule table
CREATE TABLE Schedule (
    ScheduleID UUID DEFAULT UUID_GENERATE_V4() PRIMARY KEY,
    ClassID UUID NOT NULL,
    Subject VARCHAR(255),
    Day VARCHAR(20),
    Time TIME,
    StaffID UUID NOT NULL,
    FOREIGN KEY (ClassID) REFERENCES Classes(ClassID),
    FOREIGN KEY (StaffID) REFERENCES Staff(StaffID)
);

-- Create the Classes table
CREATE TABLE Classes (
    ClassID UUID DEFAULT UUID_GENERATE_V4() PRIMARY KEY,
    Name VARCHAR(255),
    GradeLevel VARCHAR(20),
    TeacherID UUID NOT NULL,
    FOREIGN KEY (TeacherID) REFERENCES Staff(StaffID)
);

-- Create the Subjects table
CREATE TABLE Subjects (
    SubjectID UUID DEFAULT UUID_GENERATE_V4() PRIMARY KEY,
    Name VARCHAR(255),
    Description TEXT,
    GradeLevel VARCHAR(20)
);

-- Create the Exams table
CREATE TABLE Exams (
    ExamID UUID DEFAULT UUID_GENERATE_V4() PRIMARY KEY,
    SubjectID UUID NOT NULL,
    Date DATE,
    Type VARCHAR(20),
    FOREIGN KEY (SubjectID) REFERENCES Subjects(SubjectID)
);

-- Create the Invoices table
CREATE TABLE Invoices (
    InvoiceID UUID DEFAULT UUID_GENERATE_V4() PRIMARY KEY,
    StudentID UUID NOT NULL,
    IssueDate DATE,
    DueDate DATE,
    TotalAmount DECIMAL(10, 2),
    Status VARCHAR(20),
    PaymentMethod VARCHAR(20),
    TransactionReference VARCHAR(255),
    FOREIGN KEY (StudentID) REFERENCES Students(StudentID)
);

-- Create the Payments table
CREATE TABLE Payments (
    PaymentID UUID DEFAULT UUID_GENERATE_V4() PRIMARY KEY,
    InvoiceID UUID NOT NULL,
    Date DATE,
    Amount DECIMAL(10, 2),
    PaymentMethod VARCHAR(20),
    TransactionReference VARCHAR(255),
    FOREIGN KEY (InvoiceID) REFERENCES Invoices(InvoiceID)
);

-- Create the Fees table
CREATE TABLE Fees (
    FeeID UUID DEFAULT UUID_GENERATE_V4() PRIMARY KEY,
    StudentID UUID NOT NULL,
    Description VARCHAR(255),
    Amount DECIMAL(10, 2),
    DueDate DATE,
    FOREIGN KEY (StudentID) REFERENCES Students(StudentID)
);

-- Create the ParentPickup table
CREATE TABLE Pickup (
    PickupID UUID DEFAULT UUID_GENERATE_V4() PRIMARY KEY,
    ParentID UUID NOT NULL,
    StudentID UUID NOT NULL,
    PickupDate DATE,
    PickupTime TIME,
    AuthorizedGuardian VARCHAR(255),
    FOREIGN KEY (ParentID) REFERENCES Parents(ParentID),
    FOREIGN KEY (StudentID) REFERENCES Students(StudentID)
);