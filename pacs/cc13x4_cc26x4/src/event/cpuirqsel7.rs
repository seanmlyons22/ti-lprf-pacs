#[doc = "Register `CPUIRQSEL7` reader"]
pub struct R(crate::R<CPUIRQSEL7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPUIRQSEL7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPUIRQSEL7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPUIRQSEL7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CPUIRQSEL7` writer"]
pub struct W(crate::W<CPUIRQSEL7_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CPUIRQSEL7_SPEC>;
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
impl From<crate::W<CPUIRQSEL7_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CPUIRQSEL7_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EV` reader - 7:0\\]
Read only selection value"]
pub type EV_R = crate::FieldReader<u8, EV_A>;
#[doc = "7:0\\]
Read only selection value\n\nValue on reset: 34"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EV_A {
    #[doc = "34: SPI0 combined interrupt, interrupt flags are found here SPI0:MIS"]
    SPI0_COMB = 34,
}
impl From<EV_A> for u8 {
    #[inline(always)]
    fn from(variant: EV_A) -> Self {
        variant as _
    }
}
impl EV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EV_A> {
        match self.bits {
            34 => Some(EV_A::SPI0_COMB),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SPI0_COMB`"]
    #[inline(always)]
    pub fn is_spi0_comb(&self) -> bool {
        *self == EV_A::SPI0_COMB
    }
}
#[doc = "Field `EV` writer - 7:0\\]
Read only selection value"]
pub type EV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CPUIRQSEL7_SPEC, u8, EV_A, 8, O>;
impl<'a, const O: u8> EV_W<'a, O> {
    #[doc = "SPI0 combined interrupt, interrupt flags are found here SPI0:MIS"]
    #[inline(always)]
    pub fn spi0_comb(self) -> &'a mut W {
        self.variant(EV_A::SPI0_COMB)
    }
}
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Read only selection value"]
    #[inline(always)]
    pub fn ev(&self) -> EV_R {
        EV_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Read only selection value"]
    #[inline(always)]
    #[must_use]
    pub fn ev(&mut self) -> EV_W<0> {
        EV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Output Selection for CPU Interrupt 7\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpuirqsel7](index.html) module"]
pub struct CPUIRQSEL7_SPEC;
impl crate::RegisterSpec for CPUIRQSEL7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpuirqsel7::R](R) reader structure"]
impl crate::Readable for CPUIRQSEL7_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cpuirqsel7::W](W) writer structure"]
impl crate::Writable for CPUIRQSEL7_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CPUIRQSEL7 to value 0x22"]
impl crate::Resettable for CPUIRQSEL7_SPEC {
    const RESET_VALUE: Self::Ux = 0x22;
}
