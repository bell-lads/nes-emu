use nes_emu::{cpu::Cpu, traits::Memory};

struct MemoryMock {
    pub memory: [u8; 0xFFFF],
}

impl MemoryMock {
    pub fn new(program: &[u8], origin: u16) -> Self {
        let mut mock = Self {
            memory: [0x00; 0xFFFF],
        };
        unsafe {
            mock.load(program, origin);
            mock.mem_write_u16(0xFFFC, origin);
        }
        mock
    }
}

impl Memory for MemoryMock {
    unsafe fn load(&mut self, data: &[u8], dest: u16) {
        self.memory[usize::from(dest)..usize::from(dest) + data.len()].copy_from_slice(data);
    }

    unsafe fn mem_read_u8(&mut self, addr: u16) -> u8 {
        self.memory[usize::from(addr)]
    }

    unsafe fn mem_write_u8(&mut self, addr: u16, byte: u8) {
        self.memory[usize::from(addr)] = byte
    }
}

fn create_mock_from_script(script: &str) -> MemoryMock {
    let program = asm_6502::compile(script.to_string(), 0x8000);
    MemoryMock::new(&program, 0x8000)
}

fn create_branch_forward_test_scipt(command: &str, value: u8) -> String {
    format!(
 r#"{command} forward
    LDX #{value}
    STX $55
    {command} end
forward:
    LDX #33
    STX $55
end:
    BRK"#)
}

fn create_branch_backward_test_scipt(command: &str) -> String {
    format!(
 r#"{command} forward
backward:
    LDX #42
    STX $42
    {command} end
forward:
    {command} backward
    LDX #99
    STX $42
end:
    BRK"#)
}

fn create_adc_test_scipt(command: &str) -> String {
    format!(
 r#"{command} forward
backward:
    LDX #42
    STX $42
    {command} end
forward:
    {command} backward
    LDX #99
    STX $42
end:
    BRK"#)
}

#[test]
fn ldx_immediate_and_stx_zero_page() {
    let mut mock = create_mock_from_script(
        r#"LDX #25 ; load 25 into register X
           STX $9  ; store register X at address 0x0 + 0x9 = 0x09
           BRK"#,
    );
    let mut cpu = Cpu::new(&mut mock);
    unsafe { cpu.run() };
    assert_eq!(25, mock.memory[0x0009])
}

#[test]
fn ldx_absolute_and_stx_absolute() {
    let mut mock = create_mock_from_script(
        r#"LDX $2341 ; load X at $2341
           STX $3214 ; store register X at 3214
           BRK"#,
    );
    let mut cpu = Cpu::new(&mut mock);
    unsafe {
        mock.mem_write_u8(0x2341, 99);
        cpu.run()
    }
    assert_eq!(99, mock.memory[0x3214])
}

#[test]
fn bcc_forward() {
    let script = create_branch_forward_test_scipt("BCC", 10);
    let mut mock = create_mock_from_script(&script);
    let mut cpu = Cpu::new(&mut mock);
    unsafe {
        cpu.run()
    }
    assert_eq!(33, mock.memory[0x55])
}

#[test]
fn bcc_backward() {
    let script = create_branch_backward_test_scipt("BCC");
    let mut mock = create_mock_from_script(&script);
    let mut cpu = Cpu::new(&mut mock);
    unsafe {
        cpu.run()
    }
    assert_eq!(42, mock.memory[0x42])
}


#[test]
fn adc_without_carry(){
    let mut mock = create_mock_from_script(
 r#"LDA #30
    ADC #12
    STA $42"# 
    );
    let mut cpu = Cpu::new(&mut mock);
    unsafe {
        cpu.run()
    }
    assert_eq!(42, mock.memory[0x42])
}

#[test]
fn adc_signed_without_carry(){
    let mut mock = create_mock_from_script(format!(
 r#"LDA #30
    ADC #{}
    STA $42"#, 
    40u8.wrapping_neg()).as_ref());
    let mut cpu = Cpu::new(&mut mock);
    unsafe {
        cpu.run()
    }
    assert_eq!(10u8.wrapping_neg(), mock.memory[0x42])
}