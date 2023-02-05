#[doc = "Register `DFSR` reader"]
pub struct R(crate::R<DFSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DFSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DFSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DFSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DFSR` writer"]
pub struct W(crate::W<DFSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DFSR_SPEC>;
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
impl From<crate::W<DFSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DFSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HALTED` reader - 0:0\\]
Halt request flag. The processor is halted on the next instruction. 0x0: No halt request 0x1: Halt requested by NVIC, including step"]
pub type HALTED_R = crate::BitReader<bool>;
#[doc = "Field `HALTED` writer - 0:0\\]
Halt request flag. The processor is halted on the next instruction. 0x0: No halt request 0x1: Halt requested by NVIC, including step"]
pub type HALTED_W<'a, const O: u8> = crate::BitWriter<'a, u32, DFSR_SPEC, bool, O>;
#[doc = "Field `BKPT` reader - 1:1\\]
BKPT flag. The BKPT flag is set by a BKPT instruction in flash patch code, and also by normal code. Return PC points to breakpoint containing instruction. 0x0: No BKPT instruction execution 0x1: BKPT instruction execution"]
pub type BKPT_R = crate::BitReader<bool>;
#[doc = "Field `BKPT` writer - 1:1\\]
BKPT flag. The BKPT flag is set by a BKPT instruction in flash patch code, and also by normal code. Return PC points to breakpoint containing instruction. 0x0: No BKPT instruction execution 0x1: BKPT instruction execution"]
pub type BKPT_W<'a, const O: u8> = crate::BitWriter<'a, u32, DFSR_SPEC, bool, O>;
#[doc = "Field `DWTTRAP` reader - 2:2\\]
Data Watchpoint and Trace (DWT) flag. The processor stops at the current instruction or at the next instruction. 0x0: No DWT match 0x1: DWT match"]
pub type DWTTRAP_R = crate::BitReader<bool>;
#[doc = "Field `DWTTRAP` writer - 2:2\\]
Data Watchpoint and Trace (DWT) flag. The processor stops at the current instruction or at the next instruction. 0x0: No DWT match 0x1: DWT match"]
pub type DWTTRAP_W<'a, const O: u8> = crate::BitWriter<'a, u32, DFSR_SPEC, bool, O>;
#[doc = "Field `VCATCH` reader - 3:3\\]
Vector catch flag. When this flag is set, a flag in one of the local fault status registers is also set to indicate the type of fault. 0x0: No vector catch occurred 0x1: Vector catch occurred"]
pub type VCATCH_R = crate::BitReader<bool>;
#[doc = "Field `VCATCH` writer - 3:3\\]
Vector catch flag. When this flag is set, a flag in one of the local fault status registers is also set to indicate the type of fault. 0x0: No vector catch occurred 0x1: Vector catch occurred"]
pub type VCATCH_W<'a, const O: u8> = crate::BitWriter<'a, u32, DFSR_SPEC, bool, O>;
#[doc = "Field `EXTERNAL` reader - 4:4\\]
External debug request flag. The processor stops on next instruction boundary. 0x0: External debug request signal not asserted 0x1: External debug request signal asserted"]
pub type EXTERNAL_R = crate::BitReader<bool>;
#[doc = "Field `EXTERNAL` writer - 4:4\\]
External debug request flag. The processor stops on next instruction boundary. 0x0: External debug request signal not asserted 0x1: External debug request signal asserted"]
pub type EXTERNAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, DFSR_SPEC, bool, O>;
#[doc = "Field `RESERVED5` reader - 31:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED5_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED5` writer - 31:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DFSR_SPEC, u32, u32, 27, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Halt request flag. The processor is halted on the next instruction. 0x0: No halt request 0x1: Halt requested by NVIC, including step"]
    #[inline(always)]
    pub fn halted(&self) -> HALTED_R {
        HALTED_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
BKPT flag. The BKPT flag is set by a BKPT instruction in flash patch code, and also by normal code. Return PC points to breakpoint containing instruction. 0x0: No BKPT instruction execution 0x1: BKPT instruction execution"]
    #[inline(always)]
    pub fn bkpt(&self) -> BKPT_R {
        BKPT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Data Watchpoint and Trace (DWT) flag. The processor stops at the current instruction or at the next instruction. 0x0: No DWT match 0x1: DWT match"]
    #[inline(always)]
    pub fn dwttrap(&self) -> DWTTRAP_R {
        DWTTRAP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Vector catch flag. When this flag is set, a flag in one of the local fault status registers is also set to indicate the type of fault. 0x0: No vector catch occurred 0x1: Vector catch occurred"]
    #[inline(always)]
    pub fn vcatch(&self) -> VCATCH_R {
        VCATCH_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
External debug request flag. The processor stops on next instruction boundary. 0x0: External debug request signal not asserted 0x1: External debug request signal asserted"]
    #[inline(always)]
    pub fn external(&self) -> EXTERNAL_R {
        EXTERNAL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:31 - 31:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved5(&self) -> RESERVED5_R {
        RESERVED5_R::new((self.bits >> 5) & 0x07ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Halt request flag. The processor is halted on the next instruction. 0x0: No halt request 0x1: Halt requested by NVIC, including step"]
    #[inline(always)]
    #[must_use]
    pub fn halted(&mut self) -> HALTED_W<0> {
        HALTED_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
BKPT flag. The BKPT flag is set by a BKPT instruction in flash patch code, and also by normal code. Return PC points to breakpoint containing instruction. 0x0: No BKPT instruction execution 0x1: BKPT instruction execution"]
    #[inline(always)]
    #[must_use]
    pub fn bkpt(&mut self) -> BKPT_W<1> {
        BKPT_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
Data Watchpoint and Trace (DWT) flag. The processor stops at the current instruction or at the next instruction. 0x0: No DWT match 0x1: DWT match"]
    #[inline(always)]
    #[must_use]
    pub fn dwttrap(&mut self) -> DWTTRAP_W<2> {
        DWTTRAP_W::new(self)
    }
    #[doc = "Bit 3 - 3:3\\]
Vector catch flag. When this flag is set, a flag in one of the local fault status registers is also set to indicate the type of fault. 0x0: No vector catch occurred 0x1: Vector catch occurred"]
    #[inline(always)]
    #[must_use]
    pub fn vcatch(&mut self) -> VCATCH_W<3> {
        VCATCH_W::new(self)
    }
    #[doc = "Bit 4 - 4:4\\]
External debug request flag. The processor stops on next instruction boundary. 0x0: External debug request signal not asserted 0x1: External debug request signal asserted"]
    #[inline(always)]
    #[must_use]
    pub fn external(&mut self) -> EXTERNAL_W<4> {
        EXTERNAL_W::new(self)
    }
    #[doc = "Bits 5:31 - 31:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved5(&mut self) -> RESERVED5_W<5> {
        RESERVED5_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Debug Fault Status This register is used to monitor external debug requests, vector catches, data watchpoint match, BKPT instruction execution, halt requests. Multiple flags in the Debug Fault Status Register can be set when multiple fault conditions occur. The register is read/write clear. This means that it can be read normally. Writing a 1 to a bit clears that bit. Note that these bits are not set unless the event is caught. This means that it causes a stop of some sort. If halting debug is enabled, these events stop the processor into debug. If debug is disabled and the debug monitor is enabled, then this becomes a debug monitor handler call, if priority permits. If debug and the monitor are both disabled, some of these events are Hard Faults, and some are ignored.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsr](index.html) module"]
pub struct DFSR_SPEC;
impl crate::RegisterSpec for DFSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dfsr::R](R) reader structure"]
impl crate::Readable for DFSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dfsr::W](W) writer structure"]
impl crate::Writable for DFSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DFSR to value 0"]
impl crate::Resettable for DFSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
