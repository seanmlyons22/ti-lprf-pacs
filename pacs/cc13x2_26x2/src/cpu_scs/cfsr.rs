#[doc = "Register `CFSR` reader"]
pub type R = crate::R<CfsrSpec>;
#[doc = "Register `CFSR` writer"]
pub type W = crate::W<CfsrSpec>;
#[doc = "Field `IACCVIOL` reader - 0:0\\]
Instruction access violation flag. Attempting to fetch an instruction from a location that does not permit execution sets this flag. This occurs on any access to an XN region, even when the MPU is disabled or not present. The return PC points to the faulting instruction. MMFAR is not written."]
pub type IaccviolR = crate::BitReader;
#[doc = "Field `IACCVIOL` writer - 0:0\\]
Instruction access violation flag. Attempting to fetch an instruction from a location that does not permit execution sets this flag. This occurs on any access to an XN region, even when the MPU is disabled or not present. The return PC points to the faulting instruction. MMFAR is not written."]
pub type IaccviolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DACCVIOL` reader - 1:1\\]
Data access violation flag. Attempting to load or store at a location that does not permit the operation sets this flag. The return PC points to the faulting instruction. This error loads MMFAR with the address of the attempted access."]
pub type DaccviolR = crate::BitReader;
#[doc = "Field `DACCVIOL` writer - 1:1\\]
Data access violation flag. Attempting to load or store at a location that does not permit the operation sets this flag. The return PC points to the faulting instruction. This error loads MMFAR with the address of the attempted access."]
pub type DaccviolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED2` reader - 2:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `RESERVED2` writer - 2:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MUNSTKERR` reader - 3:3\\]
Unstack from exception return has caused one or more access violations. This is chained to the handler, so that the original return stack is still present. SP is not adjusted from failing return and new save is not performed. MMFAR is not written."]
pub type MunstkerrR = crate::BitReader;
#[doc = "Field `MUNSTKERR` writer - 3:3\\]
Unstack from exception return has caused one or more access violations. This is chained to the handler, so that the original return stack is still present. SP is not adjusted from failing return and new save is not performed. MMFAR is not written."]
pub type MunstkerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSTKERR` reader - 4:4\\]
Stacking from exception has caused one or more access violations. The SP is still adjusted and the values in the context area on the stack might be incorrect. MMFAR is not written."]
pub type MstkerrR = crate::BitReader;
#[doc = "Field `MSTKERR` writer - 4:4\\]
Stacking from exception has caused one or more access violations. The SP is still adjusted and the values in the context area on the stack might be incorrect. MMFAR is not written."]
pub type MstkerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED5` reader - 6:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved5R = crate::FieldReader;
#[doc = "Field `RESERVED5` writer - 6:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved5W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MMARVALID` reader - 7:7\\]
Memory Manage Address Register (MMFAR) address valid flag. A later-arriving fault, such as a bus fault, can clear a memory manage fault.. If a MemManage fault occurs that is escalated to a Hard Fault because of priority, the Hard Fault handler must clear this bit. This prevents problems on return to a stacked active MemManage handler whose MMFAR value has been overwritten."]
pub type MmarvalidR = crate::BitReader;
#[doc = "Field `MMARVALID` writer - 7:7\\]
Memory Manage Address Register (MMFAR) address valid flag. A later-arriving fault, such as a bus fault, can clear a memory manage fault.. If a MemManage fault occurs that is escalated to a Hard Fault because of priority, the Hard Fault handler must clear this bit. This prevents problems on return to a stacked active MemManage handler whose MMFAR value has been overwritten."]
pub type MmarvalidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IBUSERR` reader - 8:8\\]
Instruction bus error flag. This flag is set by a prefetch error. The fault stops on the instruction, so if the error occurs under a branch shadow, no fault occurs. BFAR is not written."]
pub type IbuserrR = crate::BitReader;
#[doc = "Field `IBUSERR` writer - 8:8\\]
Instruction bus error flag. This flag is set by a prefetch error. The fault stops on the instruction, so if the error occurs under a branch shadow, no fault occurs. BFAR is not written."]
pub type IbuserrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRECISERR` reader - 9:9\\]
Precise data bus error return."]
pub type PreciserrR = crate::BitReader;
#[doc = "Field `PRECISERR` writer - 9:9\\]
Precise data bus error return."]
pub type PreciserrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMPRECISERR` reader - 10:10\\]
Imprecise data bus error. It is a BusFault, but the Return PC is not related to the causing instruction. This is not a synchronous fault. So, if detected when the priority of the current activation is higher than the Bus Fault, it only pends. Bus fault activates when returning to a lower priority activation. If a precise fault occurs before returning to a lower priority exception, the handler detects both IMPRECISERR set and one of the precise fault status bits set at the same time. BFAR is not written."]
pub type ImpreciserrR = crate::BitReader;
#[doc = "Field `IMPRECISERR` writer - 10:10\\]
Imprecise data bus error. It is a BusFault, but the Return PC is not related to the causing instruction. This is not a synchronous fault. So, if detected when the priority of the current activation is higher than the Bus Fault, it only pends. Bus fault activates when returning to a lower priority activation. If a precise fault occurs before returning to a lower priority exception, the handler detects both IMPRECISERR set and one of the precise fault status bits set at the same time. BFAR is not written."]
pub type ImpreciserrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNSTKERR` reader - 11:11\\]
Unstack from exception return has caused one or more bus faults. This is chained to the handler, so that the original return stack is still present. SP is not adjusted from failing return and new save is not performed. BFAR is not written."]
pub type UnstkerrR = crate::BitReader;
#[doc = "Field `UNSTKERR` writer - 11:11\\]
Unstack from exception return has caused one or more bus faults. This is chained to the handler, so that the original return stack is still present. SP is not adjusted from failing return and new save is not performed. BFAR is not written."]
pub type UnstkerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STKERR` reader - 12:12\\]
Stacking from exception has caused one or more bus faults. The SP is still adjusted and the values in the context area on the stack might be incorrect. BFAR is not written."]
pub type StkerrR = crate::BitReader;
#[doc = "Field `STKERR` writer - 12:12\\]
Stacking from exception has caused one or more bus faults. The SP is still adjusted and the values in the context area on the stack might be incorrect. BFAR is not written."]
pub type StkerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED13` reader - 14:13\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved13R = crate::FieldReader;
#[doc = "Field `RESERVED13` writer - 14:13\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved13W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BFARVALID` reader - 15:15\\]
This bit is set if the Bus Fault Address Register (BFAR) contains a valid address. This is true after a bus fault where the address is known. Other faults can clear this bit, such as a Mem Manage fault occurring later. If a Bus fault occurs that is escalated to a Hard Fault because of priority, the Hard Fault handler must clear this bit. This prevents problems if returning to a stacked active Bus fault handler whose BFAR value has been overwritten."]
pub type BfarvalidR = crate::BitReader;
#[doc = "Field `BFARVALID` writer - 15:15\\]
This bit is set if the Bus Fault Address Register (BFAR) contains a valid address. This is true after a bus fault where the address is known. Other faults can clear this bit, such as a Mem Manage fault occurring later. If a Bus fault occurs that is escalated to a Hard Fault because of priority, the Hard Fault handler must clear this bit. This prevents problems if returning to a stacked active Bus fault handler whose BFAR value has been overwritten."]
pub type BfarvalidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNDEFINSTR` reader - 16:16\\]
This bit is set when the processor attempts to execute an undefined instruction. This is an instruction that the processor cannot decode. The return PC points to the undefined instruction."]
pub type UndefinstrR = crate::BitReader;
#[doc = "Field `UNDEFINSTR` writer - 16:16\\]
This bit is set when the processor attempts to execute an undefined instruction. This is an instruction that the processor cannot decode. The return PC points to the undefined instruction."]
pub type UndefinstrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INVSTATE` reader - 17:17\\]
Indicates an attempt to execute in an invalid EPSR state (e.g. after a BX type instruction has changed state). This includes state change after entry to or return from exception, as well as from inter-working instructions. Return PC points to faulting instruction, with the invalid state."]
pub type InvstateR = crate::BitReader;
#[doc = "Field `INVSTATE` writer - 17:17\\]
Indicates an attempt to execute in an invalid EPSR state (e.g. after a BX type instruction has changed state). This includes state change after entry to or return from exception, as well as from inter-working instructions. Return PC points to faulting instruction, with the invalid state."]
pub type InvstateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INVPC` reader - 18:18\\]
Attempt to load EXC_RETURN into PC illegally. Invalid instruction, invalid context, invalid value. The return PC points to the instruction that tried to set the PC."]
pub type InvpcR = crate::BitReader;
#[doc = "Field `INVPC` writer - 18:18\\]
Attempt to load EXC_RETURN into PC illegally. Invalid instruction, invalid context, invalid value. The return PC points to the instruction that tried to set the PC."]
pub type InvpcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOCP` reader - 19:19\\]
Attempt to use a coprocessor instruction. The processor does not support coprocessor instructions."]
pub type NocpR = crate::BitReader;
#[doc = "Field `NOCP` writer - 19:19\\]
Attempt to use a coprocessor instruction. The processor does not support coprocessor instructions."]
pub type NocpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED20` reader - 23:20\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved20R = crate::FieldReader;
#[doc = "Field `RESERVED20` writer - 23:20\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved20W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `UNALIGNED` reader - 24:24\\]
When CCR.UNALIGN_TRP is enabled, and there is an attempt to make an unaligned memory access, then this fault occurs. Unaligned LDM/STM/LDRD/STRD instructions always fault irrespective of the setting of CCR.UNALIGN_TRP."]
pub type UnalignedR = crate::BitReader;
#[doc = "Field `UNALIGNED` writer - 24:24\\]
When CCR.UNALIGN_TRP is enabled, and there is an attempt to make an unaligned memory access, then this fault occurs. Unaligned LDM/STM/LDRD/STRD instructions always fault irrespective of the setting of CCR.UNALIGN_TRP."]
pub type UnalignedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIVBYZERO` reader - 25:25\\]
When CCR.DIV_0_TRP (see Configuration Control Register on page 8-26) is enabled and an SDIV or UDIV instruction is used with a divisor of 0, this fault occurs The instruction is executed and the return PC points to it. If CCR.DIV_0_TRP is not set, then the divide returns a quotient of 0."]
pub type DivbyzeroR = crate::BitReader;
#[doc = "Field `DIVBYZERO` writer - 25:25\\]
When CCR.DIV_0_TRP (see Configuration Control Register on page 8-26) is enabled and an SDIV or UDIV instruction is used with a divisor of 0, this fault occurs The instruction is executed and the return PC points to it. If CCR.DIV_0_TRP is not set, then the divide returns a quotient of 0."]
pub type DivbyzeroW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED26` reader - 31:26\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved26R = crate::FieldReader;
#[doc = "Field `RESERVED26` writer - 31:26\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved26W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Instruction access violation flag. Attempting to fetch an instruction from a location that does not permit execution sets this flag. This occurs on any access to an XN region, even when the MPU is disabled or not present. The return PC points to the faulting instruction. MMFAR is not written."]
    #[inline(always)]
    pub fn iaccviol(&self) -> IaccviolR {
        IaccviolR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Data access violation flag. Attempting to load or store at a location that does not permit the operation sets this flag. The return PC points to the faulting instruction. This error loads MMFAR with the address of the attempted access."]
    #[inline(always)]
    pub fn daccviol(&self) -> DaccviolR {
        DaccviolR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Unstack from exception return has caused one or more access violations. This is chained to the handler, so that the original return stack is still present. SP is not adjusted from failing return and new save is not performed. MMFAR is not written."]
    #[inline(always)]
    pub fn munstkerr(&self) -> MunstkerrR {
        MunstkerrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Stacking from exception has caused one or more access violations. The SP is still adjusted and the values in the context area on the stack might be incorrect. MMFAR is not written."]
    #[inline(always)]
    pub fn mstkerr(&self) -> MstkerrR {
        MstkerrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - 6:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - 7:7\\]
Memory Manage Address Register (MMFAR) address valid flag. A later-arriving fault, such as a bus fault, can clear a memory manage fault.. If a MemManage fault occurs that is escalated to a Hard Fault because of priority, the Hard Fault handler must clear this bit. This prevents problems on return to a stacked active MemManage handler whose MMFAR value has been overwritten."]
    #[inline(always)]
    pub fn mmarvalid(&self) -> MmarvalidR {
        MmarvalidR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Instruction bus error flag. This flag is set by a prefetch error. The fault stops on the instruction, so if the error occurs under a branch shadow, no fault occurs. BFAR is not written."]
    #[inline(always)]
    pub fn ibuserr(&self) -> IbuserrR {
        IbuserrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Precise data bus error return."]
    #[inline(always)]
    pub fn preciserr(&self) -> PreciserrR {
        PreciserrR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Imprecise data bus error. It is a BusFault, but the Return PC is not related to the causing instruction. This is not a synchronous fault. So, if detected when the priority of the current activation is higher than the Bus Fault, it only pends. Bus fault activates when returning to a lower priority activation. If a precise fault occurs before returning to a lower priority exception, the handler detects both IMPRECISERR set and one of the precise fault status bits set at the same time. BFAR is not written."]
    #[inline(always)]
    pub fn impreciserr(&self) -> ImpreciserrR {
        ImpreciserrR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Unstack from exception return has caused one or more bus faults. This is chained to the handler, so that the original return stack is still present. SP is not adjusted from failing return and new save is not performed. BFAR is not written."]
    #[inline(always)]
    pub fn unstkerr(&self) -> UnstkerrR {
        UnstkerrR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Stacking from exception has caused one or more bus faults. The SP is still adjusted and the values in the context area on the stack might be incorrect. BFAR is not written."]
    #[inline(always)]
    pub fn stkerr(&self) -> StkerrR {
        StkerrR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:14 - 14:13\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved13(&self) -> Reserved13R {
        Reserved13R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15 - 15:15\\]
This bit is set if the Bus Fault Address Register (BFAR) contains a valid address. This is true after a bus fault where the address is known. Other faults can clear this bit, such as a Mem Manage fault occurring later. If a Bus fault occurs that is escalated to a Hard Fault because of priority, the Hard Fault handler must clear this bit. This prevents problems if returning to a stacked active Bus fault handler whose BFAR value has been overwritten."]
    #[inline(always)]
    pub fn bfarvalid(&self) -> BfarvalidR {
        BfarvalidR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
This bit is set when the processor attempts to execute an undefined instruction. This is an instruction that the processor cannot decode. The return PC points to the undefined instruction."]
    #[inline(always)]
    pub fn undefinstr(&self) -> UndefinstrR {
        UndefinstrR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Indicates an attempt to execute in an invalid EPSR state (e.g. after a BX type instruction has changed state). This includes state change after entry to or return from exception, as well as from inter-working instructions. Return PC points to faulting instruction, with the invalid state."]
    #[inline(always)]
    pub fn invstate(&self) -> InvstateR {
        InvstateR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Attempt to load EXC_RETURN into PC illegally. Invalid instruction, invalid context, invalid value. The return PC points to the instruction that tried to set the PC."]
    #[inline(always)]
    pub fn invpc(&self) -> InvpcR {
        InvpcR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
Attempt to use a coprocessor instruction. The processor does not support coprocessor instructions."]
    #[inline(always)]
    pub fn nocp(&self) -> NocpR {
        NocpR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved20(&self) -> Reserved20R {
        Reserved20R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - 24:24\\]
When CCR.UNALIGN_TRP is enabled, and there is an attempt to make an unaligned memory access, then this fault occurs. Unaligned LDM/STM/LDRD/STRD instructions always fault irrespective of the setting of CCR.UNALIGN_TRP."]
    #[inline(always)]
    pub fn unaligned(&self) -> UnalignedR {
        UnalignedR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
When CCR.DIV_0_TRP (see Configuration Control Register on page 8-26) is enabled and an SDIV or UDIV instruction is used with a divisor of 0, this fault occurs The instruction is executed and the return PC points to it. If CCR.DIV_0_TRP is not set, then the divide returns a quotient of 0."]
    #[inline(always)]
    pub fn divbyzero(&self) -> DivbyzeroR {
        DivbyzeroR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:31 - 31:26\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved26(&self) -> Reserved26R {
        Reserved26R::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Instruction access violation flag. Attempting to fetch an instruction from a location that does not permit execution sets this flag. This occurs on any access to an XN region, even when the MPU is disabled or not present. The return PC points to the faulting instruction. MMFAR is not written."]
    #[inline(always)]
    #[must_use]
    pub fn iaccviol(&mut self) -> IaccviolW<CfsrSpec> {
        IaccviolW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Data access violation flag. Attempting to load or store at a location that does not permit the operation sets this flag. The return PC points to the faulting instruction. This error loads MMFAR with the address of the attempted access."]
    #[inline(always)]
    #[must_use]
    pub fn daccviol(&mut self) -> DaccviolW<CfsrSpec> {
        DaccviolW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> Reserved2W<CfsrSpec> {
        Reserved2W::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Unstack from exception return has caused one or more access violations. This is chained to the handler, so that the original return stack is still present. SP is not adjusted from failing return and new save is not performed. MMFAR is not written."]
    #[inline(always)]
    #[must_use]
    pub fn munstkerr(&mut self) -> MunstkerrW<CfsrSpec> {
        MunstkerrW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Stacking from exception has caused one or more access violations. The SP is still adjusted and the values in the context area on the stack might be incorrect. MMFAR is not written."]
    #[inline(always)]
    #[must_use]
    pub fn mstkerr(&mut self) -> MstkerrW<CfsrSpec> {
        MstkerrW::new(self, 4)
    }
    #[doc = "Bits 5:6 - 6:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved5(&mut self) -> Reserved5W<CfsrSpec> {
        Reserved5W::new(self, 5)
    }
    #[doc = "Bit 7 - 7:7\\]
Memory Manage Address Register (MMFAR) address valid flag. A later-arriving fault, such as a bus fault, can clear a memory manage fault.. If a MemManage fault occurs that is escalated to a Hard Fault because of priority, the Hard Fault handler must clear this bit. This prevents problems on return to a stacked active MemManage handler whose MMFAR value has been overwritten."]
    #[inline(always)]
    #[must_use]
    pub fn mmarvalid(&mut self) -> MmarvalidW<CfsrSpec> {
        MmarvalidW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Instruction bus error flag. This flag is set by a prefetch error. The fault stops on the instruction, so if the error occurs under a branch shadow, no fault occurs. BFAR is not written."]
    #[inline(always)]
    #[must_use]
    pub fn ibuserr(&mut self) -> IbuserrW<CfsrSpec> {
        IbuserrW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Precise data bus error return."]
    #[inline(always)]
    #[must_use]
    pub fn preciserr(&mut self) -> PreciserrW<CfsrSpec> {
        PreciserrW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
Imprecise data bus error. It is a BusFault, but the Return PC is not related to the causing instruction. This is not a synchronous fault. So, if detected when the priority of the current activation is higher than the Bus Fault, it only pends. Bus fault activates when returning to a lower priority activation. If a precise fault occurs before returning to a lower priority exception, the handler detects both IMPRECISERR set and one of the precise fault status bits set at the same time. BFAR is not written."]
    #[inline(always)]
    #[must_use]
    pub fn impreciserr(&mut self) -> ImpreciserrW<CfsrSpec> {
        ImpreciserrW::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
Unstack from exception return has caused one or more bus faults. This is chained to the handler, so that the original return stack is still present. SP is not adjusted from failing return and new save is not performed. BFAR is not written."]
    #[inline(always)]
    #[must_use]
    pub fn unstkerr(&mut self) -> UnstkerrW<CfsrSpec> {
        UnstkerrW::new(self, 11)
    }
    #[doc = "Bit 12 - 12:12\\]
Stacking from exception has caused one or more bus faults. The SP is still adjusted and the values in the context area on the stack might be incorrect. BFAR is not written."]
    #[inline(always)]
    #[must_use]
    pub fn stkerr(&mut self) -> StkerrW<CfsrSpec> {
        StkerrW::new(self, 12)
    }
    #[doc = "Bits 13:14 - 14:13\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved13(&mut self) -> Reserved13W<CfsrSpec> {
        Reserved13W::new(self, 13)
    }
    #[doc = "Bit 15 - 15:15\\]
This bit is set if the Bus Fault Address Register (BFAR) contains a valid address. This is true after a bus fault where the address is known. Other faults can clear this bit, such as a Mem Manage fault occurring later. If a Bus fault occurs that is escalated to a Hard Fault because of priority, the Hard Fault handler must clear this bit. This prevents problems if returning to a stacked active Bus fault handler whose BFAR value has been overwritten."]
    #[inline(always)]
    #[must_use]
    pub fn bfarvalid(&mut self) -> BfarvalidW<CfsrSpec> {
        BfarvalidW::new(self, 15)
    }
    #[doc = "Bit 16 - 16:16\\]
This bit is set when the processor attempts to execute an undefined instruction. This is an instruction that the processor cannot decode. The return PC points to the undefined instruction."]
    #[inline(always)]
    #[must_use]
    pub fn undefinstr(&mut self) -> UndefinstrW<CfsrSpec> {
        UndefinstrW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Indicates an attempt to execute in an invalid EPSR state (e.g. after a BX type instruction has changed state). This includes state change after entry to or return from exception, as well as from inter-working instructions. Return PC points to faulting instruction, with the invalid state."]
    #[inline(always)]
    #[must_use]
    pub fn invstate(&mut self) -> InvstateW<CfsrSpec> {
        InvstateW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
Attempt to load EXC_RETURN into PC illegally. Invalid instruction, invalid context, invalid value. The return PC points to the instruction that tried to set the PC."]
    #[inline(always)]
    #[must_use]
    pub fn invpc(&mut self) -> InvpcW<CfsrSpec> {
        InvpcW::new(self, 18)
    }
    #[doc = "Bit 19 - 19:19\\]
Attempt to use a coprocessor instruction. The processor does not support coprocessor instructions."]
    #[inline(always)]
    #[must_use]
    pub fn nocp(&mut self) -> NocpW<CfsrSpec> {
        NocpW::new(self, 19)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved20(&mut self) -> Reserved20W<CfsrSpec> {
        Reserved20W::new(self, 20)
    }
    #[doc = "Bit 24 - 24:24\\]
When CCR.UNALIGN_TRP is enabled, and there is an attempt to make an unaligned memory access, then this fault occurs. Unaligned LDM/STM/LDRD/STRD instructions always fault irrespective of the setting of CCR.UNALIGN_TRP."]
    #[inline(always)]
    #[must_use]
    pub fn unaligned(&mut self) -> UnalignedW<CfsrSpec> {
        UnalignedW::new(self, 24)
    }
    #[doc = "Bit 25 - 25:25\\]
When CCR.DIV_0_TRP (see Configuration Control Register on page 8-26) is enabled and an SDIV or UDIV instruction is used with a divisor of 0, this fault occurs The instruction is executed and the return PC points to it. If CCR.DIV_0_TRP is not set, then the divide returns a quotient of 0."]
    #[inline(always)]
    #[must_use]
    pub fn divbyzero(&mut self) -> DivbyzeroW<CfsrSpec> {
        DivbyzeroW::new(self, 25)
    }
    #[doc = "Bits 26:31 - 31:26\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved26(&mut self) -> Reserved26W<CfsrSpec> {
        Reserved26W::new(self, 26)
    }
}
#[doc = "Configurable Fault Status This register is used to obtain information about local faults. These registers include three subsections: The first byte is Memory Manage Fault Status Register (MMFSR). The second byte is Bus Fault Status Register (BFSR). The higher half-word is Usage Fault Status Register (UFSR). The flags in these registers indicate the causes of local faults. Multiple flags can be set if more than one fault occurs. These register are read/write-clear. This means that they can be read normally, but writing a 1 to any bit clears that bit. The CFSR is byte accessible. CFSR or its subregisters can be accessed as follows: The following accesses are possible to the CFSR register: - access the complete register with a word access to 0xE000ED28. - access the MMFSR with a byte access to 0xE000ED28 - access the MMFSR and BFSR with a halfword access to 0xE000ED28 - access the BFSR with a byte access to 0xE000ED29 - access the UFSR with a halfword access to 0xE000ED2A.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfsrSpec;
impl crate::RegisterSpec for CfsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfsr::R`](R) reader structure"]
impl crate::Readable for CfsrSpec {}
#[doc = "`write(|w| ..)` method takes [`cfsr::W`](W) writer structure"]
impl crate::Writable for CfsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFSR to value 0"]
impl crate::Resettable for CfsrSpec {
    const RESET_VALUE: u32 = 0;
}
