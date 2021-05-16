use crate::
{
    schema::rating,
    model::geometry_dash::gd
};

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

impl RatingForm
{
    pub(crate) fn level_exists(&self) -> bool
    {
        let maybe_level_stream = gd::prepare_search_request(self.level_id.to_string().as_str(), 1);

        match maybe_level_stream
        {
            Ok(level_stream) =>
            {
                let maybe_level = gd::process_levels21_response(level_stream.as_str());

                match maybe_level
                {
                    Ok(level) =>
                    {
                        true
                    }
                    Err(e) =>
                    {
                        false
                    }
                }
            }
            Err(e) =>
            {
                false
            }
        }
    }
}