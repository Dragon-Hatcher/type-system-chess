use crate::values;

pub(crate) trait FileEn {
    fn reify() -> values::File;
}
pub(crate) struct FA;
pub(crate) struct FB;
pub(crate) struct FC;
pub(crate) struct FD;
pub(crate) struct FE;
pub(crate) struct FF;
pub(crate) struct FG;
pub(crate) struct FH;

impl FileEn for FA {
    fn reify() -> values::File {
        values::File::A
    }
}
impl FileEn for FB {
    fn reify() -> values::File {
        values::File::B
    }
}
impl FileEn for FC {
    fn reify() -> values::File {
        values::File::C
    }
}
impl FileEn for FD {
    fn reify() -> values::File {
        values::File::D
    }
}
impl FileEn for FE {
    fn reify() -> values::File {
        values::File::E
    }
}
impl FileEn for FF {
    fn reify() -> values::File {
        values::File::F
    }
}
impl FileEn for FG {
    fn reify() -> values::File {
        values::File::G
    }
}
impl FileEn for FH {
    fn reify() -> values::File {
        values::File::H
    }
}
