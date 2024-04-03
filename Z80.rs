//8bit Registers
let regA: u8 = 0; //Accumulator
let regB: u8 = 0;
let regC: u8 = 0;
let regD: u8 = 0;
let regE: u8 = 0;
let regF: u8 = 0; //Flags(ZNHC) + 4 bits
let regH: u8 = 0;
let regL: u8 = 0;

//16bit Registers
let regAF: u16 = 0; //Accumulator + F
let regBC: u16 = 0; //B + C
let regDE: u16 = 0; //D + E
let regHL: u16 = 0; //H + L

//Special
let regPC: u16 = 0; //Program counter
let regSP: u16 = 0; //Stack pointer

//RAM
RAM = [u8; 32000];
VRAM = [u8; 16000];

//Flags
enum Flags {
    Z = (1 << 7) //Zero flag
    N = (1 << 6) //Subtract flag
    H = (1 << 5) //Half-carry flag
    C = (1 << 4) //Carry flag
}

//Signals
void z80::clock() {
    if (cycles==0) {
        opcode = read(pc);
        pc++;

        //TODO: read from inst_table for cycle count
    }    
}
void reset();
void interrupt();
void nm_interrupt();

u8 fetch();
u8 fetched = 0x00;

u16 abs_addr = 0x0000;
u16 rel_addr = 0x00;
u8 opcode = 0x00;
u8 cycles = 0;

//Addressing modes



//Opcodes

u8 NOP();   u8 ADD();   u8 ADC();   u8 SUB();   u8 SBC();   u8 AND();
u8 XOR();   u8 OR();    u8 CP();    u8 LD();    u8 LDH();   u8 INC();   
u8 DEC();   u8 BIT();   u8 RES();   u8 SET();   u8 SWAP();  u8 RL();    
u8 RLA();   u8 RLC();   u8 RLCA();  u8 RR();    u8 RRA();   u8 RRC();   
u8 RRCA();  u8 SLA();   u8 SRA();   u8 SRL();   u8 CALL();  u8 JP();
u8 JR();    u8 RET();   u8 RETI();  u8 RST();   u8 POP();   u8 PUSH();
u8 CCF();   u8 CPL();   u8 DAA();   u8 DI();    u8 EI();    u8 HALT();
u8 SCF();   u8 STOP();       


//Instruction



