#[doc = "Register `WDTTEST` reader"]
pub struct R(crate::R<WDTTEST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WDTTEST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WDTTEST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WDTTEST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WDTTEST` writer"]
pub struct W(crate::W<WDTTEST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WDTTEST_SPEC>;
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
impl From<crate::W<WDTTEST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WDTTEST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STALLEN` reader - 0:0\\]
WDT Stall Enable 0: The WDT timer continues counting if the CPU is stopped with a debugger. 1: If the CPU is stopped with a debugger, the WDT stops counting. Once the CPU is restarted, the WDT resumes counting."]
pub type STALLEN_R = crate::BitReader<bool>;
#[doc = "Field `STALLEN` writer - 0:0\\]
WDT Stall Enable 0: The WDT timer continues counting if the CPU is stopped with a debugger. 1: If the CPU is stopped with a debugger, the WDT stops counting. Once the CPU is restarted, the WDT resumes counting."]
pub type STALLEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, WDTTEST_SPEC, bool, O>;
#[doc = "Field `RESERVED0` reader - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED0_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED0` writer - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WDTTEST_SPEC, u32, u32, 31, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
WDT Stall Enable 0: The WDT timer continues counting if the CPU is stopped with a debugger. 1: If the CPU is stopped with a debugger, the WDT stops counting. Once the CPU is restarted, the WDT resumes counting."]
    #[inline(always)]
    pub fn stallen(&self) -> STALLEN_R {
        STALLEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&self) -> RESERVED0_R {
        RESERVED0_R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
WDT Stall Enable 0: The WDT timer continues counting if the CPU is stopped with a debugger. 1: If the CPU is stopped with a debugger, the WDT stops counting. Once the CPU is restarted, the WDT resumes counting."]
    #[inline(always)]
    #[must_use]
    pub fn stallen(&mut self) -> STALLEN_W<0> {
        STALLEN_W::new(self)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved0(&mut self) -> RESERVED0_W<1> {
        RESERVED0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Test Mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdttest](index.html) module"]
pub struct WDTTEST_SPEC;
impl crate::RegisterSpec for WDTTEST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wdttest::R](R) reader structure"]
impl crate::Readable for WDTTEST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wdttest::W](W) writer structure"]
impl crate::Writable for WDTTEST_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WDTTEST to value 0"]
impl crate::Resettable for WDTTEST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
