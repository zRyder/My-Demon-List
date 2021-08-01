use crate::
{
    schema::rating,
    model::geometry_dash::gd
};

use std::num::ParseIntError;

#[derive(FromForm)]
pub struct RatingForm
{
    pub(super) rating: u8,
    pub(super) level_id: u64,
}

#[table_name = "rating"]
#[derive(Insertable, Default)]
pub struct RateInfo
{
    #[column_name = "userId"]
    pub(super) user_id: u32,

    #[column_name = "ratingId"]
    pub(super) rating_id: u32,

    #[column_name = "levelId"]
    pub(super) level_id: u64,

    #[column_name = "rate"]
    pub(super) rate: u8,
}