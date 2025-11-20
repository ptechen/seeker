use bevy::prelude::*;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash, States)]
pub enum SeekerState {
    #[default]
    Home,
    Edit,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, SubStates, enum_from_derive::From)]
#[source(SeekerState = SeekerState::Home)]
#[states(scoped_entities)]
pub enum SeekerHomeSubFnState {
    Project,
    NewProject,
    CloneRepo,
    #[default]
    None,
}

impl From<String> for SeekerHomeSubFnState {
    fn from(s: String) -> Self {
        Self::from(s.as_str())
    }
}

#[test]
fn test() {
    assert_eq!(
        SeekerHomeSubFnState::from("Project 1".to_string()),
        SeekerHomeSubFnState::Project
    );
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, SubStates, enum_from_derive::From)]
#[source(SeekerState = SeekerState::Home)]
#[states(scoped_entities)]
pub enum SeekerFileDialogFnState {
    #[default]
    None,
    Open,
}

impl From<String> for SeekerFileDialogFnState {
    fn from(s: String) -> Self {
        Self::from(s.as_str())
    }
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, SubStates, enum_from_derive::From)]
#[source(SeekerFileDialogFnState = SeekerFileDialogFnState::Open)]
#[states(scoped_entities)]
pub enum SeekerNewFolderState {
    #[default]
    None,
    Open,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, SubStates)]
#[source(SeekerState = SeekerState::Home)]
#[states(scoped_entities)]
pub enum SeekerHomeSubLoadState {
    #[default]
    Loading,
    Loaded,
}
