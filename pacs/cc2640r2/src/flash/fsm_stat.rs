#[doc = "Register `FSM_STAT` reader"]
pub struct R(crate::R<FSM_STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FSM_STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FSM_STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FSM_STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FSM_STAT` writer"]
pub struct W(crate::W<FSM_STAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FSM_STAT_SPEC>;
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
impl From<crate::W<FSM_STAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FSM_STAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INV_DAT` reader - 0:0\\]
Internal. Only to be used through TI provided API."]
pub type INV_DAT_R = crate::BitReader<bool>;
#[doc = "Field `INV_DAT` writer - 0:0\\]
Internal. Only to be used through TI provided API."]
pub type INV_DAT_W<'a, const O: u8> = crate::BitWriter<'a, u32, FSM_STAT_SPEC, bool, O>;
#[doc = "Field `OVR_PUL_CNT` reader - 1:1\\]
Internal. Only to be used through TI provided API."]
pub type OVR_PUL_CNT_R = crate::BitReader<bool>;
#[doc = "Field `OVR_PUL_CNT` writer - 1:1\\]
Internal. Only to be used through TI provided API."]
pub type OVR_PUL_CNT_W<'a, const O: u8> = crate::BitWriter<'a, u32, FSM_STAT_SPEC, bool, O>;
#[doc = "Field `NON_OP` reader - 2:2\\]
Internal. Only to be used through TI provided API."]
pub type NON_OP_R = crate::BitReader<bool>;
#[doc = "Field `NON_OP` writer - 2:2\\]
Internal. Only to be used through TI provided API."]
pub type NON_OP_W<'a, const O: u8> = crate::BitWriter<'a, u32, FSM_STAT_SPEC, bool, O>;
#[doc = "Field `RESERVED3` reader - 31:3\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED3_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED3` writer - 31:3\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FSM_STAT_SPEC, u32, u32, 29, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn inv_dat(&self) -> INV_DAT_R {
        INV_DAT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ovr_pul_cnt(&self) -> OVR_PUL_CNT_R {
        OVR_PUL_CNT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn non_op(&self) -> NON_OP_R {
        NON_OP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:31 - 31:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved3(&self) -> RESERVED3_R {
        RESERVED3_R::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn inv_dat(&mut self) -> INV_DAT_W<0> {
        INV_DAT_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn ovr_pul_cnt(&mut self) -> OVR_PUL_CNT_W<1> {
        OVR_PUL_CNT_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn non_op(&mut self) -> NON_OP_W<2> {
        NON_OP_W::new(self)
    }
    #[doc = "Bits 3:31 - 31:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> RESERVED3_W<3> {
        RESERVED3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsm_stat](index.html) module"]
pub struct FSM_STAT_SPEC;
impl crate::RegisterSpec for FSM_STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fsm_stat::R](R) reader structure"]
impl crate::Readable for FSM_STAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fsm_stat::W](W) writer structure"]
impl crate::Writable for FSM_STAT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FSM_STAT to value 0x04"]
impl crate::Resettable for FSM_STAT_SPEC {
    const RESET_VALUE: Self::Ux = 0x04;
}
