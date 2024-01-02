

use std::collections::HashSet;

use crate::structures::params;
use crate::structures::people;

// Classes should have 
// * name
// * allowed periods int (Some classes can only happen once eg sol or band) 
// * periods ()

pub struct Class{
    course: params::CourseNames,
    id: params::ClassId,
    period: params::Period,
    teacher: people::Teacher, // This chould be a set as well if we want to allow for teacher aides or mulitiple teachers in a room
    students: HashSet<people::Student>,
}

pub struct Course{
    name: params::CourseNames, 
    allowed_multiple: bool,
    instances: HashSet<Class>,
}