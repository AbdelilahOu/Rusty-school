use super::types::*;
use super::utils::filters::*;
use ::entity::prelude::*;
use sea_orm::{prelude::Uuid, *};
use serde_json::{json, Value as SerdValue};

pub struct ServiceQuery;

type JsonV = JsonValue;
type VecJsonV = Vec<JsonV>;

impl ServiceQuery {
    // students entity
    pub async fn list_students(db: &DbConn, qf: QueriesFilters) -> Result<VecJsonV, String> {
        let list_students = Student::find()
            .offset((qf.queries.page - 1) * qf.queries.limit)
            .limit(qf.queries.limit)
            .filter(generate_student_filters(qf.filters))
            .into_json()
            .all(db)
            .await;

        match list_students {
            Ok(students) => Ok(students),
            Err(err) => Err(err.to_string()),
        }
    }
    //
    pub async fn get_student(db: &DbConn, id: Uuid) -> Result<JsonV, String> {
        let selected_student = Student::find_by_id(id).into_json().one(db).await;
        match selected_student {
            Ok(student_op) => match student_op {
                Some(some_student) => Ok(some_student),
                None => Err(String::from("student doesnt exist")),
            },
            Err(e) => Err(e.to_string()),
        }
    }
    //
    pub async fn list_teachers(db: &DbConn, qf: QueriesFilters) -> Result<VecJsonV, String> {
        let list_teachers = Teacher::find()
            .offset((qf.queries.page - 1) * qf.queries.limit)
            .limit(qf.queries.limit)
            .filter(generate_teacher_filters(qf.filters))
            .into_json()
            .all(db)
            .await;

        match list_teachers {
            Ok(teachers) => Ok(teachers),
            Err(err) => Err(err.to_string()),
        }
    }
    //
    pub async fn get_teacher(db: &DbConn, id: Uuid) -> Result<JsonV, String> {
        let selected_teacher = Teacher::find_by_id(id).into_json().one(db).await;
        match selected_teacher {
            Ok(teacher_op) => match teacher_op {
                Some(teacher) => Ok(teacher),
                None => Err(String::from("teacher doesnt exist")),
            },
            Err(e) => Err(e.to_string()),
        }
    }
    //
    pub async fn list_parents(db: &DbConn, qf: QueriesFilters) -> Result<VecJsonV, String> {
        let list_parents = Parent::find()
            .offset((qf.queries.page - 1) * qf.queries.limit)
            .limit(qf.queries.limit)
            .filter(generate_parent_filters(qf.filters))
            .into_json()
            .all(db)
            .await;

        match list_parents {
            Ok(parents) => Ok(parents),
            Err(err) => Err(err.to_string()),
        }
    }
    //
    pub async fn get_parent(db: &DbConn, id: Uuid) -> Result<JsonV, String> {
        let selected_parent = Parent::find_by_id(id).into_json().one(db).await;
        match selected_parent {
            Ok(parent_op) => match parent_op {
                Some(parent) => Ok(parent),
                None => Err(String::from("parent doesnt exist")),
            },
            Err(e) => Err(e.to_string()),
        }
    }
    //
    pub async fn get_contact(db: &DbConn, id: Uuid) -> Result<JsonV, String> {
        let selected_contact = Contact::find_by_id(id).into_json().one(db).await;
        match selected_contact {
            Ok(contact_op) => match contact_op {
                Some(contact) => Ok(contact),
                None => Err(String::from("contact doesnt exist")),
            },
            Err(e) => Err(e.to_string()),
        }
    }
    //
    pub async fn list_contacts(db: &DbConn, qf: QueriesFilters) -> Result<VecJsonV, String> {
        let list_contacts = Contact::find()
            .offset((qf.queries.page - 1) * qf.queries.limit)
            .limit(qf.queries.limit)
            // .filter(generate_contact_filters(qf.filters))
            .into_json()
            .all(db)
            .await;

        match list_contacts {
            Ok(contacts) => Ok(contacts),
            Err(err) => Err(err.to_string()),
        }
    }
    //
    pub async fn list_contacts_ex(db: &DbConn, qf: QueriesFilters) -> Result<VecJsonV, String> {
        let list_contacts = Contact::find()
            .offset((qf.queries.page - 1) * qf.queries.limit)
            .limit(qf.queries.limit)
            // .filter(generate_contact_filters(qf.filters))
            // .into_json()
            .all(db)
            .await;

        match list_contacts {
            Ok(contacts) => {
                let mut result = Vec::<SerdValue>::new();
                for contact in contacts {
                    let country = contact
                        .find_related(Country)
                        .into_json()
                        .all(db)
                        .await
                        .unwrap_or(Vec::new());
                    let state = contact
                        .find_related(State)
                        .into_json()
                        .all(db)
                        .await
                        .unwrap_or(Vec::new());
                    let city = contact
                        .find_related(City)
                        .into_json()
                        .all(db)
                        .await
                        .unwrap_or(Vec::new());

                    let ditsrict = contact
                        .find_related(District)
                        .into_json()
                        .all(db)
                        .await
                        .unwrap_or(Vec::new());

                    let street = contact
                        .find_related(Street)
                        .into_json()
                        .all(db)
                        .await
                        .unwrap_or(Vec::new());

                    result.push(json!({
                        "id": contact.id,
                        "phone": contact.phone_number,
                        "email": contact.email,
                        "country": country,
                        "state": state,
                        "city": city,
                        "district": ditsrict,
                        "street": street,
                    }));
                }
                Ok(result)
            }
            Err(err) => Err(err.to_string()),
        }
    }
    //
    pub async fn get_country(db: &DbConn, id: Uuid) -> Result<JsonV, String> {
        let selected_country = Country::find_by_id(id).into_json().one(db).await;
        match selected_country {
            Ok(country_op) => match country_op {
                Some(country) => Ok(country),
                None => Err(String::from("country doesnt exist")),
            },
            Err(e) => Err(e.to_string()),
        }
    }
    //
    pub async fn list_countries(db: &DbConn, qf: QueriesFilters) -> Result<VecJsonV, String> {
        let list_countries = Country::find()
            .offset((qf.queries.page - 1) * qf.queries.limit)
            .limit(qf.queries.limit)
            // .filter(generate_country_filters(qf.filters))
            .into_json()
            .all(db)
            .await;

        match list_countries {
            Ok(countries) => Ok(countries),
            Err(err) => Err(err.to_string()),
        }
    }
    //
    pub async fn list_countries_ex(db: &DbConn, qf: QueriesFilters) -> Result<VecJsonV, String> {
        let list_countries = Country::find()
            .offset((qf.queries.page - 1) * qf.queries.limit)
            .limit(qf.queries.limit)
            // .filter(generate_countrie_filters(qf.filters))
            .find_with_related(State)
            .all(db)
            .await;

        match list_countries {
            Ok(countries) => {
                // init result
                let mut result = Vec::<SerdValue>::new();
                // map through countries
                for (country, states) in countries {
                    // populate res
                    let states_json = states
                        .into_iter()
                        .map(|state| json!({ "id": state.id, "name": state.state_name, "initiales": state.state_initials, "code": state.state_code  }))
                        .collect::<VecJsonV>();

                    let countries_json = json!({
                        "id": country.id,
                        "name": country.country_name,
                        "states": states_json
                    });

                    result.push(countries_json);
                }
                Ok(result)
            }
            Err(err) => Err(err.to_string()),
        }
    }
    //
    pub async fn list_states(db: &DbConn, qf: QueriesFilters) -> Result<VecJsonV, String> {
        let list_states = State::find()
            .offset((qf.queries.page - 1) * qf.queries.limit)
            .limit(qf.queries.limit)
            // .filter(generate_state_filters(qf.filters))
            .into_json()
            .all(db)
            .await;

        match list_states {
            Ok(states) => Ok(states),
            Err(err) => Err(err.to_string()),
        }
    }
    //
    pub async fn list_states_ex(db: &DbConn, qf: QueriesFilters) -> Result<VecJsonV, String> {
        let list_states = State::find()
            .offset((qf.queries.page - 1) * qf.queries.limit)
            .limit(qf.queries.limit)
            // .filter(generate_state_filters(qf.filters))
            .find_with_related(City)
            .all(db)
            .await;

        match list_states {
            Ok(states) => {
                // init result
                let mut result = Vec::<SerdValue>::new();
                // map through states
                for (state, cities) in states {
                    // populate res
                    let cities_json = cities
                        .into_iter()
                        .map(|city| json!({ "id": city.id, "name": city.city_name }))
                        .collect::<VecJsonV>();

                    let state_json = json!({
                        "id": state.id,
                        "name": state.state_name,
                        "initials": state.state_initials,
                        "code": state.state_code,
                        "cities": cities_json
                    });

                    result.push(state_json);
                }
                Ok(result)
            }
            Err(err) => Err(err.to_string()),
        }
    }
    //
    pub async fn list_cities(db: &DbConn, qf: QueriesFilters) -> Result<VecJsonV, String> {
        let list_cities = City::find()
            .offset((qf.queries.page - 1) * qf.queries.limit)
            .limit(qf.queries.limit)
            // .filter(generate_state_filters(qf.filters))
            .into_json()
            .all(db)
            .await;

        match list_cities {
            Ok(cities) => Ok(cities),
            Err(err) => Err(err.to_string()),
        }
    }
    //
    pub async fn list_cities_ex(db: &DbConn, qf: QueriesFilters) -> Result<VecJsonV, String> {
        let list_cities = City::find()
            .offset((qf.queries.page - 1) * qf.queries.limit)
            .limit(qf.queries.limit)
            // .filter(generate_countrie_filters(qf.filters))
            .find_with_related(District)
            .all(db)
            .await;

        match list_cities {
            Ok(cities) => {
                // init result
                let mut result = Vec::<SerdValue>::new();
                // map through cities
                for (city, districts) in cities {
                    // populate res
                    let districts_json = districts
                        .into_iter()
                        .map(|district| json!({ "id": district.id, "name": district.district_name  }))
                        .collect::<VecJsonV>();

                    let cities_json = json!({
                        "id": city.id,
                        "name": city.city_name,
                        "districts": districts_json
                    });

                    result.push(cities_json);
                }
                Ok(result)
            }
            Err(err) => Err(err.to_string()),
        }
    }
    //
    pub async fn list_districts(db: &DbConn, qf: QueriesFilters) -> Result<VecJsonV, String> {
        let list_districts = District::find()
            .offset((qf.queries.page - 1) * qf.queries.limit)
            .limit(qf.queries.limit)
            // .filter(generate_state_filters(qf.filters))
            .into_json()
            .all(db)
            .await;

        match list_districts {
            Ok(districts) => Ok(districts),
            Err(err) => Err(err.to_string()),
        }
    }
    //
    pub async fn list_districts_ex(db: &DbConn, qf: QueriesFilters) -> Result<VecJsonV, String> {
        let list_districts = District::find()
            .offset((qf.queries.page - 1) * qf.queries.limit)
            .limit(qf.queries.limit)
            // .filter(generate_countrie_filters(qf.filters))
            .find_with_related(Street)
            .all(db)
            .await;

        match list_districts {
            Ok(districts) => {
                // init result
                let mut result = Vec::<SerdValue>::new();
                // map through districts
                for (district, streets) in districts {
                    // populate res
                    let streets_json = streets
                        .into_iter()
                        .map(|street| json!({ "id": street.id, "name": street.street_name, "type":street.street_type, "code":street.zip_code  }))
                        .collect::<VecJsonV>();

                    let districts_json = json!({
                        "id": district.id,
                        "name": district.district_name,
                        "districts": streets_json
                    });

                    result.push(districts_json);
                }
                Ok(result)
            }
            Err(err) => Err(err.to_string()),
        }
    }
    //
    pub async fn list_streets(db: &DbConn, qf: QueriesFilters) -> Result<VecJsonV, String> {
        let list_streets = Street::find()
            .offset((qf.queries.page - 1) * qf.queries.limit)
            .limit(qf.queries.limit)
            // .filter(generate_state_filters(qf.filters))
            .into_json()
            .all(db)
            .await;

        match list_streets {
            Ok(streets) => Ok(streets),
            Err(err) => Err(err.to_string()),
        }
    }
}
