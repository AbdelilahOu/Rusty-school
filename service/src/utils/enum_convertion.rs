use crate::entities::{AnnouncementCategories, Audiences, PerformanceLevels, Roles, WeekDays};

pub fn to_day_of_week(i: u32) -> Option<WeekDays> {
    match i {
        0 => Some(WeekDays::Sunday),
        1 => Some(WeekDays::Monday),
        2 => Some(WeekDays::Tuesday),
        3 => Some(WeekDays::Wednesday),
        4 => Some(WeekDays::Thursday),
        5 => Some(WeekDays::Friday),
        6 => Some(WeekDays::Saturday),
        _ => None,
    }
}

pub fn to_announcement_category(category: String) -> AnnouncementCategories {
    match category.as_str() {
        "academic" => AnnouncementCategories::Academic,
        "event" => AnnouncementCategories::Event,
        _ => AnnouncementCategories::General,
    }
}

pub fn to_audience(audience: Option<String>) -> Option<Audiences> {
    match audience {
        Some(audience) => match audience.as_str() {
            "groups" => Some(Audiences::Groups),
            "parents" => Some(Audiences::Parents),
            "students" => Some(Audiences::Students),
            "teachers" => Some(Audiences::Teachers),
            _ => None,
        },
        None => None,
    }
}

pub fn to_performance(performance: String) -> PerformanceLevels {
    match performance.as_str() {
        "exceeds_expectations" => PerformanceLevels::ExceedsExpectations,
        "meets_expectations" => PerformanceLevels::MeetsExpectations,
        "below_expectations" => PerformanceLevels::BelowExpectations,
        "needs_improvement" => PerformanceLevels::NeedsImprovement,
        _ => PerformanceLevels::NotYetMeetingExpectations,
    }
}

pub fn roles_to_string(role: Roles) -> String {
    match role {
        Roles::Admin => "admin".to_string(),
        Roles::Assistant => "assistant".to_string(),
        Roles::Student => "student".to_string(),
        Roles::Teacher => "teacher".to_string(),
        Roles::Parent => "parent".to_string(),
        Roles::NotDefined => "not-defined".to_string(),
    }
}
