pub enum ErrB {
    Turtles
}

pub enum ErrA {
    Turtles
}

pub enum AppErr {
    OopsA(crate::ErrA),
    OopsB(crate::ErrB)
}

impl From<crate::ErrA> for AppErr {
    fn from(error: crate::ErrA) -> Self {
        AppErr::OopsA(error)
    }
}

impl From<crate::ErrB> for AppErr {
    fn from(error: crate::ErrB) -> Self {
        AppErr::OopsB(error)
    }
}

fn danger_will_robinson() -> Result<i32, AppErr> {
    let e:AppErr = ErrA::Turtles.into();
    let e:AppErr = ErrB::Turtles.into();

    Err(e)
}

fn main() {
    danger_will_robinson();
}
