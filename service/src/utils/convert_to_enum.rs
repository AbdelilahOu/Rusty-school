use crate::entities::{
    AnnouncementCategoryEnum, AudienceEnum, DayOfWeekEnum, PerformanceLevelEnum,
};

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

pub fn to_announcement_category(category: String) -> AnnouncementCategoryEnum {
    match category.as_str() {
        "academic" => AnnouncementCategoryEnum::Academic,
        "event" => AnnouncementCategoryEnum::Event,
        _ => AnnouncementCategoryEnum::General,
    }
}

pub fn to_audience(audience: Option<String>) -> Option<AudienceEnum> {
    match audience {
        Some(audience) => match audience.as_str() {
            "groups" => Some(AudienceEnum::Groups),
            "parents" => Some(AudienceEnum::Parents),
            "students" => Some(AudienceEnum::Students),
            "teachers" => Some(AudienceEnum::Teachers),
            _ => None,
        },
        None => None,
    }
}

pub fn to_performance(performance: String) -> PerformanceLevelEnum {
    match performance.as_str() {
        "exceeds_expectations" => PerformanceLevelEnum::ExceedsExpectations,
        "meets_expectations" => PerformanceLevelEnum::MeetsExpectations,
        "below_expectations" => PerformanceLevelEnum::BelowExpectations,
        "needs_improvement" => PerformanceLevelEnum::NeedsImprovement,
        _ => PerformanceLevelEnum::NotYetMeetingExpectations,
    }
}
