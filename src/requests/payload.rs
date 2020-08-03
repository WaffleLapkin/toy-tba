#[doc(spotlight)]
pub trait Payload {
    type Output;

    const NAME: &'static str;
}
