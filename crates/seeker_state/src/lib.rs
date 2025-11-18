use bevy::prelude::*;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash, States)]
pub enum SeekerState {
    #[default]
    Home,
    Edit,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, SubStates)]
#[source(SeekerState = SeekerState::Home)]
#[states(scoped_entities)]
pub enum SeekerHomeSubFnState {
    #[default]
    Project,
    NewProject,
    Open,
    CloneRepo,
    FileDialog,
    None,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, SubStates)]
#[source(SeekerState = SeekerState::Home)]
#[states(scoped_entities)]
pub enum SeekerHomeSubLoadState {
    #[default]
    Loading,
    Loaded,
}