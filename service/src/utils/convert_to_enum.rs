use entity::sea_orm_active_enums::DayOfWeekEnum;

pub fn to_day_of_week(i: u32) -> Option<DayOfWeekEnum> {
    match i {
        0 => Some(DayOfWeekEnum::Sunday),
        1 => Some(DayOfWeekEnum::Monday),
        2 => Some(DayOfWeekEnum::Tuesday),
        3 => Some(DayOfWeekEnum::Wednesday),
        4 => Some(DayOfWeekEnum::Thursday),
        5 => Some(DayOfWeekEnum::Friday),
        6 => Some(DayOfWeekEnum::Saturday),
        _ => None,
    }
}
