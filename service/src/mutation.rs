use ::entity::prelude::*;
use chrono::Utc;
use sea_orm::{prelude::Uuid, *};

use super::types::*;

pub struct ServiceMutation;

type DyResult<T> = Result<T, DbErr>;

impl ServiceMutation {
    // students entity
    pub async fn create_student(db: &DbConn, data: CStudent) -> DyResult<Uuid> {
        let c_student = StudentActiveModel {
            first_name: Set(data.first_name),
            last_name: Set(data.last_name),
            level: Set(data.level),
            ..Default::default()
        };
        let student = Student::insert(c_student).exec(db).await?;
        Ok(student.last_insert_id)
    }
    pub async fn delete_student(db: &DbConn, id: Uuid) -> DyResult<u64> {
        let student_model = Student::find_by_id(id).one(db).await?;
        match student_model {
            Some(student_model) => {
                let student = student_model.delete(db).await?;
                Ok(student.rows_affected)
            }
            None => Ok(0),
        }
    }
    pub async fn update_student(db: &DbConn, id: Uuid, data: CStudent) -> DyResult<CStudent> {
        let student_model = Student::find_by_id(id).one(db).await?;
        match student_model {
            Some(student_model) => {
                let mut student_model: StudentActiveModel = student_model.into();
                // set new feild
                student_model.first_name = Set(data.first_name);
                student_model.last_name = Set(data.last_name);
                student_model.level = Set(data.level);
                student_model.id = Set(id);
                //
                let student = student_model.update(db).await?;
                Ok(CStudent {
                    first_name: student.first_name,
                    last_name: student.last_name,
                    level: student.level,
                })
            }
            None => Ok(data),
        }
    }
    // teachers entity
    pub async fn create_teacher(db: &DbConn, data: CTeacher) -> DyResult<Uuid> {
        let teacher_model = TeacherActiveModel {
            first_name: Set(data.first_name),
            last_name: Set(data.last_name),
            ..Default::default()
        };
        let teacher = Teacher::insert(teacher_model).exec(db).await?;
        Ok(teacher.last_insert_id)
    }
    pub async fn delete_teacher(db: &DbConn, id: Uuid) -> DyResult<u64> {
        let teacher_model = Teacher::find_by_id(id).one(db).await?;
        match teacher_model {
            Some(teacher_model) => {
                let teacher = teacher_model.delete(db).await?;
                Ok(teacher.rows_affected)
            }
            None => Ok(0),
        }
    }
    pub async fn update_teacher(db: &DbConn, id: Uuid, data: CTeacher) -> DyResult<CTeacher> {
        let teacher_model = Teacher::find_by_id(id).one(db).await?;
        match teacher_model {
            Some(teacher_model) => {
                //
                let mut teacher_model: TeacherActiveModel = teacher_model.into();
                teacher_model.first_name = Set(data.first_name);
                teacher_model.last_name = Set(data.last_name);
                teacher_model.id = Set(id);
                //
                let teacher = teacher_model.update(db).await?;
                Ok(CTeacher {
                    first_name: teacher.first_name,
                    last_name: teacher.last_name,
                })
            }
            None => Ok(data),
        }
    }
    // parents entity
    pub async fn create_parent(db: &DbConn, data: CParent) -> DyResult<Uuid> {
        let parent_model = ParentActiveModel {
            first_name: Set(data.first_name),
            last_name: Set(data.last_name),
            ..Default::default()
        };
        let parent = Parent::insert(parent_model).exec(db).await?;
        Ok(parent.last_insert_id)
    }
    pub async fn delete_parent(db: &DbConn, id: Uuid) -> DyResult<u64> {
        let parent_model = Parent::find_by_id(id).one(db).await?;
        match parent_model {
            Some(parent_model) => {
                let parent = parent_model.delete(db).await?;
                Ok(parent.rows_affected)
            }
            None => Ok(0),
        }
    }
    pub async fn update_parent(db: &DbConn, id: Uuid, data: CParent) -> DyResult<CParent> {
        let parent_model = Parent::find_by_id(id).one(db).await?;
        match parent_model {
            Some(parent_model) => {
                let mut parent_model: ParentActiveModel = parent_model.into();
                parent_model.first_name = Set(data.first_name);
                parent_model.last_name = Set(data.last_name);
                parent_model.id = Set(id);
                let parent = parent_model.update(db).await?;
                Ok(CParent {
                    first_name: parent.first_name,
                    last_name: parent.last_name,
                })
            }
            None => Ok(data),
        }
    }
    // contacts
    pub async fn create_contact(db: &DbConn, data: CContact) -> DyResult<Uuid> {
        let contact_model = ContactActiveModel {
            phone_number: Set(data.phone),
            email: Set(data.email),
            country_id: Set(data.country_id),
            state_id: Set(data.state_id),
            city_id: Set(data.city_id),
            district_id: Set(data.district_id),
            street_id: Set(data.street_id),
            ..Default::default()
        };
        let contacts = Contact::insert(contact_model).exec(db).await?;
        Ok(contacts.last_insert_id)
    }
    pub async fn delete_contact(db: &DbConn, id: Uuid) -> DyResult<u64> {
        let contact_model = Contact::find_by_id(id).one(db).await?;
        match contact_model {
            Some(contact_model) => {
                let contact = contact_model.delete(db).await?;
                Ok(contact.rows_affected)
            }
            None => Ok(0),
        }
    }
    pub async fn update_contact(db: &DbConn, id: Uuid, data: CContact) -> DyResult<CContact> {
        let contact_model = Contact::find_by_id(id).one(db).await?;
        match contact_model {
            Some(contact_model) => {
                //
                let mut contact_model: ContactActiveModel = contact_model.into();
                contact_model.phone_number = Set(data.phone);
                contact_model.email = Set(data.email);
                contact_model.country_id = Set(data.country_id);
                contact_model.state_id = Set(data.state_id);
                contact_model.city_id = Set(data.city_id);
                contact_model.district_id = Set(data.district_id);
                contact_model.street_id = Set(data.street_id);
                contact_model.id = Set(id);
                //
                let contact = contact_model.update(db).await?;
                Ok(CContact {
                    phone: contact.phone_number,
                    email: contact.email,
                    country_id: contact.country_id,
                    state_id: contact.state_id,
                    city_id: contact.city_id,
                    district_id: contact.district_id,
                    street_id: contact.street_id,
                })
            }
            None => Ok(data),
        }
    }
    // country
    pub async fn create_country(db: &DbConn, data: CCountry) -> DyResult<Uuid> {
        let country_model = CountryActiveModel {
            country_name: Set(data.name),
            country_initials: Set(data.initials),
            ..Default::default()
        };
        let country = Country::insert(country_model).exec(db).await?;
        Ok(country.last_insert_id)
    }
    pub async fn delete_country(db: &DbConn, id: Uuid) -> DyResult<u64> {
        let country_model = Country::find_by_id(id).one(db).await?;
        match country_model {
            Some(country_model) => {
                let country = country_model.delete(db).await?;
                Ok(country.rows_affected)
            }
            None => Ok(0),
        }
    }
    pub async fn update_country(db: &DbConn, id: Uuid, data: CCountry) -> DyResult<CCountry> {
        let country_model = Country::find_by_id(id).one(db).await?;
        match country_model {
            Some(country_model) => {
                //
                let mut country_model: CountryActiveModel = country_model.into();
                country_model.country_name = Set(data.name);
                country_model.country_initials = Set(data.initials);
                //
                let country = country_model.update(db).await?;
                Ok(CCountry {
                    name: country.country_name,
                    initials: country.country_initials,
                })
            }
            None => Ok(data),
        }
    }
    // districts
    pub async fn create_district(db: &DbConn, data: CDistrict) -> DyResult<Uuid> {
        let district_model = DistrictActiveModel {
            district_name: Set(data.name),
            city_id: Set(data.city_id),
            ..Default::default()
        };
        let district = District::insert(district_model).exec(db).await?;
        Ok(district.last_insert_id)
    }
    pub async fn delete_district(db: &DbConn, id: Uuid) -> DyResult<u64> {
        let district_model = District::find_by_id(id).one(db).await?;
        match district_model {
            Some(district_model) => {
                let district = district_model.delete(db).await?;
                Ok(district.rows_affected)
            }
            None => Ok(0),
        }
    }
    pub async fn update_district(db: &DbConn, id: Uuid, data: CDistrict) -> DyResult<CDistrict> {
        let district_model = District::find_by_id(id).one(db).await?;
        match district_model {
            Some(district_model) => {
                //
                let mut district_model: DistrictActiveModel = district_model.into();
                district_model.district_name = Set(data.name);
                district_model.city_id = Set(data.city_id);
                //
                let district = district_model.update(db).await?;
                Ok(CDistrict {
                    name: district.district_name,
                    city_id: district.city_id,
                })
            }
            None => Ok(data),
        }
    }
    // steets
    pub async fn create_street(db: &DbConn, data: CStreet) -> DyResult<Uuid> {
        let street_model = StreetActiveModel {
            street_name: Set(data.name),
            district_id: Set(data.district_id),
            street_type: Set(data.street_type),
            zip_code: Set(data.zip_code),
            ..Default::default()
        };
        //
        let street = Street::insert(street_model).exec(db).await?;
        Ok(street.last_insert_id)
    }
    pub async fn delete_street(db: &DbConn, id: Uuid) -> DyResult<u64> {
        let street_model = Street::find_by_id(id).one(db).await?;
        match street_model {
            Some(street_model) => {
                let street = street_model.delete(db).await?;
                Ok(street.rows_affected)
            }
            None => Ok(0),
        }
    }
    pub async fn update_street(db: &DbConn, id: Uuid, data: CStreet) -> DyResult<CStreet> {
        let street_model = Street::find_by_id(id).one(db).await?;
        match street_model {
            Some(street_model) => {
                //
                let mut street_model: StreetActiveModel = street_model.into();
                street_model.street_name = Set(data.name);
                street_model.district_id = Set(data.district_id);
                street_model.street_type = Set(data.street_type);
                street_model.zip_code = Set(data.zip_code);
                //
                let street = street_model.update(db).await?;
                Ok(CStreet {
                    name: street.street_name,
                    district_id: street.district_id,
                    zip_code: street.zip_code,
                    street_type: street.street_type,
                })
            }
            None => Ok(data),
        }
    }
    // state
    pub async fn create_state(db: &DbConn, data: CState) -> DyResult<Uuid> {
        let state_model = StateActiveModel {
            state_name: Set(data.name),
            state_initials: Set(data.initials),
            state_code: Set(data.code),
            country_id: Set(data.country_id),
            ..Default::default()
        };
        let state = State::insert(state_model).exec(db).await?;
        Ok(state.last_insert_id)
    }
    pub async fn delete_state(db: &DbConn, id: Uuid) -> DyResult<u64> {
        let state_model = State::find_by_id(id).one(db).await?;
        match state_model {
            Some(state_model) => {
                let state = state_model.delete(db).await?;
                Ok(state.rows_affected)
            }
            None => Ok(0),
        }
    }
    pub async fn update_state(db: &DbConn, id: Uuid, data: CState) -> DyResult<CState> {
        let state_model = State::find_by_id(id).one(db).await?;
        match state_model {
            Some(state_model) => {
                //
                let mut state_model: StateActiveModel = state_model.into();
                state_model.state_name = Set(data.name);
                state_model.state_initials = Set(data.initials);
                state_model.state_code = Set(data.code);
                state_model.country_id = Set(data.country_id);
                //
                let state = state_model.update(db).await?;
                Ok(CState {
                    name: state.state_name,
                    initials: state.state_initials,
                    code: state.state_code,
                    country_id: state.country_id,
                })
            }
            None => Ok(data),
        }
    }
    // city
    pub async fn create_city(db: &DbConn, data: CCity) -> DyResult<Uuid> {
        let city_model = CityActiveModel {
            city_name: Set(data.name),
            state_id: Set(data.state_id),
            ..Default::default()
        };
        let city = City::insert(city_model).exec(db).await?;
        Ok(city.last_insert_id)
    }
    pub async fn delete_city(db: &DbConn, id: Uuid) -> DyResult<u64> {
        let city_model = City::find_by_id(id).one(db).await?;
        match city_model {
            Some(city_model) => {
                let city = city_model.delete(db).await?;
                Ok(city.rows_affected)
            }
            None => Ok(0),
        }
    }
    pub async fn update_city(db: &DbConn, id: Uuid, data: CCity) -> DyResult<CCity> {
        let city_model = City::find_by_id(id).one(db).await?;
        match city_model {
            Some(city_model) => {
                //
                let mut city_model: CityActiveModel = city_model.into();
                city_model.city_name = Set(data.name);
                city_model.state_id = Set(data.state_id);
                //
                let city = city_model.update(db).await?;
                Ok(CCity {
                    name: city.city_name,
                    state_id: city.state_id,
                })
            }
            None => Ok(data),
        }
    }
    // scans
    pub async fn create_scan(db: &DbConn, data: CScan) -> DyResult<Uuid> {
        let now = Utc::now();
        let scan_model = ScanActiveModel {
            person_id: Set(data.person_id),
            scan_date: Set(now.naive_utc()),
            ..Default::default()
        };
        let scan = Scans::insert(scan_model).exec(db).await?;
        Ok(scan.last_insert_id)
    }
}
