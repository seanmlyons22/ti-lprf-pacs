#[doc = "Register `CFSR` reader"]
pub struct R(crate::R<CFSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFSR` writer"]
pub struct W(crate::W<CFSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CFSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IACCVIOL` reader - 0:0\\]
Instruction access violation flag. Attempting to fetch an instruction from a location that does not permit execution sets this flag. This occurs on any access to an XN region, even when the MPU is disabled or not present. The return PC points to the faulting instruction. MMFAR is not written."]
pub type IACCVIOL_R = crate::BitReader<bool>;
#[doc = "Field `IACCVIOL` writer - 0:0\\]
Instruction access violation flag. Attempting to fetch an instruction from a location that does not permit execution sets this flag. This occurs on any access to an XN region, even when the MPU is disabled or not present. The return PC points to the faulting instruction. MMFAR is not written."]
pub type IACCVIOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFSR_SPEC, bool, O>;
#[doc = "Field `DACCVIOL` reader - 1:1\\]
Data access violation flag. Attempting to load or store at a location that does not permit the operation sets this flag. The return PC points to the faulting instruction. This error loads MMFAR with the address of the attempted access."]
pub type DACCVIOL_R = crate::BitReader<bool>;
#[doc = "Field `DACCVIOL` writer - 1:1\\]
Data access violation flag. Attempting to load or store at a location that does not permit the operation sets this flag. The return PC points to the faulting instruction. This error loads MMFAR with the address of the attempted access."]
pub type DACCVIOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFSR_SPEC, bool, O>;
#[doc = "Field `RESERVED2` reader - 2:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED2_R = crate::BitReader<bool>;
#[doc = "Field `RESERVED2` writer - 2:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFSR_SPEC, bool, O>;
#[doc = "Field `MUNSTKERR` reader - 3:3\\]
Unstack from exception return has caused one or more access violations. This is chained to the handler, so that the original return stack is still present. SP is not adjusted from failing return and new save is not performed. MMFAR is not written."]
pub type MUNSTKERR_R = crate::BitReader<bool>;
#[doc = "Field `MUNSTKERR` writer - 3:3\\]
Unstack from exception return has caused one or more access violations. This is chained to the handler, so that the original return stack is still present. SP is not adjusted from failing return and new save is not performed. MMFAR is not written."]
pub type MUNSTKERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFSR_SPEC, bool, O>;
#[doc = "Field `MSTKERR` reader - 4:4\\]
Stacking from exception has caused one or more access violations. The SP is still adjusted and the values in the context area on the stack might be incorrect. MMFAR is not written."]
pub type MSTKERR_R = crate::BitReader<bool>;
#[doc = "Field `MSTKERR` writer - 4:4\\]
Stacking from exception has caused one or more access violations. The SP is still adjusted and the values in the context area on the stack might be incorrect. MMFAR is not written."]
pub type MSTKERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFSR_SPEC, bool, O>;
#[doc = "Field `RESERVED5` reader - 6:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED5` writer - 6:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFSR_SPEC, u8, u8, 2, O>;
#[doc = "Field `MMARVALID` reader - 7:7\\]
Memory Manage Address Register (MMFAR) address valid flag. A later-arriving fault, such as a bus fault, can clear a memory manage fault.. If a MemManage fault occurs that is escalated to a Hard Fault because of priority, the Hard Fault handler must clear this bit. This prevents problems on return to a stacked active MemManage handler whose MMFAR value has been overwritten."]
pub type MMARVALID_R = crate::BitReader<bool>;
#[doc = "Field `MMARVALID` writer - 7:7\\]
Memory Manage Address Register (MMFAR) address valid flag. A later-arriving fault, such as a bus fault, can clear a memory manage fault.. If a MemManage fault occurs that is escalated to a Hard Fault because of priority, the Hard Fault handler must clear this bit. This prevents problems on return to a stacked active MemManage handler whose MMFAR value has been overwritten."]
pub type MMARVALID_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFSR_SPEC, bool, O>;
#[doc = "Field `IBUSERR` reader - 8:8\\]
Instruction bus error flag. This flag is set by a prefetch error. The fault stops on the instruction, so if the error occurs under a branch shadow, no fault occurs. BFAR is not written."]
pub type IBUSERR_R = crate::BitReader<bool>;
#[doc = "Field `IBUSERR` writer - 8:8\\]
Instruction bus error flag. This flag is set by a prefetch error. The fault stops on the instruction, so if the error occurs under a branch shadow, no fault occurs. BFAR is not written."]
pub type IBUSERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFSR_SPEC, bool, O>;
#[doc = "Field `PRECISERR` reader - 9:9\\]
Precise data bus error return."]
pub type PRECISERR_R = crate::BitReader<bool>;
#[doc = "Field `PRECISERR` writer - 9:9\\]
Precise data bus error return."]
pub type PRECISERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFSR_SPEC, bool, O>;
#[doc = "Field `IMPRECISERR` reader - 10:10\\]
Imprecise data bus error. It is a BusFault, but the Return PC is not related to the causing instruction. This is not a synchronous fault. So, if detected when the priority of the current activation is higher than the Bus Fault, it only pends. Bus fault activates when returning to a lower priority activation. If a precise fault occurs before returning to a lower priority exception, the handler detects both IMPRECISERR set and one of the precise fault status bits set at the same time. BFAR is not written."]
pub type IMPRECISERR_R = crate::BitReader<bool>;
#[doc = "Field `IMPRECISERR` writer - 10:10\\]
Imprecise data bus error. It is a BusFault, but the Return PC is not related to the causing instruction. This is not a synchronous fault. So, if detected when the priority of the current activation is higher than the Bus Fault, it only pends. Bus fault activates when returning to a lower priority activation. If a precise fault occurs before returning to a lower priority exception, the handler detects both IMPRECISERR set and one of the precise fault status bits set at the same time. BFAR is not written."]
pub type IMPRECISERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFSR_SPEC, bool, O>;
#[doc = "Field `UNSTKERR` reader - 11:11\\]
Unstack from exception return has caused one or more bus faults. This is chained to the handler, so that the original return stack is still present. SP is not adjusted from failing return and new save is not performed. BFAR is not written."]
pub type UNSTKERR_R = crate::BitReader<bool>;
#[doc = "Field `UNSTKERR` writer - 11:11\\]
Unstack from exception return has caused one or more bus faults. This is chained to the handler, so that the original return stack is still present. SP is not adjusted from failing return and new save is not performed. BFAR is not written."]
pub type UNSTKERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFSR_SPEC, bool, O>;
#[doc = "Field `STKERR` reader - 12:12\\]
Stacking from exception has caused one or more bus faults. The SP is still adjusted and the values in the context area on the stack might be incorrect. BFAR is not written."]
pub type STKERR_R = crate::BitReader<bool>;
#[doc = "Field `STKERR` writer - 12:12\\]
Stacking from exception has caused one or more bus faults. The SP is still adjusted and the values in the context area on the stack might be incorrect. BFAR is not written."]
pub type STKERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFSR_SPEC, bool, O>;
#[doc = "Field `RESERVED13` reader - 14:13\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED13_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED13` writer - 14:13\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED13_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFSR_SPEC, u8, u8, 2, O>;
#[doc = "Field `BFARVALID` reader - 15:15\\]
This bit is set if the Bus Fault Address Register (BFAR) contains a valid address. This is true after a bus fault where the address is known. Other faults can clear this bit, such as a Mem Manage fault occurring later. If a Bus fault occurs that is escalated to a Hard Fault because of priority, the Hard Fault handler must clear this bit. This prevents problems if returning to a stacked active Bus fault handler whose BFAR value has been overwritten."]
pub type BFARVALID_R = crate::BitReader<bool>;
#[doc = "Field `BFARVALID` writer - 15:15\\]
This bit is set if the Bus Fault Address Register (BFAR) contains a valid address. This is true after a bus fault where the address is known. Other faults can clear this bit, such as a Mem Manage fault occurring later. If a Bus fault occurs that is escalated to a Hard Fault because of priority, the Hard Fault handler must clear this bit. This prevents problems if returning to a stacked active Bus fault handler whose BFAR value has been overwritten."]
pub type BFARVALID_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFSR_SPEC, bool, O>;
#[doc = "Field `UNDEFINSTR` reader - 16:16\\]
This bit is set when the processor attempts to execute an undefined instruction. This is an instruction that the processor cannot decode. The return PC points to the undefined instruction."]
pub type UNDEFINSTR_R = crate::BitReader<bool>;
#[doc = "Field `UNDEFINSTR` writer - 16:16\\]
This bit is set when the processor attempts to execute an undefined instruction. This is an instruction that the processor cannot decode. The return PC points to the undefined instruction."]
pub type UNDEFINSTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFSR_SPEC, bool, O>;
#[doc = "Field `INVSTATE` reader - 17:17\\]
Indicates an attempt to execute in an invalid EPSR state (e.g. after a BX type instruction has changed state). This includes state change after entry to or return from exception, as well as from inter-working instructions. Return PC points to faulting instruction, with the invalid state."]
pub type INVSTATE_R = crate::BitReader<bool>;
#[doc = "Field `INVSTATE` writer - 17:17\\]
Indicates an attempt to execute in an invalid EPSR state (e.g. after a BX type instruction has changed state). This includes state change after entry to or return from exception, as well as from inter-working instructions. Return PC points to faulting instruction, with the invalid state."]
pub type INVSTATE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFSR_SPEC, bool, O>;
#[doc = "Field `INVPC` reader - 18:18\\]
Attempt to load EXC_RETURN into PC illegally. Invalid instruction, invalid context, invalid value. The return PC points to the instruction that tried to set the PC."]
pub type INVPC_R = crate::BitReader<bool>;
#[doc = "Field `INVPC` writer - 18:18\\]
Attempt to load EXC_RETURN into PC illegally. Invalid instruction, invalid context, invalid value. The return PC points to the instruction that tried to set the PC."]
pub type INVPC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFSR_SPEC, bool, O>;
#[doc = "Field `NOCP` reader - 19:19\\]
Attempt to use a coprocessor instruction. The processor does not support coprocessor instructions."]
pub type NOCP_R = crate::BitReader<bool>;
#[doc = "Field `NOCP` writer - 19:19\\]
Attempt to use a coprocessor instruction. The processor does not support coprocessor instructions."]
pub type NOCP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFSR_SPEC, bool, O>;
#[doc = "Field `RESERVED20` reader - 23:20\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED20_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED20` writer - 23:20\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED20_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFSR_SPEC, u8, u8, 4, O>;
#[doc = "Field `UNALIGNED` reader - 24:24\\]
When CCR.UNALIGN_TRP is enabled, and there is an attempt to make an unaligned memory access, then this fault occurs. Unaligned LDM/STM/LDRD/STRD instructions always fault irrespective of the setting of CCR.UNALIGN_TRP."]
pub type UNALIGNED_R = crate::BitReader<bool>;
#[doc = "Field `UNALIGNED` writer - 24:24\\]
When CCR.UNALIGN_TRP is enabled, and there is an attempt to make an unaligned memory access, then this fault occurs. Unaligned LDM/STM/LDRD/STRD instructions always fault irrespective of the setting of CCR.UNALIGN_TRP."]
pub type UNALIGNED_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFSR_SPEC, bool, O>;
#[doc = "Field `DIVBYZERO` reader - 25:25\\]
When CCR.DIV_0_TRP (see Configuration Control Register on page 8-26) is enabled and an SDIV or UDIV instruction is used with a divisor of 0, this fault occurs The instruction is executed and the return PC points to it. If CCR.DIV_0_TRP is not set, then the divide returns a quotient of 0."]
pub type DIVBYZERO_R = crate::BitReader<bool>;
#[doc = "Field `DIVBYZERO` writer - 25:25\\]
When CCR.DIV_0_TRP (see Configuration Control Register on page 8-26) is enabled and an SDIV or UDIV instruction is used with a divisor of 0, this fault occurs The instruction is executed and the return PC points to it. If CCR.DIV_0_TRP is not set, then the divide returns a quotient of 0."]
pub type DIVBYZERO_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFSR_SPEC, bool, O>;
#[doc = "Field `RESERVED26` reader - 31:26\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED26_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED26` writer - 31:26\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED26_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFSR_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Instruction access violation flag. Attempting to fetch an instruction from a location that does not permit execution sets this flag. This occurs on any access to an XN region, even when the MPU is disabled or not present. The return PC points to the faulting instruction. MMFAR is not written."]
    #[inline(always)]
    pub fn iaccviol(&self) -> IACCVIOL_R {
        IACCVIOL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Data access violation flag. Attempting to load or store at a location that does not permit the operation sets this flag. The return PC points to the faulting instruction. This error loads MMFAR with the address of the attempted access."]
    #[inline(always)]
    pub fn daccviol(&self) -> DACCVIOL_R {
        DACCVIOL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> RESERVED2_R {
        RESERVED2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Unstack from exception return has caused one or more access violations. This is chained to the handler, so that the original return stack is still present. SP is not adjusted from failing return and new save is not performed. MMFAR is not written."]
    #[inline(always)]
    pub fn munstkerr(&self) -> MUNSTKERR_R {
        MUNSTKERR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Stacking from exception has caused one or more access violations. The SP is still adjusted and the values in the context area on the stack might be incorrect. MMFAR is not written."]
    #[inline(always)]
    pub fn mstkerr(&self) -> MSTKERR_R {
        MSTKERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - 6:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved5(&self) -> RESERVED5_R {
        RESERVED5_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - 7:7\\]
Memory Manage Address Register (MMFAR) address valid flag. A later-arriving fault, such as a bus fault, can clear a memory manage fault.. If a MemManage fault occurs that is escalated to a Hard Fault because of priority, the Hard Fault handler must clear this bit. This prevents problems on return to a stacked active MemManage handler whose MMFAR value has been overwritten."]
    #[inline(always)]
    pub fn mmarvalid(&self) -> MMARVALID_R {
        MMARVALID_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Instruction bus error flag. This flag is set by a prefetch error. The fault stops on the instruction, so if the error occurs under a branch shadow, no fault occurs. BFAR is not written."]
    #[inline(always)]
    pub fn ibuserr(&self) -> IBUSERR_R {
        IBUSERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Precise data bus error return."]
    #[inline(always)]
    pub fn preciserr(&self) -> PRECISERR_R {
        PRECISERR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Imprecise data bus error. It is a BusFault, but the Return PC is not related to the causing instruction. This is not a synchronous fault. So, if detected when the priority of the current activation is higher than the Bus Fault, it only pends. Bus fault activates when returning to a lower priority activation. If a precise fault occurs before returning to a lower priority exception, the handler detects both IMPRECISERR set and one of the precise fault status bits set at the same time. BFAR is not written."]
    #[inline(always)]
    pub fn impreciserr(&self) -> IMPRECISERR_R {
        IMPRECISERR_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Unstack from exception return has caused one or more bus faults. This is chained to the handler, so that the original return stack is still present. SP is not adjusted from failing return and new save is not performed. BFAR is not written."]
    #[inline(always)]
    pub fn unstkerr(&self) -> UNSTKERR_R {
        UNSTKERR_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Stacking from exception has caused one or more bus faults. The SP is still adjusted and the values in the context area on the stack might be incorrect. BFAR is not written."]
    #[inline(always)]
    pub fn stkerr(&self) -> STKERR_R {
        STKERR_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:14 - 14:13\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved13(&self) -> RESERVED13_R {
        RESERVED13_R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15 - 15:15\\]
This bit is set if the Bus Fault Address Register (BFAR) contains a valid address. This is true after a bus fault where the address is known. Other faults can clear this bit, such as a Mem Manage fault occurring later. If a Bus fault occurs that is escalated to a Hard Fault because of priority, the Hard Fault handler must clear this bit. This prevents problems if returning to a stacked active Bus fault handler whose BFAR value has been overwritten."]
    #[inline(always)]
    pub fn bfarvalid(&self) -> BFARVALID_R {
        BFARVALID_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
This bit is set when the processor attempts to execute an undefined instruction. This is an instruction that the processor cannot decode. The return PC points to the undefined instruction."]
    #[inline(always)]
    pub fn undefinstr(&self) -> UNDEFINSTR_R {
        UNDEFINSTR_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Indicates an attempt to execute in an invalid EPSR state (e.g. after a BX type instruction has changed state). This includes state change after entry to or return from exception, as well as from inter-working instructions. Return PC points to faulting instruction, with the invalid state."]
    #[inline(always)]
    pub fn invstate(&self) -> INVSTATE_R {
        INVSTATE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Attempt to load EXC_RETURN into PC illegally. Invalid instruction, invalid context, invalid value. The return PC points to the instruction that tried to set the PC."]
    #[inline(always)]
    pub fn invpc(&self) -> INVPC_R {
        INVPC_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
Attempt to use a coprocessor instruction. The processor does not support coprocessor instructions."]
    #[inline(always)]
    pub fn nocp(&self) -> NOCP_R {
        NOCP_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved20(&self) -> RESERVED20_R {
        RESERVED20_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - 24:24\\]
When CCR.UNALIGN_TRP is enabled, and there is an attempt to make an unaligned memory access, then this fault occurs. Unaligned LDM/STM/LDRD/STRD instructions always fault irrespective of the setting of CCR.UNALIGN_TRP."]
    #[inline(always)]
    pub fn unaligned(&self) -> UNALIGNED_R {
        UNALIGNED_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
When CCR.DIV_0_TRP (see Configuration Control Register on page 8-26) is enabled and an SDIV or UDIV instruction is used with a divisor of 0, this fault occurs The instruction is executed and the return PC points to it. If CCR.DIV_0_TRP is not set, then the divide returns a quotient of 0."]
    #[inline(always)]
    pub fn divbyzero(&self) -> DIVBYZERO_R {
        DIVBYZERO_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:31 - 31:26\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved26(&self) -> RESERVED26_R {
        RESERVED26_R::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Instruction access violation flag. Attempting to fetch an instruction from a location that does not permit execution sets this flag. This occurs on any access to an XN region, even when the MPU is disabled or not present. The return PC points to the faulting instruction. MMFAR is not written."]
    #[inline(always)]
    #[must_use]
    pub fn iaccviol(&mut self) -> IACCVIOL_W<0> {
        IACCVIOL_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
Data access violation flag. Attempting to load or store at a location that does not permit the operation sets this flag. The return PC points to the faulting instruction. This error loads MMFAR with the address of the attempted access."]
    #[inline(always)]
    #[must_use]
    pub fn daccviol(&mut self) -> DACCVIOL_W<1> {
        DACCVIOL_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> RESERVED2_W<2> {
        RESERVED2_W::new(self)
    }
    #[doc = "Bit 3 - 3:3\\]
Unstack from exception return has caused one or more access violations. This is chained to the handler, so that the original return stack is still present. SP is not adjusted from failing return and new save is not performed. MMFAR is not written."]
    #[inline(always)]
    #[must_use]
    pub fn munstkerr(&mut self) -> MUNSTKERR_W<3> {
        MUNSTKERR_W::new(self)
    }
    #[doc = "Bit 4 - 4:4\\]
Stacking from exception has caused one or more access violations. The SP is still adjusted and the values in the context area on the stack might be incorrect. MMFAR is not written."]
    #[inline(always)]
    #[must_use]
    pub fn mstkerr(&mut self) -> MSTKERR_W<4> {
        MSTKERR_W::new(self)
    }
    #[doc = "Bits 5:6 - 6:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved5(&mut self) -> RESERVED5_W<5> {
        RESERVED5_W::new(self)
    }
    #[doc = "Bit 7 - 7:7\\]
Memory Manage Address Register (MMFAR) address valid flag. A later-arriving fault, such as a bus fault, can clear a memory manage fault.. If a MemManage fault occurs that is escalated to a Hard Fault because of priority, the Hard Fault handler must clear this bit. This prevents problems on return to a stacked active MemManage handler whose MMFAR value has been overwritten."]
    #[inline(always)]
    #[must_use]
    pub fn mmarvalid(&mut self) -> MMARVALID_W<7> {
        MMARVALID_W::new(self)
    }
    #[doc = "Bit 8 - 8:8\\]
Instruction bus error flag. This flag is set by a prefetch error. The fault stops on the instruction, so if the error occurs under a branch shadow, no fault occurs. BFAR is not written."]
    #[inline(always)]
    #[must_use]
    pub fn ibuserr(&mut self) -> IBUSERR_W<8> {
        IBUSERR_W::new(self)
    }
    #[doc = "Bit 9 - 9:9\\]
Precise data bus error return."]
    #[inline(always)]
    #[must_use]
    pub fn preciserr(&mut self) -> PRECISERR_W<9> {
        PRECISERR_W::new(self)
    }
    #[doc = "Bit 10 - 10:10\\]
Imprecise data bus error. It is a BusFault, but the Return PC is not related to the causing instruction. This is not a synchronous fault. So, if detected when the priority of the current activation is higher than the Bus Fault, it only pends. Bus fault activates when returning to a lower priority activation. If a precise fault occurs before returning to a lower priority exception, the handler detects both IMPRECISERR set and one of the precise fault status bits set at the same time. BFAR is not written."]
    #[inline(always)]
    #[must_use]
    pub fn impreciserr(&mut self) -> IMPRECISERR_W<10> {
        IMPRECISERR_W::new(self)
    }
    #[doc = "Bit 11 - 11:11\\]
Unstack from exception return has caused one or more bus faults. This is chained to the handler, so that the original return stack is still present. SP is not adjusted from failing return and new save is not performed. BFAR is not written."]
    #[inline(always)]
    #[must_use]
    pub fn unstkerr(&mut self) -> UNSTKERR_W<11> {
        UNSTKERR_W::new(self)
    }
    #[doc = "Bit 12 - 12:12\\]
Stacking from exception has caused one or more bus faults. The SP is still adjusted and the values in the context area on the stack might be incorrect. BFAR is not written."]
    #[inline(always)]
    #[must_use]
    pub fn stkerr(&mut self) -> STKERR_W<12> {
        STKERR_W::new(self)
    }
    #[doc = "Bits 13:14 - 14:13\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved13(&mut self) -> RESERVED13_W<13> {
        RESERVED13_W::new(self)
    }
    #[doc = "Bit 15 - 15:15\\]
This bit is set if the Bus Fault Address Register (BFAR) contains a valid address. This is true after a bus fault where the address is known. Other faults can clear this bit, such as a Mem Manage fault occurring later. If a Bus fault occurs that is escalated to a Hard Fault because of priority, the Hard Fault handler must clear this bit. This prevents problems if returning to a stacked active Bus fault handler whose BFAR value has been overwritten."]
    #[inline(always)]
    #[must_use]
    pub fn bfarvalid(&mut self) -> BFARVALID_W<15> {
        BFARVALID_W::new(self)
    }
    #[doc = "Bit 16 - 16:16\\]
This bit is set when the processor attempts to execute an undefined instruction. This is an instruction that the processor cannot decode. The return PC points to the undefined instruction."]
    #[inline(always)]
    #[must_use]
    pub fn undefinstr(&mut self) -> UNDEFINSTR_W<16> {
        UNDEFINSTR_W::new(self)
    }
    #[doc = "Bit 17 - 17:17\\]
Indicates an attempt to execute in an invalid EPSR state (e.g. after a BX type instruction has changed state). This includes state change after entry to or return from exception, as well as from inter-working instructions. Return PC points to faulting instruction, with the invalid state."]
    #[inline(always)]
    #[must_use]
    pub fn invstate(&mut self) -> INVSTATE_W<17> {
        INVSTATE_W::new(self)
    }
    #[doc = "Bit 18 - 18:18\\]
Attempt to load EXC_RETURN into PC illegally. Invalid instruction, invalid context, invalid value. The return PC points to the instruction that tried to set the PC."]
    #[inline(always)]
    #[must_use]
    pub fn invpc(&mut self) -> INVPC_W<18> {
        INVPC_W::new(self)
    }
    #[doc = "Bit 19 - 19:19\\]
Attempt to use a coprocessor instruction. The processor does not support coprocessor instructions."]
    #[inline(always)]
    #[must_use]
    pub fn nocp(&mut self) -> NOCP_W<19> {
        NOCP_W::new(self)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved20(&mut self) -> RESERVED20_W<20> {
        RESERVED20_W::new(self)
    }
    #[doc = "Bit 24 - 24:24\\]
When CCR.UNALIGN_TRP is enabled, and there is an attempt to make an unaligned memory access, then this fault occurs. Unaligned LDM/STM/LDRD/STRD instructions always fault irrespective of the setting of CCR.UNALIGN_TRP."]
    #[inline(always)]
    #[must_use]
    pub fn unaligned(&mut self) -> UNALIGNED_W<24> {
        UNALIGNED_W::new(self)
    }
    #[doc = "Bit 25 - 25:25\\]
When CCR.DIV_0_TRP (see Configuration Control Register on page 8-26) is enabled and an SDIV or UDIV instruction is used with a divisor of 0, this fault occurs The instruction is executed and the return PC points to it. If CCR.DIV_0_TRP is not set, then the divide returns a quotient of 0."]
    #[inline(always)]
    #[must_use]
    pub fn divbyzero(&mut self) -> DIVBYZERO_W<25> {
        DIVBYZERO_W::new(self)
    }
    #[doc = "Bits 26:31 - 31:26\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved26(&mut self) -> RESERVED26_W<26> {
        RESERVED26_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configurable Fault Status This register is used to obtain information about local faults. These registers include three subsections: The first byte is Memory Manage Fault Status Register (MMFSR). The second byte is Bus Fault Status Register (BFSR). The higher half-word is Usage Fault Status Register (UFSR). The flags in these registers indicate the causes of local faults. Multiple flags can be set if more than one fault occurs. These register are read/write-clear. This means that they can be read normally, but writing a 1 to any bit clears that bit. The CFSR is byte accessible. CFSR or its subregisters can be accessed as follows: The following accesses are possible to the CFSR register: - access the complete register with a word access to 0xE000ED28. - access the MMFSR with a byte access to 0xE000ED28 - access the MMFSR and BFSR with a halfword access to 0xE000ED28 - access the BFSR with a byte access to 0xE000ED29 - access the UFSR with a halfword access to 0xE000ED2A.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfsr](index.html) module"]
pub struct CFSR_SPEC;
impl crate::RegisterSpec for CFSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfsr::R](R) reader structure"]
impl crate::Readable for CFSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfsr::W](W) writer structure"]
impl crate::Writable for CFSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFSR to value 0"]
impl crate::Resettable for CFSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
