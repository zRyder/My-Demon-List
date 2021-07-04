pub mod tests
{
    use crate::model::{
        rating::rate::{RateInfo, RatingForm},
        geometry_dash::gd,
    };

    #[test]
    pub async fn valid_level_test()
    {
        let rate_form_1 = RatingForm
        {
            rating: 5,
            level_id: 66291197
        };

        let rate_form_2 = RatingForm
        {
            rating: 5,
            level_id: 1000000000
        };

        assert_eq!(true, gd::level_exists(&rate_form_1.level_id));
        assert_eq!(false, gd::level_exists(&rate_form_2.level_id));
    }
}