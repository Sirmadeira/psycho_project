//! Here are located every single struct that is synced and envolves world
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Component)]
pub struct FloorMarker;
