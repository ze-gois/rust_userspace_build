#[macro_use]
pub mod default;

pub trait ResultDefaultTrait {
    type ResultType;

    fn from_no(no: Self::ResultType) -> Self;

    fn acronym(&self) -> &'static str;

    fn description(&self) -> &'static str;
}
