pub trait DisplayNodes {
    fn display(&self) -> &'static str;
}

pub trait OperationTrait {
    fn literal(&self) -> String;
    fn description(&self) -> String;
}
