pub struct GdLevel<'a>
{
    id: u16,
    name: &'a str,
    rating: Difficulty,
}

enum Difficulty
{
    Easy,
    Normal,
    Hard,
    Harder,
    Insane,
    Demon(DemonRating),
}

enum DemonRating
{
    Easy,
    Medium,
    Hard,
    Insane,
    Extreme,
}