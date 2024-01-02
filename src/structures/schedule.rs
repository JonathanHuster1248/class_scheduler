

use crate::structures::course::Class;
use crate::structures::params;
use crate::structures::people;

use std::collections::HashSet;
use std::collections::HashMap;


// A scehdule should have 
// * Class schedule (dict - period: set[classes])
// * is_valid (bool - whether the set up is allowed ) - should check for no double listing, no unavailabilities, and full schedules
// * score (float - How well the scehdule acchieves the goals of the preferences and class sizes and allowed teacher free periods)

pub struct SchoolSchedule{
    students: HashSet<people::Student>,
    teachers: HashSet<people::Teacher>,
    mut scehdule: HashMap<params::Period, HashSet<Class>>,
    mut is_valid: bool,
    mut score: i32,
}

pub fn validate_schedule(students: HashSet<people::Student>, teachers: HashSet<people::Teacher>){
    // Students not double booked

    // Teachers not double booked

    // Student schedules full

    // Students have all required classes

    // Teacher schedules contain at least one break period

    // Teachers only teach what they can
}

pub fn score_students(students: HashSet<people::Student>){
    // Student preferences 
}

pub fn score_teachers(teachers: HashSet<people::Student>){
    // Number of down periods
    
    // low diversity of classes? 
}

pub fn update_score(sched: SchoolSchedule){ // Could pass in a scoring function as well to 
    // 
}

