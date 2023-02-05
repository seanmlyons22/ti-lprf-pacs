#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CPUID Base This register determines the ID number of the processor core, the version number of the processor core and the implementation details of the processor core."]
    pub cpuid: CPUID,
    #[doc = "0x04 - Interrupt Control State This register is used to set a pending Non-Maskable Interrupt (NMI), set or clear a pending SVC, set or clear a pending SysTick, check for pending exceptions, check the vector number of the highest priority pended exception, and check the vector number of the active exception."]
    pub icsr: ICSR,
    #[doc = "0x08 - Vector Table Offset This register is used to relocated the vector table base address. The vector table base offset determines the offset from the bottom of the memory map. The two most significant bits and the seven least significant bits of the vector table base offset must be 0. The portion of vector table base offset that is allowed to change is TBLOFF."]
    pub vtor: VTOR,
    #[doc = "0x0c - Application Interrupt/Reset Control This register is used to determine data endianness, clear all active state information for debug or to recover from a hard failure, execute a system reset, alter the priority grouping position (binary point)."]
    pub aircr: AIRCR,
    #[doc = "0x10 - System Control This register is used for power-management functions, i.e., signaling to the system when the processor can enter a low power state, controlling how the processor enters and exits low power states."]
    pub scr: SCR,
    #[doc = "0x14 - Configuration Control This register is used to enable NMI, HardFault and FAULTMASK to ignore bus fault, trap divide by zero and unaligned accesses, enable user access to the Software Trigger Interrupt Register (STIR), control entry to Thread Mode."]
    pub ccr: CCR,
    #[doc = "0x18 - System Handlers 4-7 Priority This register is used to prioritize the following system handlers: Memory manage, Bus fault, and Usage fault. System Handlers are a special class of exception handler that can have their priority set to any of the priority levels. Most can be masked on (enabled) or off (disabled). When disabled, the fault is always treated as a Hard Fault."]
    pub shpr1: SHPR1,
    #[doc = "0x1c - System Handlers 8-11 Priority This register is used to prioritize the SVC handler. System Handlers are a special class of exception handler that can have their priority set to any of the priority levels. Most can be masked on (enabled) or off (disabled). When disabled, the fault is always treated as a Hard Fault."]
    pub shpr2: SHPR2,
    #[doc = "0x20 - System Handlers 12-15 Priority This register is used to prioritize the following system handlers: SysTick, PendSV and Debug Monitor. System Handlers are a special class of exception handler that can have their priority set to any of the priority levels. Most can be masked on (enabled) or off (disabled). When disabled, the fault is always treated as a Hard Fault."]
    pub shpr3: SHPR3,
    #[doc = "0x24 - System Handler Control and State This register is used to enable or disable the system handlers, determine the pending status of bus fault, mem manage fault, and SVC, determine the active status of the system handlers. If a fault condition occurs while its fault handler is disabled, the fault escalates to a Hard Fault."]
    pub shcsr: SHCSR,
    #[doc = "0x28 - Configurable Fault Status This register is used to obtain information about local faults. These registers include three subsections: The first byte is Memory Manage Fault Status Register (MMFSR). The second byte is Bus Fault Status Register (BFSR). The higher half-word is Usage Fault Status Register (UFSR). The flags in these registers indicate the causes of local faults. Multiple flags can be set if more than one fault occurs. These register are read/write-clear. This means that they can be read normally, but writing a 1 to any bit clears that bit. The CFSR is byte accessible. CFSR or its subregisters can be accessed as follows: The following accesses are possible to the CFSR register: - access the complete register with a word access to 0xE000ED28. - access the MMFSR with a byte access to 0xE000ED28 - access the MMFSR and BFSR with a halfword access to 0xE000ED28 - access the BFSR with a byte access to 0xE000ED29 - access the UFSR with a halfword access to 0xE000ED2A."]
    pub cfsr: CFSR,
    #[doc = "0x2c - Hard Fault Status This register is used to obtain information about events that activate the Hard Fault handler. This register is a write-clear register. This means that writing a 1 to a bit clears that bit."]
    pub hfsr: HFSR,
    #[doc = "0x30 - Debug Fault Status This register is used to monitor external debug requests, vector catches, data watchpoint match, BKPT instruction execution, halt requests. Multiple flags in the Debug Fault Status Register can be set when multiple fault conditions occur. The register is read/write clear. This means that it can be read normally. Writing a 1 to a bit clears that bit. Note that these bits are not set unless the event is caught. This means that it causes a stop of some sort. If halting debug is enabled, these events stop the processor into debug. If debug is disabled and the debug monitor is enabled, then this becomes a debug monitor handler call, if priority permits. If debug and the monitor are both disabled, some of these events are Hard Faults, and some are ignored."]
    pub dfsr: DFSR,
    #[doc = "0x34 - Mem Manage Fault Address This register is used to read the address of the location that caused a Memory Manage Fault."]
    pub mmfar: MMFAR,
    #[doc = "0x38 - Bus Fault Address This register is used to read the address of the location that generated a Bus Fault."]
    pub bfar: BFAR,
    #[doc = "0x3c - Auxiliary Fault Status This register is used to determine additional system fault information to software. Single-cycle high level on an auxiliary faults is latched as one. The bit can only be cleared by writing a one to the corresponding bit. Auxiliary fault inputs to the CPU are tied to 0."]
    pub afsr: AFSR,
    #[doc = "0x40 - Processor Feature 0"]
    pub id_pfr0: ID_PFR0,
    #[doc = "0x44 - Processor Feature 1"]
    pub id_pfr1: ID_PFR1,
    #[doc = "0x48 - Debug Feature 0 This register provides a high level view of the debug system. Further details are provided in the debug infrastructure itself."]
    pub id_dfr0: ID_DFR0,
    #[doc = "0x4c - Auxiliary Feature 0 This register provides some freedom for implementation defined features to be registered. Not used in Cortex-M."]
    pub id_afr0: ID_AFR0,
    #[doc = "0x50 - Memory Model Feature 0 General information on the memory model and memory management support."]
    pub id_mmfr0: ID_MMFR0,
    #[doc = "0x54 - Memory Model Feature 1 General information on the memory model and memory management support."]
    pub id_mmfr1: ID_MMFR1,
    #[doc = "0x58 - Memory Model Feature 2 General information on the memory model and memory management support."]
    pub id_mmfr2: ID_MMFR2,
    #[doc = "0x5c - Memory Model Feature 3 General information on the memory model and memory management support."]
    pub id_mmfr3: ID_MMFR3,
    #[doc = "0x60 - ISA Feature 0 Information on the instruction set attributes register"]
    pub id_isar0: ID_ISAR0,
    #[doc = "0x64 - ISA Feature 1 Information on the instruction set attributes register"]
    pub id_isar1: ID_ISAR1,
    #[doc = "0x68 - ISA Feature 2 Information on the instruction set attributes register"]
    pub id_isar2: ID_ISAR2,
    #[doc = "0x6c - ISA Feature 3 Information on the instruction set attributes register"]
    pub id_isar3: ID_ISAR3,
    #[doc = "0x70 - ISA Feature 4 Information on the instruction set attributes register"]
    pub id_isar4: ID_ISAR4,
}
#[doc = "CPUID (rw) register accessor: an alias for `Reg<CPUID_SPEC>`"]
pub type CPUID = crate::Reg<cpuid::CPUID_SPEC>;
#[doc = "CPUID Base This register determines the ID number of the processor core, the version number of the processor core and the implementation details of the processor core."]
pub mod cpuid;
#[doc = "ICSR (rw) register accessor: an alias for `Reg<ICSR_SPEC>`"]
pub type ICSR = crate::Reg<icsr::ICSR_SPEC>;
#[doc = "Interrupt Control State This register is used to set a pending Non-Maskable Interrupt (NMI), set or clear a pending SVC, set or clear a pending SysTick, check for pending exceptions, check the vector number of the highest priority pended exception, and check the vector number of the active exception."]
pub mod icsr;
#[doc = "VTOR (rw) register accessor: an alias for `Reg<VTOR_SPEC>`"]
pub type VTOR = crate::Reg<vtor::VTOR_SPEC>;
#[doc = "Vector Table Offset This register is used to relocated the vector table base address. The vector table base offset determines the offset from the bottom of the memory map. The two most significant bits and the seven least significant bits of the vector table base offset must be 0. The portion of vector table base offset that is allowed to change is TBLOFF."]
pub mod vtor;
#[doc = "AIRCR (rw) register accessor: an alias for `Reg<AIRCR_SPEC>`"]
pub type AIRCR = crate::Reg<aircr::AIRCR_SPEC>;
#[doc = "Application Interrupt/Reset Control This register is used to determine data endianness, clear all active state information for debug or to recover from a hard failure, execute a system reset, alter the priority grouping position (binary point)."]
pub mod aircr;
#[doc = "SCR (rw) register accessor: an alias for `Reg<SCR_SPEC>`"]
pub type SCR = crate::Reg<scr::SCR_SPEC>;
#[doc = "System Control This register is used for power-management functions, i.e., signaling to the system when the processor can enter a low power state, controlling how the processor enters and exits low power states."]
pub mod scr;
#[doc = "CCR (rw) register accessor: an alias for `Reg<CCR_SPEC>`"]
pub type CCR = crate::Reg<ccr::CCR_SPEC>;
#[doc = "Configuration Control This register is used to enable NMI, HardFault and FAULTMASK to ignore bus fault, trap divide by zero and unaligned accesses, enable user access to the Software Trigger Interrupt Register (STIR), control entry to Thread Mode."]
pub mod ccr;
#[doc = "SHPR1 (rw) register accessor: an alias for `Reg<SHPR1_SPEC>`"]
pub type SHPR1 = crate::Reg<shpr1::SHPR1_SPEC>;
#[doc = "System Handlers 4-7 Priority This register is used to prioritize the following system handlers: Memory manage, Bus fault, and Usage fault. System Handlers are a special class of exception handler that can have their priority set to any of the priority levels. Most can be masked on (enabled) or off (disabled). When disabled, the fault is always treated as a Hard Fault."]
pub mod shpr1;
#[doc = "SHPR2 (rw) register accessor: an alias for `Reg<SHPR2_SPEC>`"]
pub type SHPR2 = crate::Reg<shpr2::SHPR2_SPEC>;
#[doc = "System Handlers 8-11 Priority This register is used to prioritize the SVC handler. System Handlers are a special class of exception handler that can have their priority set to any of the priority levels. Most can be masked on (enabled) or off (disabled). When disabled, the fault is always treated as a Hard Fault."]
pub mod shpr2;
#[doc = "SHPR3 (rw) register accessor: an alias for `Reg<SHPR3_SPEC>`"]
pub type SHPR3 = crate::Reg<shpr3::SHPR3_SPEC>;
#[doc = "System Handlers 12-15 Priority This register is used to prioritize the following system handlers: SysTick, PendSV and Debug Monitor. System Handlers are a special class of exception handler that can have their priority set to any of the priority levels. Most can be masked on (enabled) or off (disabled). When disabled, the fault is always treated as a Hard Fault."]
pub mod shpr3;
#[doc = "SHCSR (rw) register accessor: an alias for `Reg<SHCSR_SPEC>`"]
pub type SHCSR = crate::Reg<shcsr::SHCSR_SPEC>;
#[doc = "System Handler Control and State This register is used to enable or disable the system handlers, determine the pending status of bus fault, mem manage fault, and SVC, determine the active status of the system handlers. If a fault condition occurs while its fault handler is disabled, the fault escalates to a Hard Fault."]
pub mod shcsr;
#[doc = "CFSR (rw) register accessor: an alias for `Reg<CFSR_SPEC>`"]
pub type CFSR = crate::Reg<cfsr::CFSR_SPEC>;
#[doc = "Configurable Fault Status This register is used to obtain information about local faults. These registers include three subsections: The first byte is Memory Manage Fault Status Register (MMFSR). The second byte is Bus Fault Status Register (BFSR). The higher half-word is Usage Fault Status Register (UFSR). The flags in these registers indicate the causes of local faults. Multiple flags can be set if more than one fault occurs. These register are read/write-clear. This means that they can be read normally, but writing a 1 to any bit clears that bit. The CFSR is byte accessible. CFSR or its subregisters can be accessed as follows: The following accesses are possible to the CFSR register: - access the complete register with a word access to 0xE000ED28. - access the MMFSR with a byte access to 0xE000ED28 - access the MMFSR and BFSR with a halfword access to 0xE000ED28 - access the BFSR with a byte access to 0xE000ED29 - access the UFSR with a halfword access to 0xE000ED2A."]
pub mod cfsr;
#[doc = "HFSR (rw) register accessor: an alias for `Reg<HFSR_SPEC>`"]
pub type HFSR = crate::Reg<hfsr::HFSR_SPEC>;
#[doc = "Hard Fault Status This register is used to obtain information about events that activate the Hard Fault handler. This register is a write-clear register. This means that writing a 1 to a bit clears that bit."]
pub mod hfsr;
#[doc = "DFSR (rw) register accessor: an alias for `Reg<DFSR_SPEC>`"]
pub type DFSR = crate::Reg<dfsr::DFSR_SPEC>;
#[doc = "Debug Fault Status This register is used to monitor external debug requests, vector catches, data watchpoint match, BKPT instruction execution, halt requests. Multiple flags in the Debug Fault Status Register can be set when multiple fault conditions occur. The register is read/write clear. This means that it can be read normally. Writing a 1 to a bit clears that bit. Note that these bits are not set unless the event is caught. This means that it causes a stop of some sort. If halting debug is enabled, these events stop the processor into debug. If debug is disabled and the debug monitor is enabled, then this becomes a debug monitor handler call, if priority permits. If debug and the monitor are both disabled, some of these events are Hard Faults, and some are ignored."]
pub mod dfsr;
#[doc = "MMFAR (rw) register accessor: an alias for `Reg<MMFAR_SPEC>`"]
pub type MMFAR = crate::Reg<mmfar::MMFAR_SPEC>;
#[doc = "Mem Manage Fault Address This register is used to read the address of the location that caused a Memory Manage Fault."]
pub mod mmfar;
#[doc = "BFAR (rw) register accessor: an alias for `Reg<BFAR_SPEC>`"]
pub type BFAR = crate::Reg<bfar::BFAR_SPEC>;
#[doc = "Bus Fault Address This register is used to read the address of the location that generated a Bus Fault."]
pub mod bfar;
#[doc = "AFSR (rw) register accessor: an alias for `Reg<AFSR_SPEC>`"]
pub type AFSR = crate::Reg<afsr::AFSR_SPEC>;
#[doc = "Auxiliary Fault Status This register is used to determine additional system fault information to software. Single-cycle high level on an auxiliary faults is latched as one. The bit can only be cleared by writing a one to the corresponding bit. Auxiliary fault inputs to the CPU are tied to 0."]
pub mod afsr;
#[doc = "ID_PFR0 (rw) register accessor: an alias for `Reg<ID_PFR0_SPEC>`"]
pub type ID_PFR0 = crate::Reg<id_pfr0::ID_PFR0_SPEC>;
#[doc = "Processor Feature 0"]
pub mod id_pfr0;
#[doc = "ID_PFR1 (rw) register accessor: an alias for `Reg<ID_PFR1_SPEC>`"]
pub type ID_PFR1 = crate::Reg<id_pfr1::ID_PFR1_SPEC>;
#[doc = "Processor Feature 1"]
pub mod id_pfr1;
#[doc = "ID_DFR0 (rw) register accessor: an alias for `Reg<ID_DFR0_SPEC>`"]
pub type ID_DFR0 = crate::Reg<id_dfr0::ID_DFR0_SPEC>;
#[doc = "Debug Feature 0 This register provides a high level view of the debug system. Further details are provided in the debug infrastructure itself."]
pub mod id_dfr0;
#[doc = "ID_AFR0 (rw) register accessor: an alias for `Reg<ID_AFR0_SPEC>`"]
pub type ID_AFR0 = crate::Reg<id_afr0::ID_AFR0_SPEC>;
#[doc = "Auxiliary Feature 0 This register provides some freedom for implementation defined features to be registered. Not used in Cortex-M."]
pub mod id_afr0;
#[doc = "ID_MMFR0 (rw) register accessor: an alias for `Reg<ID_MMFR0_SPEC>`"]
pub type ID_MMFR0 = crate::Reg<id_mmfr0::ID_MMFR0_SPEC>;
#[doc = "Memory Model Feature 0 General information on the memory model and memory management support."]
pub mod id_mmfr0;
#[doc = "ID_MMFR1 (rw) register accessor: an alias for `Reg<ID_MMFR1_SPEC>`"]
pub type ID_MMFR1 = crate::Reg<id_mmfr1::ID_MMFR1_SPEC>;
#[doc = "Memory Model Feature 1 General information on the memory model and memory management support."]
pub mod id_mmfr1;
#[doc = "ID_MMFR2 (rw) register accessor: an alias for `Reg<ID_MMFR2_SPEC>`"]
pub type ID_MMFR2 = crate::Reg<id_mmfr2::ID_MMFR2_SPEC>;
#[doc = "Memory Model Feature 2 General information on the memory model and memory management support."]
pub mod id_mmfr2;
#[doc = "ID_MMFR3 (rw) register accessor: an alias for `Reg<ID_MMFR3_SPEC>`"]
pub type ID_MMFR3 = crate::Reg<id_mmfr3::ID_MMFR3_SPEC>;
#[doc = "Memory Model Feature 3 General information on the memory model and memory management support."]
pub mod id_mmfr3;
#[doc = "ID_ISAR0 (rw) register accessor: an alias for `Reg<ID_ISAR0_SPEC>`"]
pub type ID_ISAR0 = crate::Reg<id_isar0::ID_ISAR0_SPEC>;
#[doc = "ISA Feature 0 Information on the instruction set attributes register"]
pub mod id_isar0;
#[doc = "ID_ISAR1 (rw) register accessor: an alias for `Reg<ID_ISAR1_SPEC>`"]
pub type ID_ISAR1 = crate::Reg<id_isar1::ID_ISAR1_SPEC>;
#[doc = "ISA Feature 1 Information on the instruction set attributes register"]
pub mod id_isar1;
#[doc = "ID_ISAR2 (rw) register accessor: an alias for `Reg<ID_ISAR2_SPEC>`"]
pub type ID_ISAR2 = crate::Reg<id_isar2::ID_ISAR2_SPEC>;
#[doc = "ISA Feature 2 Information on the instruction set attributes register"]
pub mod id_isar2;
#[doc = "ID_ISAR3 (rw) register accessor: an alias for `Reg<ID_ISAR3_SPEC>`"]
pub type ID_ISAR3 = crate::Reg<id_isar3::ID_ISAR3_SPEC>;
#[doc = "ISA Feature 3 Information on the instruction set attributes register"]
pub mod id_isar3;
#[doc = "ID_ISAR4 (rw) register accessor: an alias for `Reg<ID_ISAR4_SPEC>`"]
pub type ID_ISAR4 = crate::Reg<id_isar4::ID_ISAR4_SPEC>;
#[doc = "ISA Feature 4 Information on the instruction set attributes register"]
pub mod id_isar4;
