use crate::CoreState;

pub trait Register {
    fn load(state: &CoreState) -> i32;
    fn store(state: &mut CoreState, val: i32);
}



pub struct X0;
impl Register for X0 {
    fn load(_: &CoreState) -> i32 {
        0
    }
    fn store(_: &mut CoreState, _: i32) {
        // No-op
    }
}
pub use X0 as x0;

macro_rules! reg {
    ($name:ident $n:literal $name2:ident) => {
        pub struct $name;
        impl Register for $name {
            fn load(state: &CoreState) -> i32 {
                state.registers[$n - 1]
            }
            fn store(state: &mut CoreState, val: i32) {
                state.registers[$n - 1] = val;
            }
        }

        #[allow(unused)]
        pub use $name as $name2;
    };
}


reg!(X1 1 x1);
reg!(X2 2 x2);
reg!(X3 3 x3);
reg!(X4 4 x4);
reg!(X5 5 x5);
reg!(X6 6 x6);
reg!(X7 7 x7);
reg!(X8 8 x8);
reg!(X9 9 x9);
reg!(X10 10 x10);
reg!(X11 11 x11);
reg!(X12 12 x12);
reg!(X13 13 x13);
reg!(X14 14 x14);
reg!(X15 15 x15);
reg!(X16 16 x16);
reg!(X17 17 x17);
reg!(X18 18 x18);
reg!(X19 19 x19);
reg!(X20 20 x20);
reg!(X21 21 x21);
reg!(X22 22 x22);
reg!(X23 23 x23);
reg!(X24 24 x24);
reg!(X25 25 x25);
reg!(X26 26 x26);
reg!(X27 27 x27);
reg!(X28 28 x28);
reg!(X29 29 x29);
reg!(X30 30 x30);
reg!(X31 31 x31);
