#[derive(Clone, Default)]
pub enum State {
    #[default]
    Start,
    SetSpeciality,
    ReceiveSpeciality {
        speciality: String,
    },
}
