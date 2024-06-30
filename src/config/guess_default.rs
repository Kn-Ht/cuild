pub trait GuessDefault {
    /// Try to conform to the user's preferences, otherwise default.
    fn guess_default() -> Self;
}