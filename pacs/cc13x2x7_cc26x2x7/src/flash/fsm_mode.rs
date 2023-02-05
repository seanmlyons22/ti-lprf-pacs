#[doc = "Register `FSM_MODE` reader"]
pub struct R(crate::R<FSM_MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FSM_MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FSM_MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FSM_MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FSM_MODE` writer"]
pub struct W(crate::W<FSM_MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FSM_MODE_SPEC>;
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
impl From<crate::W<FSM_MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FSM_MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMD` reader - 2:0\\]
Internal. Only to be used through TI provided API."]
pub type CMD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CMD` writer - 2:0\\]
Internal. Only to be used through TI provided API."]
pub type CMD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FSM_MODE_SPEC, u8, u8, 3, O>;
#[doc = "Field `MODE` reader - 5:3\\]
Internal. Only to be used through TI provided API."]
pub type MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MODE` writer - 5:3\\]
Internal. Only to be used through TI provided API."]
pub type MODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FSM_MODE_SPEC, u8, u8, 3, O>;
#[doc = "Field `SAV_ERA_MODE` reader - 8:6\\]
Internal. Only to be used through TI provided API."]
pub type SAV_ERA_MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SAV_ERA_MODE` writer - 8:6\\]
Internal. Only to be used through TI provided API."]
pub type SAV_ERA_MODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FSM_MODE_SPEC, u8, u8, 3, O>;
#[doc = "Field `SAV_PGM_CMD` reader - 11:9\\]
Internal. Only to be used through TI provided API."]
pub type SAV_PGM_CMD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SAV_PGM_CMD` writer - 11:9\\]
Internal. Only to be used through TI provided API."]
pub type SAV_PGM_CMD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FSM_MODE_SPEC, u8, u8, 3, O>;
#[doc = "Field `SUBMODE` reader - 13:12\\]
Internal. Only to be used through TI provided API."]
pub type SUBMODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SUBMODE` writer - 13:12\\]
Internal. Only to be used through TI provided API."]
pub type SUBMODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FSM_MODE_SPEC, u8, u8, 2, O>;
#[doc = "Field `ERA_SUBMODE` reader - 15:14\\]
Internal. Only to be used through TI provided API."]
pub type ERA_SUBMODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ERA_SUBMODE` writer - 15:14\\]
Internal. Only to be used through TI provided API."]
pub type ERA_SUBMODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FSM_MODE_SPEC, u8, u8, 2, O>;
#[doc = "Field `PGM_SUBMODE` reader - 17:16\\]
Internal. Only to be used through TI provided API."]
pub type PGM_SUBMODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PGM_SUBMODE` writer - 17:16\\]
Internal. Only to be used through TI provided API."]
pub type PGM_SUBMODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FSM_MODE_SPEC, u8, u8, 2, O>;
#[doc = "Field `RDV_SUBMODE` reader - 19:18\\]
Internal. Only to be used through TI provided API."]
pub type RDV_SUBMODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RDV_SUBMODE` writer - 19:18\\]
Internal. Only to be used through TI provided API."]
pub type RDV_SUBMODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FSM_MODE_SPEC, u8, u8, 2, O>;
#[doc = "Field `RESERVED20` reader - 31:20\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED20_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RESERVED20` writer - 31:20\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED20_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FSM_MODE_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn cmd(&self) -> CMD_R {
        CMD_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - 5:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - 8:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn sav_era_mode(&self) -> SAV_ERA_MODE_R {
        SAV_ERA_MODE_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - 11:9\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn sav_pgm_cmd(&self) -> SAV_PGM_CMD_R {
        SAV_PGM_CMD_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:13 - 13:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn submode(&self) -> SUBMODE_R {
        SUBMODE_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - 15:14\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn era_submode(&self) -> ERA_SUBMODE_R {
        ERA_SUBMODE_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn pgm_submode(&self) -> PGM_SUBMODE_R {
        PGM_SUBMODE_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - 19:18\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rdv_submode(&self) -> RDV_SUBMODE_R {
        RDV_SUBMODE_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:31 - 31:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved20(&self) -> RESERVED20_R {
        RESERVED20_R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn cmd(&mut self) -> CMD_W<0> {
        CMD_W::new(self)
    }
    #[doc = "Bits 3:5 - 5:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<3> {
        MODE_W::new(self)
    }
    #[doc = "Bits 6:8 - 8:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn sav_era_mode(&mut self) -> SAV_ERA_MODE_W<6> {
        SAV_ERA_MODE_W::new(self)
    }
    #[doc = "Bits 9:11 - 11:9\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn sav_pgm_cmd(&mut self) -> SAV_PGM_CMD_W<9> {
        SAV_PGM_CMD_W::new(self)
    }
    #[doc = "Bits 12:13 - 13:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn submode(&mut self) -> SUBMODE_W<12> {
        SUBMODE_W::new(self)
    }
    #[doc = "Bits 14:15 - 15:14\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn era_submode(&mut self) -> ERA_SUBMODE_W<14> {
        ERA_SUBMODE_W::new(self)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn pgm_submode(&mut self) -> PGM_SUBMODE_W<16> {
        PGM_SUBMODE_W::new(self)
    }
    #[doc = "Bits 18:19 - 19:18\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn rdv_submode(&mut self) -> RDV_SUBMODE_W<18> {
        RDV_SUBMODE_W::new(self)
    }
    #[doc = "Bits 20:31 - 31:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved20(&mut self) -> RESERVED20_W<20> {
        RESERVED20_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsm_mode](index.html) module"]
pub struct FSM_MODE_SPEC;
impl crate::RegisterSpec for FSM_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fsm_mode::R](R) reader structure"]
impl crate::Readable for FSM_MODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fsm_mode::W](W) writer structure"]
impl crate::Writable for FSM_MODE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FSM_MODE to value 0"]
impl crate::Resettable for FSM_MODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
