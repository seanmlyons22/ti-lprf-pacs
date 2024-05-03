#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    reserved000: Reserved000,
    ictr: Ictr,
    actlr: Actlr,
    _reserved3: [u8; 0x04],
    stcsr: Stcsr,
    strvr: Strvr,
    stcvr: Stcvr,
    stcr: Stcr,
    _reserved7: [u8; 0xe0],
    nvic_iser0: NvicIser0,
    nvic_iser1: NvicIser1,
    _reserved9: [u8; 0x78],
    nvic_icer0: NvicIcer0,
    nvic_icer1: NvicIcer1,
    _reserved11: [u8; 0x78],
    nvic_ispr0: NvicIspr0,
    nvic_ispr1: NvicIspr1,
    _reserved13: [u8; 0x78],
    nvic_icpr0: NvicIcpr0,
    nvic_icpr1: NvicIcpr1,
    _reserved15: [u8; 0x78],
    nvic_iabr0: NvicIabr0,
    nvic_iabr1: NvicIabr1,
    _reserved17: [u8; 0xf8],
    nvic_ipr0: NvicIpr0,
    nvic_ipr1: NvicIpr1,
    nvic_ipr2: NvicIpr2,
    nvic_ipr3: NvicIpr3,
    nvic_ipr4: NvicIpr4,
    nvic_ipr5: NvicIpr5,
    nvic_ipr6: NvicIpr6,
    nvic_ipr7: NvicIpr7,
    nvic_ipr8: NvicIpr8,
    nvic_ipr9: NvicIpr9,
    _reserved27: [u8; 0x08d8],
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
    _reserved56: [u8; 0x14],
    cpacr: Cpacr,
    _reserved57: [u8; 0x04],
    mpu_type: MpuType,
    mpu_ctrl: MpuCtrl,
    mpu_rnr: MpuRnr,
    mpu_rbar: MpuRbar,
    mpu_rasr: MpuRasr,
    mpu_rbar_a1: MpuRbarA1,
    mpu_rasr_a1: MpuRasrA1,
    mpu_rbar_a2: MpuRbarA2,
    mpu_rasr_a2: MpuRasrA2,
    mpu_rbar_a3: MpuRbarA3,
    mpu_rasr_a3: MpuRasrA3,
    _reserved68: [u8; 0x34],
    dhcsr: Dhcsr,
    dcrsr: Dcrsr,
    dcrdr: Dcrdr,
    demcr: Demcr,
    _reserved72: [u8; 0x0100],
    stir: Stir,
}
impl RegisterBlock {
    #[doc = "0x00 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn reserved000(&self) -> &Reserved000 {
        &self.reserved000
    }
    #[doc = "0x04 - Interrupt Control Type Read this register to see the number of interrupt lines that the NVIC supports."]
    #[inline(always)]
    pub const fn ictr(&self) -> &Ictr {
        &self.ictr
    }
    #[doc = "0x08 - Auxiliary Control This register is used to disable certain aspects of functionality within the processor"]
    #[inline(always)]
    pub const fn actlr(&self) -> &Actlr {
        &self.actlr
    }
    #[doc = "0x10 - SysTick Control and Status This register enables the SysTick features and returns status flags related to SysTick."]
    #[inline(always)]
    pub const fn stcsr(&self) -> &Stcsr {
        &self.stcsr
    }
    #[doc = "0x14 - SysTick Reload Value This register is used to specify the start value to load into the current value register STCVR.CURRENT when the counter reaches 0. It can be any value between 1 and 0x00FFFFFF. A start value of 0 is possible, but has no effect because the SysTick interrupt and STCSR.COUNTFLAG are activated when counting from 1 to 0."]
    #[inline(always)]
    pub const fn strvr(&self) -> &Strvr {
        &self.strvr
    }
    #[doc = "0x18 - SysTick Current Value Read from this register returns the current value of SysTick counter. Writing to this register resets the SysTick counter (as well as STCSR.COUNTFLAG)."]
    #[inline(always)]
    pub const fn stcvr(&self) -> &Stcvr {
        &self.stcvr
    }
    #[doc = "0x1c - SysTick Calibration Value Used to enable software to scale to any required speed using divide and multiply."]
    #[inline(always)]
    pub const fn stcr(&self) -> &Stcr {
        &self.stcr
    }
    #[doc = "0x100 - Irq 0 to 31 Set Enable This register is used to enable interrupts and determine which interrupts are currently enabled."]
    #[inline(always)]
    pub const fn nvic_iser0(&self) -> &NvicIser0 {
        &self.nvic_iser0
    }
    #[doc = "0x104 - Irq 32 to 63 Set Enable This register is used to enable interrupts and determine which interrupts are currently enabled."]
    #[inline(always)]
    pub const fn nvic_iser1(&self) -> &NvicIser1 {
        &self.nvic_iser1
    }
    #[doc = "0x180 - Irq 0 to 31 Clear Enable This register is used to disable interrupts and determine which interrupts are currently enabled."]
    #[inline(always)]
    pub const fn nvic_icer0(&self) -> &NvicIcer0 {
        &self.nvic_icer0
    }
    #[doc = "0x184 - Irq 32 to 63 Clear Enable This register is used to disable interrupts and determine which interrupts are currently enabled."]
    #[inline(always)]
    pub const fn nvic_icer1(&self) -> &NvicIcer1 {
        &self.nvic_icer1
    }
    #[doc = "0x200 - Irq 0 to 31 Set Pending This register is used to force interrupts into the pending state and determine which interrupts are currently pending."]
    #[inline(always)]
    pub const fn nvic_ispr0(&self) -> &NvicIspr0 {
        &self.nvic_ispr0
    }
    #[doc = "0x204 - Irq 32 to 63 Set Pending This register is used to force interrupts into the pending state and determine which interrupts are currently pending."]
    #[inline(always)]
    pub const fn nvic_ispr1(&self) -> &NvicIspr1 {
        &self.nvic_ispr1
    }
    #[doc = "0x280 - Irq 0 to 31 Clear Pending This register is used to clear pending interrupts and determine which interrupts are currently pending."]
    #[inline(always)]
    pub const fn nvic_icpr0(&self) -> &NvicIcpr0 {
        &self.nvic_icpr0
    }
    #[doc = "0x284 - Irq 32 to 63 Clear Pending This register is used to clear pending interrupts and determine which interrupts are currently pending."]
    #[inline(always)]
    pub const fn nvic_icpr1(&self) -> &NvicIcpr1 {
        &self.nvic_icpr1
    }
    #[doc = "0x300 - Irq 0 to 31 Active Bit This register is used to determine which interrupts are active. Each flag in the register corresponds to one interrupt."]
    #[inline(always)]
    pub const fn nvic_iabr0(&self) -> &NvicIabr0 {
        &self.nvic_iabr0
    }
    #[doc = "0x304 - Irq 32 to 63 Active Bit This register is used to determine which interrupts are active. Each flag in the register corresponds to one interrupt."]
    #[inline(always)]
    pub const fn nvic_iabr1(&self) -> &NvicIabr1 {
        &self.nvic_iabr1
    }
    #[doc = "0x400 - Irq 0 to 3 Priority This register is used to assign a priority from 0 to 255 to each of the available interrupts. 0 is the highest priority, and 255 is the lowest. The interpretation of the Interrupt Priority Registers changes based on the setting in AIRCR.PRIGROUP."]
    #[inline(always)]
    pub const fn nvic_ipr0(&self) -> &NvicIpr0 {
        &self.nvic_ipr0
    }
    #[doc = "0x404 - Irq 4 to 7 Priority This register is used to assign a priority from 0 to 255 to each of the available interrupts. 0 is the highest priority, and 255 is the lowest. The interpretation of the Interrupt Priority Registers changes based on the setting in AIRCR.PRIGROUP."]
    #[inline(always)]
    pub const fn nvic_ipr1(&self) -> &NvicIpr1 {
        &self.nvic_ipr1
    }
    #[doc = "0x408 - Irq 8 to 11 Priority This register is used to assign a priority from 0 to 255 to each of the available interrupts. 0 is the highest priority, and 255 is the lowest. The interpretation of the Interrupt Priority Registers changes based on the setting in AIRCR.PRIGROUP."]
    #[inline(always)]
    pub const fn nvic_ipr2(&self) -> &NvicIpr2 {
        &self.nvic_ipr2
    }
    #[doc = "0x40c - Irq 12 to 15 Priority This register is used to assign a priority from 0 to 255 to each of the available interrupts. 0 is the highest priority, and 255 is the lowest. The interpretation of the Interrupt Priority Registers changes based on the setting in AIRCR.PRIGROUP."]
    #[inline(always)]
    pub const fn nvic_ipr3(&self) -> &NvicIpr3 {
        &self.nvic_ipr3
    }
    #[doc = "0x410 - Irq 16 to 19 Priority This register is used to assign a priority from 0 to 255 to each of the available interrupts. 0 is the highest priority, and 255 is the lowest. The interpretation of the Interrupt Priority Registers changes based on the setting in AIRCR.PRIGROUP."]
    #[inline(always)]
    pub const fn nvic_ipr4(&self) -> &NvicIpr4 {
        &self.nvic_ipr4
    }
    #[doc = "0x414 - Irq 20 to 23 Priority This register is used to assign a priority from 0 to 255 to each of the available interrupts. 0 is the highest priority, and 255 is the lowest. The interpretation of the Interrupt Priority Registers changes based on the setting in AIRCR.PRIGROUP."]
    #[inline(always)]
    pub const fn nvic_ipr5(&self) -> &NvicIpr5 {
        &self.nvic_ipr5
    }
    #[doc = "0x418 - Irq 24 to 27 Priority This register is used to assign a priority from 0 to 255 to each of the available interrupts. 0 is the highest priority, and 255 is the lowest. The interpretation of the Interrupt Priority Registers changes based on the setting in AIRCR.PRIGROUP."]
    #[inline(always)]
    pub const fn nvic_ipr6(&self) -> &NvicIpr6 {
        &self.nvic_ipr6
    }
    #[doc = "0x41c - Irq 28 to 31 Priority This register is used to assign a priority from 0 to 255 to each of the available interrupts. 0 is the highest priority, and 255 is the lowest. The interpretation of the Interrupt Priority Registers changes based on the setting in AIRCR.PRIGROUP."]
    #[inline(always)]
    pub const fn nvic_ipr7(&self) -> &NvicIpr7 {
        &self.nvic_ipr7
    }
    #[doc = "0x420 - Irq 32 to 35 Priority This register is used to assign a priority from 0 to 255 to each of the available interrupts. 0 is the highest priority, and 255 is the lowest. The interpretation of the Interrupt Priority Registers changes based on the setting in AIRCR.PRIGROUP."]
    #[inline(always)]
    pub const fn nvic_ipr8(&self) -> &NvicIpr8 {
        &self.nvic_ipr8
    }
    #[doc = "0x424 - Irq 32 to 35 Priority This register is used to assign a priority from 0 to 255 to each of the available interrupts. 0 is the highest priority, and 255 is the lowest. The interpretation of the Interrupt Priority Registers changes based on the setting in AIRCR.PRIGROUP."]
    #[inline(always)]
    pub const fn nvic_ipr9(&self) -> &NvicIpr9 {
        &self.nvic_ipr9
    }
    #[doc = "0xd00 - CPUID Base This register determines the ID number of the processor core, the version number of the processor core and the implementation details of the processor core."]
    #[inline(always)]
    pub const fn cpuid(&self) -> &Cpuid {
        &self.cpuid
    }
    #[doc = "0xd04 - Interrupt Control State This register is used to set a pending Non-Maskable Interrupt (NMI), set or clear a pending SVC, set or clear a pending SysTick, check for pending exceptions, check the vector number of the highest priority pended exception, and check the vector number of the active exception."]
    #[inline(always)]
    pub const fn icsr(&self) -> &Icsr {
        &self.icsr
    }
    #[doc = "0xd08 - Vector Table Offset This register is used to relocated the vector table base address. The vector table base offset determines the offset from the bottom of the memory map. The two most significant bits and the seven least significant bits of the vector table base offset must be 0. The portion of vector table base offset that is allowed to change is TBLOFF."]
    #[inline(always)]
    pub const fn vtor(&self) -> &Vtor {
        &self.vtor
    }
    #[doc = "0xd0c - Application Interrupt/Reset Control This register is used to determine data endianness, clear all active state information for debug or to recover from a hard failure, execute a system reset, alter the priority grouping position (binary point)."]
    #[inline(always)]
    pub const fn aircr(&self) -> &Aircr {
        &self.aircr
    }
    #[doc = "0xd10 - System Control This register is used for power-management functions, i.e., signaling to the system when the processor can enter a low power state, controlling how the processor enters and exits low power states."]
    #[inline(always)]
    pub const fn scr(&self) -> &Scr {
        &self.scr
    }
    #[doc = "0xd14 - Configuration Control This register is used to enable NMI, HardFault and FAULTMASK to ignore bus fault, trap divide by zero and unaligned accesses, enable user access to the Software Trigger Interrupt Register (STIR), control entry to Thread Mode."]
    #[inline(always)]
    pub const fn ccr(&self) -> &Ccr {
        &self.ccr
    }
    #[doc = "0xd18 - System Handlers 4-7 Priority This register is used to prioritize the following system handlers: Memory manage, Bus fault, and Usage fault. System Handlers are a special class of exception handler that can have their priority set to any of the priority levels. Most can be masked on (enabled) or off (disabled). When disabled, the fault is always treated as a Hard Fault."]
    #[inline(always)]
    pub const fn shpr1(&self) -> &Shpr1 {
        &self.shpr1
    }
    #[doc = "0xd1c - System Handlers 8-11 Priority This register is used to prioritize the SVC handler. System Handlers are a special class of exception handler that can have their priority set to any of the priority levels. Most can be masked on (enabled) or off (disabled). When disabled, the fault is always treated as a Hard Fault."]
    #[inline(always)]
    pub const fn shpr2(&self) -> &Shpr2 {
        &self.shpr2
    }
    #[doc = "0xd20 - System Handlers 12-15 Priority This register is used to prioritize the following system handlers: SysTick, PendSV and Debug Monitor. System Handlers are a special class of exception handler that can have their priority set to any of the priority levels. Most can be masked on (enabled) or off (disabled). When disabled, the fault is always treated as a Hard Fault."]
    #[inline(always)]
    pub const fn shpr3(&self) -> &Shpr3 {
        &self.shpr3
    }
    #[doc = "0xd24 - System Handler Control and State This register is used to enable or disable the system handlers, determine the pending status of bus fault, mem manage fault, and SVC, determine the active status of the system handlers. If a fault condition occurs while its fault handler is disabled, the fault escalates to a Hard Fault."]
    #[inline(always)]
    pub const fn shcsr(&self) -> &Shcsr {
        &self.shcsr
    }
    #[doc = "0xd28 - Configurable Fault Status This register is used to obtain information about local faults. These registers include three subsections: The first byte is Memory Manage Fault Status Register (MMFSR). The second byte is Bus Fault Status Register (BFSR). The higher half-word is Usage Fault Status Register (UFSR). The flags in these registers indicate the causes of local faults. Multiple flags can be set if more than one fault occurs. These register are read/write-clear. This means that they can be read normally, but writing a 1 to any bit clears that bit. The CFSR is byte accessible. CFSR or its subregisters can be accessed as follows: The following accesses are possible to the CFSR register: - access the complete register with a word access to 0xE000ED28. - access the MMFSR with a byte access to 0xE000ED28 - access the MMFSR and BFSR with a halfword access to 0xE000ED28 - access the BFSR with a byte access to 0xE000ED29 - access the UFSR with a halfword access to 0xE000ED2A."]
    #[inline(always)]
    pub const fn cfsr(&self) -> &Cfsr {
        &self.cfsr
    }
    #[doc = "0xd2c - Hard Fault Status This register is used to obtain information about events that activate the Hard Fault handler. This register is a write-clear register. This means that writing a 1 to a bit clears that bit."]
    #[inline(always)]
    pub const fn hfsr(&self) -> &Hfsr {
        &self.hfsr
    }
    #[doc = "0xd30 - Debug Fault Status This register is used to monitor external debug requests, vector catches, data watchpoint match, BKPT instruction execution, halt requests. Multiple flags in the Debug Fault Status Register can be set when multiple fault conditions occur. The register is read/write clear. This means that it can be read normally. Writing a 1 to a bit clears that bit. Note that these bits are not set unless the event is caught. This means that it causes a stop of some sort. If halting debug is enabled, these events stop the processor into debug. If debug is disabled and the debug monitor is enabled, then this becomes a debug monitor handler call, if priority permits. If debug and the monitor are both disabled, some of these events are Hard Faults, and some are ignored."]
    #[inline(always)]
    pub const fn dfsr(&self) -> &Dfsr {
        &self.dfsr
    }
    #[doc = "0xd34 - Mem Manage Fault Address This register is used to read the address of the location that caused a Memory Manage Fault."]
    #[inline(always)]
    pub const fn mmfar(&self) -> &Mmfar {
        &self.mmfar
    }
    #[doc = "0xd38 - Bus Fault Address This register is used to read the address of the location that generated a Bus Fault."]
    #[inline(always)]
    pub const fn bfar(&self) -> &Bfar {
        &self.bfar
    }
    #[doc = "0xd3c - Auxiliary Fault Status This register is used to determine additional system fault information to software. Single-cycle high level on an auxiliary faults is latched as one. The bit can only be cleared by writing a one to the corresponding bit. Auxiliary fault inputs to the CPU are tied to 0."]
    #[inline(always)]
    pub const fn afsr(&self) -> &Afsr {
        &self.afsr
    }
    #[doc = "0xd40 - Processor Feature 0"]
    #[inline(always)]
    pub const fn id_pfr0(&self) -> &IdPfr0 {
        &self.id_pfr0
    }
    #[doc = "0xd44 - Processor Feature 1"]
    #[inline(always)]
    pub const fn id_pfr1(&self) -> &IdPfr1 {
        &self.id_pfr1
    }
    #[doc = "0xd48 - Debug Feature 0 This register provides a high level view of the debug system. Further details are provided in the debug infrastructure itself."]
    #[inline(always)]
    pub const fn id_dfr0(&self) -> &IdDfr0 {
        &self.id_dfr0
    }
    #[doc = "0xd4c - Auxiliary Feature 0 This register provides some freedom for implementation defined features to be registered. Not used in Cortex-M."]
    #[inline(always)]
    pub const fn id_afr0(&self) -> &IdAfr0 {
        &self.id_afr0
    }
    #[doc = "0xd50 - Memory Model Feature 0 General information on the memory model and memory management support."]
    #[inline(always)]
    pub const fn id_mmfr0(&self) -> &IdMmfr0 {
        &self.id_mmfr0
    }
    #[doc = "0xd54 - Memory Model Feature 1 General information on the memory model and memory management support."]
    #[inline(always)]
    pub const fn id_mmfr1(&self) -> &IdMmfr1 {
        &self.id_mmfr1
    }
    #[doc = "0xd58 - Memory Model Feature 2 General information on the memory model and memory management support."]
    #[inline(always)]
    pub const fn id_mmfr2(&self) -> &IdMmfr2 {
        &self.id_mmfr2
    }
    #[doc = "0xd5c - Memory Model Feature 3 General information on the memory model and memory management support."]
    #[inline(always)]
    pub const fn id_mmfr3(&self) -> &IdMmfr3 {
        &self.id_mmfr3
    }
    #[doc = "0xd60 - ISA Feature 0 Information on the instruction set attributes register"]
    #[inline(always)]
    pub const fn id_isar0(&self) -> &IdIsar0 {
        &self.id_isar0
    }
    #[doc = "0xd64 - ISA Feature 1 Information on the instruction set attributes register"]
    #[inline(always)]
    pub const fn id_isar1(&self) -> &IdIsar1 {
        &self.id_isar1
    }
    #[doc = "0xd68 - ISA Feature 2 Information on the instruction set attributes register"]
    #[inline(always)]
    pub const fn id_isar2(&self) -> &IdIsar2 {
        &self.id_isar2
    }
    #[doc = "0xd6c - ISA Feature 3 Information on the instruction set attributes register"]
    #[inline(always)]
    pub const fn id_isar3(&self) -> &IdIsar3 {
        &self.id_isar3
    }
    #[doc = "0xd70 - ISA Feature 4 Information on the instruction set attributes register"]
    #[inline(always)]
    pub const fn id_isar4(&self) -> &IdIsar4 {
        &self.id_isar4
    }
    #[doc = "0xd88 - Coprocessor Access Control This register specifies the access privileges for coprocessors."]
    #[inline(always)]
    pub const fn cpacr(&self) -> &Cpacr {
        &self.cpacr
    }
    #[doc = "0xd90 - MPU Type This register indicates many regions the MPU supports."]
    #[inline(always)]
    pub const fn mpu_type(&self) -> &MpuType {
        &self.mpu_type
    }
    #[doc = "0xd94 - MPU Control This register is used to enable the MPU, enable the default memory map (background region), and enable the MPU when in Hard Fault, Non-maskable Interrupt (NMI), and FAULTMASK escalated handlers. When the MPU is enabled, at least one region of the memory map must be enabled for the MPU to function unless the PRIVDEFENA bit is set. If the PRIVDEFENA bit is set and no regions are enabled, then only privileged code can operate. When the MPU is disabled, the default address map is used, as if no MPU is present. When the MPU is enabled, only the system partition and vector table loads are always accessible. Other areas are accessible based on regions and whether PRIVDEFENA is enabled. Unless HFNMIENA is set, the MPU is not enabled when the exception priority is -1 or -2. These priorities are only possible when in Hard fault, NMI, or when FAULTMASK is enabled. The HFNMIENA bit enables the MPU when operating with these two priorities."]
    #[inline(always)]
    pub const fn mpu_ctrl(&self) -> &MpuCtrl {
        &self.mpu_ctrl
    }
    #[doc = "0xd98 - MPU Region Number This register is used to select which protection region is accessed. The following write to MPU_RASR or MPU_RBAR configures the characteristics of the protection region that is selected by this register."]
    #[inline(always)]
    pub const fn mpu_rnr(&self) -> &MpuRnr {
        &self.mpu_rnr
    }
    #[doc = "0xd9c - MPU Region Base Address This register writes the base address of a region. It also contains a REGION field that can be used to override MPU_RNR.REGION, if the VALID bit is set. This register sets the base for the region. It is aligned by the size. So, a 64-KB sized region must be aligned on a multiple of 64KB, for example, 0x00010000 or 0x00020000. The region always reads back as the current MPU region number. VALID always reads back as 0. Writing VALID = 1 and REGION = n changes the region number to n. This is a short-hand way to write the MPU_RNR. This register is unpredictable if accessed other than as a word."]
    #[inline(always)]
    pub const fn mpu_rbar(&self) -> &MpuRbar {
        &self.mpu_rbar
    }
    #[doc = "0xda0 - MPU Region Attribute and Size This register controls the MPU access permissions. The register is made up of two part registers, each of halfword size. These can be accessed using the halfword size, or they can both be simultaneously accessed using a word operation. The sub-region disable bits are not supported for region sizes of 32 bytes, 64 bytes, and 128 bytes. When these region sizes are used, the subregion disable bits must be programmed as 0."]
    #[inline(always)]
    pub const fn mpu_rasr(&self) -> &MpuRasr {
        &self.mpu_rasr
    }
    #[doc = "0xda4 - MPU Alias 1 Region Base Address Alias for MPU_RBAR"]
    #[inline(always)]
    pub const fn mpu_rbar_a1(&self) -> &MpuRbarA1 {
        &self.mpu_rbar_a1
    }
    #[doc = "0xda8 - MPU Alias 1 Region Attribute and Size Alias for MPU_RASR"]
    #[inline(always)]
    pub const fn mpu_rasr_a1(&self) -> &MpuRasrA1 {
        &self.mpu_rasr_a1
    }
    #[doc = "0xdac - MPU Alias 2 Region Base Address Alias for MPU_RBAR"]
    #[inline(always)]
    pub const fn mpu_rbar_a2(&self) -> &MpuRbarA2 {
        &self.mpu_rbar_a2
    }
    #[doc = "0xdb0 - MPU Alias 2 Region Attribute and Size Alias for MPU_RASR"]
    #[inline(always)]
    pub const fn mpu_rasr_a2(&self) -> &MpuRasrA2 {
        &self.mpu_rasr_a2
    }
    #[doc = "0xdb4 - MPU Alias 3 Region Base Address Alias for MPU_RBAR"]
    #[inline(always)]
    pub const fn mpu_rbar_a3(&self) -> &MpuRbarA3 {
        &self.mpu_rbar_a3
    }
    #[doc = "0xdb8 - MPU Alias 3 Region Attribute and Size Alias for MPU_RASR"]
    #[inline(always)]
    pub const fn mpu_rasr_a3(&self) -> &MpuRasrA3 {
        &self.mpu_rasr_a3
    }
    #[doc = "0xdf0 - Debug Halting Control and Status The purpose of this register is to provide status information about the state of the processor, enable core debug, halt and step the processor. For writes, 0xA05F must be written to higher half-word of this register, otherwise the write operation is ignored and no bits are written into the register. If not enabled for Halting mode, C_DEBUGEN = 1, all other fields are disabled. This register is not reset on a core reset. It is reset by a power-on reset. However, C_HALT always clears on a core reset. To halt on a reset, the following bits must be enabled: DEMCR.VC_CORERESET and C_DEBUGEN. Note that writes to this register in any size other than word are unpredictable. It is acceptable to read in any size, and it can be used to avoid or intentionally change a sticky bit. Behavior of the system when writing to this register while CPU is halted (i.e. C_DEBUGEN = 1 and S_HALT= 1): C_HALT=0, C_STEP=0, C_MASKINTS=0 Exit Debug state and start instruction execution. Exceptions activate according to the exception configuration rules. C_HALT=0, C_STEP=0, C_MASKINTS=1 Exit Debug state and start instruction execution. PendSV, SysTick and external configurable interrupts are disabled, otherwise exceptions activate according to standard configuration rules. C_HALT=0, C_STEP=1, C_MASKINTS=0 Exit Debug state, step an instruction and halt. Exceptions activate according to the exception configuration rules. C_HALT=0, C_STEP=1, C_MASKINTS=1 Exit Debug state, step an instruction and halt. PendSV, SysTick and external configurable interrupts are disabled, otherwise exceptions activate according to standard configuration rules. C_HALT=1, C_STEP=x, C_MASKINTS=x Remain in Debug state"]
    #[inline(always)]
    pub const fn dhcsr(&self) -> &Dhcsr {
        &self.dhcsr
    }
    #[doc = "0xdf4 - Deubg Core Register Selector The purpose of this register is to select the processor register to transfer data to or from. This write-only register generates a handshake to the core to transfer data to or from Debug Core Register Data Register and the selected register. Until this core transaction is complete, DHCSR.S_REGRDY is 0. Note that writes to this register in any size but word are Unpredictable. Note that PSR registers are fully accessible this way, whereas some read as 0 when using MRS instructions. Note that all bits can be written, but some combinations cause a fault when execution is resumed."]
    #[inline(always)]
    pub const fn dcrsr(&self) -> &Dcrsr {
        &self.dcrsr
    }
    #[doc = "0xdf8 - Debug Core Register Data"]
    #[inline(always)]
    pub const fn dcrdr(&self) -> &Dcrdr {
        &self.dcrdr
    }
    #[doc = "0xdfc - Debug Exception and Monitor Control The purpose of this register is vector catching and debug monitor control. This register manages exception behavior under debug. Vector catching is only available to halting debug. The upper halfword is for monitor controls and the lower halfword is for halting exception support. This register is not reset on a system reset. This register is reset by a power-on reset. The fields MON_EN, MON_PEND, MON_STEP and MON_REQ are always cleared on a core reset. The debug monitor is enabled by software in the reset handler or later, or by the **AHB-AP** port. Vector catching is semi-synchronous. When a matching event is seen, a Halt is requested. Because the processor can only halt on an instruction boundary, it must wait until the next instruction boundary. As a result, it stops on the first instruction of the exception handler. However, two special cases exist when a vector catch has triggered: 1. If a fault is taken during a vector read or stack push error the halt occurs on the corresponding fault handler for the vector error or stack push. 2. If a late arriving interrupt detected during a vector read or stack push error it is not taken. That is, an implementation that supports the late arrival optimization must suppress it in this case."]
    #[inline(always)]
    pub const fn demcr(&self) -> &Demcr {
        &self.demcr
    }
    #[doc = "0xf00 - Software Trigger Interrupt"]
    #[inline(always)]
    pub const fn stir(&self) -> &Stir {
        &self.stir
    }
}
#[doc = "RESERVED000 (rw) register accessor: Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reserved000::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reserved000::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reserved000`]
module"]
#[doc(alias = "RESERVED000")]
pub type Reserved000 = crate::Reg<reserved000::Reserved000Spec>;
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub mod reserved000;
#[doc = "ICTR (rw) register accessor: Interrupt Control Type Read this register to see the number of interrupt lines that the NVIC supports.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ictr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ictr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ictr`]
module"]
#[doc(alias = "ICTR")]
pub type Ictr = crate::Reg<ictr::IctrSpec>;
#[doc = "Interrupt Control Type Read this register to see the number of interrupt lines that the NVIC supports."]
pub mod ictr;
#[doc = "ACTLR (rw) register accessor: Auxiliary Control This register is used to disable certain aspects of functionality within the processor\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`actlr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`actlr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@actlr`]
module"]
#[doc(alias = "ACTLR")]
pub type Actlr = crate::Reg<actlr::ActlrSpec>;
#[doc = "Auxiliary Control This register is used to disable certain aspects of functionality within the processor"]
pub mod actlr;
#[doc = "STCSR (rw) register accessor: SysTick Control and Status This register enables the SysTick features and returns status flags related to SysTick.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stcsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stcsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stcsr`]
module"]
#[doc(alias = "STCSR")]
pub type Stcsr = crate::Reg<stcsr::StcsrSpec>;
#[doc = "SysTick Control and Status This register enables the SysTick features and returns status flags related to SysTick."]
pub mod stcsr;
#[doc = "STRVR (rw) register accessor: SysTick Reload Value This register is used to specify the start value to load into the current value register STCVR.CURRENT when the counter reaches 0. It can be any value between 1 and 0x00FFFFFF. A start value of 0 is possible, but has no effect because the SysTick interrupt and STCSR.COUNTFLAG are activated when counting from 1 to 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`strvr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`strvr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@strvr`]
module"]
#[doc(alias = "STRVR")]
pub type Strvr = crate::Reg<strvr::StrvrSpec>;
#[doc = "SysTick Reload Value This register is used to specify the start value to load into the current value register STCVR.CURRENT when the counter reaches 0. It can be any value between 1 and 0x00FFFFFF. A start value of 0 is possible, but has no effect because the SysTick interrupt and STCSR.COUNTFLAG are activated when counting from 1 to 0."]
pub mod strvr;
#[doc = "STCVR (rw) register accessor: SysTick Current Value Read from this register returns the current value of SysTick counter. Writing to this register resets the SysTick counter (as well as STCSR.COUNTFLAG).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stcvr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stcvr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stcvr`]
module"]
#[doc(alias = "STCVR")]
pub type Stcvr = crate::Reg<stcvr::StcvrSpec>;
#[doc = "SysTick Current Value Read from this register returns the current value of SysTick counter. Writing to this register resets the SysTick counter (as well as STCSR.COUNTFLAG)."]
pub mod stcvr;
#[doc = "STCR (rw) register accessor: SysTick Calibration Value Used to enable software to scale to any required speed using divide and multiply.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stcr`]
module"]
#[doc(alias = "STCR")]
pub type Stcr = crate::Reg<stcr::StcrSpec>;
#[doc = "SysTick Calibration Value Used to enable software to scale to any required speed using divide and multiply."]
pub mod stcr;
#[doc = "NVIC_ISER0 (rw) register accessor: Irq 0 to 31 Set Enable This register is used to enable interrupts and determine which interrupts are currently enabled.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_iser0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_iser0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nvic_iser0`]
module"]
#[doc(alias = "NVIC_ISER0")]
pub type NvicIser0 = crate::Reg<nvic_iser0::NvicIser0Spec>;
#[doc = "Irq 0 to 31 Set Enable This register is used to enable interrupts and determine which interrupts are currently enabled."]
pub mod nvic_iser0;
#[doc = "NVIC_ISER1 (rw) register accessor: Irq 32 to 63 Set Enable This register is used to enable interrupts and determine which interrupts are currently enabled.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_iser1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_iser1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nvic_iser1`]
module"]
#[doc(alias = "NVIC_ISER1")]
pub type NvicIser1 = crate::Reg<nvic_iser1::NvicIser1Spec>;
#[doc = "Irq 32 to 63 Set Enable This register is used to enable interrupts and determine which interrupts are currently enabled."]
pub mod nvic_iser1;
#[doc = "NVIC_ICER0 (rw) register accessor: Irq 0 to 31 Clear Enable This register is used to disable interrupts and determine which interrupts are currently enabled.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_icer0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_icer0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nvic_icer0`]
module"]
#[doc(alias = "NVIC_ICER0")]
pub type NvicIcer0 = crate::Reg<nvic_icer0::NvicIcer0Spec>;
#[doc = "Irq 0 to 31 Clear Enable This register is used to disable interrupts and determine which interrupts are currently enabled."]
pub mod nvic_icer0;
#[doc = "NVIC_ICER1 (rw) register accessor: Irq 32 to 63 Clear Enable This register is used to disable interrupts and determine which interrupts are currently enabled.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_icer1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_icer1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nvic_icer1`]
module"]
#[doc(alias = "NVIC_ICER1")]
pub type NvicIcer1 = crate::Reg<nvic_icer1::NvicIcer1Spec>;
#[doc = "Irq 32 to 63 Clear Enable This register is used to disable interrupts and determine which interrupts are currently enabled."]
pub mod nvic_icer1;
#[doc = "NVIC_ISPR0 (rw) register accessor: Irq 0 to 31 Set Pending This register is used to force interrupts into the pending state and determine which interrupts are currently pending.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_ispr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_ispr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nvic_ispr0`]
module"]
#[doc(alias = "NVIC_ISPR0")]
pub type NvicIspr0 = crate::Reg<nvic_ispr0::NvicIspr0Spec>;
#[doc = "Irq 0 to 31 Set Pending This register is used to force interrupts into the pending state and determine which interrupts are currently pending."]
pub mod nvic_ispr0;
#[doc = "NVIC_ISPR1 (rw) register accessor: Irq 32 to 63 Set Pending This register is used to force interrupts into the pending state and determine which interrupts are currently pending.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_ispr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_ispr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nvic_ispr1`]
module"]
#[doc(alias = "NVIC_ISPR1")]
pub type NvicIspr1 = crate::Reg<nvic_ispr1::NvicIspr1Spec>;
#[doc = "Irq 32 to 63 Set Pending This register is used to force interrupts into the pending state and determine which interrupts are currently pending."]
pub mod nvic_ispr1;
#[doc = "NVIC_ICPR0 (rw) register accessor: Irq 0 to 31 Clear Pending This register is used to clear pending interrupts and determine which interrupts are currently pending.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_icpr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_icpr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nvic_icpr0`]
module"]
#[doc(alias = "NVIC_ICPR0")]
pub type NvicIcpr0 = crate::Reg<nvic_icpr0::NvicIcpr0Spec>;
#[doc = "Irq 0 to 31 Clear Pending This register is used to clear pending interrupts and determine which interrupts are currently pending."]
pub mod nvic_icpr0;
#[doc = "NVIC_ICPR1 (rw) register accessor: Irq 32 to 63 Clear Pending This register is used to clear pending interrupts and determine which interrupts are currently pending.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_icpr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_icpr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nvic_icpr1`]
module"]
#[doc(alias = "NVIC_ICPR1")]
pub type NvicIcpr1 = crate::Reg<nvic_icpr1::NvicIcpr1Spec>;
#[doc = "Irq 32 to 63 Clear Pending This register is used to clear pending interrupts and determine which interrupts are currently pending."]
pub mod nvic_icpr1;
#[doc = "NVIC_IABR0 (rw) register accessor: Irq 0 to 31 Active Bit This register is used to determine which interrupts are active. Each flag in the register corresponds to one interrupt.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_iabr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_iabr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nvic_iabr0`]
module"]
#[doc(alias = "NVIC_IABR0")]
pub type NvicIabr0 = crate::Reg<nvic_iabr0::NvicIabr0Spec>;
#[doc = "Irq 0 to 31 Active Bit This register is used to determine which interrupts are active. Each flag in the register corresponds to one interrupt."]
pub mod nvic_iabr0;
#[doc = "NVIC_IABR1 (rw) register accessor: Irq 32 to 63 Active Bit This register is used to determine which interrupts are active. Each flag in the register corresponds to one interrupt.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_iabr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_iabr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nvic_iabr1`]
module"]
#[doc(alias = "NVIC_IABR1")]
pub type NvicIabr1 = crate::Reg<nvic_iabr1::NvicIabr1Spec>;
#[doc = "Irq 32 to 63 Active Bit This register is used to determine which interrupts are active. Each flag in the register corresponds to one interrupt."]
pub mod nvic_iabr1;
#[doc = "NVIC_IPR0 (rw) register accessor: Irq 0 to 3 Priority This register is used to assign a priority from 0 to 255 to each of the available interrupts. 0 is the highest priority, and 255 is the lowest. The interpretation of the Interrupt Priority Registers changes based on the setting in AIRCR.PRIGROUP.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_ipr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_ipr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nvic_ipr0`]
module"]
#[doc(alias = "NVIC_IPR0")]
pub type NvicIpr0 = crate::Reg<nvic_ipr0::NvicIpr0Spec>;
#[doc = "Irq 0 to 3 Priority This register is used to assign a priority from 0 to 255 to each of the available interrupts. 0 is the highest priority, and 255 is the lowest. The interpretation of the Interrupt Priority Registers changes based on the setting in AIRCR.PRIGROUP."]
pub mod nvic_ipr0;
#[doc = "NVIC_IPR1 (rw) register accessor: Irq 4 to 7 Priority This register is used to assign a priority from 0 to 255 to each of the available interrupts. 0 is the highest priority, and 255 is the lowest. The interpretation of the Interrupt Priority Registers changes based on the setting in AIRCR.PRIGROUP.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_ipr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_ipr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nvic_ipr1`]
module"]
#[doc(alias = "NVIC_IPR1")]
pub type NvicIpr1 = crate::Reg<nvic_ipr1::NvicIpr1Spec>;
#[doc = "Irq 4 to 7 Priority This register is used to assign a priority from 0 to 255 to each of the available interrupts. 0 is the highest priority, and 255 is the lowest. The interpretation of the Interrupt Priority Registers changes based on the setting in AIRCR.PRIGROUP."]
pub mod nvic_ipr1;
#[doc = "NVIC_IPR2 (rw) register accessor: Irq 8 to 11 Priority This register is used to assign a priority from 0 to 255 to each of the available interrupts. 0 is the highest priority, and 255 is the lowest. The interpretation of the Interrupt Priority Registers changes based on the setting in AIRCR.PRIGROUP.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_ipr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_ipr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nvic_ipr2`]
module"]
#[doc(alias = "NVIC_IPR2")]
pub type NvicIpr2 = crate::Reg<nvic_ipr2::NvicIpr2Spec>;
#[doc = "Irq 8 to 11 Priority This register is used to assign a priority from 0 to 255 to each of the available interrupts. 0 is the highest priority, and 255 is the lowest. The interpretation of the Interrupt Priority Registers changes based on the setting in AIRCR.PRIGROUP."]
pub mod nvic_ipr2;
#[doc = "NVIC_IPR3 (rw) register accessor: Irq 12 to 15 Priority This register is used to assign a priority from 0 to 255 to each of the available interrupts. 0 is the highest priority, and 255 is the lowest. The interpretation of the Interrupt Priority Registers changes based on the setting in AIRCR.PRIGROUP.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_ipr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_ipr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nvic_ipr3`]
module"]
#[doc(alias = "NVIC_IPR3")]
pub type NvicIpr3 = crate::Reg<nvic_ipr3::NvicIpr3Spec>;
#[doc = "Irq 12 to 15 Priority This register is used to assign a priority from 0 to 255 to each of the available interrupts. 0 is the highest priority, and 255 is the lowest. The interpretation of the Interrupt Priority Registers changes based on the setting in AIRCR.PRIGROUP."]
pub mod nvic_ipr3;
#[doc = "NVIC_IPR4 (rw) register accessor: Irq 16 to 19 Priority This register is used to assign a priority from 0 to 255 to each of the available interrupts. 0 is the highest priority, and 255 is the lowest. The interpretation of the Interrupt Priority Registers changes based on the setting in AIRCR.PRIGROUP.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_ipr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_ipr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nvic_ipr4`]
module"]
#[doc(alias = "NVIC_IPR4")]
pub type NvicIpr4 = crate::Reg<nvic_ipr4::NvicIpr4Spec>;
#[doc = "Irq 16 to 19 Priority This register is used to assign a priority from 0 to 255 to each of the available interrupts. 0 is the highest priority, and 255 is the lowest. The interpretation of the Interrupt Priority Registers changes based on the setting in AIRCR.PRIGROUP."]
pub mod nvic_ipr4;
#[doc = "NVIC_IPR5 (rw) register accessor: Irq 20 to 23 Priority This register is used to assign a priority from 0 to 255 to each of the available interrupts. 0 is the highest priority, and 255 is the lowest. The interpretation of the Interrupt Priority Registers changes based on the setting in AIRCR.PRIGROUP.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_ipr5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_ipr5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nvic_ipr5`]
module"]
#[doc(alias = "NVIC_IPR5")]
pub type NvicIpr5 = crate::Reg<nvic_ipr5::NvicIpr5Spec>;
#[doc = "Irq 20 to 23 Priority This register is used to assign a priority from 0 to 255 to each of the available interrupts. 0 is the highest priority, and 255 is the lowest. The interpretation of the Interrupt Priority Registers changes based on the setting in AIRCR.PRIGROUP."]
pub mod nvic_ipr5;
#[doc = "NVIC_IPR6 (rw) register accessor: Irq 24 to 27 Priority This register is used to assign a priority from 0 to 255 to each of the available interrupts. 0 is the highest priority, and 255 is the lowest. The interpretation of the Interrupt Priority Registers changes based on the setting in AIRCR.PRIGROUP.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_ipr6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_ipr6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nvic_ipr6`]
module"]
#[doc(alias = "NVIC_IPR6")]
pub type NvicIpr6 = crate::Reg<nvic_ipr6::NvicIpr6Spec>;
#[doc = "Irq 24 to 27 Priority This register is used to assign a priority from 0 to 255 to each of the available interrupts. 0 is the highest priority, and 255 is the lowest. The interpretation of the Interrupt Priority Registers changes based on the setting in AIRCR.PRIGROUP."]
pub mod nvic_ipr6;
#[doc = "NVIC_IPR7 (rw) register accessor: Irq 28 to 31 Priority This register is used to assign a priority from 0 to 255 to each of the available interrupts. 0 is the highest priority, and 255 is the lowest. The interpretation of the Interrupt Priority Registers changes based on the setting in AIRCR.PRIGROUP.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_ipr7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_ipr7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nvic_ipr7`]
module"]
#[doc(alias = "NVIC_IPR7")]
pub type NvicIpr7 = crate::Reg<nvic_ipr7::NvicIpr7Spec>;
#[doc = "Irq 28 to 31 Priority This register is used to assign a priority from 0 to 255 to each of the available interrupts. 0 is the highest priority, and 255 is the lowest. The interpretation of the Interrupt Priority Registers changes based on the setting in AIRCR.PRIGROUP."]
pub mod nvic_ipr7;
#[doc = "NVIC_IPR8 (rw) register accessor: Irq 32 to 35 Priority This register is used to assign a priority from 0 to 255 to each of the available interrupts. 0 is the highest priority, and 255 is the lowest. The interpretation of the Interrupt Priority Registers changes based on the setting in AIRCR.PRIGROUP.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_ipr8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_ipr8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nvic_ipr8`]
module"]
#[doc(alias = "NVIC_IPR8")]
pub type NvicIpr8 = crate::Reg<nvic_ipr8::NvicIpr8Spec>;
#[doc = "Irq 32 to 35 Priority This register is used to assign a priority from 0 to 255 to each of the available interrupts. 0 is the highest priority, and 255 is the lowest. The interpretation of the Interrupt Priority Registers changes based on the setting in AIRCR.PRIGROUP."]
pub mod nvic_ipr8;
#[doc = "NVIC_IPR9 (rw) register accessor: Irq 32 to 35 Priority This register is used to assign a priority from 0 to 255 to each of the available interrupts. 0 is the highest priority, and 255 is the lowest. The interpretation of the Interrupt Priority Registers changes based on the setting in AIRCR.PRIGROUP.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_ipr9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_ipr9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nvic_ipr9`]
module"]
#[doc(alias = "NVIC_IPR9")]
pub type NvicIpr9 = crate::Reg<nvic_ipr9::NvicIpr9Spec>;
#[doc = "Irq 32 to 35 Priority This register is used to assign a priority from 0 to 255 to each of the available interrupts. 0 is the highest priority, and 255 is the lowest. The interpretation of the Interrupt Priority Registers changes based on the setting in AIRCR.PRIGROUP."]
pub mod nvic_ipr9;
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
#[doc = "CPACR (rw) register accessor: Coprocessor Access Control This register specifies the access privileges for coprocessors.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpacr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpacr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpacr`]
module"]
#[doc(alias = "CPACR")]
pub type Cpacr = crate::Reg<cpacr::CpacrSpec>;
#[doc = "Coprocessor Access Control This register specifies the access privileges for coprocessors."]
pub mod cpacr;
#[doc = "MPU_TYPE (rw) register accessor: MPU Type This register indicates many regions the MPU supports.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpu_type::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpu_type::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpu_type`]
module"]
#[doc(alias = "MPU_TYPE")]
pub type MpuType = crate::Reg<mpu_type::MpuTypeSpec>;
#[doc = "MPU Type This register indicates many regions the MPU supports."]
pub mod mpu_type;
#[doc = "MPU_CTRL (rw) register accessor: MPU Control This register is used to enable the MPU, enable the default memory map (background region), and enable the MPU when in Hard Fault, Non-maskable Interrupt (NMI), and FAULTMASK escalated handlers. When the MPU is enabled, at least one region of the memory map must be enabled for the MPU to function unless the PRIVDEFENA bit is set. If the PRIVDEFENA bit is set and no regions are enabled, then only privileged code can operate. When the MPU is disabled, the default address map is used, as if no MPU is present. When the MPU is enabled, only the system partition and vector table loads are always accessible. Other areas are accessible based on regions and whether PRIVDEFENA is enabled. Unless HFNMIENA is set, the MPU is not enabled when the exception priority is -1 or -2. These priorities are only possible when in Hard fault, NMI, or when FAULTMASK is enabled. The HFNMIENA bit enables the MPU when operating with these two priorities.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpu_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpu_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpu_ctrl`]
module"]
#[doc(alias = "MPU_CTRL")]
pub type MpuCtrl = crate::Reg<mpu_ctrl::MpuCtrlSpec>;
#[doc = "MPU Control This register is used to enable the MPU, enable the default memory map (background region), and enable the MPU when in Hard Fault, Non-maskable Interrupt (NMI), and FAULTMASK escalated handlers. When the MPU is enabled, at least one region of the memory map must be enabled for the MPU to function unless the PRIVDEFENA bit is set. If the PRIVDEFENA bit is set and no regions are enabled, then only privileged code can operate. When the MPU is disabled, the default address map is used, as if no MPU is present. When the MPU is enabled, only the system partition and vector table loads are always accessible. Other areas are accessible based on regions and whether PRIVDEFENA is enabled. Unless HFNMIENA is set, the MPU is not enabled when the exception priority is -1 or -2. These priorities are only possible when in Hard fault, NMI, or when FAULTMASK is enabled. The HFNMIENA bit enables the MPU when operating with these two priorities."]
pub mod mpu_ctrl;
#[doc = "MPU_RNR (rw) register accessor: MPU Region Number This register is used to select which protection region is accessed. The following write to MPU_RASR or MPU_RBAR configures the characteristics of the protection region that is selected by this register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpu_rnr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpu_rnr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpu_rnr`]
module"]
#[doc(alias = "MPU_RNR")]
pub type MpuRnr = crate::Reg<mpu_rnr::MpuRnrSpec>;
#[doc = "MPU Region Number This register is used to select which protection region is accessed. The following write to MPU_RASR or MPU_RBAR configures the characteristics of the protection region that is selected by this register."]
pub mod mpu_rnr;
#[doc = "MPU_RBAR (rw) register accessor: MPU Region Base Address This register writes the base address of a region. It also contains a REGION field that can be used to override MPU_RNR.REGION, if the VALID bit is set. This register sets the base for the region. It is aligned by the size. So, a 64-KB sized region must be aligned on a multiple of 64KB, for example, 0x00010000 or 0x00020000. The region always reads back as the current MPU region number. VALID always reads back as 0. Writing VALID = 1 and REGION = n changes the region number to n. This is a short-hand way to write the MPU_RNR. This register is unpredictable if accessed other than as a word.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpu_rbar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpu_rbar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpu_rbar`]
module"]
#[doc(alias = "MPU_RBAR")]
pub type MpuRbar = crate::Reg<mpu_rbar::MpuRbarSpec>;
#[doc = "MPU Region Base Address This register writes the base address of a region. It also contains a REGION field that can be used to override MPU_RNR.REGION, if the VALID bit is set. This register sets the base for the region. It is aligned by the size. So, a 64-KB sized region must be aligned on a multiple of 64KB, for example, 0x00010000 or 0x00020000. The region always reads back as the current MPU region number. VALID always reads back as 0. Writing VALID = 1 and REGION = n changes the region number to n. This is a short-hand way to write the MPU_RNR. This register is unpredictable if accessed other than as a word."]
pub mod mpu_rbar;
#[doc = "MPU_RASR (rw) register accessor: MPU Region Attribute and Size This register controls the MPU access permissions. The register is made up of two part registers, each of halfword size. These can be accessed using the halfword size, or they can both be simultaneously accessed using a word operation. The sub-region disable bits are not supported for region sizes of 32 bytes, 64 bytes, and 128 bytes. When these region sizes are used, the subregion disable bits must be programmed as 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpu_rasr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpu_rasr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpu_rasr`]
module"]
#[doc(alias = "MPU_RASR")]
pub type MpuRasr = crate::Reg<mpu_rasr::MpuRasrSpec>;
#[doc = "MPU Region Attribute and Size This register controls the MPU access permissions. The register is made up of two part registers, each of halfword size. These can be accessed using the halfword size, or they can both be simultaneously accessed using a word operation. The sub-region disable bits are not supported for region sizes of 32 bytes, 64 bytes, and 128 bytes. When these region sizes are used, the subregion disable bits must be programmed as 0."]
pub mod mpu_rasr;
#[doc = "MPU_RBAR_A1 (rw) register accessor: MPU Alias 1 Region Base Address Alias for MPU_RBAR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpu_rbar_a1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpu_rbar_a1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpu_rbar_a1`]
module"]
#[doc(alias = "MPU_RBAR_A1")]
pub type MpuRbarA1 = crate::Reg<mpu_rbar_a1::MpuRbarA1Spec>;
#[doc = "MPU Alias 1 Region Base Address Alias for MPU_RBAR"]
pub mod mpu_rbar_a1;
#[doc = "MPU_RASR_A1 (rw) register accessor: MPU Alias 1 Region Attribute and Size Alias for MPU_RASR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpu_rasr_a1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpu_rasr_a1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpu_rasr_a1`]
module"]
#[doc(alias = "MPU_RASR_A1")]
pub type MpuRasrA1 = crate::Reg<mpu_rasr_a1::MpuRasrA1Spec>;
#[doc = "MPU Alias 1 Region Attribute and Size Alias for MPU_RASR"]
pub mod mpu_rasr_a1;
#[doc = "MPU_RBAR_A2 (rw) register accessor: MPU Alias 2 Region Base Address Alias for MPU_RBAR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpu_rbar_a2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpu_rbar_a2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpu_rbar_a2`]
module"]
#[doc(alias = "MPU_RBAR_A2")]
pub type MpuRbarA2 = crate::Reg<mpu_rbar_a2::MpuRbarA2Spec>;
#[doc = "MPU Alias 2 Region Base Address Alias for MPU_RBAR"]
pub mod mpu_rbar_a2;
#[doc = "MPU_RASR_A2 (rw) register accessor: MPU Alias 2 Region Attribute and Size Alias for MPU_RASR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpu_rasr_a2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpu_rasr_a2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpu_rasr_a2`]
module"]
#[doc(alias = "MPU_RASR_A2")]
pub type MpuRasrA2 = crate::Reg<mpu_rasr_a2::MpuRasrA2Spec>;
#[doc = "MPU Alias 2 Region Attribute and Size Alias for MPU_RASR"]
pub mod mpu_rasr_a2;
#[doc = "MPU_RBAR_A3 (rw) register accessor: MPU Alias 3 Region Base Address Alias for MPU_RBAR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpu_rbar_a3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpu_rbar_a3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpu_rbar_a3`]
module"]
#[doc(alias = "MPU_RBAR_A3")]
pub type MpuRbarA3 = crate::Reg<mpu_rbar_a3::MpuRbarA3Spec>;
#[doc = "MPU Alias 3 Region Base Address Alias for MPU_RBAR"]
pub mod mpu_rbar_a3;
#[doc = "MPU_RASR_A3 (rw) register accessor: MPU Alias 3 Region Attribute and Size Alias for MPU_RASR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpu_rasr_a3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpu_rasr_a3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpu_rasr_a3`]
module"]
#[doc(alias = "MPU_RASR_A3")]
pub type MpuRasrA3 = crate::Reg<mpu_rasr_a3::MpuRasrA3Spec>;
#[doc = "MPU Alias 3 Region Attribute and Size Alias for MPU_RASR"]
pub mod mpu_rasr_a3;
#[doc = "DHCSR (rw) register accessor: Debug Halting Control and Status The purpose of this register is to provide status information about the state of the processor, enable core debug, halt and step the processor. For writes, 0xA05F must be written to higher half-word of this register, otherwise the write operation is ignored and no bits are written into the register. If not enabled for Halting mode, C_DEBUGEN = 1, all other fields are disabled. This register is not reset on a core reset. It is reset by a power-on reset. However, C_HALT always clears on a core reset. To halt on a reset, the following bits must be enabled: DEMCR.VC_CORERESET and C_DEBUGEN. Note that writes to this register in any size other than word are unpredictable. It is acceptable to read in any size, and it can be used to avoid or intentionally change a sticky bit. Behavior of the system when writing to this register while CPU is halted (i.e. C_DEBUGEN = 1 and S_HALT= 1): C_HALT=0, C_STEP=0, C_MASKINTS=0 Exit Debug state and start instruction execution. Exceptions activate according to the exception configuration rules. C_HALT=0, C_STEP=0, C_MASKINTS=1 Exit Debug state and start instruction execution. PendSV, SysTick and external configurable interrupts are disabled, otherwise exceptions activate according to standard configuration rules. C_HALT=0, C_STEP=1, C_MASKINTS=0 Exit Debug state, step an instruction and halt. Exceptions activate according to the exception configuration rules. C_HALT=0, C_STEP=1, C_MASKINTS=1 Exit Debug state, step an instruction and halt. PendSV, SysTick and external configurable interrupts are disabled, otherwise exceptions activate according to standard configuration rules. C_HALT=1, C_STEP=x, C_MASKINTS=x Remain in Debug state\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dhcsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dhcsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dhcsr`]
module"]
#[doc(alias = "DHCSR")]
pub type Dhcsr = crate::Reg<dhcsr::DhcsrSpec>;
#[doc = "Debug Halting Control and Status The purpose of this register is to provide status information about the state of the processor, enable core debug, halt and step the processor. For writes, 0xA05F must be written to higher half-word of this register, otherwise the write operation is ignored and no bits are written into the register. If not enabled for Halting mode, C_DEBUGEN = 1, all other fields are disabled. This register is not reset on a core reset. It is reset by a power-on reset. However, C_HALT always clears on a core reset. To halt on a reset, the following bits must be enabled: DEMCR.VC_CORERESET and C_DEBUGEN. Note that writes to this register in any size other than word are unpredictable. It is acceptable to read in any size, and it can be used to avoid or intentionally change a sticky bit. Behavior of the system when writing to this register while CPU is halted (i.e. C_DEBUGEN = 1 and S_HALT= 1): C_HALT=0, C_STEP=0, C_MASKINTS=0 Exit Debug state and start instruction execution. Exceptions activate according to the exception configuration rules. C_HALT=0, C_STEP=0, C_MASKINTS=1 Exit Debug state and start instruction execution. PendSV, SysTick and external configurable interrupts are disabled, otherwise exceptions activate according to standard configuration rules. C_HALT=0, C_STEP=1, C_MASKINTS=0 Exit Debug state, step an instruction and halt. Exceptions activate according to the exception configuration rules. C_HALT=0, C_STEP=1, C_MASKINTS=1 Exit Debug state, step an instruction and halt. PendSV, SysTick and external configurable interrupts are disabled, otherwise exceptions activate according to standard configuration rules. C_HALT=1, C_STEP=x, C_MASKINTS=x Remain in Debug state"]
pub mod dhcsr;
#[doc = "DCRSR (rw) register accessor: Deubg Core Register Selector The purpose of this register is to select the processor register to transfer data to or from. This write-only register generates a handshake to the core to transfer data to or from Debug Core Register Data Register and the selected register. Until this core transaction is complete, DHCSR.S_REGRDY is 0. Note that writes to this register in any size but word are Unpredictable. Note that PSR registers are fully accessible this way, whereas some read as 0 when using MRS instructions. Note that all bits can be written, but some combinations cause a fault when execution is resumed.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcrsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcrsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcrsr`]
module"]
#[doc(alias = "DCRSR")]
pub type Dcrsr = crate::Reg<dcrsr::DcrsrSpec>;
#[doc = "Deubg Core Register Selector The purpose of this register is to select the processor register to transfer data to or from. This write-only register generates a handshake to the core to transfer data to or from Debug Core Register Data Register and the selected register. Until this core transaction is complete, DHCSR.S_REGRDY is 0. Note that writes to this register in any size but word are Unpredictable. Note that PSR registers are fully accessible this way, whereas some read as 0 when using MRS instructions. Note that all bits can be written, but some combinations cause a fault when execution is resumed."]
pub mod dcrsr;
#[doc = "DCRDR (rw) register accessor: Debug Core Register Data\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcrdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcrdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcrdr`]
module"]
#[doc(alias = "DCRDR")]
pub type Dcrdr = crate::Reg<dcrdr::DcrdrSpec>;
#[doc = "Debug Core Register Data"]
pub mod dcrdr;
#[doc = "DEMCR (rw) register accessor: Debug Exception and Monitor Control The purpose of this register is vector catching and debug monitor control. This register manages exception behavior under debug. Vector catching is only available to halting debug. The upper halfword is for monitor controls and the lower halfword is for halting exception support. This register is not reset on a system reset. This register is reset by a power-on reset. The fields MON_EN, MON_PEND, MON_STEP and MON_REQ are always cleared on a core reset. The debug monitor is enabled by software in the reset handler or later, or by the **AHB-AP** port. Vector catching is semi-synchronous. When a matching event is seen, a Halt is requested. Because the processor can only halt on an instruction boundary, it must wait until the next instruction boundary. As a result, it stops on the first instruction of the exception handler. However, two special cases exist when a vector catch has triggered: 1. If a fault is taken during a vector read or stack push error the halt occurs on the corresponding fault handler for the vector error or stack push. 2. If a late arriving interrupt detected during a vector read or stack push error it is not taken. That is, an implementation that supports the late arrival optimization must suppress it in this case.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`demcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`demcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@demcr`]
module"]
#[doc(alias = "DEMCR")]
pub type Demcr = crate::Reg<demcr::DemcrSpec>;
#[doc = "Debug Exception and Monitor Control The purpose of this register is vector catching and debug monitor control. This register manages exception behavior under debug. Vector catching is only available to halting debug. The upper halfword is for monitor controls and the lower halfword is for halting exception support. This register is not reset on a system reset. This register is reset by a power-on reset. The fields MON_EN, MON_PEND, MON_STEP and MON_REQ are always cleared on a core reset. The debug monitor is enabled by software in the reset handler or later, or by the **AHB-AP** port. Vector catching is semi-synchronous. When a matching event is seen, a Halt is requested. Because the processor can only halt on an instruction boundary, it must wait until the next instruction boundary. As a result, it stops on the first instruction of the exception handler. However, two special cases exist when a vector catch has triggered: 1. If a fault is taken during a vector read or stack push error the halt occurs on the corresponding fault handler for the vector error or stack push. 2. If a late arriving interrupt detected during a vector read or stack push error it is not taken. That is, an implementation that supports the late arrival optimization must suppress it in this case."]
pub mod demcr;
#[doc = "STIR (rw) register accessor: Software Trigger Interrupt\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stir::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stir::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stir`]
module"]
#[doc(alias = "STIR")]
pub type Stir = crate::Reg<stir::StirSpec>;
#[doc = "Software Trigger Interrupt"]
pub mod stir;
