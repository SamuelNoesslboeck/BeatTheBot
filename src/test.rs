/// Checks if the API function "game_create" is functioning
#[test]
fn game_create() -> Result<(), crate::Error> {
    let mut a = crate::api::Api::new(include_str!("../assets/token.key").to_owned());

    a.game_close()?;

    assert!(a.game_create()?.is_some());
    assert!(a.game_create()?.is_none());

    assert!(a.game_close()?.is_some());
    assert!(a.game_close()?.is_none());

    Ok(())
}