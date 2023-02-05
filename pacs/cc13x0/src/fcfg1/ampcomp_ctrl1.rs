#[doc = "Register `AMPCOMP_CTRL1` reader"]
pub struct R(crate::R<AMPCOMP_CTRL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AMPCOMP_CTRL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AMPCOMP_CTRL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AMPCOMP_CTRL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AMPCOMP_CTRL1` writer"]
pub struct W(crate::W<AMPCOMP_CTRL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AMPCOMP_CTRL1_SPEC>;
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
impl From<crate::W<AMPCOMP_CTRL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AMPCOMP_CTRL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IBIASCAP_HPTOLP_OL_CNT` reader - 3:0\\]
Internal. Only to be used through TI provided API."]
pub type IBIASCAP_HPTOLP_OL_CNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IBIASCAP_HPTOLP_OL_CNT` writer - 3:0\\]
Internal. Only to be used through TI provided API."]
pub type IBIASCAP_HPTOLP_OL_CNT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, AMPCOMP_CTRL1_SPEC, u8, u8, 4, O>;
#[doc = "Field `CAP_STEP` reader - 7:4\\]
Internal. Only to be used through TI provided API."]
pub type CAP_STEP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CAP_STEP` writer - 7:4\\]
Internal. Only to be used through TI provided API."]
pub type CAP_STEP_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, AMPCOMP_CTRL1_SPEC, u8, u8, 4, O>;
#[doc = "Field `LPM_IBIAS_WAIT_CNT_FINAL` reader - 15:8\\]
Internal. Only to be used through TI provided API."]
pub type LPM_IBIAS_WAIT_CNT_FINAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LPM_IBIAS_WAIT_CNT_FINAL` writer - 15:8\\]
Internal. Only to be used through TI provided API."]
pub type LPM_IBIAS_WAIT_CNT_FINAL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, AMPCOMP_CTRL1_SPEC, u8, u8, 8, O>;
#[doc = "Field `IBIAS_INIT` reader - 19:16\\]
Internal. Only to be used through TI provided API."]
pub type IBIAS_INIT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IBIAS_INIT` writer - 19:16\\]
Internal. Only to be used through TI provided API."]
pub type IBIAS_INIT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, AMPCOMP_CTRL1_SPEC, u8, u8, 4, O>;
#[doc = "Field `IBIAS_OFFSET` reader - 23:20\\]
Internal. Only to be used through TI provided API."]
pub type IBIAS_OFFSET_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IBIAS_OFFSET` writer - 23:20\\]
Internal. Only to be used through TI provided API."]
pub type IBIAS_OFFSET_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, AMPCOMP_CTRL1_SPEC, u8, u8, 4, O>;
#[doc = "Field `RESERVED0` reader - 29:24\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED0` writer - 29:24\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, AMPCOMP_CTRL1_SPEC, u8, u8, 6, O>;
#[doc = "Field `AMPCOMP_REQ_MODE` reader - 30:30\\]
Internal. Only to be used through TI provided API."]
pub type AMPCOMP_REQ_MODE_R = crate::BitReader<bool>;
#[doc = "Field `AMPCOMP_REQ_MODE` writer - 30:30\\]
Internal. Only to be used through TI provided API."]
pub type AMPCOMP_REQ_MODE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AMPCOMP_CTRL1_SPEC, bool, O>;
#[doc = "Field `RESERVED1` reader - 31:31\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED1_R = crate::BitReader<bool>;
#[doc = "Field `RESERVED1` writer - 31:31\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED1_W<'a, const O: u8> = crate::BitWriter<'a, u32, AMPCOMP_CTRL1_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ibiascap_hptolp_ol_cnt(&self) -> IBIASCAP_HPTOLP_OL_CNT_R {
        IBIASCAP_HPTOLP_OL_CNT_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn cap_step(&self) -> CAP_STEP_R {
        CAP_STEP_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn lpm_ibias_wait_cnt_final(&self) -> LPM_IBIAS_WAIT_CNT_FINAL_R {
        LPM_IBIAS_WAIT_CNT_FINAL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ibias_init(&self) -> IBIAS_INIT_R {
        IBIAS_INIT_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ibias_offset(&self) -> IBIAS_OFFSET_R {
        IBIAS_OFFSET_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:29 - 29:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved0(&self) -> RESERVED0_R {
        RESERVED0_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bit 30 - 30:30\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ampcomp_req_mode(&self) -> AMPCOMP_REQ_MODE_R {
        AMPCOMP_REQ_MODE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn ibiascap_hptolp_ol_cnt(&mut self) -> IBIASCAP_HPTOLP_OL_CNT_W<0> {
        IBIASCAP_HPTOLP_OL_CNT_W::new(self)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn cap_step(&mut self) -> CAP_STEP_W<4> {
        CAP_STEP_W::new(self)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn lpm_ibias_wait_cnt_final(&mut self) -> LPM_IBIAS_WAIT_CNT_FINAL_W<8> {
        LPM_IBIAS_WAIT_CNT_FINAL_W::new(self)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn ibias_init(&mut self) -> IBIAS_INIT_W<16> {
        IBIAS_INIT_W::new(self)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn ibias_offset(&mut self) -> IBIAS_OFFSET_W<20> {
        IBIAS_OFFSET_W::new(self)
    }
    #[doc = "Bits 24:29 - 29:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved0(&mut self) -> RESERVED0_W<24> {
        RESERVED0_W::new(self)
    }
    #[doc = "Bit 30 - 30:30\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn ampcomp_req_mode(&mut self) -> AMPCOMP_REQ_MODE_W<30> {
        AMPCOMP_REQ_MODE_W::new(self)
    }
    #[doc = "Bit 31 - 31:31\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> RESERVED1_W<31> {
        RESERVED1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ampcomp_ctrl1](index.html) module"]
pub struct AMPCOMP_CTRL1_SPEC;
impl crate::RegisterSpec for AMPCOMP_CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ampcomp_ctrl1::R](R) reader structure"]
impl crate::Readable for AMPCOMP_CTRL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ampcomp_ctrl1::W](W) writer structure"]
impl crate::Writable for AMPCOMP_CTRL1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AMPCOMP_CTRL1 to value 0xff18_3f47"]
impl crate::Resettable for AMPCOMP_CTRL1_SPEC {
    const RESET_VALUE: Self::Ux = 0xff18_3f47;
}
