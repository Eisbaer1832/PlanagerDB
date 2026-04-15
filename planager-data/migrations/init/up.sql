CREATE TABLE classes (
    year INTEGER NOT NULL,
    postFix TEXT,
    Primary Key (year, postFix)
);

CREATE TABLE lessons (
    subject TEXT NOT NULL ,
    subjectId TEXT NOT NULL ,
    teacher TEXT NOT NULL ,
    time INTEGER NOT NULL ,
    note TEXT,
    canceled INTEGER,
    year INTEGER NOT NULL,
    postFix TEXT NOT NULL,
    FOREIGN KEY (year, postFix) REFERENCES  classes(year, postFix),
    PRIMARY KEY (subject, teacher, time)
);

