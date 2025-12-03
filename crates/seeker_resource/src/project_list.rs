use bevy::prelude::*;
use seeker_sqlite::seeker::project::Project;

#[derive(Resource)]
pub struct ProjectListResource {
    pub projects: Vec<Project>,
}

impl Default for ProjectListResource {
    fn default() -> Self {
        ProjectListResource {
            projects: Project::select_all().unwrap_or_default(),
        }
    }
}
