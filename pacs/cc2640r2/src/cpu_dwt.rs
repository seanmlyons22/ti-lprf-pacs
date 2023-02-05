#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Use the DWT Control Register to enable the DWT unit."]
    pub ctrl: CTRL,
    #[doc = "0x04 - Current PC Sampler Cycle Count This register is used to count the number of core cycles. This counter can measure elapsed execution time. This is a free-running counter (this counter will not advance in power modes where free-running clock to CPU stops). The counter has three functions: 1: When CTRL.PCSAMPLEENA = 1, the PC is sampled and emitted when the selected tapped bit changes value (0 to 1 or 1 to 0) and any post-scalar value counts to 0. 2: When CTRL.CYCEVTENA = 1 , (and CTRL.PCSAMPLEENA = 0), an event is emitted when the selected tapped bit changes value (0 to 1 or 1 to 0) and any post-scalar value counts to 0. 3: Applications and debuggers can use the counter to measure elapsed execution time. By subtracting a start and an end time, an application can measure time between in-core clocks (other than when Halted in debug). This is valid to 2^32 core clock cycles (for example, almost 89.5 seconds at 48MHz)."]
    pub cyccnt: CYCCNT,
    #[doc = "0x08 - CPI Count This register is used to count the total number of instruction cycles beyond the first cycle."]
    pub cpicnt: CPICNT,
    #[doc = "0x0c - Exception Overhead Count This register is used to count the total cycles spent in interrupt processing."]
    pub exccnt: EXCCNT,
    #[doc = "0x10 - Sleep Count This register is used to count the total number of cycles during which the processor is sleeping."]
    pub sleepcnt: SLEEPCNT,
    #[doc = "0x14 - LSU Count This register is used to count the total number of cycles during which the processor is processing an LSU operation beyond the first cycle."]
    pub lsucnt: LSUCNT,
    #[doc = "0x18 - Fold Count This register is used to count the total number of folded instructions. The counter increments on each instruction which takes 0 cycles."]
    pub foldcnt: FOLDCNT,
    #[doc = "0x1c - Program Counter Sample This register is used to enable coarse-grained software profiling using a debug agent, without changing the currently executing code. If the core is not in debug state, the value returned is the instruction address of a recently executed instruction. If the core is in debug state, the value returned is 0xFFFFFFFF."]
    pub pcsr: PCSR,
    #[doc = "0x20 - Comparator 0 This register is used to write the reference value for comparator 0."]
    pub comp0: COMP0,
    #[doc = "0x24 - Mask 0 Use the DWT Mask Registers 0 to apply a mask to data addresses when matching against COMP0."]
    pub mask0: MASK0,
    #[doc = "0x28 - Function 0 Use the DWT Function Registers 0 to control the operation of the comparator 0. This comparator can: 1. Match against either the PC or the data address. This is controlled by CYCMATCH. This function is only available for comparator 0 (COMP0). 2. Emit data or PC couples, trigger the ETM, or generate a watchpoint depending on the operation defined by FUNCTION."]
    pub function0: FUNCTION0,
    _reserved11: [u8; 0x04],
    #[doc = "0x30 - Comparator 1 This register is used to write the reference value for comparator 1."]
    pub comp1: COMP1,
    #[doc = "0x34 - Mask 1 Use the DWT Mask Registers 1 to apply a mask to data addresses when matching against COMP1."]
    pub mask1: MASK1,
    #[doc = "0x38 - Function 1 Use the DWT Function Registers 1 to control the operation of the comparator 1. This comparator can: 1. Perform data value comparisons if associated address comparators have performed an address match. This function is only available for comparator 1 (COMP1). 2. Emit data or PC couples, trigger the ETM, or generate a watchpoint depending on the operation defined by FUNCTION."]
    pub function1: FUNCTION1,
    _reserved14: [u8; 0x04],
    #[doc = "0x40 - Comparator 2 This register is used to write the reference value for comparator 2."]
    pub comp2: COMP2,
    #[doc = "0x44 - Mask 2 Use the DWT Mask Registers 2 to apply a mask to data addresses when matching against COMP2."]
    pub mask2: MASK2,
    #[doc = "0x48 - Function 2 Use the DWT Function Registers 2 to control the operation of the comparator 2. This comparator can emit data or PC couples, trigger the ETM, or generate a watchpoint depending on the operation defined by FUNCTION."]
    pub function2: FUNCTION2,
    _reserved17: [u8; 0x04],
    #[doc = "0x50 - Comparator 3 This register is used to write the reference value for comparator 3."]
    pub comp3: COMP3,
    #[doc = "0x54 - Mask 3 Use the DWT Mask Registers 3 to apply a mask to data addresses when matching against COMP3."]
    pub mask3: MASK3,
    #[doc = "0x58 - Function 3 Use the DWT Function Registers 3 to control the operation of the comparator 3. This comparator can emit data or PC couples, trigger the ETM, or generate a watchpoint depending on the operation defined by FUNCTION."]
    pub function3: FUNCTION3,
}
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control Use the DWT Control Register to enable the DWT unit."]
pub mod ctrl;
#[doc = "CYCCNT (rw) register accessor: an alias for `Reg<CYCCNT_SPEC>`"]
pub type CYCCNT = crate::Reg<cyccnt::CYCCNT_SPEC>;
#[doc = "Current PC Sampler Cycle Count This register is used to count the number of core cycles. This counter can measure elapsed execution time. This is a free-running counter (this counter will not advance in power modes where free-running clock to CPU stops). The counter has three functions: 1: When CTRL.PCSAMPLEENA = 1, the PC is sampled and emitted when the selected tapped bit changes value (0 to 1 or 1 to 0) and any post-scalar value counts to 0. 2: When CTRL.CYCEVTENA = 1 , (and CTRL.PCSAMPLEENA = 0), an event is emitted when the selected tapped bit changes value (0 to 1 or 1 to 0) and any post-scalar value counts to 0. 3: Applications and debuggers can use the counter to measure elapsed execution time. By subtracting a start and an end time, an application can measure time between in-core clocks (other than when Halted in debug). This is valid to 2^32 core clock cycles (for example, almost 89.5 seconds at 48MHz)."]
pub mod cyccnt;
#[doc = "CPICNT (rw) register accessor: an alias for `Reg<CPICNT_SPEC>`"]
pub type CPICNT = crate::Reg<cpicnt::CPICNT_SPEC>;
#[doc = "CPI Count This register is used to count the total number of instruction cycles beyond the first cycle."]
pub mod cpicnt;
#[doc = "EXCCNT (rw) register accessor: an alias for `Reg<EXCCNT_SPEC>`"]
pub type EXCCNT = crate::Reg<exccnt::EXCCNT_SPEC>;
#[doc = "Exception Overhead Count This register is used to count the total cycles spent in interrupt processing."]
pub mod exccnt;
#[doc = "SLEEPCNT (rw) register accessor: an alias for `Reg<SLEEPCNT_SPEC>`"]
pub type SLEEPCNT = crate::Reg<sleepcnt::SLEEPCNT_SPEC>;
#[doc = "Sleep Count This register is used to count the total number of cycles during which the processor is sleeping."]
pub mod sleepcnt;
#[doc = "LSUCNT (rw) register accessor: an alias for `Reg<LSUCNT_SPEC>`"]
pub type LSUCNT = crate::Reg<lsucnt::LSUCNT_SPEC>;
#[doc = "LSU Count This register is used to count the total number of cycles during which the processor is processing an LSU operation beyond the first cycle."]
pub mod lsucnt;
#[doc = "FOLDCNT (rw) register accessor: an alias for `Reg<FOLDCNT_SPEC>`"]
pub type FOLDCNT = crate::Reg<foldcnt::FOLDCNT_SPEC>;
#[doc = "Fold Count This register is used to count the total number of folded instructions. The counter increments on each instruction which takes 0 cycles."]
pub mod foldcnt;
#[doc = "PCSR (rw) register accessor: an alias for `Reg<PCSR_SPEC>`"]
pub type PCSR = crate::Reg<pcsr::PCSR_SPEC>;
#[doc = "Program Counter Sample This register is used to enable coarse-grained software profiling using a debug agent, without changing the currently executing code. If the core is not in debug state, the value returned is the instruction address of a recently executed instruction. If the core is in debug state, the value returned is 0xFFFFFFFF."]
pub mod pcsr;
#[doc = "COMP0 (rw) register accessor: an alias for `Reg<COMP0_SPEC>`"]
pub type COMP0 = crate::Reg<comp0::COMP0_SPEC>;
#[doc = "Comparator 0 This register is used to write the reference value for comparator 0."]
pub mod comp0;
#[doc = "MASK0 (rw) register accessor: an alias for `Reg<MASK0_SPEC>`"]
pub type MASK0 = crate::Reg<mask0::MASK0_SPEC>;
#[doc = "Mask 0 Use the DWT Mask Registers 0 to apply a mask to data addresses when matching against COMP0."]
pub mod mask0;
#[doc = "FUNCTION0 (rw) register accessor: an alias for `Reg<FUNCTION0_SPEC>`"]
pub type FUNCTION0 = crate::Reg<function0::FUNCTION0_SPEC>;
#[doc = "Function 0 Use the DWT Function Registers 0 to control the operation of the comparator 0. This comparator can: 1. Match against either the PC or the data address. This is controlled by CYCMATCH. This function is only available for comparator 0 (COMP0). 2. Emit data or PC couples, trigger the ETM, or generate a watchpoint depending on the operation defined by FUNCTION."]
pub mod function0;
#[doc = "COMP1 (rw) register accessor: an alias for `Reg<COMP1_SPEC>`"]
pub type COMP1 = crate::Reg<comp1::COMP1_SPEC>;
#[doc = "Comparator 1 This register is used to write the reference value for comparator 1."]
pub mod comp1;
#[doc = "MASK1 (rw) register accessor: an alias for `Reg<MASK1_SPEC>`"]
pub type MASK1 = crate::Reg<mask1::MASK1_SPEC>;
#[doc = "Mask 1 Use the DWT Mask Registers 1 to apply a mask to data addresses when matching against COMP1."]
pub mod mask1;
#[doc = "FUNCTION1 (rw) register accessor: an alias for `Reg<FUNCTION1_SPEC>`"]
pub type FUNCTION1 = crate::Reg<function1::FUNCTION1_SPEC>;
#[doc = "Function 1 Use the DWT Function Registers 1 to control the operation of the comparator 1. This comparator can: 1. Perform data value comparisons if associated address comparators have performed an address match. This function is only available for comparator 1 (COMP1). 2. Emit data or PC couples, trigger the ETM, or generate a watchpoint depending on the operation defined by FUNCTION."]
pub mod function1;
#[doc = "COMP2 (rw) register accessor: an alias for `Reg<COMP2_SPEC>`"]
pub type COMP2 = crate::Reg<comp2::COMP2_SPEC>;
#[doc = "Comparator 2 This register is used to write the reference value for comparator 2."]
pub mod comp2;
#[doc = "MASK2 (rw) register accessor: an alias for `Reg<MASK2_SPEC>`"]
pub type MASK2 = crate::Reg<mask2::MASK2_SPEC>;
#[doc = "Mask 2 Use the DWT Mask Registers 2 to apply a mask to data addresses when matching against COMP2."]
pub mod mask2;
#[doc = "FUNCTION2 (rw) register accessor: an alias for `Reg<FUNCTION2_SPEC>`"]
pub type FUNCTION2 = crate::Reg<function2::FUNCTION2_SPEC>;
#[doc = "Function 2 Use the DWT Function Registers 2 to control the operation of the comparator 2. This comparator can emit data or PC couples, trigger the ETM, or generate a watchpoint depending on the operation defined by FUNCTION."]
pub mod function2;
#[doc = "COMP3 (rw) register accessor: an alias for `Reg<COMP3_SPEC>`"]
pub type COMP3 = crate::Reg<comp3::COMP3_SPEC>;
#[doc = "Comparator 3 This register is used to write the reference value for comparator 3."]
pub mod comp3;
#[doc = "MASK3 (rw) register accessor: an alias for `Reg<MASK3_SPEC>`"]
pub type MASK3 = crate::Reg<mask3::MASK3_SPEC>;
#[doc = "Mask 3 Use the DWT Mask Registers 3 to apply a mask to data addresses when matching against COMP3."]
pub mod mask3;
#[doc = "FUNCTION3 (rw) register accessor: an alias for `Reg<FUNCTION3_SPEC>`"]
pub type FUNCTION3 = crate::Reg<function3::FUNCTION3_SPEC>;
#[doc = "Function 3 Use the DWT Function Registers 3 to control the operation of the comparator 3. This comparator can emit data or PC couples, trigger the ETM, or generate a watchpoint depending on the operation defined by FUNCTION."]
pub mod function3;
