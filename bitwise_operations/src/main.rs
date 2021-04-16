/*
Bitwise operations: on individual bits rather than sets of bytes.
binary representation, sequence of bytes
underscore separator allowed for legibility
by default binary representations are store in i32
*/
fn main() {
    // stored as u8 by adding suffix u8
    let mut value = 0b1111_0101u8;

    // will print base10 representation
    println!("value is {}", value);

    /*
    :08b
        0 -> display leading zeros
        8 -> number of bits to display
        b -> display binary representation
    */
    println!("value is {:08b}", value);

    // bitwise NOT: invert individual bits
    value = !value; // 0000_1010
    println!("value is {:08b}", value);

    // bitwise AND: use to clear value of a specific bit
    value = value & 0b1111_0111; // -> 0000_0010
    println!("value is {:08b}", value);

    // bitwise AND: use to check value of a specific bit
    // if a specific bit is 0 or 1, useful to check status registers for process state
    println!("value is {:08b}", value & 0b0100_0000); // -> 0000_0000

    // bitwise OR: if either operand is 1, result is one
    // useful to set value of a specific bit
    value = value | 0b0100_0000; // -> 0100_0010
    println!("value is {:08b}", value);

    // bitwise XOR (exclusive OR): result is 1 only when bits are different, otherwise 0
    // useful to set if bits are different
    value = value ^ 0b0101_0101; // -> 0001_0111
    println!("value is {:08b}", value);

    ////////////////////////////
    // Bit Shift operators
    ////////////////////////////

    // shift bit pattern left or right by a number of bits
    // and backfill shifted bit spaces with zeros

    // shift left by 4 bits
    value = value << 4; // -> 0111_0000
    println!("value is {:08b}", value);

    // shift right by 3 bits
    value = value >> 3; // -> 0000_1110
    println!("value is {:08b}", value);
}
