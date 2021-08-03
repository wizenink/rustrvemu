
    use std::convert::TryInto;
    pub struct Cpu
    {
        regs: [u64; 32],
        pc: u64,
        dram: Vec<u8>,
    }

    impl Cpu {
        pub fn new(memory_size:u64,code:Vec<u8>) -> Self {
            let mut regs: [u64;32] = [0;32];
            regs[2] = memory_size;
            Self{
                regs:regs,
                pc: 0,
                dram: code
            }
        }

        pub fn finished(&self) -> bool {
            self.pc >= self.dram.len().try_into().unwrap()
        }

        pub fn dump_regs(&self) -> () {
            //println!("{:?}",self.regs);
            for (i,reg) in self.regs.iter().enumerate()
            {
                println!("x{} = {:#x}({}) ",i,reg,reg);
            }
        }
        fn fetch(&self) -> u32 {
            //Little-endian. Instructions are 32 bits, and memory is in 8 bits, so we read 4 elements.

            let i = self.pc as usize;

            return (self.dram[i] as u32)
                | ((self.dram[i + 1] as u32) << 8)
                | ((self.dram[i + 2] as u32) << 16)
                | ((self.dram[i + 3] as u32) << 24);

        }

        fn execute(&mut self,inst: u32) {
            let opcode = inst & 0x7f;

            match opcode {
                0x13 => {
                    //addi
                    let imm = ((inst & 0xfff00000) as i64 >> 20) as u64;
                    let rs1 = ((inst >> 15) & 0x1f) as usize;
                    let rd = ((inst >> 7) & 0x1f) as usize;

                    self.regs[rd] = self.regs[rs1].wrapping_add(imm);
                }
                0x33 => {
                    //add
                    let rs1 = ((inst >> 15) & 0x1f) as usize;
                    let rs2 = ((inst >> 20) & 0x1f) as usize;
                    let rd = ((inst >> 7) & 0x1f) as usize;

                    self.regs[rd] = self.regs[rs1].wrapping_add(self.regs[rs2]);
                }
                _ => {
                    dbg!("Not implemented!");
                }
            }
        }

        pub fn step(&mut self) -> () {
            let inst = self.fetch();

            self.pc = self.pc + 4;

            //Decode
            //Execute
            self.execute(inst);
        }
    }

