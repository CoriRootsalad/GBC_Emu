#include Z80.rs

fn read(u16 addr) {
    dat = Z80.RAM[addr];
    return dat;
}

fn write(u16 addr, u8 dat | u16 dat, bool readOnly) {
    Z80.RAM[addr] = dat;
    return 0;
}