#[doc = "Register `CPUIRQSEL24` reader"]
pub struct R(crate::R<CPUIRQSEL24_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPUIRQSEL24_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPUIRQSEL24_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPUIRQSEL24_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CPUIRQSEL24` writer"]
pub struct W(crate::W<CPUIRQSEL24_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CPUIRQSEL24_SPEC>;
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
impl From<crate::W<CPUIRQSEL24_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CPUIRQSEL24_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EV` reader - 7:0\\]
Read only selection value"]
pub type EV_R = crate::FieldReader<u8, EV_A>;
#[doc = "7:0\\]
Read only selection value\n\nValue on reset: 39"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EV_A {
    #[doc = "39: Combined DMA done, corresponding flags are here UDMA0:REQDONE"]
    DMA_DONE_COMB = 39,
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
            39 => Some(EV_A::DMA_DONE_COMB),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DMA_DONE_COMB`"]
    #[inline(always)]
    pub fn is_dma_done_comb(&self) -> bool {
        *self == EV_A::DMA_DONE_COMB
    }
}
#[doc = "Field `EV` writer - 7:0\\]
Read only selection value"]
pub type EV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CPUIRQSEL24_SPEC, u8, EV_A, 8, O>;
impl<'a, const O: u8> EV_W<'a, O> {
    #[doc = "Combined DMA done, corresponding flags are here UDMA0:REQDONE"]
    #[inline(always)]
    pub fn dma_done_comb(self) -> &'a mut W {
        self.variant(EV_A::DMA_DONE_COMB)
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
#[doc = "Output Selection for CPU Interrupt 24\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpuirqsel24](index.html) module"]
pub struct CPUIRQSEL24_SPEC;
impl crate::RegisterSpec for CPUIRQSEL24_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpuirqsel24::R](R) reader structure"]
impl crate::Readable for CPUIRQSEL24_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cpuirqsel24::W](W) writer structure"]
impl crate::Writable for CPUIRQSEL24_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CPUIRQSEL24 to value 0x27"]
impl crate::Resettable for CPUIRQSEL24_SPEC {
    const RESET_VALUE: Self::Ux = 0x27;
}
