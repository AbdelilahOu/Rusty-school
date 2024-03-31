mod assignments;
mod attendance;
mod classes;
mod disciplinary_actions;
mod groups;
mod levels;
mod rooms;
mod scans;
mod subjects;
mod timetable;

pub use assignments::load_assignments_routes;
pub use attendance::load_attendance_routes;
pub use classes::load_classes_routes;
pub use disciplinary_actions::load_disciplinary_actions_routes;
pub use groups::load_groups_routes;
pub use levels::load_levels_routes;
pub use rooms::load_rooms_routes;
pub use scans::load_scans_routes;
pub use subjects::load_subjects_routes;
pub use timetable::load_timetable_routes;
