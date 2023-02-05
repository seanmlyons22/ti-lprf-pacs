#[doc = "Register `BANK1_TRIM_CFG_1` reader"]
pub struct R(crate::R<BANK1_TRIM_CFG_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BANK1_TRIM_CFG_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BANK1_TRIM_CFG_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BANK1_TRIM_CFG_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BANK1_TRIM_CFG_1` writer"]
pub struct W(crate::W<BANK1_TRIM_CFG_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BANK1_TRIM_CFG_1_SPEC>;
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
impl From<crate::W<BANK1_TRIM_CFG_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BANK1_TRIM_CFG_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REDSWENW0` reader - 0:0\\]
Internal. Only to be used through TI provided API."]
pub type REDSWENW0_R = crate::BitReader<bool>;
#[doc = "Field `REDSWENW0` writer - 0:0\\]
Internal. Only to be used through TI provided API."]
pub type REDSWENW0_W<'a, const O: u8> = crate::BitWriter<'a, u32, BANK1_TRIM_CFG_1_SPEC, bool, O>;
#[doc = "Field `REDSWENW1` reader - 1:1\\]
Internal. Only to be used through TI provided API."]
pub type REDSWENW1_R = crate::BitReader<bool>;
#[doc = "Field `REDSWENW1` writer - 1:1\\]
Internal. Only to be used through TI provided API."]
pub type REDSWENW1_W<'a, const O: u8> = crate::BitWriter<'a, u32, BANK1_TRIM_CFG_1_SPEC, bool, O>;
#[doc = "Field `REDSWENW2` reader - 2:2\\]
Internal. Only to be used through TI provided API."]
pub type REDSWENW2_R = crate::BitReader<bool>;
#[doc = "Field `REDSWENW2` writer - 2:2\\]
Internal. Only to be used through TI provided API."]
pub type REDSWENW2_W<'a, const O: u8> = crate::BitWriter<'a, u32, BANK1_TRIM_CFG_1_SPEC, bool, O>;
#[doc = "Field `REDSWENW3` reader - 3:3\\]
Internal. Only to be used through TI provided API."]
pub type REDSWENW3_R = crate::BitReader<bool>;
#[doc = "Field `REDSWENW3` writer - 3:3\\]
Internal. Only to be used through TI provided API."]
pub type REDSWENW3_W<'a, const O: u8> = crate::BitWriter<'a, u32, BANK1_TRIM_CFG_1_SPEC, bool, O>;
#[doc = "Field `REDSWSELW0` reader - 9:4\\]
Internal. Only to be used through TI provided API."]
pub type REDSWSELW0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `REDSWSELW0` writer - 9:4\\]
Internal. Only to be used through TI provided API."]
pub type REDSWSELW0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, BANK1_TRIM_CFG_1_SPEC, u8, u8, 6, O>;
#[doc = "Field `REDSWSELW1` reader - 15:10\\]
Internal. Only to be used through TI provided API."]
pub type REDSWSELW1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `REDSWSELW1` writer - 15:10\\]
Internal. Only to be used through TI provided API."]
pub type REDSWSELW1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, BANK1_TRIM_CFG_1_SPEC, u8, u8, 6, O>;
#[doc = "Field `REDSWSELW2` reader - 21:16\\]
Internal. Only to be used through TI provided API."]
pub type REDSWSELW2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `REDSWSELW2` writer - 21:16\\]
Internal. Only to be used through TI provided API."]
pub type REDSWSELW2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, BANK1_TRIM_CFG_1_SPEC, u8, u8, 6, O>;
#[doc = "Field `REDSWSELW3` reader - 27:22\\]
Internal. Only to be used through TI provided API."]
pub type REDSWSELW3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `REDSWSELW3` writer - 27:22\\]
Internal. Only to be used through TI provided API."]
pub type REDSWSELW3_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, BANK1_TRIM_CFG_1_SPEC, u8, u8, 6, O>;
#[doc = "Field `RESERVED6` reader - 31:28\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED6` writer - 31:28\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED6_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, BANK1_TRIM_CFG_1_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn redswenw0(&self) -> REDSWENW0_R {
        REDSWENW0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn redswenw1(&self) -> REDSWENW1_R {
        REDSWENW1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn redswenw2(&self) -> REDSWENW2_R {
        REDSWENW2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn redswenw3(&self) -> REDSWENW3_R {
        REDSWENW3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:9 - 9:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn redswselw0(&self) -> REDSWSELW0_R {
        REDSWSELW0_R::new(((self.bits >> 4) & 0x3f) as u8)
    }
    #[doc = "Bits 10:15 - 15:10\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn redswselw1(&self) -> REDSWSELW1_R {
        REDSWSELW1_R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - 21:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn redswselw2(&self) -> REDSWSELW2_R {
        REDSWSELW2_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 22:27 - 27:22\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn redswselw3(&self) -> REDSWSELW3_R {
        REDSWSELW3_R::new(((self.bits >> 22) & 0x3f) as u8)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved6(&self) -> RESERVED6_R {
        RESERVED6_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn redswenw0(&mut self) -> REDSWENW0_W<0> {
        REDSWENW0_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn redswenw1(&mut self) -> REDSWENW1_W<1> {
        REDSWENW1_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn redswenw2(&mut self) -> REDSWENW2_W<2> {
        REDSWENW2_W::new(self)
    }
    #[doc = "Bit 3 - 3:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn redswenw3(&mut self) -> REDSWENW3_W<3> {
        REDSWENW3_W::new(self)
    }
    #[doc = "Bits 4:9 - 9:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn redswselw0(&mut self) -> REDSWSELW0_W<4> {
        REDSWSELW0_W::new(self)
    }
    #[doc = "Bits 10:15 - 15:10\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn redswselw1(&mut self) -> REDSWSELW1_W<10> {
        REDSWSELW1_W::new(self)
    }
    #[doc = "Bits 16:21 - 21:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn redswselw2(&mut self) -> REDSWSELW2_W<16> {
        REDSWSELW2_W::new(self)
    }
    #[doc = "Bits 22:27 - 27:22\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn redswselw3(&mut self) -> REDSWSELW3_W<22> {
        REDSWSELW3_W::new(self)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved6(&mut self) -> RESERVED6_W<28> {
        RESERVED6_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bank1_trim_cfg_1](index.html) module"]
pub struct BANK1_TRIM_CFG_1_SPEC;
impl crate::RegisterSpec for BANK1_TRIM_CFG_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bank1_trim_cfg_1::R](R) reader structure"]
impl crate::Readable for BANK1_TRIM_CFG_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bank1_trim_cfg_1::W](W) writer structure"]
impl crate::Writable for BANK1_TRIM_CFG_1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BANK1_TRIM_CFG_1 to value 0"]
impl crate::Resettable for BANK1_TRIM_CFG_1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
