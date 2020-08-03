use crate::requests::Payload;

pub trait HasPayload {
    type Payload: Payload;

    fn payload_mut(&mut self) -> &mut Self::Payload;
}

impl<P> HasPayload for P
where
    P: Payload,
{
    type Payload = Self;

    fn payload_mut(&mut self) -> &mut Self::Payload {
        self
    }
}
