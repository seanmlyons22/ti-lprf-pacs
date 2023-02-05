#[doc = "Register `FEDACSTAT` reader"]
pub struct R(crate::R<FEDACSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FEDACSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FEDACSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FEDACSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FEDACSTAT` writer"]
pub struct W(crate::W<FEDACSTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FEDACSTAT_SPEC>;
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
impl From<crate::W<FEDACSTAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FEDACSTAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ERR_PRF_FLG` reader - 23:0\\]
Internal. Only to be used through TI provided API."]
pub type ERR_PRF_FLG_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ERR_PRF_FLG` writer - 23:0\\]
Internal. Only to be used through TI provided API."]
pub type ERR_PRF_FLG_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FEDACSTAT_SPEC, u32, u32, 24, O>;
#[doc = "Field `FSM_DONE` reader - 24:24\\]
Internal. Only to be used through TI provided API."]
pub type FSM_DONE_R = crate::BitReader<bool>;
#[doc = "Field `FSM_DONE` writer - 24:24\\]
Internal. Only to be used through TI provided API."]
pub type FSM_DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FEDACSTAT_SPEC, bool, O>;
#[doc = "Field `RVF_INT` reader - 25:25\\]
Internal. Only to be used through TI provided API."]
pub type RVF_INT_R = crate::BitReader<bool>;
#[doc = "Field `RVF_INT` writer - 25:25\\]
Internal. Only to be used through TI provided API."]
pub type RVF_INT_W<'a, const O: u8> = crate::BitWriter<'a, u32, FEDACSTAT_SPEC, bool, O>;
#[doc = "Field `RESERVED26` reader - 31:26\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED26_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED26` writer - 31:26\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED26_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FEDACSTAT_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:23 - 23:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn err_prf_flg(&self) -> ERR_PRF_FLG_R {
        ERR_PRF_FLG_R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bit 24 - 24:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn fsm_done(&self) -> FSM_DONE_R {
        FSM_DONE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rvf_int(&self) -> RVF_INT_R {
        RVF_INT_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:31 - 31:26\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved26(&self) -> RESERVED26_R {
        RESERVED26_R::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - 23:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn err_prf_flg(&mut self) -> ERR_PRF_FLG_W<0> {
        ERR_PRF_FLG_W::new(self)
    }
    #[doc = "Bit 24 - 24:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn fsm_done(&mut self) -> FSM_DONE_W<24> {
        FSM_DONE_W::new(self)
    }
    #[doc = "Bit 25 - 25:25\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn rvf_int(&mut self) -> RVF_INT_W<25> {
        RVF_INT_W::new(self)
    }
    #[doc = "Bits 26:31 - 31:26\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved26(&mut self) -> RESERVED26_W<26> {
        RESERVED26_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fedacstat](index.html) module"]
pub struct FEDACSTAT_SPEC;
impl crate::RegisterSpec for FEDACSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fedacstat::R](R) reader structure"]
impl crate::Readable for FEDACSTAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fedacstat::W](W) writer structure"]
impl crate::Writable for FEDACSTAT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FEDACSTAT to value 0"]
impl crate::Resettable for FEDACSTAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
