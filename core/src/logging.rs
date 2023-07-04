use log::{Metadata, Record};

use crate::{e3, e8};

pub(crate) struct ArcdpsLogger {
    name: &'static str,
}

impl ArcdpsLogger {
    pub(crate) fn new(name: &'static str) -> Self {
        Self { name }
    }

    fn format_message(&self, record: &Record<'_>) -> String {
        format!(
            "{} - {}:{} {}: {}\0",
            self.name,
            record.file().unwrap_or_default(),
            record.line().unwrap_or_default(),
            record.level(),
            record.args(),
        )
    }
}

impl log::Log for ArcdpsLogger {
    fn enabled(&self, _metadata: &Metadata<'_>) -> bool {
        true
    }

    fn log(&self, record: &Record<'_>) {
        let body = self.format_message(record);
        let body = body.as_ptr() as _;
        unsafe {
            // log to file
            e3(body);
            // log to arcdps log window
            e8(body);
        }
    }

    fn flush(&self) {}
}
