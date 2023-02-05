#[doc = "Register `ITCTRL` reader"]
pub struct R(crate::R<ITCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ITCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ITCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ITCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ITCTRL` writer"]
pub struct W(crate::W<ITCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ITCTRL_SPEC>;
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
impl From<crate::W<ITCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ITCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IME` reader - 0:0\\]
Integration mode enable bit - The possible values are: 0 - The trace unit is not in integration mode. 1 - The trace unit is in integration mode. This mode enables: A debug agent to perform topology detection. SoC test software to perform integration testing."]
pub type IME_R = crate::BitReader<bool>;
#[doc = "Field `IME` writer - 0:0\\]
Integration mode enable bit - The possible values are: 0 - The trace unit is not in integration mode. 1 - The trace unit is in integration mode. This mode enables: A debug agent to perform topology detection. SoC test software to perform integration testing."]
pub type IME_W<'a, const O: u8> = crate::BitWriter<'a, u32, ITCTRL_SPEC, bool, O>;
#[doc = "Field `RESERVED1` reader - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED1_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED1` writer - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ITCTRL_SPEC, u32, u32, 31, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Integration mode enable bit - The possible values are: 0 - The trace unit is not in integration mode. 1 - The trace unit is in integration mode. This mode enables: A debug agent to perform topology detection. SoC test software to perform integration testing."]
    #[inline(always)]
    pub fn ime(&self) -> IME_R {
        IME_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Integration mode enable bit - The possible values are: 0 - The trace unit is not in integration mode. 1 - The trace unit is in integration mode. This mode enables: A debug agent to perform topology detection. SoC test software to perform integration testing."]
    #[inline(always)]
    #[must_use]
    pub fn ime(&mut self) -> IME_W<0> {
        IME_W::new(self)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> RESERVED1_W<1> {
        RESERVED1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Integration Mode Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itctrl](index.html) module"]
pub struct ITCTRL_SPEC;
impl crate::RegisterSpec for ITCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [itctrl::R](R) reader structure"]
impl crate::Readable for ITCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [itctrl::W](W) writer structure"]
impl crate::Writable for ITCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ITCTRL to value 0"]
impl crate::Resettable for ITCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
