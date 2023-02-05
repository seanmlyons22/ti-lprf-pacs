#[doc = "Register `AUXIOLATCH` reader"]
pub struct R(crate::R<AUXIOLATCH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AUXIOLATCH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AUXIOLATCH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AUXIOLATCH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AUXIOLATCH` writer"]
pub struct W(crate::W<AUXIOLATCH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AUXIOLATCH_SPEC>;
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
impl From<crate::W<AUXIOLATCH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AUXIOLATCH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - 0:0\\]
Opens (1) or closes (0) the AUX_AIODIO0/AUX_AIODIO1 signal latching. At startup, set EN = TRANSP before configuring AUX_AIODIO0/AUX_AIODIO1 and subsequently selecting AUX mode in the AON_IOC. When powering off the AUX domain (using PWROFFREQ.REQ), set EN = STATIC in advance preserve the current state (mode and output value) of the I/O pins."]
pub type EN_R = crate::BitReader<EN_A>;
#[doc = "0:0\\]
Opens (1) or closes (0) the AUX_AIODIO0/AUX_AIODIO1 signal latching. At startup, set EN = TRANSP before configuring AUX_AIODIO0/AUX_AIODIO1 and subsequently selecting AUX mode in the AON_IOC. When powering off the AUX domain (using PWROFFREQ.REQ), set EN = STATIC in advance preserve the current state (mode and output value) of the I/O pins.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EN_A {
    #[doc = "1: Latches are transparent ( open )"]
    TRANSP = 1,
    #[doc = "0: Latches are static ( closed )"]
    STATIC = 0,
}
impl From<EN_A> for bool {
    #[inline(always)]
    fn from(variant: EN_A) -> Self {
        variant as u8 != 0
    }
}
impl EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN_A {
        match self.bits {
            true => EN_A::TRANSP,
            false => EN_A::STATIC,
        }
    }
    #[doc = "Checks if the value of the field is `TRANSP`"]
    #[inline(always)]
    pub fn is_transp(&self) -> bool {
        *self == EN_A::TRANSP
    }
    #[doc = "Checks if the value of the field is `STATIC`"]
    #[inline(always)]
    pub fn is_static(&self) -> bool {
        *self == EN_A::STATIC
    }
}
#[doc = "Field `EN` writer - 0:0\\]
Opens (1) or closes (0) the AUX_AIODIO0/AUX_AIODIO1 signal latching. At startup, set EN = TRANSP before configuring AUX_AIODIO0/AUX_AIODIO1 and subsequently selecting AUX mode in the AON_IOC. When powering off the AUX domain (using PWROFFREQ.REQ), set EN = STATIC in advance preserve the current state (mode and output value) of the I/O pins."]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AUXIOLATCH_SPEC, EN_A, O>;
impl<'a, const O: u8> EN_W<'a, O> {
    #[doc = "Latches are transparent ( open )"]
    #[inline(always)]
    pub fn transp(self) -> &'a mut W {
        self.variant(EN_A::TRANSP)
    }
    #[doc = "Latches are static ( closed )"]
    #[inline(always)]
    pub fn static_(self) -> &'a mut W {
        self.variant(EN_A::STATIC)
    }
}
impl R {
    #[doc = "Bit 0 - 0:0\\]
Opens (1) or closes (0) the AUX_AIODIO0/AUX_AIODIO1 signal latching. At startup, set EN = TRANSP before configuring AUX_AIODIO0/AUX_AIODIO1 and subsequently selecting AUX mode in the AON_IOC. When powering off the AUX domain (using PWROFFREQ.REQ), set EN = STATIC in advance preserve the current state (mode and output value) of the I/O pins."]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Opens (1) or closes (0) the AUX_AIODIO0/AUX_AIODIO1 signal latching. At startup, set EN = TRANSP before configuring AUX_AIODIO0/AUX_AIODIO1 and subsequently selecting AUX mode in the AON_IOC. When powering off the AUX domain (using PWROFFREQ.REQ), set EN = STATIC in advance preserve the current state (mode and output value) of the I/O pins."]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AUX Input Output Latch Controls latching of signals between AUX_AIODIO0/AUX_AIODIO1 and AON_IOC.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [auxiolatch](index.html) module"]
pub struct AUXIOLATCH_SPEC;
impl crate::RegisterSpec for AUXIOLATCH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [auxiolatch::R](R) reader structure"]
impl crate::Readable for AUXIOLATCH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [auxiolatch::W](W) writer structure"]
impl crate::Writable for AUXIOLATCH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AUXIOLATCH to value 0"]
impl crate::Resettable for AUXIOLATCH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
