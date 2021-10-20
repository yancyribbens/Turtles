pub enum OriginalErr {
    Turtles
}

pub enum AppErr {
    Oops(crate::OriginalErr)
}

impl From<crate::OriginalErr> for AppErr {
    fn from(error: crate::OriginalErr) -> Self {
        AppErr::Oops(error)
    }
}

fn danger_will_robinson() -> Result<i32, AppErr> {
    let e:AppErr = OriginalErr::Turtles.into();
    Err(e)
}

fn main() {
    danger_will_robinson();
}
