pub enum ControllerMap {
    Forward,
    Backward,
    Left,
    Right,
}

// These should be macros
impl std::str::FromStr for ControllerMap {
    type Err = ();

    fn from_str(s: &str) -> Result<ControllerMap, ()> {
        match s {
            "Forward" => Ok(ControllerMap::Forward),
            "Backward" => Ok(ControllerMap::Backward),
            "Left" => Ok(ControllerMap::Left),
            "Right" => Ok(ControllerMap::Right),
            _ => Err(())
        }
    }
}

impl std::fmt::Display for ControllerMap {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ControllerMap::Forward => write!(f, "Forward"),
            ControllerMap::Backward => write!(f, "Backward"),
            ControllerMap::Left => write!(f, "Left"),
            ControllerMap::Right => write!(f, "Right")
        }
    }
}