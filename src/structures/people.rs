use crate::structures::params;

use std::collections::HashMap;
use std::collections::HashSet;

// Students should have 
// * name
// * Schedule (dict period - class)
// * preference (dict class - ranking[int])

type PersonalSchedule = HashMap<params::Period, params::ClassId>;

pub struct Student{
    name: String,
    schedule: PersonalSchedule, //TODO: should this be a reference to the enum? or an instance of the enum itself? 
    preference: HashMap<params::CourseNames, u8>,
}


// Teachers should have 
// * name (str)
// * Schedule (dict period - class)
// * availability (set - classes)

pub struct Teacher{
    name: String,
    schedule: PersonalSchedule,
    availability: HashMap<params::Period, bool>,
    courses: HashSet<params::CourseNames>,
}


// MAKE NEW IMPLEMENTATION FUNCTIONS GIVEN A NAME AND (OPTIONALLY) A PREFERENCE OR AVAILABILITY