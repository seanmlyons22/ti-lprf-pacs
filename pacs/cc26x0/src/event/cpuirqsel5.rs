#[doc = "Register `CPUIRQSEL5` reader"]
pub struct R(crate::R<CPUIRQSEL5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPUIRQSEL5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPUIRQSEL5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPUIRQSEL5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CPUIRQSEL5` writer"]
pub struct W(crate::W<CPUIRQSEL5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CPUIRQSEL5_SPEC>;
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
impl From<crate::W<CPUIRQSEL5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CPUIRQSEL5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EV` reader - 6:0\\]
Read only selection value"]
pub type EV_R = crate::FieldReader<u8, EV_A>;
#[doc = "6:0\\]
Read only selection value\n\nValue on reset: 36"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EV_A {
    #[doc = "36: UART0 combined interrupt, interrupt flags are found here UART0:MIS"]
    UART0_COMB = 36,
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
            36 => Some(EV_A::UART0_COMB),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `UART0_COMB`"]
    #[inline(always)]
    pub fn is_uart0_comb(&self) -> bool {
        *self == EV_A::UART0_COMB
    }
}
#[doc = "Field `EV` writer - 6:0\\]
Read only selection value"]
pub type EV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CPUIRQSEL5_SPEC, u8, EV_A, 7, O>;
impl<'a, const O: u8> EV_W<'a, O> {
    #[doc = "UART0 combined interrupt, interrupt flags are found here UART0:MIS"]
    #[inline(always)]
    pub fn uart0_comb(self) -> &'a mut W {
        self.variant(EV_A::UART0_COMB)
    }
}
impl R {
    #[doc = "Bits 0:6 - 6:0\\]
Read only selection value"]
    #[inline(always)]
    pub fn ev(&self) -> EV_R {
        EV_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - 6:0\\]
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
#[doc = "Output Selection for CPU Interrupt 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpuirqsel5](index.html) module"]
pub struct CPUIRQSEL5_SPEC;
impl crate::RegisterSpec for CPUIRQSEL5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpuirqsel5::R](R) reader structure"]
impl crate::Readable for CPUIRQSEL5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cpuirqsel5::W](W) writer structure"]
impl crate::Writable for CPUIRQSEL5_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CPUIRQSEL5 to value 0x24"]
impl crate::Resettable for CPUIRQSEL5_SPEC {
    const RESET_VALUE: Self::Ux = 0x24;
}
