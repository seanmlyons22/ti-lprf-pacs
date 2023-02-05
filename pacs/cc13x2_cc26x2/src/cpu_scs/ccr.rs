#[doc = "Register `CCR` reader"]
pub struct R(crate::R<CCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCR` writer"]
pub struct W(crate::W<CCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCR_SPEC>;
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
impl From<crate::W<CCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NONBASETHREDENA` reader - 0:0\\]
Indicates how the processor enters Thread mode: 0: Processor can enter Thread mode only when no exception is active. 1: Processor can enter Thread mode from any level using the appropriate return value (EXC_RETURN). Exception returns occur when one of the following instructions loads a value of 0xFXXXXXXX into the PC while in Handler mode: - POP/LDM which includes loading the PC. - LDR with PC as a destination. - BX with any register. The value written to the PC is intercepted and is referred to as the EXC_RETURN value."]
pub type NONBASETHREDENA_R = crate::BitReader<bool>;
#[doc = "Field `NONBASETHREDENA` writer - 0:0\\]
Indicates how the processor enters Thread mode: 0: Processor can enter Thread mode only when no exception is active. 1: Processor can enter Thread mode from any level using the appropriate return value (EXC_RETURN). Exception returns occur when one of the following instructions loads a value of 0xFXXXXXXX into the PC while in Handler mode: - POP/LDM which includes loading the PC. - LDR with PC as a destination. - BX with any register. The value written to the PC is intercepted and is referred to as the EXC_RETURN value."]
pub type NONBASETHREDENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, bool, O>;
#[doc = "Field `USERSETMPEND` reader - 1:1\\]
Enables unprivileged software access to STIR: 0: User code is not allowed to write to the Software Trigger Interrupt register (STIR). 1: User code can write the Software Trigger Interrupt register (STIR) to trigger (pend) a Main exception, which is associated with the Main stack pointer."]
pub type USERSETMPEND_R = crate::BitReader<bool>;
#[doc = "Field `USERSETMPEND` writer - 1:1\\]
Enables unprivileged software access to STIR: 0: User code is not allowed to write to the Software Trigger Interrupt register (STIR). 1: User code can write the Software Trigger Interrupt register (STIR) to trigger (pend) a Main exception, which is associated with the Main stack pointer."]
pub type USERSETMPEND_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, bool, O>;
#[doc = "Field `RESERVED2` reader - 2:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED2_R = crate::BitReader<bool>;
#[doc = "Field `RESERVED2` writer - 2:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, bool, O>;
#[doc = "Field `UNALIGN_TRP` reader - 3:3\\]
Enables unaligned access traps: 0: Do not trap unaligned halfword and word accesses 1: Trap unaligned halfword and word accesses. The relevant Usage Fault Status Register bit is CFSR.UNALIGNED. If this bit is set to 1, an unaligned access generates a UsageFault. Unaligned LDM, STM, LDRD, and STRD instructions always fault regardless of the value in UNALIGN_TRP."]
pub type UNALIGN_TRP_R = crate::BitReader<bool>;
#[doc = "Field `UNALIGN_TRP` writer - 3:3\\]
Enables unaligned access traps: 0: Do not trap unaligned halfword and word accesses 1: Trap unaligned halfword and word accesses. The relevant Usage Fault Status Register bit is CFSR.UNALIGNED. If this bit is set to 1, an unaligned access generates a UsageFault. Unaligned LDM, STM, LDRD, and STRD instructions always fault regardless of the value in UNALIGN_TRP."]
pub type UNALIGN_TRP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, bool, O>;
#[doc = "Field `DIV_0_TRP` reader - 4:4\\]
Enables faulting or halting when the processor executes an SDIV or UDIV instruction with a divisor of 0: 0: Do not trap divide by 0. In this mode, a divide by zero returns a quotient of 0. 1: Trap divide by 0. The relevant Usage Fault Status Register bit is CFSR.DIVBYZERO."]
pub type DIV_0_TRP_R = crate::BitReader<bool>;
#[doc = "Field `DIV_0_TRP` writer - 4:4\\]
Enables faulting or halting when the processor executes an SDIV or UDIV instruction with a divisor of 0: 0: Do not trap divide by 0. In this mode, a divide by zero returns a quotient of 0. 1: Trap divide by 0. The relevant Usage Fault Status Register bit is CFSR.DIVBYZERO."]
pub type DIV_0_TRP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, bool, O>;
#[doc = "Field `RESERVED5` reader - 7:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED5` writer - 7:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCR_SPEC, u8, u8, 3, O>;
#[doc = "Field `BFHFNMIGN` reader - 8:8\\]
Enables handlers with priority -1 or -2 to ignore data BusFaults caused by load and store instructions. This applies to the HardFault, NMI, and FAULTMASK escalated handlers: 0: Data BusFaults caused by load and store instructions cause a lock-up 1: Data BusFaults caused by load and store instructions are ignored. Set this bit to 1 only when the handler and its data are in absolutely safe memory. The normal use of this bit is to probe system devices and bridges to detect problems."]
pub type BFHFNMIGN_R = crate::BitReader<bool>;
#[doc = "Field `BFHFNMIGN` writer - 8:8\\]
Enables handlers with priority -1 or -2 to ignore data BusFaults caused by load and store instructions. This applies to the HardFault, NMI, and FAULTMASK escalated handlers: 0: Data BusFaults caused by load and store instructions cause a lock-up 1: Data BusFaults caused by load and store instructions are ignored. Set this bit to 1 only when the handler and its data are in absolutely safe memory. The normal use of this bit is to probe system devices and bridges to detect problems."]
pub type BFHFNMIGN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, bool, O>;
#[doc = "Field `STKALIGN` reader - 9:9\\]
Stack alignment bit. 0: Only 4-byte alignment is guaranteed for the SP used prior to the exception on exception entry. 1: On exception entry, the SP used prior to the exception is adjusted to be 8-byte aligned and the context to restore it is saved. The SP is restored on the associated exception return."]
pub type STKALIGN_R = crate::BitReader<bool>;
#[doc = "Field `STKALIGN` writer - 9:9\\]
Stack alignment bit. 0: Only 4-byte alignment is guaranteed for the SP used prior to the exception on exception entry. 1: On exception entry, the SP used prior to the exception is adjusted to be 8-byte aligned and the context to restore it is saved. The SP is restored on the associated exception return."]
pub type STKALIGN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, bool, O>;
#[doc = "Field `RESERVED10` reader - 31:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED10_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED10` writer - 31:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED10_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCR_SPEC, u32, u32, 22, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Indicates how the processor enters Thread mode: 0: Processor can enter Thread mode only when no exception is active. 1: Processor can enter Thread mode from any level using the appropriate return value (EXC_RETURN). Exception returns occur when one of the following instructions loads a value of 0xFXXXXXXX into the PC while in Handler mode: - POP/LDM which includes loading the PC. - LDR with PC as a destination. - BX with any register. The value written to the PC is intercepted and is referred to as the EXC_RETURN value."]
    #[inline(always)]
    pub fn nonbasethredena(&self) -> NONBASETHREDENA_R {
        NONBASETHREDENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Enables unprivileged software access to STIR: 0: User code is not allowed to write to the Software Trigger Interrupt register (STIR). 1: User code can write the Software Trigger Interrupt register (STIR) to trigger (pend) a Main exception, which is associated with the Main stack pointer."]
    #[inline(always)]
    pub fn usersetmpend(&self) -> USERSETMPEND_R {
        USERSETMPEND_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> RESERVED2_R {
        RESERVED2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Enables unaligned access traps: 0: Do not trap unaligned halfword and word accesses 1: Trap unaligned halfword and word accesses. The relevant Usage Fault Status Register bit is CFSR.UNALIGNED. If this bit is set to 1, an unaligned access generates a UsageFault. Unaligned LDM, STM, LDRD, and STRD instructions always fault regardless of the value in UNALIGN_TRP."]
    #[inline(always)]
    pub fn unalign_trp(&self) -> UNALIGN_TRP_R {
        UNALIGN_TRP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Enables faulting or halting when the processor executes an SDIV or UDIV instruction with a divisor of 0: 0: Do not trap divide by 0. In this mode, a divide by zero returns a quotient of 0. 1: Trap divide by 0. The relevant Usage Fault Status Register bit is CFSR.DIVBYZERO."]
    #[inline(always)]
    pub fn div_0_trp(&self) -> DIV_0_TRP_R {
        DIV_0_TRP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - 7:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved5(&self) -> RESERVED5_R {
        RESERVED5_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Enables handlers with priority -1 or -2 to ignore data BusFaults caused by load and store instructions. This applies to the HardFault, NMI, and FAULTMASK escalated handlers: 0: Data BusFaults caused by load and store instructions cause a lock-up 1: Data BusFaults caused by load and store instructions are ignored. Set this bit to 1 only when the handler and its data are in absolutely safe memory. The normal use of this bit is to probe system devices and bridges to detect problems."]
    #[inline(always)]
    pub fn bfhfnmign(&self) -> BFHFNMIGN_R {
        BFHFNMIGN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Stack alignment bit. 0: Only 4-byte alignment is guaranteed for the SP used prior to the exception on exception entry. 1: On exception entry, the SP used prior to the exception is adjusted to be 8-byte aligned and the context to restore it is saved. The SP is restored on the associated exception return."]
    #[inline(always)]
    pub fn stkalign(&self) -> STKALIGN_R {
        STKALIGN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:31 - 31:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved10(&self) -> RESERVED10_R {
        RESERVED10_R::new((self.bits >> 10) & 0x003f_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Indicates how the processor enters Thread mode: 0: Processor can enter Thread mode only when no exception is active. 1: Processor can enter Thread mode from any level using the appropriate return value (EXC_RETURN). Exception returns occur when one of the following instructions loads a value of 0xFXXXXXXX into the PC while in Handler mode: - POP/LDM which includes loading the PC. - LDR with PC as a destination. - BX with any register. The value written to the PC is intercepted and is referred to as the EXC_RETURN value."]
    #[inline(always)]
    #[must_use]
    pub fn nonbasethredena(&mut self) -> NONBASETHREDENA_W<0> {
        NONBASETHREDENA_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
Enables unprivileged software access to STIR: 0: User code is not allowed to write to the Software Trigger Interrupt register (STIR). 1: User code can write the Software Trigger Interrupt register (STIR) to trigger (pend) a Main exception, which is associated with the Main stack pointer."]
    #[inline(always)]
    #[must_use]
    pub fn usersetmpend(&mut self) -> USERSETMPEND_W<1> {
        USERSETMPEND_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> RESERVED2_W<2> {
        RESERVED2_W::new(self)
    }
    #[doc = "Bit 3 - 3:3\\]
Enables unaligned access traps: 0: Do not trap unaligned halfword and word accesses 1: Trap unaligned halfword and word accesses. The relevant Usage Fault Status Register bit is CFSR.UNALIGNED. If this bit is set to 1, an unaligned access generates a UsageFault. Unaligned LDM, STM, LDRD, and STRD instructions always fault regardless of the value in UNALIGN_TRP."]
    #[inline(always)]
    #[must_use]
    pub fn unalign_trp(&mut self) -> UNALIGN_TRP_W<3> {
        UNALIGN_TRP_W::new(self)
    }
    #[doc = "Bit 4 - 4:4\\]
Enables faulting or halting when the processor executes an SDIV or UDIV instruction with a divisor of 0: 0: Do not trap divide by 0. In this mode, a divide by zero returns a quotient of 0. 1: Trap divide by 0. The relevant Usage Fault Status Register bit is CFSR.DIVBYZERO."]
    #[inline(always)]
    #[must_use]
    pub fn div_0_trp(&mut self) -> DIV_0_TRP_W<4> {
        DIV_0_TRP_W::new(self)
    }
    #[doc = "Bits 5:7 - 7:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved5(&mut self) -> RESERVED5_W<5> {
        RESERVED5_W::new(self)
    }
    #[doc = "Bit 8 - 8:8\\]
Enables handlers with priority -1 or -2 to ignore data BusFaults caused by load and store instructions. This applies to the HardFault, NMI, and FAULTMASK escalated handlers: 0: Data BusFaults caused by load and store instructions cause a lock-up 1: Data BusFaults caused by load and store instructions are ignored. Set this bit to 1 only when the handler and its data are in absolutely safe memory. The normal use of this bit is to probe system devices and bridges to detect problems."]
    #[inline(always)]
    #[must_use]
    pub fn bfhfnmign(&mut self) -> BFHFNMIGN_W<8> {
        BFHFNMIGN_W::new(self)
    }
    #[doc = "Bit 9 - 9:9\\]
Stack alignment bit. 0: Only 4-byte alignment is guaranteed for the SP used prior to the exception on exception entry. 1: On exception entry, the SP used prior to the exception is adjusted to be 8-byte aligned and the context to restore it is saved. The SP is restored on the associated exception return."]
    #[inline(always)]
    #[must_use]
    pub fn stkalign(&mut self) -> STKALIGN_W<9> {
        STKALIGN_W::new(self)
    }
    #[doc = "Bits 10:31 - 31:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved10(&mut self) -> RESERVED10_W<10> {
        RESERVED10_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration Control This register is used to enable NMI, HardFault and FAULTMASK to ignore bus fault, trap divide by zero and unaligned accesses, enable user access to the Software Trigger Interrupt Register (STIR), control entry to Thread Mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccr](index.html) module"]
pub struct CCR_SPEC;
impl crate::RegisterSpec for CCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccr::R](R) reader structure"]
impl crate::Readable for CCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccr::W](W) writer structure"]
impl crate::Writable for CCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CCR to value 0x0200"]
impl crate::Resettable for CCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0200;
}
