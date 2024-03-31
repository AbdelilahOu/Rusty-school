use std::ops::Range;

use fake::{
    faker::{
        company::en::Industry,
        lorem::en::{Sentence, Word},
        name::en::{FirstName, LastName},
    },
    Fake, Faker,
};

pub struct RandStudent {
    pub first_name: String,
    pub last_name: String,
}

pub fn generate_random_student() -> RandStudent {
    return RandStudent {
        first_name: FirstName().fake(),
        last_name: LastName().fake(),
    };
}

pub struct RandTeacher {
    pub first_name: String,
    pub last_name: String,
}

pub fn generate_random_teacher() -> RandTeacher {
    return RandTeacher {
        first_name: FirstName().fake(),
        last_name: LastName().fake(),
    };
}

pub struct RandLevel {
    pub level_name: String,
    pub level_description: String,
}

pub fn generate_random_level() -> RandLevel {
    return RandLevel {
        level_name: Word().fake(),
        level_description: Sentence(Range { start: 5, end: 10 }).fake(),
    };
}

pub struct RandRoom {
    pub room_name: String,
    pub room_description: String,
}

pub fn generate_random_room() -> RandRoom {
    return RandRoom {
        room_name: format!("{}-{}", Faker.fake::<u8>(), Industry().fake::<String>()),
        room_description: Sentence(Range { start: 10, end: 15 }).fake(),
    };
}

pub struct RandGroup {
    pub group_name: String,
    pub group_description: String,
}

pub fn generate_random_group() -> RandGroup {
    return RandGroup {
        group_name: Word().fake(),
        group_description: Sentence(Range { start: 5, end: 10 }).fake(),
    };
}

pub struct RandSubject {
    pub subject_name: String,
    pub subject_description: String,
}

pub fn generate_random_subject() -> RandSubject {
    return RandSubject {
        subject_name: Industry().fake(),
        subject_description: Sentence(Range { start: 5, end: 10 }).fake(),
    };
}

pub struct RandParent {
    pub first_name: String,
    pub last_name: String,
}

pub fn generate_random_parent() -> RandParent {
    return RandParent {
        first_name: FirstName().fake(),
        last_name: LastName().fake(),
    };
}

pub struct RandActivity {
    pub title: String,
    pub description: String,
    pub activity_type: String,
}

pub fn generate_random_activity() -> RandActivity {
    return RandActivity {
        title: Word().fake(),
        description: Sentence(Range { start: 5, end: 10 }).fake(),
        activity_type: Word().fake(),
    };
}

pub struct RandEvent {
    pub title: String,
    pub description: String,
}

pub fn generate_random_event() -> RandEvent {
    return RandEvent {
        title: Word().fake(),
        description: Sentence(Range { start: 5, end: 10 }).fake(),
    };
}
