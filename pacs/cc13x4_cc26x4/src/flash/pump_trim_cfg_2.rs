#[doc = "Register `PUMP_TRIM_CFG_2` reader"]
pub struct R(crate::R<PUMP_TRIM_CFG_2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PUMP_TRIM_CFG_2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PUMP_TRIM_CFG_2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PUMP_TRIM_CFG_2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PUMP_TRIM_CFG_2` writer"]
pub struct W(crate::W<PUMP_TRIM_CFG_2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PUMP_TRIM_CFG_2_SPEC>;
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
impl From<crate::W<PUMP_TRIM_CFG_2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PUMP_TRIM_CFG_2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VINHICCORCT` reader - 3:0\\]
Internal. Only to be used through TI provided API."]
pub type VINHICCORCT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VINHICCORCT` writer - 3:0\\]
Internal. Only to be used through TI provided API."]
pub type VINHICCORCT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PUMP_TRIM_CFG_2_SPEC, u8, u8, 4, O>;
#[doc = "Field `VINLOWCCORCT` reader - 8:4\\]
Internal. Only to be used through TI provided API."]
pub type VINLOWCCORCT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VINLOWCCORCT` writer - 8:4\\]
Internal. Only to be used through TI provided API."]
pub type VINLOWCCORCT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PUMP_TRIM_CFG_2_SPEC, u8, u8, 5, O>;
#[doc = "Field `VREADCT` reader - 13:9\\]
Internal. Only to be used through TI provided API."]
pub type VREADCT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VREADCT` writer - 13:9\\]
Internal. Only to be used through TI provided API."]
pub type VREADCT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PUMP_TRIM_CFG_2_SPEC, u8, u8, 5, O>;
#[doc = "Field `VSLCT` reader - 19:14\\]
Internal. Only to be used through TI provided API."]
pub type VSLCT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VSLCT` writer - 19:14\\]
Internal. Only to be used through TI provided API."]
pub type VSLCT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PUMP_TRIM_CFG_2_SPEC, u8, u8, 6, O>;
#[doc = "Field `VWLCT` reader - 25:20\\]
Internal. Only to be used through TI provided API."]
pub type VWLCT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VWLCT` writer - 25:20\\]
Internal. Only to be used through TI provided API."]
pub type VWLCT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PUMP_TRIM_CFG_2_SPEC, u8, u8, 6, O>;
#[doc = "Field `RESERVED6` reader - 31:26\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED6` writer - 31:26\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED6_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PUMP_TRIM_CFG_2_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vinhiccorct(&self) -> VINHICCORCT_R {
        VINHICCORCT_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:8 - 8:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vinlowccorct(&self) -> VINLOWCCORCT_R {
        VINLOWCCORCT_R::new(((self.bits >> 4) & 0x1f) as u8)
    }
    #[doc = "Bits 9:13 - 13:9\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vreadct(&self) -> VREADCT_R {
        VREADCT_R::new(((self.bits >> 9) & 0x1f) as u8)
    }
    #[doc = "Bits 14:19 - 19:14\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vslct(&self) -> VSLCT_R {
        VSLCT_R::new(((self.bits >> 14) & 0x3f) as u8)
    }
    #[doc = "Bits 20:25 - 25:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vwlct(&self) -> VWLCT_R {
        VWLCT_R::new(((self.bits >> 20) & 0x3f) as u8)
    }
    #[doc = "Bits 26:31 - 31:26\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved6(&self) -> RESERVED6_R {
        RESERVED6_R::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn vinhiccorct(&mut self) -> VINHICCORCT_W<0> {
        VINHICCORCT_W::new(self)
    }
    #[doc = "Bits 4:8 - 8:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn vinlowccorct(&mut self) -> VINLOWCCORCT_W<4> {
        VINLOWCCORCT_W::new(self)
    }
    #[doc = "Bits 9:13 - 13:9\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn vreadct(&mut self) -> VREADCT_W<9> {
        VREADCT_W::new(self)
    }
    #[doc = "Bits 14:19 - 19:14\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn vslct(&mut self) -> VSLCT_W<14> {
        VSLCT_W::new(self)
    }
    #[doc = "Bits 20:25 - 25:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn vwlct(&mut self) -> VWLCT_W<20> {
        VWLCT_W::new(self)
    }
    #[doc = "Bits 26:31 - 31:26\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved6(&mut self) -> RESERVED6_W<26> {
        RESERVED6_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pump_trim_cfg_2](index.html) module"]
pub struct PUMP_TRIM_CFG_2_SPEC;
impl crate::RegisterSpec for PUMP_TRIM_CFG_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pump_trim_cfg_2::R](R) reader structure"]
impl crate::Readable for PUMP_TRIM_CFG_2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pump_trim_cfg_2::W](W) writer structure"]
impl crate::Writable for PUMP_TRIM_CFG_2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PUMP_TRIM_CFG_2 to value 0x0055_9400"]
impl crate::Resettable for PUMP_TRIM_CFG_2_SPEC {
    const RESET_VALUE: Self::Ux = 0x0055_9400;
}
