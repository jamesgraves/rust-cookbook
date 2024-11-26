use bitflags::bitflags;
use std::fmt;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    struct MyFlags: u32 {
        const FLAG_A       = 0b00000001;
        const FLAG_B       = 0b00000010;
        const FLAG_C       = 0b00000100;
    }
}

impl fmt::Display for MyFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:032b}", self )
    }
}

fn main() {
    let all_flags = MyFlags::FLAG_A | MyFlags::FLAG_B | MyFlags::FLAG_C;
    let e1 = MyFlags::FLAG_A | MyFlags::FLAG_C;
    let e2 = MyFlags::FLAG_B | MyFlags::FLAG_C;
    assert_eq!((e1 | e2), all_flags);
    assert_eq!((e1 & e2), MyFlags::FLAG_C);
    assert_eq!((e1 - e2), MyFlags::FLAG_A);
    assert_eq!(!e2, MyFlags::FLAG_A);

    assert_eq!(format!("{}", all_flags), "00000000000000000000000000000111");
    assert_eq!(format!("{}", !all_flags), "00000000000000000000000000000000");
    assert_eq!(format!("{:?}", MyFlags::FLAG_B), "MyFlags(FLAG_B)");
    assert_eq!(format!("{:?}", MyFlags::FLAG_A | MyFlags::FLAG_B), "MyFlags(FLAG_A | FLAG_B)");
}
