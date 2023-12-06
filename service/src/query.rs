use super::types::*;
use super::utils::filters::*;
use ::entity::prelude::*;
use sea_orm::{prelude::Uuid, *};
use serde_json::{json, Value as SerdValue};

type JsonV = SerdValue;
type VecJsonV = Vec<JsonV>;

pub struct ServiceQuery;

impl ServiceQuery {
    // students entity
    pub async fn list_students(db: &DbConn, qf: QueriesFilters) -> Result<VecJsonV, DbErr> {
        let students = Student::find()
            .offset((qf.queries.page - 1) * qf.queries.limit)
            .limit(qf.queries.limit)
            .filter(generate_student_filters(qf.filters))
            .into_json()
            .all(db)
            .await?;

        Ok(students)
    }
    //
    pub async fn get_student(db: &DbConn, id: Uuid) -> Result<Option<JsonV>, DbErr> {
        let student = Student::find_by_id(id).into_json().one(db).await?;
        Ok(student)
    }
    //
    pub async fn list_teachers(db: &DbConn, qf: QueriesFilters) -> Result<VecJsonV, DbErr> {
        let teachers = Teacher::find()
            .offset((qf.queries.page - 1) * qf.queries.limit)
            .limit(qf.queries.limit)
            .filter(generate_teacher_filters(qf.filters))
            .into_json()
            .all(db)
            .await?;

        Ok(teachers)
    }
    //
    pub async fn get_teacher(db: &DbConn, id: Uuid) -> Result<Option<JsonV>, DbErr> {
        let teacher = Teacher::find_by_id(id).into_json().one(db).await?;
        Ok(teacher)
    }
    //
    pub async fn list_parents(db: &DbConn, qf: QueriesFilters) -> Result<VecJsonV, DbErr> {
        let parents = Parent::find()
            .offset((qf.queries.page - 1) * qf.queries.limit)
            .limit(qf.queries.limit)
            .filter(generate_parent_filters(qf.filters))
            .into_json()
            .all(db)
            .await?;

        Ok(parents)
    }
    //
    pub async fn get_parent(db: &DbConn, id: Uuid) -> Result<Option<JsonV>, DbErr> {
        let parent = Parent::find_by_id(id).into_json().one(db).await?;
        Ok(parent)
    }
    //
    pub async fn get_contact(db: &DbConn, id: Uuid) -> Result<Option<JsonV>, DbErr> {
        let contact = Contact::find_by_id(id).into_json().one(db).await?;
        Ok(contact)
    }
    //
    pub async fn list_contacts(db: &DbConn, qf: QueriesFilters) -> Result<VecJsonV, DbErr> {
        let contacts = Contact::find()
            .offset((qf.queries.page - 1) * qf.queries.limit)
            .limit(qf.queries.limit)
            .into_json()
            .all(db)
            .await?;

        Ok(contacts)
    }
    //
    pub async fn list_contacts_ex(db: &DbConn, qf: QueriesFilters) -> Result<VecJsonV, DbErr> {
        let contacts = Contact::find()
            .offset((qf.queries.page - 1) * qf.queries.limit)
            .limit(qf.queries.limit)
            .all(db)
            .await?;

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
    //
    pub async fn get_country(db: &DbConn, id: Uuid) -> Result<Option<JsonV>, DbErr> {
        let country = Country::find_by_id(id).into_json().one(db).await?;
        Ok(country)
    }
    //
    pub async fn list_countries(db: &DbConn, qf: QueriesFilters) -> Result<VecJsonV, DbErr> {
        let countries = Country::find()
            .offset((qf.queries.page - 1) * qf.queries.limit)
            .limit(qf.queries.limit)
            .into_json()
            .all(db)
            .await?;

        Ok(countries)
    }
    //
    pub async fn list_countries_ex(db: &DbConn, qf: QueriesFilters) -> Result<VecJsonV, DbErr> {
        let countries = Country::find()
            .offset((qf.queries.page - 1) * qf.queries.limit)
            .limit(qf.queries.limit)
            .find_with_related(State)
            .all(db)
            .await?;

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
    //
    pub async fn list_states(db: &DbConn, qf: QueriesFilters) -> Result<VecJsonV, DbErr> {
        let states = State::find()
            .offset((qf.queries.page - 1) * qf.queries.limit)
            .limit(qf.queries.limit)
            .into_json()
            .all(db)
            .await?;

        Ok(states)
    }
    //
    pub async fn list_states_ex(db: &DbConn, qf: QueriesFilters) -> Result<VecJsonV, DbErr> {
        let states = State::find()
            .offset((qf.queries.page - 1) * qf.queries.limit)
            .limit(qf.queries.limit)
            .find_with_related(City)
            .all(db)
            .await?;

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
    //
    pub async fn list_cities(db: &DbConn, qf: QueriesFilters) -> Result<VecJsonV, DbErr> {
        let cities = City::find()
            .offset((qf.queries.page - 1) * qf.queries.limit)
            .limit(qf.queries.limit)
            .into_json()
            .all(db)
            .await?;

        Ok(cities)
    }
    //
    pub async fn list_cities_ex(db: &DbConn, qf: QueriesFilters) -> Result<VecJsonV, DbErr> {
        let cities = City::find()
            .offset((qf.queries.page - 1) * qf.queries.limit)
            .limit(qf.queries.limit)
            .find_with_related(District)
            .all(db)
            .await?;

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
    //
    pub async fn list_districts(db: &DbConn, qf: QueriesFilters) -> Result<VecJsonV, DbErr> {
        let districts = District::find()
            .offset((qf.queries.page - 1) * qf.queries.limit)
            .limit(qf.queries.limit)
            .into_json()
            .all(db)
            .await?;

        Ok(districts)
    }
    //
    pub async fn list_districts_ex(db: &DbConn, qf: QueriesFilters) -> Result<VecJsonV, DbErr> {
        let districts = District::find()
            .offset((qf.queries.page - 1) * qf.queries.limit)
            .limit(qf.queries.limit)
            .find_with_related(Street)
            .all(db)
            .await?;

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
    //
    pub async fn list_streets(db: &DbConn, qf: QueriesFilters) -> Result<VecJsonV, DbErr> {
        let streets = Street::find()
            .offset((qf.queries.page - 1) * qf.queries.limit)
            .limit(qf.queries.limit)
            .into_json()
            .all(db)
            .await?;

        Ok(streets)
    }
    //
    pub async fn list_scans(db: &DbConn, qf: QueriesFilters) -> Result<VecJsonV, DbErr> {
        let scans = Scans::find()
            .offset((qf.queries.page - 1) * qf.queries.limit)
            .limit(qf.queries.limit)
            .all(db)
            .await?;

        let mut result = Vec::<SerdValue>::new();
        for scan in scans {
            let person = scan.find_related(Persons).one(db).await?;
            let related = match person.clone().unwrap().person_type.as_str() {
                "student" => {
                    let student = person
                        .unwrap()
                        .find_related(Student)
                        .into_json()
                        .one(db)
                        .await?;
                    student
                }
                "parent" => {
                    let parent = person
                        .unwrap()
                        .find_related(Parent)
                        .into_json()
                        .one(db)
                        .await?;
                    parent
                }
                "teacher" => {
                    let teacher = person
                        .unwrap()
                        .find_related(Teacher)
                        .into_json()
                        .one(db)
                        .await?;
                    teacher
                }
                _ => continue,
            };

            result.push(json!({
                "id": scan.id,
                "date": scan.scan_date,
                "person_id":scan.person_id,
                "person": related
            }));
        }
        Ok(result)
    }
}
