use super::qsm::*;
use num_enum::TryFromPrimitive;

#[derive(TryFromPrimitive)]
#[repr(i64)]
pub enum QEventHeader {
    DEFAULT = 0,
    // Write Custom Message . . .
}

impl QEventHeader {
    pub fn listen_event(&self, mut message : QMessage) {
        match self {
            QEventHeader::DEFAULT => {

            }
            // Write Custom Event from Message . . .
        }
    }
}
