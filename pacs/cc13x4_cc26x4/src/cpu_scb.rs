#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cpuid: Cpuid,
    icsr: Icsr,
    vtor: Vtor,
    aircr: Aircr,
    scr: Scr,
    ccr: Ccr,
    shpr1: Shpr1,
    shpr2: Shpr2,
    shpr3: Shpr3,
    shcsr: Shcsr,
    cfsr: Cfsr,
    hfsr: Hfsr,
    dfsr: Dfsr,
    mmfar: Mmfar,
    bfar: Bfar,
    afsr: Afsr,
    id_pfr0: IdPfr0,
    id_pfr1: IdPfr1,
    id_dfr0: IdDfr0,
    id_afr0: IdAfr0,
    id_mmfr0: IdMmfr0,
    id_mmfr1: IdMmfr1,
    id_mmfr2: IdMmfr2,
    id_mmfr3: IdMmfr3,
    id_isar0: IdIsar0,
    id_isar1: IdIsar1,
    id_isar2: IdIsar2,
    id_isar3: IdIsar3,
    id_isar4: IdIsar4,
}
impl RegisterBlock {
    #[doc = "0x00 - CPUID Base This register determines the ID number of the processor core, the version number of the processor core and the implementation details of the processor core."]
    #[inline(always)]
    pub const fn cpuid(&self) -> &Cpuid {
        &self.cpuid
    }
    #[doc = "0x04 - Interrupt Control State This register is used to set a pending Non-Maskable Interrupt (NMI), set or clear a pending SVC, set or clear a pending SysTick, check for pending exceptions, check the vector number of the highest priority pended exception, and check the vector number of the active exception."]
    #[inline(always)]
    pub const fn icsr(&self) -> &Icsr {
        &self.icsr
    }
    #[doc = "0x08 - Vector Table Offset This register is used to relocated the vector table base address. The vector table base offset determines the offset from the bottom of the memory map. The two most significant bits and the seven least significant bits of the vector table base offset must be 0. The portion of vector table base offset that is allowed to change is TBLOFF."]
    #[inline(always)]
    pub const fn vtor(&self) -> &Vtor {
        &self.vtor
    }
    #[doc = "0x0c - Application Interrupt/Reset Control This register is used to determine data endianness, clear all active state information for debug or to recover from a hard failure, execute a system reset, alter the priority grouping position (binary point)."]
    #[inline(always)]
    pub const fn aircr(&self) -> &Aircr {
        &self.aircr
    }
    #[doc = "0x10 - System Control This register is used for power-management functions, i.e., signaling to the system when the processor can enter a low power state, controlling how the processor enters and exits low power states."]
    #[inline(always)]
    pub const fn scr(&self) -> &Scr {
        &self.scr
    }
    #[doc = "0x14 - Configuration Control This register is used to enable NMI, HardFault and FAULTMASK to ignore bus fault, trap divide by zero and unaligned accesses, enable user access to the Software Trigger Interrupt Register (STIR), control entry to Thread Mode."]
    #[inline(always)]
    pub const fn ccr(&self) -> &Ccr {
        &self.ccr
    }
    #[doc = "0x18 - System Handlers 4-7 Priority This register is used to prioritize the following system handlers: Memory manage, Bus fault, and Usage fault. System Handlers are a special class of exception handler that can have their priority set to any of the priority levels. Most can be masked on (enabled) or off (disabled). When disabled, the fault is always treated as a Hard Fault."]
    #[inline(always)]
    pub const fn shpr1(&self) -> &Shpr1 {
        &self.shpr1
    }
    #[doc = "0x1c - System Handlers 8-11 Priority This register is used to prioritize the SVC handler. System Handlers are a special class of exception handler that can have their priority set to any of the priority levels. Most can be masked on (enabled) or off (disabled). When disabled, the fault is always treated as a Hard Fault."]
    #[inline(always)]
    pub const fn shpr2(&self) -> &Shpr2 {
        &self.shpr2
    }
    #[doc = "0x20 - System Handlers 12-15 Priority This register is used to prioritize the following system handlers: SysTick, PendSV and Debug Monitor. System Handlers are a special class of exception handler that can have their priority set to any of the priority levels. Most can be masked on (enabled) or off (disabled). When disabled, the fault is always treated as a Hard Fault."]
    #[inline(always)]
    pub const fn shpr3(&self) -> &Shpr3 {
        &self.shpr3
    }
    #[doc = "0x24 - System Handler Control and State This register is used to enable or disable the system handlers, determine the pending status of bus fault, mem manage fault, and SVC, determine the active status of the system handlers. If a fault condition occurs while its fault handler is disabled, the fault escalates to a Hard Fault."]
    #[inline(always)]
    pub const fn shcsr(&self) -> &Shcsr {
        &self.shcsr
    }
    #[doc = "0x28 - Configurable Fault Status This register is used to obtain information about local faults. These registers include three subsections: The first byte is Memory Manage Fault Status Register (MMFSR). The second byte is Bus Fault Status Register (BFSR). The higher half-word is Usage Fault Status Register (UFSR). The flags in these registers indicate the causes of local faults. Multiple flags can be set if more than one fault occurs. These register are read/write-clear. This means that they can be read normally, but writing a 1 to any bit clears that bit. The CFSR is byte accessible. CFSR or its subregisters can be accessed as follows: The following accesses are possible to the CFSR register: - access the complete register with a word access to 0xE000ED28. - access the MMFSR with a byte access to 0xE000ED28 - access the MMFSR and BFSR with a halfword access to 0xE000ED28 - access the BFSR with a byte access to 0xE000ED29 - access the UFSR with a halfword access to 0xE000ED2A."]
    #[inline(always)]
    pub const fn cfsr(&self) -> &Cfsr {
        &self.cfsr
    }
    #[doc = "0x2c - Hard Fault Status This register is used to obtain information about events that activate the Hard Fault handler. This register is a write-clear register. This means that writing a 1 to a bit clears that bit."]
    #[inline(always)]
    pub const fn hfsr(&self) -> &Hfsr {
        &self.hfsr
    }
    #[doc = "0x30 - Debug Fault Status This register is used to monitor external debug requests, vector catches, data watchpoint match, BKPT instruction execution, halt requests. Multiple flags in the Debug Fault Status Register can be set when multiple fault conditions occur. The register is read/write clear. This means that it can be read normally. Writing a 1 to a bit clears that bit. Note that these bits are not set unless the event is caught. This means that it causes a stop of some sort. If halting debug is enabled, these events stop the processor into debug. If debug is disabled and the debug monitor is enabled, then this becomes a debug monitor handler call, if priority permits. If debug and the monitor are both disabled, some of these events are Hard Faults, and some are ignored."]
    #[inline(always)]
    pub const fn dfsr(&self) -> &Dfsr {
        &self.dfsr
    }
    #[doc = "0x34 - Mem Manage Fault Address This register is used to read the address of the location that caused a Memory Manage Fault."]
    #[inline(always)]
    pub const fn mmfar(&self) -> &Mmfar {
        &self.mmfar
    }
    #[doc = "0x38 - Bus Fault Address This register is used to read the address of the location that generated a Bus Fault."]
    #[inline(always)]
    pub const fn bfar(&self) -> &Bfar {
        &self.bfar
    }
    #[doc = "0x3c - Auxiliary Fault Status This register is used to determine additional system fault information to software. Single-cycle high level on an auxiliary faults is latched as one. The bit can only be cleared by writing a one to the corresponding bit. Auxiliary fault inputs to the CPU are tied to 0."]
    #[inline(always)]
    pub const fn afsr(&self) -> &Afsr {
        &self.afsr
    }
    #[doc = "0x40 - Processor Feature 0"]
    #[inline(always)]
    pub const fn id_pfr0(&self) -> &IdPfr0 {
        &self.id_pfr0
    }
    #[doc = "0x44 - Processor Feature 1"]
    #[inline(always)]
    pub const fn id_pfr1(&self) -> &IdPfr1 {
        &self.id_pfr1
    }
    #[doc = "0x48 - Debug Feature 0 This register provides a high level view of the debug system. Further details are provided in the debug infrastructure itself."]
    #[inline(always)]
    pub const fn id_dfr0(&self) -> &IdDfr0 {
        &self.id_dfr0
    }
    #[doc = "0x4c - Auxiliary Feature 0 This register provides some freedom for implementation defined features to be registered. Not used in Cortex-M."]
    #[inline(always)]
    pub const fn id_afr0(&self) -> &IdAfr0 {
        &self.id_afr0
    }
    #[doc = "0x50 - Memory Model Feature 0 General information on the memory model and memory management support."]
    #[inline(always)]
    pub const fn id_mmfr0(&self) -> &IdMmfr0 {
        &self.id_mmfr0
    }
    #[doc = "0x54 - Memory Model Feature 1 General information on the memory model and memory management support."]
    #[inline(always)]
    pub const fn id_mmfr1(&self) -> &IdMmfr1 {
        &self.id_mmfr1
    }
    #[doc = "0x58 - Memory Model Feature 2 General information on the memory model and memory management support."]
    #[inline(always)]
    pub const fn id_mmfr2(&self) -> &IdMmfr2 {
        &self.id_mmfr2
    }
    #[doc = "0x5c - Memory Model Feature 3 General information on the memory model and memory management support."]
    #[inline(always)]
    pub const fn id_mmfr3(&self) -> &IdMmfr3 {
        &self.id_mmfr3
    }
    #[doc = "0x60 - ISA Feature 0 Information on the instruction set attributes register"]
    #[inline(always)]
    pub const fn id_isar0(&self) -> &IdIsar0 {
        &self.id_isar0
    }
    #[doc = "0x64 - ISA Feature 1 Information on the instruction set attributes register"]
    #[inline(always)]
    pub const fn id_isar1(&self) -> &IdIsar1 {
        &self.id_isar1
    }
    #[doc = "0x68 - ISA Feature 2 Information on the instruction set attributes register"]
    #[inline(always)]
    pub const fn id_isar2(&self) -> &IdIsar2 {
        &self.id_isar2
    }
    #[doc = "0x6c - ISA Feature 3 Information on the instruction set attributes register"]
    #[inline(always)]
    pub const fn id_isar3(&self) -> &IdIsar3 {
        &self.id_isar3
    }
    #[doc = "0x70 - ISA Feature 4 Information on the instruction set attributes register"]
    #[inline(always)]
    pub const fn id_isar4(&self) -> &IdIsar4 {
        &self.id_isar4
    }
}
#[doc = "CPUID (rw) register accessor: CPUID Base This register determines the ID number of the processor core, the version number of the processor core and the implementation details of the processor core.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpuid::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpuid::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpuid`]
module"]
#[doc(alias = "CPUID")]
pub type Cpuid = crate::Reg<cpuid::CpuidSpec>;
#[doc = "CPUID Base This register determines the ID number of the processor core, the version number of the processor core and the implementation details of the processor core."]
pub mod cpuid;
#[doc = "ICSR (rw) register accessor: Interrupt Control State This register is used to set a pending Non-Maskable Interrupt (NMI), set or clear a pending SVC, set or clear a pending SysTick, check for pending exceptions, check the vector number of the highest priority pended exception, and check the vector number of the active exception.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icsr`]
module"]
#[doc(alias = "ICSR")]
pub type Icsr = crate::Reg<icsr::IcsrSpec>;
#[doc = "Interrupt Control State This register is used to set a pending Non-Maskable Interrupt (NMI), set or clear a pending SVC, set or clear a pending SysTick, check for pending exceptions, check the vector number of the highest priority pended exception, and check the vector number of the active exception."]
pub mod icsr;
#[doc = "VTOR (rw) register accessor: Vector Table Offset This register is used to relocated the vector table base address. The vector table base offset determines the offset from the bottom of the memory map. The two most significant bits and the seven least significant bits of the vector table base offset must be 0. The portion of vector table base offset that is allowed to change is TBLOFF.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vtor::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vtor::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vtor`]
module"]
#[doc(alias = "VTOR")]
pub type Vtor = crate::Reg<vtor::VtorSpec>;
#[doc = "Vector Table Offset This register is used to relocated the vector table base address. The vector table base offset determines the offset from the bottom of the memory map. The two most significant bits and the seven least significant bits of the vector table base offset must be 0. The portion of vector table base offset that is allowed to change is TBLOFF."]
pub mod vtor;
#[doc = "AIRCR (rw) register accessor: Application Interrupt/Reset Control This register is used to determine data endianness, clear all active state information for debug or to recover from a hard failure, execute a system reset, alter the priority grouping position (binary point).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aircr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aircr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aircr`]
module"]
#[doc(alias = "AIRCR")]
pub type Aircr = crate::Reg<aircr::AircrSpec>;
#[doc = "Application Interrupt/Reset Control This register is used to determine data endianness, clear all active state information for debug or to recover from a hard failure, execute a system reset, alter the priority grouping position (binary point)."]
pub mod aircr;
#[doc = "SCR (rw) register accessor: System Control This register is used for power-management functions, i.e., signaling to the system when the processor can enter a low power state, controlling how the processor enters and exits low power states.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scr`]
module"]
#[doc(alias = "SCR")]
pub type Scr = crate::Reg<scr::ScrSpec>;
#[doc = "System Control This register is used for power-management functions, i.e., signaling to the system when the processor can enter a low power state, controlling how the processor enters and exits low power states."]
pub mod scr;
#[doc = "CCR (rw) register accessor: Configuration Control This register is used to enable NMI, HardFault and FAULTMASK to ignore bus fault, trap divide by zero and unaligned accesses, enable user access to the Software Trigger Interrupt Register (STIR), control entry to Thread Mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr`]
module"]
#[doc(alias = "CCR")]
pub type Ccr = crate::Reg<ccr::CcrSpec>;
#[doc = "Configuration Control This register is used to enable NMI, HardFault and FAULTMASK to ignore bus fault, trap divide by zero and unaligned accesses, enable user access to the Software Trigger Interrupt Register (STIR), control entry to Thread Mode."]
pub mod ccr;
#[doc = "SHPR1 (rw) register accessor: System Handlers 4-7 Priority This register is used to prioritize the following system handlers: Memory manage, Bus fault, and Usage fault. System Handlers are a special class of exception handler that can have their priority set to any of the priority levels. Most can be masked on (enabled) or off (disabled). When disabled, the fault is always treated as a Hard Fault.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`shpr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`shpr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shpr1`]
module"]
#[doc(alias = "SHPR1")]
pub type Shpr1 = crate::Reg<shpr1::Shpr1Spec>;
#[doc = "System Handlers 4-7 Priority This register is used to prioritize the following system handlers: Memory manage, Bus fault, and Usage fault. System Handlers are a special class of exception handler that can have their priority set to any of the priority levels. Most can be masked on (enabled) or off (disabled). When disabled, the fault is always treated as a Hard Fault."]
pub mod shpr1;
#[doc = "SHPR2 (rw) register accessor: System Handlers 8-11 Priority This register is used to prioritize the SVC handler. System Handlers are a special class of exception handler that can have their priority set to any of the priority levels. Most can be masked on (enabled) or off (disabled). When disabled, the fault is always treated as a Hard Fault.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`shpr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`shpr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shpr2`]
module"]
#[doc(alias = "SHPR2")]
pub type Shpr2 = crate::Reg<shpr2::Shpr2Spec>;
#[doc = "System Handlers 8-11 Priority This register is used to prioritize the SVC handler. System Handlers are a special class of exception handler that can have their priority set to any of the priority levels. Most can be masked on (enabled) or off (disabled). When disabled, the fault is always treated as a Hard Fault."]
pub mod shpr2;
#[doc = "SHPR3 (rw) register accessor: System Handlers 12-15 Priority This register is used to prioritize the following system handlers: SysTick, PendSV and Debug Monitor. System Handlers are a special class of exception handler that can have their priority set to any of the priority levels. Most can be masked on (enabled) or off (disabled). When disabled, the fault is always treated as a Hard Fault.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`shpr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`shpr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shpr3`]
module"]
#[doc(alias = "SHPR3")]
pub type Shpr3 = crate::Reg<shpr3::Shpr3Spec>;
#[doc = "System Handlers 12-15 Priority This register is used to prioritize the following system handlers: SysTick, PendSV and Debug Monitor. System Handlers are a special class of exception handler that can have their priority set to any of the priority levels. Most can be masked on (enabled) or off (disabled). When disabled, the fault is always treated as a Hard Fault."]
pub mod shpr3;
#[doc = "SHCSR (rw) register accessor: System Handler Control and State This register is used to enable or disable the system handlers, determine the pending status of bus fault, mem manage fault, and SVC, determine the active status of the system handlers. If a fault condition occurs while its fault handler is disabled, the fault escalates to a Hard Fault.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`shcsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`shcsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shcsr`]
module"]
#[doc(alias = "SHCSR")]
pub type Shcsr = crate::Reg<shcsr::ShcsrSpec>;
#[doc = "System Handler Control and State This register is used to enable or disable the system handlers, determine the pending status of bus fault, mem manage fault, and SVC, determine the active status of the system handlers. If a fault condition occurs while its fault handler is disabled, the fault escalates to a Hard Fault."]
pub mod shcsr;
#[doc = "CFSR (rw) register accessor: Configurable Fault Status This register is used to obtain information about local faults. These registers include three subsections: The first byte is Memory Manage Fault Status Register (MMFSR). The second byte is Bus Fault Status Register (BFSR). The higher half-word is Usage Fault Status Register (UFSR). The flags in these registers indicate the causes of local faults. Multiple flags can be set if more than one fault occurs. These register are read/write-clear. This means that they can be read normally, but writing a 1 to any bit clears that bit. The CFSR is byte accessible. CFSR or its subregisters can be accessed as follows: The following accesses are possible to the CFSR register: - access the complete register with a word access to 0xE000ED28. - access the MMFSR with a byte access to 0xE000ED28 - access the MMFSR and BFSR with a halfword access to 0xE000ED28 - access the BFSR with a byte access to 0xE000ED29 - access the UFSR with a halfword access to 0xE000ED2A.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfsr`]
module"]
#[doc(alias = "CFSR")]
pub type Cfsr = crate::Reg<cfsr::CfsrSpec>;
#[doc = "Configurable Fault Status This register is used to obtain information about local faults. These registers include three subsections: The first byte is Memory Manage Fault Status Register (MMFSR). The second byte is Bus Fault Status Register (BFSR). The higher half-word is Usage Fault Status Register (UFSR). The flags in these registers indicate the causes of local faults. Multiple flags can be set if more than one fault occurs. These register are read/write-clear. This means that they can be read normally, but writing a 1 to any bit clears that bit. The CFSR is byte accessible. CFSR or its subregisters can be accessed as follows: The following accesses are possible to the CFSR register: - access the complete register with a word access to 0xE000ED28. - access the MMFSR with a byte access to 0xE000ED28 - access the MMFSR and BFSR with a halfword access to 0xE000ED28 - access the BFSR with a byte access to 0xE000ED29 - access the UFSR with a halfword access to 0xE000ED2A."]
pub mod cfsr;
#[doc = "HFSR (rw) register accessor: Hard Fault Status This register is used to obtain information about events that activate the Hard Fault handler. This register is a write-clear register. This means that writing a 1 to a bit clears that bit.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hfsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hfsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfsr`]
module"]
#[doc(alias = "HFSR")]
pub type Hfsr = crate::Reg<hfsr::HfsrSpec>;
#[doc = "Hard Fault Status This register is used to obtain information about events that activate the Hard Fault handler. This register is a write-clear register. This means that writing a 1 to a bit clears that bit."]
pub mod hfsr;
#[doc = "DFSR (rw) register accessor: Debug Fault Status This register is used to monitor external debug requests, vector catches, data watchpoint match, BKPT instruction execution, halt requests. Multiple flags in the Debug Fault Status Register can be set when multiple fault conditions occur. The register is read/write clear. This means that it can be read normally. Writing a 1 to a bit clears that bit. Note that these bits are not set unless the event is caught. This means that it causes a stop of some sort. If halting debug is enabled, these events stop the processor into debug. If debug is disabled and the debug monitor is enabled, then this becomes a debug monitor handler call, if priority permits. If debug and the monitor are both disabled, some of these events are Hard Faults, and some are ignored.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dfsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsr`]
module"]
#[doc(alias = "DFSR")]
pub type Dfsr = crate::Reg<dfsr::DfsrSpec>;
#[doc = "Debug Fault Status This register is used to monitor external debug requests, vector catches, data watchpoint match, BKPT instruction execution, halt requests. Multiple flags in the Debug Fault Status Register can be set when multiple fault conditions occur. The register is read/write clear. This means that it can be read normally. Writing a 1 to a bit clears that bit. Note that these bits are not set unless the event is caught. This means that it causes a stop of some sort. If halting debug is enabled, these events stop the processor into debug. If debug is disabled and the debug monitor is enabled, then this becomes a debug monitor handler call, if priority permits. If debug and the monitor are both disabled, some of these events are Hard Faults, and some are ignored."]
pub mod dfsr;
#[doc = "MMFAR (rw) register accessor: Mem Manage Fault Address This register is used to read the address of the location that caused a Memory Manage Fault.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmfar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmfar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmfar`]
module"]
#[doc(alias = "MMFAR")]
pub type Mmfar = crate::Reg<mmfar::MmfarSpec>;
#[doc = "Mem Manage Fault Address This register is used to read the address of the location that caused a Memory Manage Fault."]
pub mod mmfar;
#[doc = "BFAR (rw) register accessor: Bus Fault Address This register is used to read the address of the location that generated a Bus Fault.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bfar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bfar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bfar`]
module"]
#[doc(alias = "BFAR")]
pub type Bfar = crate::Reg<bfar::BfarSpec>;
#[doc = "Bus Fault Address This register is used to read the address of the location that generated a Bus Fault."]
pub mod bfar;
#[doc = "AFSR (rw) register accessor: Auxiliary Fault Status This register is used to determine additional system fault information to software. Single-cycle high level on an auxiliary faults is latched as one. The bit can only be cleared by writing a one to the corresponding bit. Auxiliary fault inputs to the CPU are tied to 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`afsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`afsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@afsr`]
module"]
#[doc(alias = "AFSR")]
pub type Afsr = crate::Reg<afsr::AfsrSpec>;
#[doc = "Auxiliary Fault Status This register is used to determine additional system fault information to software. Single-cycle high level on an auxiliary faults is latched as one. The bit can only be cleared by writing a one to the corresponding bit. Auxiliary fault inputs to the CPU are tied to 0."]
pub mod afsr;
#[doc = "ID_PFR0 (rw) register accessor: Processor Feature 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`id_pfr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`id_pfr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id_pfr0`]
module"]
#[doc(alias = "ID_PFR0")]
pub type IdPfr0 = crate::Reg<id_pfr0::IdPfr0Spec>;
#[doc = "Processor Feature 0"]
pub mod id_pfr0;
#[doc = "ID_PFR1 (rw) register accessor: Processor Feature 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`id_pfr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`id_pfr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id_pfr1`]
module"]
#[doc(alias = "ID_PFR1")]
pub type IdPfr1 = crate::Reg<id_pfr1::IdPfr1Spec>;
#[doc = "Processor Feature 1"]
pub mod id_pfr1;
#[doc = "ID_DFR0 (rw) register accessor: Debug Feature 0 This register provides a high level view of the debug system. Further details are provided in the debug infrastructure itself.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`id_dfr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`id_dfr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id_dfr0`]
module"]
#[doc(alias = "ID_DFR0")]
pub type IdDfr0 = crate::Reg<id_dfr0::IdDfr0Spec>;
#[doc = "Debug Feature 0 This register provides a high level view of the debug system. Further details are provided in the debug infrastructure itself."]
pub mod id_dfr0;
#[doc = "ID_AFR0 (rw) register accessor: Auxiliary Feature 0 This register provides some freedom for implementation defined features to be registered. Not used in Cortex-M.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`id_afr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`id_afr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id_afr0`]
module"]
#[doc(alias = "ID_AFR0")]
pub type IdAfr0 = crate::Reg<id_afr0::IdAfr0Spec>;
#[doc = "Auxiliary Feature 0 This register provides some freedom for implementation defined features to be registered. Not used in Cortex-M."]
pub mod id_afr0;
#[doc = "ID_MMFR0 (rw) register accessor: Memory Model Feature 0 General information on the memory model and memory management support.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`id_mmfr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`id_mmfr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id_mmfr0`]
module"]
#[doc(alias = "ID_MMFR0")]
pub type IdMmfr0 = crate::Reg<id_mmfr0::IdMmfr0Spec>;
#[doc = "Memory Model Feature 0 General information on the memory model and memory management support."]
pub mod id_mmfr0;
#[doc = "ID_MMFR1 (rw) register accessor: Memory Model Feature 1 General information on the memory model and memory management support.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`id_mmfr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`id_mmfr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id_mmfr1`]
module"]
#[doc(alias = "ID_MMFR1")]
pub type IdMmfr1 = crate::Reg<id_mmfr1::IdMmfr1Spec>;
#[doc = "Memory Model Feature 1 General information on the memory model and memory management support."]
pub mod id_mmfr1;
#[doc = "ID_MMFR2 (rw) register accessor: Memory Model Feature 2 General information on the memory model and memory management support.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`id_mmfr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`id_mmfr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id_mmfr2`]
module"]
#[doc(alias = "ID_MMFR2")]
pub type IdMmfr2 = crate::Reg<id_mmfr2::IdMmfr2Spec>;
#[doc = "Memory Model Feature 2 General information on the memory model and memory management support."]
pub mod id_mmfr2;
#[doc = "ID_MMFR3 (rw) register accessor: Memory Model Feature 3 General information on the memory model and memory management support.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`id_mmfr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`id_mmfr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id_mmfr3`]
module"]
#[doc(alias = "ID_MMFR3")]
pub type IdMmfr3 = crate::Reg<id_mmfr3::IdMmfr3Spec>;
#[doc = "Memory Model Feature 3 General information on the memory model and memory management support."]
pub mod id_mmfr3;
#[doc = "ID_ISAR0 (rw) register accessor: ISA Feature 0 Information on the instruction set attributes register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`id_isar0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`id_isar0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id_isar0`]
module"]
#[doc(alias = "ID_ISAR0")]
pub type IdIsar0 = crate::Reg<id_isar0::IdIsar0Spec>;
#[doc = "ISA Feature 0 Information on the instruction set attributes register"]
pub mod id_isar0;
#[doc = "ID_ISAR1 (rw) register accessor: ISA Feature 1 Information on the instruction set attributes register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`id_isar1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`id_isar1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id_isar1`]
module"]
#[doc(alias = "ID_ISAR1")]
pub type IdIsar1 = crate::Reg<id_isar1::IdIsar1Spec>;
#[doc = "ISA Feature 1 Information on the instruction set attributes register"]
pub mod id_isar1;
#[doc = "ID_ISAR2 (rw) register accessor: ISA Feature 2 Information on the instruction set attributes register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`id_isar2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`id_isar2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id_isar2`]
module"]
#[doc(alias = "ID_ISAR2")]
pub type IdIsar2 = crate::Reg<id_isar2::IdIsar2Spec>;
#[doc = "ISA Feature 2 Information on the instruction set attributes register"]
pub mod id_isar2;
#[doc = "ID_ISAR3 (rw) register accessor: ISA Feature 3 Information on the instruction set attributes register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`id_isar3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`id_isar3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id_isar3`]
module"]
#[doc(alias = "ID_ISAR3")]
pub type IdIsar3 = crate::Reg<id_isar3::IdIsar3Spec>;
#[doc = "ISA Feature 3 Information on the instruction set attributes register"]
pub mod id_isar3;
#[doc = "ID_ISAR4 (rw) register accessor: ISA Feature 4 Information on the instruction set attributes register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`id_isar4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`id_isar4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id_isar4`]
module"]
#[doc(alias = "ID_ISAR4")]
pub type IdIsar4 = crate::Reg<id_isar4::IdIsar4Spec>;
#[doc = "ISA Feature 4 Information on the instruction set attributes register"]
pub mod id_isar4;
