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
    LDA #{value}
    STA $55
    {command} end
forward:
    LDA #33
    STA $55
end:
    BRK"#)
}

fn create_branch_backward_test_scipt(command: &str) -> String {
    format!(
 r#"{command} forward
backward:
    LDA #42
    STA $42
    {command} end
forward:
    {command} backward
    LDA #99
    STA $42
end:
    BRK"#)
}

fn create_adc_test_scipt(command: &str) -> String {
    format!(
 r#"{command} forward
backward:
    LDA #42
    STA $42
    {command} end
forward:
    {command} backward
    LDA #99
    STA $42
end:
    BRK"#)
}

#[test]
fn lda_immediate_and_sta_zero_page() {
    let mut mock = create_mock_from_script(
        r#"LDA #10 ; load 10 into register A
           STA $5  ; store register A at address 0x00 + 0x05 = 0x05
           BRK"#,
    );
    let mut cpu = Cpu::new(&mut mock);
    unsafe { cpu.run() };
    assert_eq!(10, mock.memory[0x0005])
}

#[test]
fn lda_absolute_and_sta_absolute() {
    let mut mock = create_mock_from_script(
        r#"LDA $1234 ; load A at $1234
           STA $4321 ; store register A at 4321
           BRK"#,
    );
    let mut cpu = Cpu::new(&mut mock);
    unsafe {
        mock.mem_write_u8(0x1234, 42);
        cpu.run()
    }
    assert_eq!(42, mock.memory[0x4321])
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
fn ldy_immediate_and_sty_zero_page() {
    let mut mock = create_mock_from_script(
        r#"LDY #33 ; load 33 into register Y
           STY $2A  ; store register A at address 0x00 + 0X2A = 0x2A
           BRK"#,
    );
    let mut cpu = Cpu::new(&mut mock);
    unsafe { cpu.run() };
    assert_eq!(33, mock.memory[0x002A])
}

#[test]
fn ldy_absolute_and_sty_absolute() {
    let mut mock = create_mock_from_script(
        r#"LDY $3412 ; load T at $3412
           STY $2143 ; store register X at $2143
           BRK"#,
    );
    let mut cpu = Cpu::new(&mut mock);
    unsafe {
        mock.mem_write_u8(0x3412, 89);
        cpu.run()
    }
    assert_eq!(89, mock.memory[0x2143])
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
fn bpl_forward() {
    let script = create_branch_forward_test_scipt("BPL", 10);
    let mut mock = create_mock_from_script(&script);
    let mut cpu = Cpu::new(&mut mock);
    unsafe {
        cpu.run()
    }
    assert_eq!(33, mock.memory[0x55])
}

#[test]
fn bpl_backward() {
    let script = create_branch_backward_test_scipt("BPL");
    let mut mock = create_mock_from_script(&script);
    let mut cpu = Cpu::new(&mut mock);
    unsafe {
        cpu.run()
    }
    assert_eq!(42, mock.memory[0x42])
}

#[test]
fn bvc_forward() {
    let script = create_branch_forward_test_scipt("BVC", 10);
    let mut mock = create_mock_from_script(&script);
    let mut cpu = Cpu::new(&mut mock);
    unsafe {
        cpu.run()
    }
    assert_eq!(33, mock.memory[0x55])
}

#[test]
fn bvc_backward() {
    let script = create_branch_backward_test_scipt("BVC");
    let mut mock = create_mock_from_script(&script);
    let mut cpu = Cpu::new(&mut mock);
    unsafe {
        cpu.run()
    }
    assert_eq!(42, mock.memory[0x42])
}

#[test]
fn bne_forward() {
    let script = create_branch_forward_test_scipt("BNE", 10);
    let mut mock = create_mock_from_script(&script);
    let mut cpu = Cpu::new(&mut mock);
    unsafe {
        cpu.run()
    }
    assert_eq!(33, mock.memory[0x55])
}

#[test]
fn bne_backward() {
    let script = create_branch_backward_test_scipt("BNE");
    let mut mock = create_mock_from_script(&script);
    let mut cpu = Cpu::new(&mut mock);
    unsafe {
        cpu.run()
    }
    assert_eq!(42, mock.memory[0x42])
}

#[test]
fn beq_forward() {
    let mut mock = create_mock_from_script(
r#" LDA #0
    BEQ forward
    LDY #99
    STY $42
forward:
    LDY #42
    STY $42"#
    );
    let mut cpu = Cpu::new(&mut mock);
    unsafe {
        cpu.run()
    }
    assert_eq!(42, mock.memory[0x42])
}

#[test]
fn beq_backward() {
    let mut mock = create_mock_from_script(
r#"BCC forward
backward:
    LDX #42
    STX $42
    LDX #0
    BEQ end
forward:
    LDX #99
    STX $42
    LDA #0
    BEQ backward
end:
    BRK"#
    );
    let mut cpu = Cpu::new(&mut mock);
    unsafe {
        cpu.run()
    }
    assert_eq!(42, mock.memory[0x42])
}

#[test]
fn bmi_forward() {
    let script = create_branch_forward_test_scipt("BMI", 10u8.wrapping_neg());
    let mut mock = create_mock_from_script(&script);
    let mut cpu = Cpu::new(&mut mock);
    unsafe {
        cpu.run()
    }
    assert_eq!(10u8.wrapping_neg(), mock.memory[0x55])
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

#[test]
fn adc_with_carry(){
    let mut mock = create_mock_from_script(
 r#"LDA #255
    ADC #2
    BCS end
    STA $42
end:
    LDA #42
    STA $42"#
    );
    let mut cpu = Cpu::new(&mut mock);
    unsafe {
        cpu.run()
    }
    assert_eq!(42, mock.memory[0x42])
}

#[test]
fn adc_with_overflow(){
    let mut mock = create_mock_from_script(
 r#"LDA #80
    ADC #80
    BVS end
    STA $42
end:
    LDA #42
    STA $42"#
    );
    let mut cpu = Cpu::new(&mut mock);
    unsafe {
        cpu.run()
    }
    assert_eq!(42, mock.memory[0x42])
}

#[test]
fn test_and_1(){
    let mut mock = create_mock_from_script(
 r#"LDA #%11110000
    AND #%00001111
    STA $00"#
    );
    let mut cpu = Cpu::new(&mut mock);
    unsafe {
        cpu.run()
    }
    assert_eq!(0b0000_0000, mock.memory[0x00])
}

#[test]
fn test_and_2(){
    let mut mock = create_mock_from_script(
 r#"LDA #%01010101
    AND #%10011001
    STA $00"#
    );
    let mut cpu = Cpu::new(&mut mock);
    unsafe {
        cpu.run()
    }
    assert_eq!(0b0001_0001, mock.memory[0x00])
}

#[test]
fn test_sec(){
    let mut mock = create_mock_from_script(
 r#"SEC
    BCS good
    LDX #99
    STX $1ABC
    BEQ end
good:
    LDX #42
    STX $1ABC
end:
    BRK"#
    );
    let mut cpu = Cpu::new(&mut mock);
    unsafe {
        cpu.run()
    }
    assert_eq!(42, mock.memory[0x1ABC])
}

#[test]
fn test_clc(){
    let mut mock = create_mock_from_script(
 r#"SEC
    CLC
    BCC good
    LDX #99
    STX $1ABC
    BEQ end
good:
    LDX #42
    STX $1ABC
end:
    BRK"#
    );
    let mut cpu = Cpu::new(&mut mock);
    unsafe {
        cpu.run()
    }
    assert_eq!(42, mock.memory[0x1ABC])
}

#[test]
fn test_clv(){
    let mut mock = create_mock_from_script(
 r#"LDA #80
    ADC #80
    CLV
    BVC good
    LDX #99
    STX $1ABC
    BEQ end
good:
    LDX #42
    STX $1ABC
end:
    BRK"#
    );
    let mut cpu = Cpu::new(&mut mock);
    unsafe {
        cpu.run()
    }
    assert_eq!(42, mock.memory[0x1ABC])
}


#[test]
fn test_asl_a(){
    let mut mock = create_mock_from_script(
 r#"LDA $05
    ASL A
    STA $10
    BCS end
    LDX #99
    STX $0042
end:
    LDX #42
    STA $0042"#
    );
    let mut cpu = Cpu::new(&mut mock);
    unsafe {
        mock.mem_write_u8(0x05, 0b1010_1010);
        cpu.run()
    }
    assert_eq!(0b0101_0100, mock.memory[0x10]);
    assert_eq!(42, mock.memory[0x42])
}