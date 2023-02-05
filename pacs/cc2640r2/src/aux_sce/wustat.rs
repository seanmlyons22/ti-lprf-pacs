#[doc = "Register `WUSTAT` reader"]
pub struct R(crate::R<WUSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WUSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WUSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WUSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WUSTAT` writer"]
pub struct W(crate::W<WUSTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WUSTAT_SPEC>;
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
impl From<crate::W<WUSTAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WUSTAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EV_SIGNALS` reader - 7:0\\]
Internal. Only to be used through TI provided API."]
pub type EV_SIGNALS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EV_SIGNALS` writer - 7:0\\]
Internal. Only to be used through TI provided API."]
pub type EV_SIGNALS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WUSTAT_SPEC, u8, u8, 8, O>;
#[doc = "Field `WU_SIGNAL` reader - 8:8\\]
Internal. Only to be used through TI provided API."]
pub type WU_SIGNAL_R = crate::BitReader<bool>;
#[doc = "Field `WU_SIGNAL` writer - 8:8\\]
Internal. Only to be used through TI provided API."]
pub type WU_SIGNAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUSTAT_SPEC, bool, O>;
#[doc = "Field `RESERVED9` reader - 15:9\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED9_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED9` writer - 15:9\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED9_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WUSTAT_SPEC, u8, u8, 7, O>;
#[doc = "Field `EXC_VECTOR` reader - 17:16\\]
Internal. Only to be used through TI provided API."]
pub type EXC_VECTOR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EXC_VECTOR` writer - 17:16\\]
Internal. Only to be used through TI provided API."]
pub type EXC_VECTOR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WUSTAT_SPEC, u8, u8, 2, O>;
#[doc = "Field `RESERVED18` reader - 31:18\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED18_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RESERVED18` writer - 31:18\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED18_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WUSTAT_SPEC, u16, u16, 14, O>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ev_signals(&self) -> EV_SIGNALS_R {
        EV_SIGNALS_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn wu_signal(&self) -> WU_SIGNAL_R {
        WU_SIGNAL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:15 - 15:9\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved9(&self) -> RESERVED9_R {
        RESERVED9_R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn exc_vector(&self) -> EXC_VECTOR_R {
        EXC_VECTOR_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:31 - 31:18\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved18(&self) -> RESERVED18_R {
        RESERVED18_R::new(((self.bits >> 18) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn ev_signals(&mut self) -> EV_SIGNALS_W<0> {
        EV_SIGNALS_W::new(self)
    }
    #[doc = "Bit 8 - 8:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn wu_signal(&mut self) -> WU_SIGNAL_W<8> {
        WU_SIGNAL_W::new(self)
    }
    #[doc = "Bits 9:15 - 15:9\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved9(&mut self) -> RESERVED9_W<9> {
        RESERVED9_W::new(self)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn exc_vector(&mut self) -> EXC_VECTOR_W<16> {
        EXC_VECTOR_W::new(self)
    }
    #[doc = "Bits 18:31 - 31:18\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved18(&mut self) -> RESERVED18_W<18> {
        RESERVED18_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wustat](index.html) module"]
pub struct WUSTAT_SPEC;
impl crate::RegisterSpec for WUSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wustat::R](R) reader structure"]
impl crate::Readable for WUSTAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wustat::W](W) writer structure"]
impl crate::Writable for WUSTAT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WUSTAT to value 0"]
impl crate::Resettable for WUSTAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
