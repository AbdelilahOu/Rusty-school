use ::entity::prelude::*;
use sea_orm::{prelude::Uuid, *};

use super::types::*;

pub struct ServiceMutation;

type CResult<T> = Result<T, DbErr>;
type GResult<T> = Result<T, String>;

impl ServiceMutation {
    // students entity
    pub async fn create_student(db: &DbConn, data: CStudent) -> CResult<Uuid> {
        let c_student = StudentActiveModel {
            first_name: Set(data.first_name),
            last_name: Set(data.last_name),
            level: Set(data.level),
            ..Default::default()
        };
        match Student::insert(c_student).exec(db).await {
            Ok(res) => Ok(res.last_insert_id),
            Err(err) => return Err(err),
        }
    }
    pub async fn delete_student(db: &DbConn, id: Uuid) -> GResult<u64> {
        let selected_student = Student::find_by_id(id).one(db).await;
        match selected_student {
            Ok(student_opt) => match student_opt {
                Some(student_model) => match student_model.delete(db).await {
                    Ok(delete_a) => Ok(delete_a.rows_affected),
                    Err(err) => Err(err.to_string()),
                },
                None => Err(String::from("student doesnt exist")),
            },
            Err(err) => Err(err.to_string()),
        }
    }
    pub async fn update_student(db: &DbConn, id: Uuid, data: CStudent) -> GResult<CStudent> {
        let selected_student = Student::find_by_id(id).one(db).await;
        match selected_student {
            Ok(student_opt) => match student_opt {
                Some(student_model) => {
                    let mut student_model: StudentActiveModel = student_model.into();
                    student_model.first_name = Set(data.first_name);
                    student_model.last_name = Set(data.last_name);
                    student_model.level = Set(data.level);
                    student_model.id = Set(id);
                    match student_model.update(db).await {
                        Ok(updated_student) => Ok(CStudent {
                            first_name: updated_student.first_name,
                            last_name: updated_student.last_name,
                            level: updated_student.level,
                        }),
                        Err(err) => Err(err.to_string()),
                    }
                }
                None => Err(String::from("student doesnt exist")),
            },
            Err(err) => Err(err.to_string()),
        }
    }
    // teachers entity
    pub async fn create_teacher(db: &DbConn, data: CTeacher) -> CResult<Uuid> {
        let c_teacher = TeacherActiveModel {
            first_name: Set(data.first_name),
            last_name: Set(data.last_name),
            ..Default::default()
        };
        match Teacher::insert(c_teacher).exec(db).await {
            Ok(res) => Ok(res.last_insert_id),
            Err(err) => return Err(err),
        }
    }
    pub async fn delete_teacher(db: &DbConn, id: Uuid) -> GResult<u64> {
        let selected_teacher = Teacher::find_by_id(id).one(db).await;
        match selected_teacher {
            Ok(teacher_opt) => match teacher_opt {
                Some(teacher_model) => match teacher_model.delete(db).await {
                    Ok(delete_a) => Ok(delete_a.rows_affected),
                    Err(err) => Err(err.to_string()),
                },
                None => Err(String::from("teacher doesnt exist")),
            },
            Err(err) => Err(err.to_string()),
        }
    }
    pub async fn update_teacher(db: &DbConn, id: Uuid, data: CTeacher) -> GResult<CTeacher> {
        let selected_teacher = Teacher::find_by_id(id).one(db).await;
        match selected_teacher {
            Ok(teacher_opt) => match teacher_opt {
                Some(teacher_model) => {
                    let mut teacher_model: TeacherActiveModel = teacher_model.into();
                    teacher_model.first_name = Set(data.first_name);
                    teacher_model.last_name = Set(data.last_name);
                    teacher_model.id = Set(id);
                    match teacher_model.update(db).await {
                        Ok(updated_teacher) => Ok(CTeacher {
                            first_name: updated_teacher.first_name,
                            last_name: updated_teacher.last_name,
                        }),
                        Err(err) => Err(err.to_string()),
                    }
                }
                None => Err(String::from("teacher doesnt exist")),
            },
            Err(err) => Err(err.to_string()),
        }
    }
    // parents entity
    pub async fn create_parent(db: &DbConn, data: CParent) -> CResult<Uuid> {
        let c_parent = ParentActiveModel {
            first_name: Set(data.first_name),
            last_name: Set(data.last_name),
            ..Default::default()
        };
        match Parent::insert(c_parent).exec(db).await {
            Ok(res) => Ok(res.last_insert_id),
            Err(err) => return Err(err),
        }
    }
    pub async fn delete_parent(db: &DbConn, id: Uuid) -> GResult<u64> {
        let selected_parent = Parent::find_by_id(id).one(db).await;
        match selected_parent {
            Ok(parent_opt) => match parent_opt {
                Some(parent_model) => match parent_model.delete(db).await {
                    Ok(delete_a) => Ok(delete_a.rows_affected),
                    Err(err) => Err(err.to_string()),
                },
                None => Err(String::from("parent doesnt exist")),
            },
            Err(err) => Err(err.to_string()),
        }
    }
    pub async fn update_parent(db: &DbConn, id: Uuid, data: CParent) -> GResult<CParent> {
        let selected_parent = Parent::find_by_id(id).one(db).await;
        match selected_parent {
            Ok(parent_opt) => match parent_opt {
                Some(parent_model) => {
                    let mut parent_model: ParentActiveModel = parent_model.into();
                    parent_model.first_name = Set(data.first_name);
                    parent_model.last_name = Set(data.last_name);
                    parent_model.id = Set(id);
                    match parent_model.update(db).await {
                        Ok(updated_parent) => Ok(CParent {
                            first_name: updated_parent.first_name,
                            last_name: updated_parent.last_name,
                        }),
                        Err(err) => Err(err.to_string()),
                    }
                }
                None => Err(String::from("parent doesnt exist")),
            },
            Err(err) => Err(err.to_string()),
        }
    }
    // contacts
    pub async fn create_contact(db: &DbConn, data: CContact) -> CResult<Uuid> {
        let c_contact = ContactActiveModel {
            phone_number: Set(data.phone),
            email: Set(data.email),
            country_id: Set(data.country_id),
            state_id: Set(data.state_id),
            city_id: Set(data.city_id),
            district_id: Set(data.district_id),
            street_id: Set(data.street_id),
            ..Default::default()
        };
        match Contact::insert(c_contact).exec(db).await {
            Ok(res) => Ok(res.last_insert_id),
            Err(err) => return Err(err),
        }
    }
    pub async fn delete_contact(db: &DbConn, id: Uuid) -> GResult<u64> {
        let selected_contact = Contact::find_by_id(id).one(db).await;
        match selected_contact {
            Ok(contact_opt) => match contact_opt {
                Some(contact_model) => match contact_model.delete(db).await {
                    Ok(delete_a) => Ok(delete_a.rows_affected),
                    Err(err) => Err(err.to_string()),
                },
                None => Err(String::from("contact doesnt exist")),
            },
            Err(err) => Err(err.to_string()),
        }
    }
    pub async fn update_contact(db: &DbConn, id: Uuid, data: CContact) -> GResult<CContact> {
        let selected_contact = Contact::find_by_id(id).one(db).await;
        match selected_contact {
            Ok(contact_opt) => match contact_opt {
                Some(contact_model) => {
                    let mut contact_model: ContactActiveModel = contact_model.into();
                    contact_model.phone_number = Set(data.phone);
                    contact_model.email = Set(data.email);
                    contact_model.country_id = Set(data.country_id);
                    contact_model.state_id = Set(data.state_id);
                    contact_model.city_id = Set(data.city_id);
                    contact_model.district_id = Set(data.district_id);
                    contact_model.street_id = Set(data.street_id);
                    contact_model.id = Set(id);
                    match contact_model.update(db).await {
                        Ok(updated_contact) => Ok(CContact {
                            phone: updated_contact.phone_number,
                            email: updated_contact.email,
                            country_id: updated_contact.country_id,
                            state_id: updated_contact.state_id,
                            city_id: updated_contact.city_id,
                            district_id: updated_contact.district_id,
                            street_id: updated_contact.street_id,
                        }),
                        Err(e) => Err(e.to_string()),
                    }
                }
                None => Err(String::from("contact doesnt exist")),
            },
            Err(e) => Err(e.to_string()),
        }
    }
    // country
    pub async fn create_country(db: &DbConn, data: CCountry) -> CResult<Uuid> {
        let c_country = CountryActiveModel {
            country_name: Set(data.name),
            country_initials: Set(data.initials),
            ..Default::default()
        };
        match Country::insert(c_country).exec(db).await {
            Ok(res) => Ok(res.last_insert_id),
            Err(err) => return Err(err),
        }
    }
    pub async fn delete_country(db: &DbConn, id: Uuid) -> GResult<u64> {
        let selected_country = Country::find_by_id(id).one(db).await;
        match selected_country {
            Ok(country_opt) => match country_opt {
                Some(country_model) => match country_model.delete(db).await {
                    Ok(delete_a) => Ok(delete_a.rows_affected),
                    Err(err) => Err(err.to_string()),
                },
                None => Err(String::from("country doesnt exist")),
            },
            Err(err) => Err(err.to_string()),
        }
    }
    pub async fn update_country(db: &DbConn, id: Uuid, data: CCountry) -> GResult<CCountry> {
        let selected_country = Country::find_by_id(id).one(db).await;
        match selected_country {
            Ok(country_opt) => match country_opt {
                Some(country_model) => {
                    let mut country_model: CountryActiveModel = country_model.into();
                    country_model.country_name = Set(data.name);
                    country_model.country_initials = Set(data.initials);
                    match country_model.update(db).await {
                        Ok(updated_country) => Ok(CCountry {
                            name: updated_country.country_name,
                            initials: updated_country.country_initials,
                        }),
                        Err(e) => Err(e.to_string()),
                    }
                }
                None => Err(String::from("country doesnt exist")),
            },
            Err(e) => Err(e.to_string()),
        }
    }
    // districts
    pub async fn create_district(db: &DbConn, data: CDistrict) -> CResult<Uuid> {
        let c_district = DistrictActiveModel {
            district_name: Set(data.name),
            city_id: Set(data.city_id),
            ..Default::default()
        };
        match District::insert(c_district).exec(db).await {
            Ok(res) => Ok(res.last_insert_id),
            Err(err) => return Err(err),
        }
    }
    pub async fn delete_district(db: &DbConn, id: Uuid) -> GResult<u64> {
        let selected_district = District::find_by_id(id).one(db).await;
        match selected_district {
            Ok(district_opt) => match district_opt {
                Some(district_model) => match district_model.delete(db).await {
                    Ok(delete_a) => Ok(delete_a.rows_affected),
                    Err(err) => Err(err.to_string()),
                },
                None => Err(String::from("district doesnt exist")),
            },
            Err(err) => Err(err.to_string()),
        }
    }
    pub async fn update_district(db: &DbConn, id: Uuid, data: CDistrict) -> GResult<CDistrict> {
        let selected_district = District::find_by_id(id).one(db).await;
        match selected_district {
            Ok(district_opt) => match district_opt {
                Some(district_model) => {
                    let mut district_model: DistrictActiveModel = district_model.into();
                    district_model.district_name = Set(data.name);
                    district_model.city_id = Set(data.city_id);
                    match district_model.update(db).await {
                        Ok(updated_district) => Ok(CDistrict {
                            name: updated_district.district_name,
                            city_id: updated_district.city_id,
                        }),
                        Err(e) => Err(e.to_string()),
                    }
                }
                None => Err(String::from("district doesnt exist")),
            },
            Err(e) => Err(e.to_string()),
        }
    }
    // steets
    pub async fn create_street(db: &DbConn, data: CStreet) -> CResult<Uuid> {
        let c_street = StreetActiveModel {
            street_name: Set(data.name),
            district_id: Set(data.district_id),
            street_type: Set(data.street_type),
            zip_code: Set(data.zip_code),
            ..Default::default()
        };
        match Street::insert(c_street).exec(db).await {
            Ok(res) => Ok(res.last_insert_id),
            Err(err) => return Err(err),
        }
    }
    pub async fn delete_street(db: &DbConn, id: Uuid) -> GResult<u64> {
        let selected_street = Street::find_by_id(id).one(db).await;
        match selected_street {
            Ok(street_opt) => match street_opt {
                Some(street_model) => match street_model.delete(db).await {
                    Ok(delete_a) => Ok(delete_a.rows_affected),
                    Err(err) => Err(err.to_string()),
                },
                None => Err(String::from("street doesnt exist")),
            },
            Err(err) => Err(err.to_string()),
        }
    }
    pub async fn update_street(db: &DbConn, id: Uuid, data: CStreet) -> GResult<CStreet> {
        let selected_street = Street::find_by_id(id).one(db).await;
        match selected_street {
            Ok(street_opt) => match street_opt {
                Some(street_model) => {
                    let mut street_model: StreetActiveModel = street_model.into();
                    street_model.street_name = Set(data.name);
                    street_model.district_id = Set(data.district_id);
                    street_model.street_type = Set(data.street_type);
                    street_model.zip_code = Set(data.zip_code);

                    match street_model.update(db).await {
                        Ok(updated_street) => Ok(CStreet {
                            name: updated_street.street_name,
                            district_id: updated_street.district_id,
                            street_type: updated_street.street_type,
                            zip_code: updated_street.zip_code,
                        }),
                        Err(e) => Err(e.to_string()),
                    }
                }
                None => Err(String::from("street doesnt exist")),
            },
            Err(e) => Err(e.to_string()),
        }
    }
    // state
    pub async fn create_state(db: &DbConn, data: CState) -> CResult<Uuid> {
        let c_state = StateActiveModel {
            state_name: Set(data.name),
            state_initials: Set(data.initials),
            state_code: Set(data.code),
            country_id: Set(data.country_id),
            ..Default::default()
        };
        match State::insert(c_state).exec(db).await {
            Ok(res) => Ok(res.last_insert_id),
            Err(err) => return Err(err),
        }
    }
    pub async fn delete_state(db: &DbConn, id: Uuid) -> GResult<u64> {
        let selected_state = State::find_by_id(id).one(db).await;
        match selected_state {
            Ok(state_opt) => match state_opt {
                Some(state_model) => match state_model.delete(db).await {
                    Ok(delete_a) => Ok(delete_a.rows_affected),
                    Err(err) => Err(err.to_string()),
                },
                None => Err(String::from("state doesnt exist")),
            },
            Err(err) => Err(err.to_string()),
        }
    }
    pub async fn update_state(db: &DbConn, id: Uuid, data: CState) -> GResult<CState> {
        let selected_state = State::find_by_id(id).one(db).await;
        match selected_state {
            Ok(state_opt) => match state_opt {
                Some(state_model) => {
                    let mut state_model: StateActiveModel = state_model.into();
                    state_model.state_name = Set(data.name);
                    state_model.state_initials = Set(data.initials);
                    state_model.state_code = Set(data.code);
                    state_model.country_id = Set(data.country_id);
                    match state_model.update(db).await {
                        Ok(updated_state) => Ok(CState {
                            name: updated_state.state_name,
                            initials: updated_state.state_initials,
                            code: updated_state.state_code,
                            country_id: updated_state.country_id,
                        }),
                        Err(e) => Err(e.to_string()),
                    }
                }
                None => Err(String::from("state doesnt exist")),
            },
            Err(e) => Err(e.to_string()),
        }
    }
    // city
    pub async fn create_city(db: &DbConn, data: CCity) -> CResult<Uuid> {
        let c_city = CityActiveModel {
            city_name: Set(data.name),
            state_id: Set(data.state_id),
            ..Default::default()
        };
        match City::insert(c_city).exec(db).await {
            Ok(res) => Ok(res.last_insert_id),
            Err(err) => return Err(err),
        }
    }
    pub async fn delete_city(db: &DbConn, id: Uuid) -> GResult<u64> {
        let selected_city = City::find_by_id(id).one(db).await;
        match selected_city {
            Ok(city_opt) => match city_opt {
                Some(city_model) => match city_model.delete(db).await {
                    Ok(delete_a) => Ok(delete_a.rows_affected),
                    Err(err) => Err(err.to_string()),
                },
                None => Err(String::from("city doesnt exist")),
            },
            Err(err) => Err(err.to_string()),
        }
    }
    pub async fn update_city(db: &DbConn, id: Uuid, data: CCity) -> GResult<CCity> {
        let selected_city = City::find_by_id(id).one(db).await;
        match selected_city {
            Ok(city_opt) => match city_opt {
                Some(city_model) => {
                    let mut city_model: CityActiveModel = city_model.into();
                    city_model.city_name = Set(data.name);
                    city_model.state_id = Set(data.state_id);
                    match city_model.update(db).await {
                        Ok(updated_city) => Ok(CCity {
                            name: updated_city.city_name,
                            state_id: updated_city.state_id,
                        }),
                        Err(e) => Err(e.to_string()),
                    }
                }
                None => Err(String::from("city doesnt exist")),
            },
            Err(e) => Err(e.to_string()),
        }
    }
    // user
    pub async fn create_user(db: &DbConn, data: CUser) -> CResult<Uuid> {
        let c_city = UserActiveModel {
            first_name: Set(data.first_name),
            last_name: Set(data.last_name),
            email: Set(data.email),
            picture: Set(data.picture),
            ..Default::default()
        };
        match User::insert(c_city).exec(db).await {
            Ok(res) => Ok(res.last_insert_id),
            Err(err) => return Err(err),
        }
    }
}
