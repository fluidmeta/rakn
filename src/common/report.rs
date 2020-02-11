pub enum OutputType {
    Rakn,
    VulsIO,
}

pub trait ReportExt {
    fn get_report(&self, pretty: &bool) -> String;
}
