pub(crate) trait Bool {
    fn reify() -> bool;
}
pub(crate) struct True;
pub(crate) struct False;

impl Bool for True {
    fn reify() -> bool {
        true
    }
}
impl Bool for False {
    fn reify() -> bool {
        false
    }
}

pub(crate) trait RunOr<A: Bool>: Bool {
    type Output: Bool;
}
pub(crate) type Or<A, B> = <A as RunOr<B>>::Output;

impl RunOr<True> for True {
    type Output = True;
}
impl RunOr<True> for False {
    type Output = True;
}
impl RunOr<False> for True {
    type Output = True;
}
impl RunOr<False> for False {
    type Output = False;
}