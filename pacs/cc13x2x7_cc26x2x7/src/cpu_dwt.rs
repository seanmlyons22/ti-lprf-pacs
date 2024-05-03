#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl: Ctrl,
    cyccnt: Cyccnt,
    cpicnt: Cpicnt,
    exccnt: Exccnt,
    sleepcnt: Sleepcnt,
    lsucnt: Lsucnt,
    foldcnt: Foldcnt,
    pcsr: Pcsr,
    comp0: Comp0,
    mask0: Mask0,
    function0: Function0,
    _reserved11: [u8; 0x04],
    comp1: Comp1,
    mask1: Mask1,
    function1: Function1,
    _reserved14: [u8; 0x04],
    comp2: Comp2,
    mask2: Mask2,
    function2: Function2,
    _reserved17: [u8; 0x04],
    comp3: Comp3,
    mask3: Mask3,
    function3: Function3,
}
impl RegisterBlock {
    #[doc = "0x00 - Control Use the DWT Control Register to enable the DWT unit."]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x04 - Current PC Sampler Cycle Count This register is used to count the number of core cycles. This counter can measure elapsed execution time. This is a free-running counter (this counter will not advance in power modes where free-running clock to CPU stops). The counter has three functions: 1: When CTRL.PCSAMPLEENA = 1, the PC is sampled and emitted when the selected tapped bit changes value (0 to 1 or 1 to 0) and any post-scalar value counts to 0. 2: When CTRL.CYCEVTENA = 1 , (and CTRL.PCSAMPLEENA = 0), an event is emitted when the selected tapped bit changes value (0 to 1 or 1 to 0) and any post-scalar value counts to 0. 3: Applications and debuggers can use the counter to measure elapsed execution time. By subtracting a start and an end time, an application can measure time between in-core clocks (other than when Halted in debug). This is valid to 2^32 core clock cycles (for example, almost 89.5 seconds at 48MHz)."]
    #[inline(always)]
    pub const fn cyccnt(&self) -> &Cyccnt {
        &self.cyccnt
    }
    #[doc = "0x08 - CPI Count This register is used to count the total number of instruction cycles beyond the first cycle."]
    #[inline(always)]
    pub const fn cpicnt(&self) -> &Cpicnt {
        &self.cpicnt
    }
    #[doc = "0x0c - Exception Overhead Count This register is used to count the total cycles spent in interrupt processing."]
    #[inline(always)]
    pub const fn exccnt(&self) -> &Exccnt {
        &self.exccnt
    }
    #[doc = "0x10 - Sleep Count This register is used to count the total number of cycles during which the processor is sleeping."]
    #[inline(always)]
    pub const fn sleepcnt(&self) -> &Sleepcnt {
        &self.sleepcnt
    }
    #[doc = "0x14 - LSU Count This register is used to count the total number of cycles during which the processor is processing an LSU operation beyond the first cycle."]
    #[inline(always)]
    pub const fn lsucnt(&self) -> &Lsucnt {
        &self.lsucnt
    }
    #[doc = "0x18 - Fold Count This register is used to count the total number of folded instructions. The counter increments on each instruction which takes 0 cycles."]
    #[inline(always)]
    pub const fn foldcnt(&self) -> &Foldcnt {
        &self.foldcnt
    }
    #[doc = "0x1c - Program Counter Sample This register is used to enable coarse-grained software profiling using a debug agent, without changing the currently executing code. If the core is not in debug state, the value returned is the instruction address of a recently executed instruction. If the core is in debug state, the value returned is 0xFFFFFFFF."]
    #[inline(always)]
    pub const fn pcsr(&self) -> &Pcsr {
        &self.pcsr
    }
    #[doc = "0x20 - Comparator 0 This register is used to write the reference value for comparator 0."]
    #[inline(always)]
    pub const fn comp0(&self) -> &Comp0 {
        &self.comp0
    }
    #[doc = "0x24 - Mask 0 Use the DWT Mask Registers 0 to apply a mask to data addresses when matching against COMP0."]
    #[inline(always)]
    pub const fn mask0(&self) -> &Mask0 {
        &self.mask0
    }
    #[doc = "0x28 - Function 0 Use the DWT Function Registers 0 to control the operation of the comparator 0. This comparator can: 1. Match against either the PC or the data address. This is controlled by CYCMATCH. This function is only available for comparator 0 (COMP0). 2. Emit data or PC couples, trigger the ETM, or generate a watchpoint depending on the operation defined by FUNCTION."]
    #[inline(always)]
    pub const fn function0(&self) -> &Function0 {
        &self.function0
    }
    #[doc = "0x30 - Comparator 1 This register is used to write the reference value for comparator 1."]
    #[inline(always)]
    pub const fn comp1(&self) -> &Comp1 {
        &self.comp1
    }
    #[doc = "0x34 - Mask 1 Use the DWT Mask Registers 1 to apply a mask to data addresses when matching against COMP1."]
    #[inline(always)]
    pub const fn mask1(&self) -> &Mask1 {
        &self.mask1
    }
    #[doc = "0x38 - Function 1 Use the DWT Function Registers 1 to control the operation of the comparator 1. This comparator can: 1. Perform data value comparisons if associated address comparators have performed an address match. This function is only available for comparator 1 (COMP1). 2. Emit data or PC couples, trigger the ETM, or generate a watchpoint depending on the operation defined by FUNCTION."]
    #[inline(always)]
    pub const fn function1(&self) -> &Function1 {
        &self.function1
    }
    #[doc = "0x40 - Comparator 2 This register is used to write the reference value for comparator 2."]
    #[inline(always)]
    pub const fn comp2(&self) -> &Comp2 {
        &self.comp2
    }
    #[doc = "0x44 - Mask 2 Use the DWT Mask Registers 2 to apply a mask to data addresses when matching against COMP2."]
    #[inline(always)]
    pub const fn mask2(&self) -> &Mask2 {
        &self.mask2
    }
    #[doc = "0x48 - Function 2 Use the DWT Function Registers 2 to control the operation of the comparator 2. This comparator can emit data or PC couples, trigger the ETM, or generate a watchpoint depending on the operation defined by FUNCTION."]
    #[inline(always)]
    pub const fn function2(&self) -> &Function2 {
        &self.function2
    }
    #[doc = "0x50 - Comparator 3 This register is used to write the reference value for comparator 3."]
    #[inline(always)]
    pub const fn comp3(&self) -> &Comp3 {
        &self.comp3
    }
    #[doc = "0x54 - Mask 3 Use the DWT Mask Registers 3 to apply a mask to data addresses when matching against COMP3."]
    #[inline(always)]
    pub const fn mask3(&self) -> &Mask3 {
        &self.mask3
    }
    #[doc = "0x58 - Function 3 Use the DWT Function Registers 3 to control the operation of the comparator 3. This comparator can emit data or PC couples, trigger the ETM, or generate a watchpoint depending on the operation defined by FUNCTION."]
    #[inline(always)]
    pub const fn function3(&self) -> &Function3 {
        &self.function3
    }
}
#[doc = "CTRL (rw) register accessor: Control Use the DWT Control Register to enable the DWT unit.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "Control Use the DWT Control Register to enable the DWT unit."]
pub mod ctrl;
#[doc = "CYCCNT (rw) register accessor: Current PC Sampler Cycle Count This register is used to count the number of core cycles. This counter can measure elapsed execution time. This is a free-running counter (this counter will not advance in power modes where free-running clock to CPU stops). The counter has three functions: 1: When CTRL.PCSAMPLEENA = 1, the PC is sampled and emitted when the selected tapped bit changes value (0 to 1 or 1 to 0) and any post-scalar value counts to 0. 2: When CTRL.CYCEVTENA = 1 , (and CTRL.PCSAMPLEENA = 0), an event is emitted when the selected tapped bit changes value (0 to 1 or 1 to 0) and any post-scalar value counts to 0. 3: Applications and debuggers can use the counter to measure elapsed execution time. By subtracting a start and an end time, an application can measure time between in-core clocks (other than when Halted in debug). This is valid to 2^32 core clock cycles (for example, almost 89.5 seconds at 48MHz).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cyccnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cyccnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cyccnt`]
module"]
#[doc(alias = "CYCCNT")]
pub type Cyccnt = crate::Reg<cyccnt::CyccntSpec>;
#[doc = "Current PC Sampler Cycle Count This register is used to count the number of core cycles. This counter can measure elapsed execution time. This is a free-running counter (this counter will not advance in power modes where free-running clock to CPU stops). The counter has three functions: 1: When CTRL.PCSAMPLEENA = 1, the PC is sampled and emitted when the selected tapped bit changes value (0 to 1 or 1 to 0) and any post-scalar value counts to 0. 2: When CTRL.CYCEVTENA = 1 , (and CTRL.PCSAMPLEENA = 0), an event is emitted when the selected tapped bit changes value (0 to 1 or 1 to 0) and any post-scalar value counts to 0. 3: Applications and debuggers can use the counter to measure elapsed execution time. By subtracting a start and an end time, an application can measure time between in-core clocks (other than when Halted in debug). This is valid to 2^32 core clock cycles (for example, almost 89.5 seconds at 48MHz)."]
pub mod cyccnt;
#[doc = "CPICNT (rw) register accessor: CPI Count This register is used to count the total number of instruction cycles beyond the first cycle.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpicnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpicnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpicnt`]
module"]
#[doc(alias = "CPICNT")]
pub type Cpicnt = crate::Reg<cpicnt::CpicntSpec>;
#[doc = "CPI Count This register is used to count the total number of instruction cycles beyond the first cycle."]
pub mod cpicnt;
#[doc = "EXCCNT (rw) register accessor: Exception Overhead Count This register is used to count the total cycles spent in interrupt processing.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exccnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exccnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exccnt`]
module"]
#[doc(alias = "EXCCNT")]
pub type Exccnt = crate::Reg<exccnt::ExccntSpec>;
#[doc = "Exception Overhead Count This register is used to count the total cycles spent in interrupt processing."]
pub mod exccnt;
#[doc = "SLEEPCNT (rw) register accessor: Sleep Count This register is used to count the total number of cycles during which the processor is sleeping.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sleepcnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sleepcnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sleepcnt`]
module"]
#[doc(alias = "SLEEPCNT")]
pub type Sleepcnt = crate::Reg<sleepcnt::SleepcntSpec>;
#[doc = "Sleep Count This register is used to count the total number of cycles during which the processor is sleeping."]
pub mod sleepcnt;
#[doc = "LSUCNT (rw) register accessor: LSU Count This register is used to count the total number of cycles during which the processor is processing an LSU operation beyond the first cycle.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lsucnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lsucnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lsucnt`]
module"]
#[doc(alias = "LSUCNT")]
pub type Lsucnt = crate::Reg<lsucnt::LsucntSpec>;
#[doc = "LSU Count This register is used to count the total number of cycles during which the processor is processing an LSU operation beyond the first cycle."]
pub mod lsucnt;
#[doc = "FOLDCNT (rw) register accessor: Fold Count This register is used to count the total number of folded instructions. The counter increments on each instruction which takes 0 cycles.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`foldcnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`foldcnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@foldcnt`]
module"]
#[doc(alias = "FOLDCNT")]
pub type Foldcnt = crate::Reg<foldcnt::FoldcntSpec>;
#[doc = "Fold Count This register is used to count the total number of folded instructions. The counter increments on each instruction which takes 0 cycles."]
pub mod foldcnt;
#[doc = "PCSR (rw) register accessor: Program Counter Sample This register is used to enable coarse-grained software profiling using a debug agent, without changing the currently executing code. If the core is not in debug state, the value returned is the instruction address of a recently executed instruction. If the core is in debug state, the value returned is 0xFFFFFFFF.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcsr`]
module"]
#[doc(alias = "PCSR")]
pub type Pcsr = crate::Reg<pcsr::PcsrSpec>;
#[doc = "Program Counter Sample This register is used to enable coarse-grained software profiling using a debug agent, without changing the currently executing code. If the core is not in debug state, the value returned is the instruction address of a recently executed instruction. If the core is in debug state, the value returned is 0xFFFFFFFF."]
pub mod pcsr;
#[doc = "COMP0 (rw) register accessor: Comparator 0 This register is used to write the reference value for comparator 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comp0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comp0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp0`]
module"]
#[doc(alias = "COMP0")]
pub type Comp0 = crate::Reg<comp0::Comp0Spec>;
#[doc = "Comparator 0 This register is used to write the reference value for comparator 0."]
pub mod comp0;
#[doc = "MASK0 (rw) register accessor: Mask 0 Use the DWT Mask Registers 0 to apply a mask to data addresses when matching against COMP0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mask0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mask0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mask0`]
module"]
#[doc(alias = "MASK0")]
pub type Mask0 = crate::Reg<mask0::Mask0Spec>;
#[doc = "Mask 0 Use the DWT Mask Registers 0 to apply a mask to data addresses when matching against COMP0."]
pub mod mask0;
#[doc = "FUNCTION0 (rw) register accessor: Function 0 Use the DWT Function Registers 0 to control the operation of the comparator 0. This comparator can: 1. Match against either the PC or the data address. This is controlled by CYCMATCH. This function is only available for comparator 0 (COMP0). 2. Emit data or PC couples, trigger the ETM, or generate a watchpoint depending on the operation defined by FUNCTION.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`function0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`function0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@function0`]
module"]
#[doc(alias = "FUNCTION0")]
pub type Function0 = crate::Reg<function0::Function0Spec>;
#[doc = "Function 0 Use the DWT Function Registers 0 to control the operation of the comparator 0. This comparator can: 1. Match against either the PC or the data address. This is controlled by CYCMATCH. This function is only available for comparator 0 (COMP0). 2. Emit data or PC couples, trigger the ETM, or generate a watchpoint depending on the operation defined by FUNCTION."]
pub mod function0;
#[doc = "COMP1 (rw) register accessor: Comparator 1 This register is used to write the reference value for comparator 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comp1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comp1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp1`]
module"]
#[doc(alias = "COMP1")]
pub type Comp1 = crate::Reg<comp1::Comp1Spec>;
#[doc = "Comparator 1 This register is used to write the reference value for comparator 1."]
pub mod comp1;
#[doc = "MASK1 (rw) register accessor: Mask 1 Use the DWT Mask Registers 1 to apply a mask to data addresses when matching against COMP1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mask1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mask1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mask1`]
module"]
#[doc(alias = "MASK1")]
pub type Mask1 = crate::Reg<mask1::Mask1Spec>;
#[doc = "Mask 1 Use the DWT Mask Registers 1 to apply a mask to data addresses when matching against COMP1."]
pub mod mask1;
#[doc = "FUNCTION1 (rw) register accessor: Function 1 Use the DWT Function Registers 1 to control the operation of the comparator 1. This comparator can: 1. Perform data value comparisons if associated address comparators have performed an address match. This function is only available for comparator 1 (COMP1). 2. Emit data or PC couples, trigger the ETM, or generate a watchpoint depending on the operation defined by FUNCTION.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`function1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`function1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@function1`]
module"]
#[doc(alias = "FUNCTION1")]
pub type Function1 = crate::Reg<function1::Function1Spec>;
#[doc = "Function 1 Use the DWT Function Registers 1 to control the operation of the comparator 1. This comparator can: 1. Perform data value comparisons if associated address comparators have performed an address match. This function is only available for comparator 1 (COMP1). 2. Emit data or PC couples, trigger the ETM, or generate a watchpoint depending on the operation defined by FUNCTION."]
pub mod function1;
#[doc = "COMP2 (rw) register accessor: Comparator 2 This register is used to write the reference value for comparator 2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comp2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comp2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp2`]
module"]
#[doc(alias = "COMP2")]
pub type Comp2 = crate::Reg<comp2::Comp2Spec>;
#[doc = "Comparator 2 This register is used to write the reference value for comparator 2."]
pub mod comp2;
#[doc = "MASK2 (rw) register accessor: Mask 2 Use the DWT Mask Registers 2 to apply a mask to data addresses when matching against COMP2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mask2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mask2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mask2`]
module"]
#[doc(alias = "MASK2")]
pub type Mask2 = crate::Reg<mask2::Mask2Spec>;
#[doc = "Mask 2 Use the DWT Mask Registers 2 to apply a mask to data addresses when matching against COMP2."]
pub mod mask2;
#[doc = "FUNCTION2 (rw) register accessor: Function 2 Use the DWT Function Registers 2 to control the operation of the comparator 2. This comparator can emit data or PC couples, trigger the ETM, or generate a watchpoint depending on the operation defined by FUNCTION.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`function2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`function2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@function2`]
module"]
#[doc(alias = "FUNCTION2")]
pub type Function2 = crate::Reg<function2::Function2Spec>;
#[doc = "Function 2 Use the DWT Function Registers 2 to control the operation of the comparator 2. This comparator can emit data or PC couples, trigger the ETM, or generate a watchpoint depending on the operation defined by FUNCTION."]
pub mod function2;
#[doc = "COMP3 (rw) register accessor: Comparator 3 This register is used to write the reference value for comparator 3.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comp3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comp3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp3`]
module"]
#[doc(alias = "COMP3")]
pub type Comp3 = crate::Reg<comp3::Comp3Spec>;
#[doc = "Comparator 3 This register is used to write the reference value for comparator 3."]
pub mod comp3;
#[doc = "MASK3 (rw) register accessor: Mask 3 Use the DWT Mask Registers 3 to apply a mask to data addresses when matching against COMP3.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mask3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mask3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mask3`]
module"]
#[doc(alias = "MASK3")]
pub type Mask3 = crate::Reg<mask3::Mask3Spec>;
#[doc = "Mask 3 Use the DWT Mask Registers 3 to apply a mask to data addresses when matching against COMP3."]
pub mod mask3;
#[doc = "FUNCTION3 (rw) register accessor: Function 3 Use the DWT Function Registers 3 to control the operation of the comparator 3. This comparator can emit data or PC couples, trigger the ETM, or generate a watchpoint depending on the operation defined by FUNCTION.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`function3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`function3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@function3`]
module"]
#[doc(alias = "FUNCTION3")]
pub type Function3 = crate::Reg<function3::Function3Spec>;
#[doc = "Function 3 Use the DWT Function Registers 3 to control the operation of the comparator 3. This comparator can emit data or PC couples, trigger the ETM, or generate a watchpoint depending on the operation defined by FUNCTION."]
pub mod function3;
