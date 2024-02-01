use ::entity::prelude::*;
use chrono::Utc;
use sea_orm::{prelude::Uuid, *};

use super::types::*;

pub struct MutationsService;

type DyResult<T> = Result<T, DbErr>;

impl MutationsService {
    // students entity
    pub async fn create_student(db: &DbConn, data: CStudent) -> DyResult<Uuid> {
        let c_student = StudentActiveModel {
            first_name: Set(data.first_name),
            last_name: Set(data.last_name),
            group_id: Set(data.group_id),
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
                student_model.group_id = Set(data.group_id);
                student_model.id = Set(id);
                //
                let student = student_model.update(db).await?;
                Ok(CStudent {
                    first_name: student.first_name,
                    last_name: student.last_name,
                    group_id: student.group_id,
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
    // detailss
    pub async fn create_details(db: &DbConn, data: CPDetails) -> DyResult<Uuid> {
        let details_model = PersonDetailsActiveModel {
            phone_number: Set(data.phone),
            email: Set(data.email),
            country_id: Set(data.country_id),
            state_id: Set(data.state_id),
            city_id: Set(data.city_id),
            district_id: Set(data.district_id),
            street_id: Set(data.street_id),
            ..Default::default()
        };
        let detailss = PersonDetails::insert(details_model).exec(db).await?;
        Ok(detailss.last_insert_id)
    }
    pub async fn delete_details(db: &DbConn, id: Uuid) -> DyResult<u64> {
        let details_model = PersonDetails::find_by_id(id).one(db).await?;
        match details_model {
            Some(details_model) => {
                let details = details_model.delete(db).await?;
                Ok(details.rows_affected)
            }
            None => Ok(0),
        }
    }
    pub async fn update_details(db: &DbConn, id: Uuid, data: CPDetails) -> DyResult<CPDetails> {
        let details_model = PersonDetails::find_by_id(id).one(db).await?;
        match details_model {
            Some(details_model) => {
                //
                let mut details_model: PersonDetailsActiveModel = details_model.into();
                details_model.phone_number = Set(data.phone);
                details_model.email = Set(data.email);
                details_model.country_id = Set(data.country_id);
                details_model.state_id = Set(data.state_id);
                details_model.city_id = Set(data.city_id);
                details_model.district_id = Set(data.district_id);
                details_model.street_id = Set(data.street_id);
                details_model.id = Set(id);
                //
                let details = details_model.update(db).await?;
                Ok(CPDetails {
                    phone: details.phone_number,
                    email: details.email,
                    country_id: details.country_id,
                    state_id: details.state_id,
                    city_id: details.city_id,
                    district_id: details.district_id,
                    street_id: details.street_id,
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
                state_model.country_id = Set(data.country_id);
                //
                let state = state_model.update(db).await?;
                Ok(CState {
                    name: state.state_name,
                    initials: state.state_initials,
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
    //
    pub async fn create_level(db: &DbConn, data: CLevel) -> DyResult<Uuid> {
        let level_model = LevelActiveModel {
            level_name: Set(data.name),
            level_description: Set(data.description),
            ..Default::default()
        };
        let level = Level::insert(level_model).exec(db).await?;
        Ok(level.last_insert_id)
    }
    pub async fn delete_level(db: &DbConn, id: Uuid) -> DyResult<u64> {
        let level_model = Level::find_by_id(id).one(db).await?;
        match level_model {
            Some(level_model) => {
                let level = level_model.delete(db).await?;
                Ok(level.rows_affected)
            }
            None => Ok(0),
        }
    }
    pub async fn update_level(db: &DbConn, id: Uuid, data: CLevel) -> DyResult<CLevel> {
        let level_model = Level::find_by_id(id).one(db).await?;
        match level_model {
            Some(level_model) => {
                //
                let mut level_model: LevelActiveModel = level_model.into();
                level_model.level_name = Set(data.name);
                level_model.level_description = Set(data.description);
                //
                let level = level_model.update(db).await?;
                Ok(CLevel {
                    name: level.level_name,
                    description: level.level_description,
                })
            }
            None => Ok(data),
        }
    }
    //
    pub async fn create_subject(db: &DbConn, data: CSubject) -> DyResult<Uuid> {
        let subject_model = SubjectActiveModel {
            subject_name: Set(data.name),
            subject_description: Set(data.description),
            level_id: Set(data.level_id),
            ..Default::default()
        };
        let subject = Subject::insert(subject_model).exec(db).await?;
        Ok(subject.last_insert_id)
    }
    pub async fn delete_subject(db: &DbConn, id: Uuid) -> DyResult<u64> {
        let subject_model = Subject::find_by_id(id).one(db).await?;
        match subject_model {
            Some(subject_model) => {
                let subject = subject_model.delete(db).await?;
                Ok(subject.rows_affected)
            }
            None => Ok(0),
        }
    }
    pub async fn update_subject(db: &DbConn, id: Uuid, data: CSubject) -> DyResult<CSubject> {
        let subject_model = Subject::find_by_id(id).one(db).await?;
        match subject_model {
            Some(subject_model) => {
                //
                let mut subject_model: SubjectActiveModel = subject_model.into();
                subject_model.subject_name = Set(data.name);
                subject_model.subject_description = Set(data.description);
                subject_model.level_id = Set(data.level_id);
                //
                let subject = subject_model.update(db).await?;
                Ok(CSubject {
                    name: subject.subject_name,
                    description: subject.subject_description,
                    level_id: subject.level_id,
                })
            }
            None => Ok(data),
        }
    }
    //
    pub async fn create_group(db: &DbConn, data: CGroup) -> DyResult<Uuid> {
        let group_model = GroupActiveModel {
            group_name: Set(data.name),
            group_description: Set(data.description),
            level_id: Set(data.level_id),
            ..Default::default()
        };
        let group = Group::insert(group_model).exec(db).await?;
        Ok(group.last_insert_id)
    }
    pub async fn delete_group(db: &DbConn, id: Uuid) -> DyResult<u64> {
        let group_model = Group::find_by_id(id).one(db).await?;
        match group_model {
            Some(group_model) => {
                let group = group_model.delete(db).await?;
                Ok(group.rows_affected)
            }
            None => Ok(0),
        }
    }
    pub async fn update_group(db: &DbConn, id: Uuid, data: CGroup) -> DyResult<CGroup> {
        let group_model = Group::find_by_id(id).one(db).await?;
        match group_model {
            Some(group_model) => {
                //
                let mut group_model: GroupActiveModel = group_model.into();
                group_model.group_name = Set(data.name);
                group_model.group_description = Set(data.description);
                group_model.level_id = Set(data.level_id);
                //
                let group = group_model.update(db).await?;
                Ok(CGroup {
                    name: group.group_name,
                    description: group.group_description,
                    level_id: group.level_id,
                })
            }
            None => Ok(data),
        }
    }
    //
    pub async fn create_room(db: &DbConn, data: CRoom) -> DyResult<Uuid> {
        let room_model = RoomActiveModel {
            room_name: Set(data.name),
            room_description: Set(data.description),
            ..Default::default()
        };
        let room = Room::insert(room_model).exec(db).await?;
        Ok(room.last_insert_id)
    }
    pub async fn delete_room(db: &DbConn, id: Uuid) -> DyResult<u64> {
        let room_model = Room::find_by_id(id).one(db).await?;
        match room_model {
            Some(room_model) => {
                let room = room_model.delete(db).await?;
                Ok(room.rows_affected)
            }
            None => Ok(0),
        }
    }
    pub async fn update_room(db: &DbConn, id: Uuid, data: CRoom) -> DyResult<CRoom> {
        let room_model = Room::find_by_id(id).one(db).await?;
        match room_model {
            Some(room_model) => {
                //
                let mut room_model: RoomActiveModel = room_model.into();
                room_model.room_name = Set(data.name);
                room_model.room_description = Set(data.description);
                //
                let room = room_model.update(db).await?;
                Ok(CRoom {
                    name: room.room_name,
                    description: room.room_description,
                })
            }
            None => Ok(data),
        }
    }
    //
    pub async fn create_class(db: &DbConn, data: CClass) -> DyResult<Uuid> {
        let class_model = ClassActiveModel {
            subject_id: Set(data.subject_id),
            teacher_id: Set(data.teacher_id),
            group_id: Set(data.group_id),
            room_id: Set(data.room_id),
            ..Default::default()
        };
        let class = Class::insert(class_model).exec(db).await?;
        Ok(class.last_insert_id)
    }
    pub async fn delete_class(db: &DbConn, id: Uuid) -> DyResult<u64> {
        let class_model = Class::find_by_id(id).one(db).await?;
        match class_model {
            Some(class_model) => {
                let class = class_model.delete(db).await?;
                Ok(class.rows_affected)
            }
            None => Ok(0),
        }
    }
    pub async fn update_class(db: &DbConn, id: Uuid, data: CClass) -> DyResult<CClass> {
        let class_model = Class::find_by_id(id).one(db).await?;
        match class_model {
            Some(class_model) => {
                //
                let mut class_model: ClassActiveModel = class_model.into();
                class_model.subject_id = Set(data.subject_id);
                class_model.teacher_id = Set(data.teacher_id);
                class_model.group_id = Set(data.group_id);
                class_model.room_id = Set(data.room_id);
                //
                let class = class_model.update(db).await?;
                Ok(CClass {
                    subject_id: class.subject_id,
                    teacher_id: class.teacher_id,
                    group_id: class.group_id,
                    room_id: class.room_id,
                })
            }
            None => Ok(data),
        }
    }
    //
    pub async fn create_time_table(_db: &DbConn, _data: CTimeTable) -> DyResult<Uuid> {
        todo!()
    }
}
