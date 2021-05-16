pub mod tests
{
    use crate::model::rating::rate::{RateInfo, RatingForm};

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

        assert_eq!(true, rate_form_1.level_exists());
        assert_eq!(false, rate_form_2.level_exists());
    }
}