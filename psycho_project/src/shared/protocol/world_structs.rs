//! Here are located every single struct that is synced and envolves world
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Marker component utilized in sync to know which entity from server I should replicated
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Component)]
pub struct FloorMarker;
