// Generated by diesel_ext
#![allow(non_snake_case)]
#![allow(unused)]
#![allow(clippy::all)]

use diesel;
use diesel::prelude::*;
use diesel::{Identifiable, Insertable, Queryable};
use serde::*;

use crate::schema::House as house;

#[derive(Queryable)]
pub struct House {
    pub ID: i32,
    pub HouseName: String,
    pub City: String,
    pub StateName: String,
    pub Photo: String,
    pub AvailableUnits: String,
    pub Wifi: bool,
    pub Laundry: bool,
}

#[derive(Insertable, Serialize)]
#[diesel(table_name=house)]
pub struct NewHouse {
    pub HouseName: String,
    pub City: String,
    pub StateName: String,
    pub Photo: String,
    pub AvailableUnits: String,
    pub Wifi: bool,
    pub Laundry: bool,
}