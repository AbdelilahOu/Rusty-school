use fake::{
    faker::{
        address::en::{CityName, CountryCode, CountryName, StateAbbr, StateName, ZipCode},
        internet::en::FreeEmail,
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
        level_name: Faker.fake(),
        level_description: Faker.fake(),
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
    return RandDetails {
        phone_number: PhoneNumber().fake(),
        email: FreeEmail().fake(),
    };
}
