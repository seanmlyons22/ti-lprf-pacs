#[doc = "Register `DHCSR` reader"]
pub type R = crate::R<DhcsrSpec>;
#[doc = "Register `DHCSR` writer"]
pub type W = crate::W<DhcsrSpec>;
#[doc = "Field `C_DEBUGEN` reader - 0:0\\]
Enables debug. This can only be written by AHB-AP and not by the core. It is ignored when written by the core, which cannot set or clear it. The core must write a 1 to it when writing C_HALT to halt itself. The values of C_HALT, C_STEP and C_MASKINTS are ignored by hardware when C_DEBUGEN = 0. The read values for C_HALT, C_STEP and C_MASKINTS fields will be unknown to software when C_DEBUGEN = 0."]
pub type CDebugenR = crate::BitReader;
#[doc = "Field `C_DEBUGEN` writer - 0:0\\]
Enables debug. This can only be written by AHB-AP and not by the core. It is ignored when written by the core, which cannot set or clear it. The core must write a 1 to it when writing C_HALT to halt itself. The values of C_HALT, C_STEP and C_MASKINTS are ignored by hardware when C_DEBUGEN = 0. The read values for C_HALT, C_STEP and C_MASKINTS fields will be unknown to software when C_DEBUGEN = 0."]
pub type CDebugenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C_HALT` reader - 1:1\\]
Halts the core. This bit is set automatically when the core Halts. For example Breakpoint. This bit clears on core reset."]
pub type CHaltR = crate::BitReader;
#[doc = "Field `C_HALT` writer - 1:1\\]
Halts the core. This bit is set automatically when the core Halts. For example Breakpoint. This bit clears on core reset."]
pub type CHaltW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C_STEP` reader - 2:2\\]
Steps the core in halted debug. When C_DEBUGEN = 0, this bit has no effect. Must only be modified when the processor is halted (S_HALT == 1). Modifying C_STEP while the system is running with halting debug support enabled (C_DEBUGEN = 1, S_HALT = 0) may cause unpredictable behavior."]
pub type CStepR = crate::BitReader;
#[doc = "Field `C_STEP` writer - 2:2\\]
Steps the core in halted debug. When C_DEBUGEN = 0, this bit has no effect. Must only be modified when the processor is halted (S_HALT == 1). Modifying C_STEP while the system is running with halting debug support enabled (C_DEBUGEN = 1, S_HALT = 0) may cause unpredictable behavior."]
pub type CStepW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C_MASKINTS` reader - 3:3\\]
Mask interrupts when stepping or running in halted debug. This masking does not affect NMI, fault exceptions and SVC caused by execution of the instructions. This bit must only be modified when the processor is halted (S_HALT == 1). C_MASKINTS must be set or cleared before halt is released (i.e., the writes to set or clear C_MASKINTS and to set or clear C_HALT must be separate). Modifying C_MASKINTS while the system is running with halting debug support enabled (C_DEBUGEN = 1, S_HALT = 0) may cause unpredictable behavior."]
pub type CMaskintsR = crate::BitReader;
#[doc = "Field `C_MASKINTS` writer - 3:3\\]
Mask interrupts when stepping or running in halted debug. This masking does not affect NMI, fault exceptions and SVC caused by execution of the instructions. This bit must only be modified when the processor is halted (S_HALT == 1). C_MASKINTS must be set or cleared before halt is released (i.e., the writes to set or clear C_MASKINTS and to set or clear C_HALT must be separate). Modifying C_MASKINTS while the system is running with halting debug support enabled (C_DEBUGEN = 1, S_HALT = 0) may cause unpredictable behavior."]
pub type CMaskintsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED4` reader - 4:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `RESERVED4` writer - 4:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C_SNAPSTALL` reader - 5:5\\]
If the core is stalled on a load/store operation the stall ceases and the instruction is forced to complete. This enables Halting debug to gain control of the core. It can only be set if: C_DEBUGEN = 1 and C_HALT = 1. The core reads S_RETIRE_ST as 0. This indicates that no instruction has advanced. This prevents misuse. The bus state is Unpredictable when this is used. S_RETIRE_ST can detect core stalls on load/store operations."]
pub type CSnapstallR = crate::BitReader;
#[doc = "Field `C_SNAPSTALL` writer - 5:5\\]
If the core is stalled on a load/store operation the stall ceases and the instruction is forced to complete. This enables Halting debug to gain control of the core. It can only be set if: C_DEBUGEN = 1 and C_HALT = 1. The core reads S_RETIRE_ST as 0. This indicates that no instruction has advanced. This prevents misuse. The bus state is Unpredictable when this is used. S_RETIRE_ST can detect core stalls on load/store operations."]
pub type CSnapstallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED6` reader - 15:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved6R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED6` writer - 15:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved6W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `S_REGRDY` reader - 16:16\\]
Register Read/Write on the Debug Core Register Selector register is available. Last transfer is complete. When writing to this register, 1 must be written this bit-field, otherwise the write operation is ignored and no bits are written into the register."]
pub type SRegrdyR = crate::BitReader;
#[doc = "Field `S_REGRDY` writer - 16:16\\]
Register Read/Write on the Debug Core Register Selector register is available. Last transfer is complete. When writing to this register, 1 must be written this bit-field, otherwise the write operation is ignored and no bits are written into the register."]
pub type SRegrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S_HALT` reader - 17:17\\]
The core is in debug state when this bit is set. When writing to this register, 1 must be written this bit-field, otherwise the write operation is ignored and no bits are written into the register."]
pub type SHaltR = crate::BitReader;
#[doc = "Field `S_HALT` writer - 17:17\\]
The core is in debug state when this bit is set. When writing to this register, 1 must be written this bit-field, otherwise the write operation is ignored and no bits are written into the register."]
pub type SHaltW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S_SLEEP` reader - 18:18\\]
Indicates that the core is sleeping (WFI, WFE, or **SLEEP-ON-EXIT**). Must use C_HALT to gain control or wait for interrupt to wake-up. When writing to this register, 1 must be written this bit-field, otherwise the write operation is ignored and no bits are written into the register."]
pub type SSleepR = crate::BitReader;
#[doc = "Field `S_SLEEP` writer - 18:18\\]
Indicates that the core is sleeping (WFI, WFE, or **SLEEP-ON-EXIT**). Must use C_HALT to gain control or wait for interrupt to wake-up. When writing to this register, 1 must be written this bit-field, otherwise the write operation is ignored and no bits are written into the register."]
pub type SSleepW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S_LOCKUP` reader - 19:19\\]
Reads as one if the core is running (not halted) and a lockup condition is present. When writing to this register, 1 must be written this bit-field, otherwise the write operation is ignored and no bits are written into the register."]
pub type SLockupR = crate::BitReader;
#[doc = "Field `S_LOCKUP` writer - 19:19\\]
Reads as one if the core is running (not halted) and a lockup condition is present. When writing to this register, 1 must be written this bit-field, otherwise the write operation is ignored and no bits are written into the register."]
pub type SLockupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED20` reader - 23:20\\]
Software should not rely on the value of a reserved. When writing to this register, 0x5 must be written this bit-field, otherwise the write operation is ignored and no bits are written into the register."]
pub type Reserved20R = crate::FieldReader;
#[doc = "Field `RESERVED20` writer - 23:20\\]
Software should not rely on the value of a reserved. When writing to this register, 0x5 must be written this bit-field, otherwise the write operation is ignored and no bits are written into the register."]
pub type Reserved20W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `S_RETIRE_ST` reader - 24:24\\]
Indicates that an instruction has completed since last read. This is a sticky bit that clears on read. This determines if the core is stalled on a load/store or fetch. When writing to this register, 0 must be written this bit-field, otherwise the write operation is ignored and no bits are written into the register."]
pub type SRetireStR = crate::BitReader;
#[doc = "Field `S_RETIRE_ST` writer - 24:24\\]
Indicates that an instruction has completed since last read. This is a sticky bit that clears on read. This determines if the core is stalled on a load/store or fetch. When writing to this register, 0 must be written this bit-field, otherwise the write operation is ignored and no bits are written into the register."]
pub type SRetireStW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S_RESET_ST` reader - 25:25\\]
Indicates that the core has been reset, or is now being reset, since the last time this bit was read. This a sticky bit that clears on read. So, reading twice and getting 1 then 0 means it was reset in the past. Reading twice and getting 1 both times means that it is being reset now (held in reset still). When writing to this register, 0 must be written this bit-field, otherwise the write operation is ignored and no bits are written into the register."]
pub type SResetStR = crate::BitReader;
#[doc = "Field `S_RESET_ST` writer - 25:25\\]
Indicates that the core has been reset, or is now being reset, since the last time this bit was read. This a sticky bit that clears on read. So, reading twice and getting 1 then 0 means it was reset in the past. Reading twice and getting 1 both times means that it is being reset now (held in reset still). When writing to this register, 0 must be written this bit-field, otherwise the write operation is ignored and no bits are written into the register."]
pub type SResetStW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED26` reader - 31:26\\]
Software should not rely on the value of a reserved. When writing to this register, 0x28 must be written this bit-field, otherwise the write operation is ignored and no bits are written into the register."]
pub type Reserved26R = crate::FieldReader;
#[doc = "Field `RESERVED26` writer - 31:26\\]
Software should not rely on the value of a reserved. When writing to this register, 0x28 must be written this bit-field, otherwise the write operation is ignored and no bits are written into the register."]
pub type Reserved26W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Enables debug. This can only be written by AHB-AP and not by the core. It is ignored when written by the core, which cannot set or clear it. The core must write a 1 to it when writing C_HALT to halt itself. The values of C_HALT, C_STEP and C_MASKINTS are ignored by hardware when C_DEBUGEN = 0. The read values for C_HALT, C_STEP and C_MASKINTS fields will be unknown to software when C_DEBUGEN = 0."]
    #[inline(always)]
    pub fn c_debugen(&self) -> CDebugenR {
        CDebugenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Halts the core. This bit is set automatically when the core Halts. For example Breakpoint. This bit clears on core reset."]
    #[inline(always)]
    pub fn c_halt(&self) -> CHaltR {
        CHaltR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Steps the core in halted debug. When C_DEBUGEN = 0, this bit has no effect. Must only be modified when the processor is halted (S_HALT == 1). Modifying C_STEP while the system is running with halting debug support enabled (C_DEBUGEN = 1, S_HALT = 0) may cause unpredictable behavior."]
    #[inline(always)]
    pub fn c_step(&self) -> CStepR {
        CStepR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Mask interrupts when stepping or running in halted debug. This masking does not affect NMI, fault exceptions and SVC caused by execution of the instructions. This bit must only be modified when the processor is halted (S_HALT == 1). C_MASKINTS must be set or cleared before halt is released (i.e., the writes to set or clear C_MASKINTS and to set or clear C_HALT must be separate). Modifying C_MASKINTS while the system is running with halting debug support enabled (C_DEBUGEN = 1, S_HALT = 0) may cause unpredictable behavior."]
    #[inline(always)]
    pub fn c_maskints(&self) -> CMaskintsR {
        CMaskintsR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
If the core is stalled on a load/store operation the stall ceases and the instruction is forced to complete. This enables Halting debug to gain control of the core. It can only be set if: C_DEBUGEN = 1 and C_HALT = 1. The core reads S_RETIRE_ST as 0. This indicates that no instruction has advanced. This prevents misuse. The bus state is Unpredictable when this is used. S_RETIRE_ST can detect core stalls on load/store operations."]
    #[inline(always)]
    pub fn c_snapstall(&self) -> CSnapstallR {
        CSnapstallR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:15 - 15:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new(((self.bits >> 6) & 0x03ff) as u16)
    }
    #[doc = "Bit 16 - 16:16\\]
Register Read/Write on the Debug Core Register Selector register is available. Last transfer is complete. When writing to this register, 1 must be written this bit-field, otherwise the write operation is ignored and no bits are written into the register."]
    #[inline(always)]
    pub fn s_regrdy(&self) -> SRegrdyR {
        SRegrdyR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
The core is in debug state when this bit is set. When writing to this register, 1 must be written this bit-field, otherwise the write operation is ignored and no bits are written into the register."]
    #[inline(always)]
    pub fn s_halt(&self) -> SHaltR {
        SHaltR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Indicates that the core is sleeping (WFI, WFE, or **SLEEP-ON-EXIT**). Must use C_HALT to gain control or wait for interrupt to wake-up. When writing to this register, 1 must be written this bit-field, otherwise the write operation is ignored and no bits are written into the register."]
    #[inline(always)]
    pub fn s_sleep(&self) -> SSleepR {
        SSleepR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
Reads as one if the core is running (not halted) and a lockup condition is present. When writing to this register, 1 must be written this bit-field, otherwise the write operation is ignored and no bits are written into the register."]
    #[inline(always)]
    pub fn s_lockup(&self) -> SLockupR {
        SLockupR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Software should not rely on the value of a reserved. When writing to this register, 0x5 must be written this bit-field, otherwise the write operation is ignored and no bits are written into the register."]
    #[inline(always)]
    pub fn reserved20(&self) -> Reserved20R {
        Reserved20R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - 24:24\\]
Indicates that an instruction has completed since last read. This is a sticky bit that clears on read. This determines if the core is stalled on a load/store or fetch. When writing to this register, 0 must be written this bit-field, otherwise the write operation is ignored and no bits are written into the register."]
    #[inline(always)]
    pub fn s_retire_st(&self) -> SRetireStR {
        SRetireStR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
Indicates that the core has been reset, or is now being reset, since the last time this bit was read. This a sticky bit that clears on read. So, reading twice and getting 1 then 0 means it was reset in the past. Reading twice and getting 1 both times means that it is being reset now (held in reset still). When writing to this register, 0 must be written this bit-field, otherwise the write operation is ignored and no bits are written into the register."]
    #[inline(always)]
    pub fn s_reset_st(&self) -> SResetStR {
        SResetStR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:31 - 31:26\\]
Software should not rely on the value of a reserved. When writing to this register, 0x28 must be written this bit-field, otherwise the write operation is ignored and no bits are written into the register."]
    #[inline(always)]
    pub fn reserved26(&self) -> Reserved26R {
        Reserved26R::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Enables debug. This can only be written by AHB-AP and not by the core. It is ignored when written by the core, which cannot set or clear it. The core must write a 1 to it when writing C_HALT to halt itself. The values of C_HALT, C_STEP and C_MASKINTS are ignored by hardware when C_DEBUGEN = 0. The read values for C_HALT, C_STEP and C_MASKINTS fields will be unknown to software when C_DEBUGEN = 0."]
    #[inline(always)]
    #[must_use]
    pub fn c_debugen(&mut self) -> CDebugenW<DhcsrSpec> {
        CDebugenW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Halts the core. This bit is set automatically when the core Halts. For example Breakpoint. This bit clears on core reset."]
    #[inline(always)]
    #[must_use]
    pub fn c_halt(&mut self) -> CHaltW<DhcsrSpec> {
        CHaltW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Steps the core in halted debug. When C_DEBUGEN = 0, this bit has no effect. Must only be modified when the processor is halted (S_HALT == 1). Modifying C_STEP while the system is running with halting debug support enabled (C_DEBUGEN = 1, S_HALT = 0) may cause unpredictable behavior."]
    #[inline(always)]
    #[must_use]
    pub fn c_step(&mut self) -> CStepW<DhcsrSpec> {
        CStepW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Mask interrupts when stepping or running in halted debug. This masking does not affect NMI, fault exceptions and SVC caused by execution of the instructions. This bit must only be modified when the processor is halted (S_HALT == 1). C_MASKINTS must be set or cleared before halt is released (i.e., the writes to set or clear C_MASKINTS and to set or clear C_HALT must be separate). Modifying C_MASKINTS while the system is running with halting debug support enabled (C_DEBUGEN = 1, S_HALT = 0) may cause unpredictable behavior."]
    #[inline(always)]
    #[must_use]
    pub fn c_maskints(&mut self) -> CMaskintsW<DhcsrSpec> {
        CMaskintsW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved4(&mut self) -> Reserved4W<DhcsrSpec> {
        Reserved4W::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
If the core is stalled on a load/store operation the stall ceases and the instruction is forced to complete. This enables Halting debug to gain control of the core. It can only be set if: C_DEBUGEN = 1 and C_HALT = 1. The core reads S_RETIRE_ST as 0. This indicates that no instruction has advanced. This prevents misuse. The bus state is Unpredictable when this is used. S_RETIRE_ST can detect core stalls on load/store operations."]
    #[inline(always)]
    #[must_use]
    pub fn c_snapstall(&mut self) -> CSnapstallW<DhcsrSpec> {
        CSnapstallW::new(self, 5)
    }
    #[doc = "Bits 6:15 - 15:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved6(&mut self) -> Reserved6W<DhcsrSpec> {
        Reserved6W::new(self, 6)
    }
    #[doc = "Bit 16 - 16:16\\]
Register Read/Write on the Debug Core Register Selector register is available. Last transfer is complete. When writing to this register, 1 must be written this bit-field, otherwise the write operation is ignored and no bits are written into the register."]
    #[inline(always)]
    #[must_use]
    pub fn s_regrdy(&mut self) -> SRegrdyW<DhcsrSpec> {
        SRegrdyW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
The core is in debug state when this bit is set. When writing to this register, 1 must be written this bit-field, otherwise the write operation is ignored and no bits are written into the register."]
    #[inline(always)]
    #[must_use]
    pub fn s_halt(&mut self) -> SHaltW<DhcsrSpec> {
        SHaltW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
Indicates that the core is sleeping (WFI, WFE, or **SLEEP-ON-EXIT**). Must use C_HALT to gain control or wait for interrupt to wake-up. When writing to this register, 1 must be written this bit-field, otherwise the write operation is ignored and no bits are written into the register."]
    #[inline(always)]
    #[must_use]
    pub fn s_sleep(&mut self) -> SSleepW<DhcsrSpec> {
        SSleepW::new(self, 18)
    }
    #[doc = "Bit 19 - 19:19\\]
Reads as one if the core is running (not halted) and a lockup condition is present. When writing to this register, 1 must be written this bit-field, otherwise the write operation is ignored and no bits are written into the register."]
    #[inline(always)]
    #[must_use]
    pub fn s_lockup(&mut self) -> SLockupW<DhcsrSpec> {
        SLockupW::new(self, 19)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Software should not rely on the value of a reserved. When writing to this register, 0x5 must be written this bit-field, otherwise the write operation is ignored and no bits are written into the register."]
    #[inline(always)]
    #[must_use]
    pub fn reserved20(&mut self) -> Reserved20W<DhcsrSpec> {
        Reserved20W::new(self, 20)
    }
    #[doc = "Bit 24 - 24:24\\]
Indicates that an instruction has completed since last read. This is a sticky bit that clears on read. This determines if the core is stalled on a load/store or fetch. When writing to this register, 0 must be written this bit-field, otherwise the write operation is ignored and no bits are written into the register."]
    #[inline(always)]
    #[must_use]
    pub fn s_retire_st(&mut self) -> SRetireStW<DhcsrSpec> {
        SRetireStW::new(self, 24)
    }
    #[doc = "Bit 25 - 25:25\\]
Indicates that the core has been reset, or is now being reset, since the last time this bit was read. This a sticky bit that clears on read. So, reading twice and getting 1 then 0 means it was reset in the past. Reading twice and getting 1 both times means that it is being reset now (held in reset still). When writing to this register, 0 must be written this bit-field, otherwise the write operation is ignored and no bits are written into the register."]
    #[inline(always)]
    #[must_use]
    pub fn s_reset_st(&mut self) -> SResetStW<DhcsrSpec> {
        SResetStW::new(self, 25)
    }
    #[doc = "Bits 26:31 - 31:26\\]
Software should not rely on the value of a reserved. When writing to this register, 0x28 must be written this bit-field, otherwise the write operation is ignored and no bits are written into the register."]
    #[inline(always)]
    #[must_use]
    pub fn reserved26(&mut self) -> Reserved26W<DhcsrSpec> {
        Reserved26W::new(self, 26)
    }
}
#[doc = "Debug Halting Control and Status The purpose of this register is to provide status information about the state of the processor, enable core debug, halt and step the processor. For writes, 0xA05F must be written to higher half-word of this register, otherwise the write operation is ignored and no bits are written into the register. If not enabled for Halting mode, C_DEBUGEN = 1, all other fields are disabled. This register is not reset on a core reset. It is reset by a power-on reset. However, C_HALT always clears on a core reset. To halt on a reset, the following bits must be enabled: DEMCR.VC_CORERESET and C_DEBUGEN. Note that writes to this register in any size other than word are unpredictable. It is acceptable to read in any size, and it can be used to avoid or intentionally change a sticky bit. Behavior of the system when writing to this register while CPU is halted (i.e. C_DEBUGEN = 1 and S_HALT= 1): C_HALT=0, C_STEP=0, C_MASKINTS=0 Exit Debug state and start instruction execution. Exceptions activate according to the exception configuration rules. C_HALT=0, C_STEP=0, C_MASKINTS=1 Exit Debug state and start instruction execution. PendSV, SysTick and external configurable interrupts are disabled, otherwise exceptions activate according to standard configuration rules. C_HALT=0, C_STEP=1, C_MASKINTS=0 Exit Debug state, step an instruction and halt. Exceptions activate according to the exception configuration rules. C_HALT=0, C_STEP=1, C_MASKINTS=1 Exit Debug state, step an instruction and halt. PendSV, SysTick and external configurable interrupts are disabled, otherwise exceptions activate according to standard configuration rules. C_HALT=1, C_STEP=x, C_MASKINTS=x Remain in Debug state\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dhcsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dhcsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DhcsrSpec;
impl crate::RegisterSpec for DhcsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dhcsr::R`](R) reader structure"]
impl crate::Readable for DhcsrSpec {}
#[doc = "`write(|w| ..)` method takes [`dhcsr::W`](W) writer structure"]
impl crate::Writable for DhcsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DHCSR to value 0"]
impl crate::Resettable for DhcsrSpec {
    const RESET_VALUE: u32 = 0;
}
