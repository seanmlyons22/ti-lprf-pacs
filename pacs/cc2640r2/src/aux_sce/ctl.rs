#[doc = "Register `CTL` reader"]
pub struct R(crate::R<CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTL` writer"]
pub struct W(crate::W<CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTL_SPEC>;
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
impl From<crate::W<CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLK_EN` reader - 0:0\\]
Internal. Only to be used through TI provided API."]
pub type CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `CLK_EN` writer - 0:0\\]
Internal. Only to be used through TI provided API."]
pub type CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
#[doc = "Field `SUSPEND` reader - 1:1\\]
Internal. Only to be used through TI provided API."]
pub type SUSPEND_R = crate::BitReader<bool>;
#[doc = "Field `SUSPEND` writer - 1:1\\]
Internal. Only to be used through TI provided API."]
pub type SUSPEND_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
#[doc = "Field `SINGLE_STEP` reader - 2:2\\]
Internal. Only to be used through TI provided API."]
pub type SINGLE_STEP_R = crate::BitReader<bool>;
#[doc = "Field `SINGLE_STEP` writer - 2:2\\]
Internal. Only to be used through TI provided API."]
pub type SINGLE_STEP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
#[doc = "Field `RESTART` reader - 3:3\\]
Internal. Only to be used through TI provided API."]
pub type RESTART_R = crate::BitReader<bool>;
#[doc = "Field `RESTART` writer - 3:3\\]
Internal. Only to be used through TI provided API."]
pub type RESTART_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
#[doc = "Field `FORCE_WU_HIGH` reader - 4:4\\]
Internal. Only to be used through TI provided API."]
pub type FORCE_WU_HIGH_R = crate::BitReader<bool>;
#[doc = "Field `FORCE_WU_HIGH` writer - 4:4\\]
Internal. Only to be used through TI provided API."]
pub type FORCE_WU_HIGH_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
#[doc = "Field `FORCE_WU_LOW` reader - 5:5\\]
Internal. Only to be used through TI provided API."]
pub type FORCE_WU_LOW_R = crate::BitReader<bool>;
#[doc = "Field `FORCE_WU_LOW` writer - 5:5\\]
Internal. Only to be used through TI provided API."]
pub type FORCE_WU_LOW_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
#[doc = "Field `DBG_FREEZE_EN` reader - 6:6\\]
Internal. Only to be used through TI provided API."]
pub type DBG_FREEZE_EN_R = crate::BitReader<bool>;
#[doc = "Field `DBG_FREEZE_EN` writer - 6:6\\]
Internal. Only to be used through TI provided API."]
pub type DBG_FREEZE_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
#[doc = "Field `RESERVED7` reader - 7:7\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED7_R = crate::BitReader<bool>;
#[doc = "Field `RESERVED7` writer - 7:7\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED7_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
#[doc = "Field `RESET_VECTOR` reader - 11:8\\]
Internal. Only to be used through TI provided API."]
pub type RESET_VECTOR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESET_VECTOR` writer - 11:8\\]
Internal. Only to be used through TI provided API."]
pub type RESET_VECTOR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTL_SPEC, u8, u8, 4, O>;
#[doc = "Field `RESERVED12` reader - 15:12\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED12_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED12` writer - 15:12\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED12_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTL_SPEC, u8, u8, 4, O>;
#[doc = "Field `FORCE_EV_HIGH` reader - 23:16\\]
Internal. Only to be used through TI provided API."]
pub type FORCE_EV_HIGH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FORCE_EV_HIGH` writer - 23:16\\]
Internal. Only to be used through TI provided API."]
pub type FORCE_EV_HIGH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTL_SPEC, u8, u8, 8, O>;
#[doc = "Field `FORCE_EV_LOW` reader - 31:24\\]
Internal. Only to be used through TI provided API."]
pub type FORCE_EV_LOW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FORCE_EV_LOW` writer - 31:24\\]
Internal. Only to be used through TI provided API."]
pub type FORCE_EV_LOW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTL_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn suspend(&self) -> SUSPEND_R {
        SUSPEND_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn single_step(&self) -> SINGLE_STEP_R {
        SINGLE_STEP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn restart(&self) -> RESTART_R {
        RESTART_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn force_wu_high(&self) -> FORCE_WU_HIGH_R {
        FORCE_WU_HIGH_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn force_wu_low(&self) -> FORCE_WU_LOW_R {
        FORCE_WU_LOW_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dbg_freeze_en(&self) -> DBG_FREEZE_EN_R {
        DBG_FREEZE_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved7(&self) -> RESERVED7_R {
        RESERVED7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reset_vector(&self) -> RESET_VECTOR_R {
        RESET_VECTOR_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved12(&self) -> RESERVED12_R {
        RESERVED12_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn force_ev_high(&self) -> FORCE_EV_HIGH_R {
        FORCE_EV_HIGH_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn force_ev_low(&self) -> FORCE_EV_LOW_R {
        FORCE_EV_LOW_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn clk_en(&mut self) -> CLK_EN_W<0> {
        CLK_EN_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn suspend(&mut self) -> SUSPEND_W<1> {
        SUSPEND_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn single_step(&mut self) -> SINGLE_STEP_W<2> {
        SINGLE_STEP_W::new(self)
    }
    #[doc = "Bit 3 - 3:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn restart(&mut self) -> RESTART_W<3> {
        RESTART_W::new(self)
    }
    #[doc = "Bit 4 - 4:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn force_wu_high(&mut self) -> FORCE_WU_HIGH_W<4> {
        FORCE_WU_HIGH_W::new(self)
    }
    #[doc = "Bit 5 - 5:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn force_wu_low(&mut self) -> FORCE_WU_LOW_W<5> {
        FORCE_WU_LOW_W::new(self)
    }
    #[doc = "Bit 6 - 6:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn dbg_freeze_en(&mut self) -> DBG_FREEZE_EN_W<6> {
        DBG_FREEZE_EN_W::new(self)
    }
    #[doc = "Bit 7 - 7:7\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved7(&mut self) -> RESERVED7_W<7> {
        RESERVED7_W::new(self)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reset_vector(&mut self) -> RESET_VECTOR_W<8> {
        RESET_VECTOR_W::new(self)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved12(&mut self) -> RESERVED12_W<12> {
        RESERVED12_W::new(self)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn force_ev_high(&mut self) -> FORCE_EV_HIGH_W<16> {
        FORCE_EV_HIGH_W::new(self)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn force_ev_low(&mut self) -> FORCE_EV_LOW_W<24> {
        FORCE_EV_LOW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl](index.html) module"]
pub struct CTL_SPEC;
impl crate::RegisterSpec for CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctl::R](R) reader structure"]
impl crate::Readable for CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctl::W](W) writer structure"]
impl crate::Writable for CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTL to value 0"]
impl crate::Resettable for CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
