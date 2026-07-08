
struct Instruction(u32);

const fn mask(high: u32, low: u32) -> u32 {
    if high >= 32 || low >= high {
        0
    } else if high == 31 {
        u32::MAX - (1 << low) + 1
    } else {
        (1u32 << (high + 1)) - (1 << low)
    }

}
const fn top_bit(val: u32) -> bool {
    val & (1 << 31) == 1
}

const _: () = {
    assert!(mask(12, 5) == 0b0001_1111_1110_0000);
    assert!(mask(30,20).count_ones() == 11);
    assert!(mask(31,11).count_ones() == (31 - 11) + 1);
    assert!(mask(31, 11) == 0b1111_1111_1111_1111_1111_1000_0000_0000u32);
};

fn i_intermediate(i: &Instruction) -> i32 {
    let val = i.0;
    let res_10_0 = val & mask(30, 20);
    let res_31_11 = if top_bit(val) { mask(31, 11) } else { 0 };

    (res_31_11 << 11 + res_10_0) as i32
}
fn s_intermediate(i: &Instruction) -> i32 {
    let val = i.0;
    let res_4_0 = val & mask(11, 7);
    let res_10_5 = val & mask(30, 25);
    let res_31_11 = if top_bit(val) { mask(31, 11) } else { 0 };

    (res_31_11 << 11 + res_10_5 << 5 + res_4_0) as i32
}
fn b_intermediate(i: &Instruction) -> i32 {
    let val = i.0;
    let res_4_1 = val & mask(11, 8);
    let res_10_5 = val & mask(30, 25);
    let res_11 = val & (1 << 7);
    let res_31_12 = if top_bit(val) { mask(31, 12) } else { 0 };

    (res_31_12 << 12 + res_11 << 1 + res_10_5 << 5 + res_4_1 << 1) as i32
}
fn u_intermediate(i: &Instruction) -> i32 {
    let val = i.0;
    let res_11_0 = 0u32;
    let res_30_12 = val & mask(30, 12);
    let res_31 = if top_bit(val) { 1u32 } else { 0 };

    (res_31 << 31 + res_30_12 << 12 + res_11_0) as i32
}
fn j_intermediate(i: &Instruction) -> i32 {
    let val = i.0;
    let res_4_1 = ((val & mask(24, 21)) >> 21) << 1;
    let res_10_5 = ((val & mask(30, 25)) >> 25) << 5;
    let res_11 = ((val & (1 << 20)) >> 20) << 11;
    let res_19_12 = val & mask(19,12);
    let res_31_20 = if top_bit(val) { mask(31, 20) } else { 0 };

    (res_31_20 + res_19_12 + res_11 + res_10_5 + res_4_1) as i32
}