use std::ops::Range;

use fake::{
    faker::{
        address::en::{CityName, CountryCode, CountryName, StateAbbr, StateName, ZipCode},
        company::en::Industry,
        internet::en::FreeEmail,
        lorem::en::{Sentence, Word},
        name::en::{FirstName, LastName},
        phone_number::en::PhoneNumber,
    },
    Fake, Faker,
};
use sea_orm::prelude::Uuid;

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

pub struct RandCountry {
    pub country_name: String,
    pub country_initials: String,
}

pub fn generate_random_country() -> RandCountry {
    return RandCountry {
        country_name: CountryName().fake(),
        country_initials: CountryCode().fake(),
    };
}

pub struct RandState {
    pub state_name: String,
    pub state_initials: String,
    pub country_id: Uuid,
}

pub fn generate_random_state(country_id: Uuid) -> RandState {
    return RandState {
        state_name: StateName().fake(),
        state_initials: StateAbbr().fake(),
        country_id,
    };
}

pub struct RandCity {
    pub city_name: String,
    pub state_id: Uuid,
}

pub fn generate_random_city(state_id: Uuid) -> RandCity {
    return RandCity {
        city_name: CityName().fake(),
        state_id,
    };
}

pub struct RandDistrict {
    pub district_name: String,
    pub city_id: Uuid,
}

pub fn generate_random_district(city_id: Uuid) -> RandDistrict {
    return RandDistrict {
        district_name: Faker.fake(),
        city_id,
    };
}

pub struct RandStreet {
    pub street_name: String,
    pub zip_code: String,
    pub street_type: String,
    pub district_id: Uuid,
}

pub fn generate_random_street(district_id: Uuid) -> RandStreet {
    return RandStreet {
        street_name: Faker.fake(),
        zip_code: ZipCode().fake(),
        street_type: Faker.fake(),
        district_id,
    };
}

pub struct RandDetails {
    pub phone_number: String,
    pub email: String,
}

pub fn generate_random_details() -> RandDetails {
    let email: String = FreeEmail().fake();
    let rand_number: u32 = Faker.fake();
    return RandDetails {
        phone_number: PhoneNumber().fake(),
        email: format!("{}{}", rand_number, email),
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
