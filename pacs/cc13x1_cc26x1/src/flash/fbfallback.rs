#[doc = "Register `FBFALLBACK` reader"]
pub struct R(crate::R<FBFALLBACK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FBFALLBACK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FBFALLBACK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FBFALLBACK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FBFALLBACK` writer"]
pub struct W(crate::W<FBFALLBACK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FBFALLBACK_SPEC>;
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
impl From<crate::W<FBFALLBACK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FBFALLBACK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BANKPWR0` reader - 1:0\\]
Internal. Only to be used through TI provided API."]
pub type BANKPWR0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BANKPWR0` writer - 1:0\\]
Internal. Only to be used through TI provided API."]
pub type BANKPWR0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FBFALLBACK_SPEC, u8, u8, 2, O>;
#[doc = "Field `BANKPWR1` reader - 3:2\\]
Internal. Only to be used through TI provided API."]
pub type BANKPWR1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BANKPWR1` writer - 3:2\\]
Internal. Only to be used through TI provided API."]
pub type BANKPWR1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FBFALLBACK_SPEC, u8, u8, 2, O>;
#[doc = "Field `BANKPWR2` reader - 5:4\\]
Internal. Only to be used through TI provided API."]
pub type BANKPWR2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BANKPWR2` writer - 5:4\\]
Internal. Only to be used through TI provided API."]
pub type BANKPWR2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FBFALLBACK_SPEC, u8, u8, 2, O>;
#[doc = "Field `BANKPWR3` reader - 7:6\\]
Internal. Only to be used through TI provided API."]
pub type BANKPWR3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BANKPWR3` writer - 7:6\\]
Internal. Only to be used through TI provided API."]
pub type BANKPWR3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FBFALLBACK_SPEC, u8, u8, 2, O>;
#[doc = "Field `BANKPWR4` reader - 9:8\\]
Internal. Only to be used through TI provided API."]
pub type BANKPWR4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BANKPWR4` writer - 9:8\\]
Internal. Only to be used through TI provided API."]
pub type BANKPWR4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FBFALLBACK_SPEC, u8, u8, 2, O>;
#[doc = "Field `BANKPWR5` reader - 11:10\\]
Internal. Only to be used through TI provided API."]
pub type BANKPWR5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BANKPWR5` writer - 11:10\\]
Internal. Only to be used through TI provided API."]
pub type BANKPWR5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FBFALLBACK_SPEC, u8, u8, 2, O>;
#[doc = "Field `BANKPWR6` reader - 13:12\\]
Internal. Only to be used through TI provided API."]
pub type BANKPWR6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BANKPWR6` writer - 13:12\\]
Internal. Only to be used through TI provided API."]
pub type BANKPWR6_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FBFALLBACK_SPEC, u8, u8, 2, O>;
#[doc = "Field `BANKPWR7` reader - 15:14\\]
Internal. Only to be used through TI provided API."]
pub type BANKPWR7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BANKPWR7` writer - 15:14\\]
Internal. Only to be used through TI provided API."]
pub type BANKPWR7_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FBFALLBACK_SPEC, u8, u8, 2, O>;
#[doc = "Field `REG_PWRSAV` reader - 19:16\\]
Internal. Only to be used through TI provided API."]
pub type REG_PWRSAV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `REG_PWRSAV` writer - 19:16\\]
Internal. Only to be used through TI provided API."]
pub type REG_PWRSAV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FBFALLBACK_SPEC, u8, u8, 4, O>;
#[doc = "Field `RESERVED20` reader - 23:20\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED20_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED20` writer - 23:20\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED20_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FBFALLBACK_SPEC, u8, u8, 4, O>;
#[doc = "Field `FSM_PWRSAV` reader - 27:24\\]
Internal. Only to be used through TI provided API."]
pub type FSM_PWRSAV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FSM_PWRSAV` writer - 27:24\\]
Internal. Only to be used through TI provided API."]
pub type FSM_PWRSAV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FBFALLBACK_SPEC, u8, u8, 4, O>;
#[doc = "Field `RESERVED28` reader - 31:28\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED28_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED28` writer - 31:28\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED28_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FBFALLBACK_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn bankpwr0(&self) -> BANKPWR0_R {
        BANKPWR0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - 3:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn bankpwr1(&self) -> BANKPWR1_R {
        BANKPWR1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - 5:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn bankpwr2(&self) -> BANKPWR2_R {
        BANKPWR2_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn bankpwr3(&self) -> BANKPWR3_R {
        BANKPWR3_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn bankpwr4(&self) -> BANKPWR4_R {
        BANKPWR4_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - 11:10\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn bankpwr5(&self) -> BANKPWR5_R {
        BANKPWR5_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - 13:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn bankpwr6(&self) -> BANKPWR6_R {
        BANKPWR6_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - 15:14\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn bankpwr7(&self) -> BANKPWR7_R {
        BANKPWR7_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reg_pwrsav(&self) -> REG_PWRSAV_R {
        REG_PWRSAV_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved20(&self) -> RESERVED20_R {
        RESERVED20_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn fsm_pwrsav(&self) -> FSM_PWRSAV_R {
        FSM_PWRSAV_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved28(&self) -> RESERVED28_R {
        RESERVED28_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn bankpwr0(&mut self) -> BANKPWR0_W<0> {
        BANKPWR0_W::new(self)
    }
    #[doc = "Bits 2:3 - 3:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn bankpwr1(&mut self) -> BANKPWR1_W<2> {
        BANKPWR1_W::new(self)
    }
    #[doc = "Bits 4:5 - 5:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn bankpwr2(&mut self) -> BANKPWR2_W<4> {
        BANKPWR2_W::new(self)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn bankpwr3(&mut self) -> BANKPWR3_W<6> {
        BANKPWR3_W::new(self)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn bankpwr4(&mut self) -> BANKPWR4_W<8> {
        BANKPWR4_W::new(self)
    }
    #[doc = "Bits 10:11 - 11:10\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn bankpwr5(&mut self) -> BANKPWR5_W<10> {
        BANKPWR5_W::new(self)
    }
    #[doc = "Bits 12:13 - 13:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn bankpwr6(&mut self) -> BANKPWR6_W<12> {
        BANKPWR6_W::new(self)
    }
    #[doc = "Bits 14:15 - 15:14\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn bankpwr7(&mut self) -> BANKPWR7_W<14> {
        BANKPWR7_W::new(self)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reg_pwrsav(&mut self) -> REG_PWRSAV_W<16> {
        REG_PWRSAV_W::new(self)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved20(&mut self) -> RESERVED20_W<20> {
        RESERVED20_W::new(self)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn fsm_pwrsav(&mut self) -> FSM_PWRSAV_W<24> {
        FSM_PWRSAV_W::new(self)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved28(&mut self) -> RESERVED28_W<28> {
        RESERVED28_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fbfallback](index.html) module"]
pub struct FBFALLBACK_SPEC;
impl crate::RegisterSpec for FBFALLBACK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fbfallback::R](R) reader structure"]
impl crate::Readable for FBFALLBACK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fbfallback::W](W) writer structure"]
impl crate::Writable for FBFALLBACK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FBFALLBACK to value 0x0505_ffff"]
impl crate::Resettable for FBFALLBACK_SPEC {
    const RESET_VALUE: Self::Ux = 0x0505_ffff;
}
