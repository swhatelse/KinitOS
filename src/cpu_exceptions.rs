#![allow(dead_code)]

use KinitOS::{println, serial_println};
use core::arch::asm;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum CPUExceptionType {
    ZeroDivide = 0,
    Debug = 1,
    NonMaskableInt = 2,
    BreakPoint = 3,
    Overflow = 4,
    BoundCheck = 5,
    InvalidOpcde = 6,
    DevNotAvail = 7,
    DoubleFault = 8,
    CoprocSegOverrun = 9,
    InvalidTSS = 10,
    SegNotPresent = 11,
    StackSegFault = 12,
    GeneralProtection = 13,
    PageFault = 14,
    /* Reserved = 15 */
    FloatPointError = 16,
    AlignCheck = 17,
    MachineCheck = 18,
    SIMDFloatException = 19
}

#[repr(u64)]
enum GateType {
    Interrupt = 0x8E,
    Trap = 0x8F,
    Task
}

/******************************************************************
 *
 *                     Interrupt Descriptor Options
 *
 *****************************************************************/
struct InterruptDescriptorOptions(u16);

impl InterruptDescriptorOptions {
    fn set_gate_type(&mut self, gate_type: GateType){
        match gate_type {
            GateType::Interrupt => todo!(),
            GateType::Trap => todo!(),
        }
    }
}

/******************************************************************
 *
 *                     Interrupt Descriptor
 *
 *****************************************************************/
#[derive(Copy, Clone)]
struct InterruptDescriptor {
    offset1: u16,                                // Low bits address of the handler function
    seg_selector: u16,                           // GDT Selector
    ist: u8,                                     // interrupt stack table
    type_attributes: InterruptDescriptorOptions, // Options
    offset2: u16,                                // Mid bits address of the handler function
    offset3: u32,                                // High bits address  of the handler function
    zero: u32                                    // Reserved
}


impl InterruptDescriptor {
    /**
     * Create an empty interrupt descriptor
     */
    pub fn new() -> Self {
        Self {
            offset1: 0,
            seg_selector: 0,
            ist: 0,
            type_attributes: 0,
            offset2: 0,
            offset3: 0,
            zero: 0
        }
    }

    /**
     *
     * isr_addr:
     * handler_addr:
     * gate_type
     *
     */
    pub fn set_options(&mut self, isr_addr, handler_addr: f(), gate_type: GateType) {
        self.ist = 0;
        self.type_attributes = 0x1 << 8 & 0x0 << 6 & 0xF; // 0x0 ??? what is that?        
    }

    /**
     *
     * addr: pointer of handler function
     *
     */
    pub fn set_address(&mut self, addr: fn()){
        self.offset1 = addr as u16;
        self.offset2 = ((addr as usize) >> 16) as u16;
        self.offset3 = ((addr as usize) >> 16) as u32;
    }
}

/******************************************************************
 *
 *                     Interrupt Descriptor
 *
 *****************************************************************/
const MAX_IDT_ENTRY: usize = 256;
#[repr(C)]
struct InterruptDescriptorTable {
    entries: [InterruptDescriptor; MAX_IDT_ENTRY]
}
    
impl InterruptDescriptorTable {
    fn new() -> Self {
        Self {entries:  [InterruptDescriptor::new(); MAX_IDT_ENTRY] } 
    }

    #[inline(always)]
    pub fn get(&self, idx: usize) -> InterruptDescriptor {
        self.entries[idx]
    }

    #[inline(always)]
    pub fn set(&mut self, idx: usize, desc: InterruptDescriptor) {
        self.entries[idx] = desc;
    }
}

fn divide_error(){
    serial_println!("divide error!");
}

fn debug(){

}

fn nmi() {

}

fn int3() {

}

fn overflow() {

}

fn bounds() {

}

fn invalid_op() {

}

fn disable_interrupt() {
    unsafe {

    }
}

// Loads the Interrupt Descriptor Table in the idtr register.
fn unsafe load_idt(idt: &InterruptDescriptorTable) {
    asm!("lidt [{}]", in("reg") idt as *const InterruptDescriptorTable);
}

fn idt_init(idt: &mut InterruptDescriptorTable) {
    disable_interrupt();

    // idt.set(0, InterruptDescriptor {});
    // idt.set(0, InterruptDescriptor {});
    unsafe {
        load_idt(idt);
    }
}

#[allow(dead_code)]
fn exception_handler(exception: CPUExceptionType) { 
    match exception {
        CPUExceptionType::ZeroDivide =>  {},
        CPUExceptionType::Debug      =>  {},
        CPUExceptionType::NonMaskableInt    =>  {},
        CPUExceptionType::BreakPoint =>  {},
        CPUExceptionType::Overflow   =>  {},
        CPUExceptionType::BoundCheck =>  {},
        CPUExceptionType::InvalidOpcde => {},
        CPUExceptionType::DevNotAvail => {},
        CPUExceptionType::DoubleFault => {},
        CPUExceptionType::CoprocSegOverrun => {},
        CPUExceptionType::InvalidTSS => {},
        CPUExceptionType::SegNotPresent => {},
        CPUExceptionType::StackSegFault => {},
        CPUExceptionType::GeneralProtection => {},
        CPUExceptionType::PageFault => {},
        CPUExceptionType::FloatPointError => {},
        CPUExceptionType::AlignCheck => {},
        CPUExceptionType::MachineCheck => {},
        CPUExceptionType::SIMDFloatException => {}
    }
    
}

#[test_case]
fn test() {
    let f = &divide_error;
    let g = &debug;
    f();
    // serial_println!("{}", f);
    // serial_println!("{}", g);
}
