use super::types::*;
use super::utils::filters::*;
use ::entity::{groups, prelude::*, subjects};
use sea_orm::{prelude::Uuid, *};
use serde_json::{json, Value as SerdValue};

type JsonV = SerdValue;
type Values = Vec<JsonV>;

pub struct QueriesService;

impl QueriesService {
    // students entity
    pub async fn list_students(db: &DbConn, qf: QueriesFilters) -> Result<Values, DbErr> {
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
    pub async fn list_teachers(db: &DbConn, qf: QueriesFilters) -> Result<Values, DbErr> {
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
    pub async fn list_parents(db: &DbConn, qf: QueriesFilters) -> Result<Values, DbErr> {
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
    pub async fn get_details(db: &DbConn, id: Uuid) -> Result<Option<JsonV>, DbErr> {
        let details = PersonDetails::find_by_id(id).into_json().one(db).await?;
        Ok(details)
    }
    //
    pub async fn list_details(db: &DbConn, qf: QueriesFilters) -> Result<Values, DbErr> {
        let details = PersonDetails::find()
            .offset((qf.queries.page - 1) * qf.queries.limit)
            .limit(qf.queries.limit)
            .into_json()
            .all(db)
            .await?;

        Ok(details)
    }
    //
    pub async fn list_details_ex(db: &DbConn, qf: QueriesFilters) -> Result<Values, DbErr> {
        let details = PersonDetails::find()
            .offset((qf.queries.page - 1) * qf.queries.limit)
            .limit(qf.queries.limit)
            .all(db)
            .await?;

        let mut result = Vec::<SerdValue>::new();
        for details in details {
            let country = details
                .find_related(Country)
                .into_json()
                .all(db)
                .await
                .unwrap_or(Vec::new());

            let state = details
                .find_related(State)
                .into_json()
                .all(db)
                .await
                .unwrap_or(Vec::new());

            let city = details
                .find_related(City)
                .into_json()
                .all(db)
                .await
                .unwrap_or(Vec::new());

            let ditsrict = details
                .find_related(District)
                .into_json()
                .all(db)
                .await
                .unwrap_or(Vec::new());

            let street = details
                .find_related(Street)
                .into_json()
                .all(db)
                .await
                .unwrap_or(Vec::new());

            result.push(json!({
                "id": details.id,
                "phone": details.phone_number,
                "email": details.email,
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
    pub async fn list_countries(db: &DbConn, qf: QueriesFilters) -> Result<Values, DbErr> {
        let countries = Country::find()
            .offset((qf.queries.page - 1) * qf.queries.limit)
            .limit(qf.queries.limit)
            .into_json()
            .all(db)
            .await?;

        Ok(countries)
    }
    //
    pub async fn list_countries_ex(db: &DbConn, qf: QueriesFilters) -> Result<Values, DbErr> {
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
                        .map(|state| json!({ "id": state.id, "name": state.state_name, "initiales": state.state_initials  }))
                        .collect::<Values>();

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
    pub async fn list_states(db: &DbConn, qf: QueriesFilters) -> Result<Values, DbErr> {
        let states = State::find()
            .offset((qf.queries.page - 1) * qf.queries.limit)
            .limit(qf.queries.limit)
            .into_json()
            .all(db)
            .await?;

        Ok(states)
    }
    //
    pub async fn list_states_ex(db: &DbConn, qf: QueriesFilters) -> Result<Values, DbErr> {
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
                .collect::<Values>();

            let state_json = json!({
                "id": state.id,
                "name": state.state_name,
                "initials": state.state_initials,
                "cities": cities_json
            });

            result.push(state_json);
        }
        Ok(result)
    }
    //
    pub async fn list_cities(db: &DbConn, qf: QueriesFilters) -> Result<Values, DbErr> {
        let cities = City::find()
            .offset((qf.queries.page - 1) * qf.queries.limit)
            .limit(qf.queries.limit)
            .into_json()
            .all(db)
            .await?;

        Ok(cities)
    }
    //
    pub async fn list_cities_ex(db: &DbConn, qf: QueriesFilters) -> Result<Values, DbErr> {
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
                .collect::<Values>();

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
    pub async fn list_districts(db: &DbConn, qf: QueriesFilters) -> Result<Values, DbErr> {
        let districts = District::find()
            .offset((qf.queries.page - 1) * qf.queries.limit)
            .limit(qf.queries.limit)
            .into_json()
            .all(db)
            .await?;

        Ok(districts)
    }
    //
    pub async fn list_districts_ex(db: &DbConn, qf: QueriesFilters) -> Result<Values, DbErr> {
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
                        .collect::<Values>();

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
    pub async fn list_streets(db: &DbConn, qf: QueriesFilters) -> Result<Values, DbErr> {
        let streets = Street::find()
            .offset((qf.queries.page - 1) * qf.queries.limit)
            .limit(qf.queries.limit)
            .into_json()
            .all(db)
            .await?;

        Ok(streets)
    }
    //
    pub async fn list_scans_related(db: &DbConn, qf: QueriesFilters) -> Result<Values, DbErr> {
        let scans = Scans::find()
            .offset((qf.queries.page - 1) * qf.queries.limit)
            .limit(qf.queries.limit)
            .all(db)
            .await?;

        // SELECT
        // s.*,
        // p.person_type,
        // CASE
        //     WHEN p.person_type = 'student' THEN (SELECT full_name FROM students where students.person_id = p.id)
        //     WHEN p.person_type = 'parent' THEN (SELECT full_name FROM parents where parents.person_id = p.id)
        //     ELSE (SELECT full_name FROM teachers where teachers.person_id = p.id)
        // END,
        // CASE
        //     WHEN p.person_type = 'student' THEN (SELECT id FROM students where students.person_id = p.id)
        //     WHEN p.person_type = 'parent' THEN (SELECT id FROM parents where parents.person_id = p.id)
        //     ELSE (SELECT id FROM teachers where teachers.person_id = p.id)
        // END as _id
        // FROM scans as s JOIN persons as p ON s.person_id = p.id;

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
                _ => None,
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
    //
    pub async fn list_levels(db: &DbConn, qf: QueriesFilters) -> Result<Values, DbErr> {
        let levels = Level::find()
            .offset((qf.queries.page - 1) * qf.queries.limit)
            .limit(qf.queries.limit)
            .into_json()
            .all(db)
            .await?;

        Ok(levels)
    }
    //
    pub async fn list_subjects(db: &DbConn, qf: QueriesFilters) -> Result<Values, DbErr> {
        let subjects = Subject::find()
            .offset((qf.queries.page - 1) * qf.queries.limit)
            .limit(qf.queries.limit)
            .into_json()
            .all(db)
            .await?;

        Ok(subjects)
    }
    //
    pub async fn list_level_subjects(db: &DbConn, level_id: Uuid) -> Result<Values, DbErr> {
        let level_subjects = Subject::find()
            .filter(subjects::Column::LevelId.eq(level_id.clone()))
            .into_json()
            .all(db)
            .await?;

        Ok(level_subjects)
    }
    pub async fn list_groups(db: &DbConn, qf: QueriesFilters) -> Result<Values, DbErr> {
        let groups = Group::find()
            .offset((qf.queries.page - 1) * qf.queries.limit)
            .limit(qf.queries.limit)
            .into_json()
            .all(db)
            .await?;

        Ok(groups)
    }
    pub async fn list_level_groups(db: &DbConn, level_id: Uuid) -> Result<Values, DbErr> {
        let level_groups = Group::find()
            .filter(groups::Column::LevelId.eq(level_id.clone()))
            .into_json()
            .all(db)
            .await?;

        Ok(level_groups)
    }
    //
    pub async fn list_rooms(db: &DbConn) -> Result<Values, DbErr> {
        let rooms = Room::find().into_json().all(db).await?;

        Ok(rooms)
    }
    //
    pub async fn list_classes(db: &DbConn) -> Result<Values, DbErr> {
        let classes = Class::find().into_json().all(db).await?;

        Ok(classes)
    }
}
