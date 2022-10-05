#[derive(Hash, Clone, Copy, PartialEq, Eq, Debug)]
pub enum AppState {
    LoadingAssets,
    Setup,
    ConnectingToLidar,
    Calibration,
    Game(GameState),
    Tracker,
}

#[derive(Hash, Clone, Copy, PartialEq, Eq, Debug)]
pub enum GameState {
    Running,
    Paused,
}
