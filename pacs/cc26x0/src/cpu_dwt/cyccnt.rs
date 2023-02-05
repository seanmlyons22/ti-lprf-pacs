#[doc = "Register `CYCCNT` reader"]
pub struct R(crate::R<CYCCNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CYCCNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CYCCNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CYCCNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CYCCNT` writer"]
pub struct W(crate::W<CYCCNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CYCCNT_SPEC>;
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
impl From<crate::W<CYCCNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CYCCNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CYCCNT` reader - 31:0\\]
Current PC Sampler Cycle Counter count value. When enabled, this counter counts the number of core cycles, except when the core is halted. The cycle counter is a free running counter, counting upwards (this counter will not advance in power modes where free-running clock to CPU stops). It wraps around to 0 on overflow. The debugger must initialize this to 0 when first enabling."]
pub type CYCCNT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CYCCNT` writer - 31:0\\]
Current PC Sampler Cycle Counter count value. When enabled, this counter counts the number of core cycles, except when the core is halted. The cycle counter is a free running counter, counting upwards (this counter will not advance in power modes where free-running clock to CPU stops). It wraps around to 0 on overflow. The debugger must initialize this to 0 when first enabling."]
pub type CYCCNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CYCCNT_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Current PC Sampler Cycle Counter count value. When enabled, this counter counts the number of core cycles, except when the core is halted. The cycle counter is a free running counter, counting upwards (this counter will not advance in power modes where free-running clock to CPU stops). It wraps around to 0 on overflow. The debugger must initialize this to 0 when first enabling."]
    #[inline(always)]
    pub fn cyccnt(&self) -> CYCCNT_R {
        CYCCNT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Current PC Sampler Cycle Counter count value. When enabled, this counter counts the number of core cycles, except when the core is halted. The cycle counter is a free running counter, counting upwards (this counter will not advance in power modes where free-running clock to CPU stops). It wraps around to 0 on overflow. The debugger must initialize this to 0 when first enabling."]
    #[inline(always)]
    #[must_use]
    pub fn cyccnt(&mut self) -> CYCCNT_W<0> {
        CYCCNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Current PC Sampler Cycle Count This register is used to count the number of core cycles. This counter can measure elapsed execution time. This is a free-running counter (this counter will not advance in power modes where free-running clock to CPU stops). The counter has three functions: 1: When CTRL.PCSAMPLEENA = 1, the PC is sampled and emitted when the selected tapped bit changes value (0 to 1 or 1 to 0) and any post-scalar value counts to 0. 2: When CTRL.CYCEVTENA = 1 , (and CTRL.PCSAMPLEENA = 0), an event is emitted when the selected tapped bit changes value (0 to 1 or 1 to 0) and any post-scalar value counts to 0. 3: Applications and debuggers can use the counter to measure elapsed execution time. By subtracting a start and an end time, an application can measure time between in-core clocks (other than when Halted in debug). This is valid to 2^32 core clock cycles (for example, almost 89.5 seconds at 48MHz).\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cyccnt](index.html) module"]
pub struct CYCCNT_SPEC;
impl crate::RegisterSpec for CYCCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cyccnt::R](R) reader structure"]
impl crate::Readable for CYCCNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cyccnt::W](W) writer structure"]
impl crate::Writable for CYCCNT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CYCCNT to value 0"]
impl crate::Resettable for CYCCNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
