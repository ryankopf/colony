

pub fn play_music(
    app_state: Res<GameState>,
) {
    match app_state.current() {
        GameState::MainMenu => {
            let mut music = Music::new();
            music.play_music(app_state);
        }
        GameState::Game => {
            let mut music = Music::new();
            music.play_music(app_state);
        }
        GameState::Paused => {
            let mut music = Music::new();
            music.play_music(app_state);
        }
    }
}