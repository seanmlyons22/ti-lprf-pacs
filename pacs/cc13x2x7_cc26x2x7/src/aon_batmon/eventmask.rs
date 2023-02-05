#[doc = "Register `EVENTMASK` reader"]
pub struct R(crate::R<EVENTMASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVENTMASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVENTMASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVENTMASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EVENTMASK` writer"]
pub struct W(crate::W<EVENTMASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVENTMASK_SPEC>;
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
impl From<crate::W<EVENTMASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EVENTMASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BATT_OVER_UL_MASK` reader - 0:0\\]
1: EVENT.BATT_OVER_UL contributes to combined event from BATMON 0: EVENT.BATT_OVER_UL does not contribute to combined event from BATMON"]
pub type BATT_OVER_UL_MASK_R = crate::BitReader<bool>;
#[doc = "Field `BATT_OVER_UL_MASK` writer - 0:0\\]
1: EVENT.BATT_OVER_UL contributes to combined event from BATMON 0: EVENT.BATT_OVER_UL does not contribute to combined event from BATMON"]
pub type BATT_OVER_UL_MASK_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVENTMASK_SPEC, bool, O>;
#[doc = "Field `BATT_BELOW_LL_MASK` reader - 1:1\\]
1: EVENT.BATT_BELOW_LL contributes to combined event from BATMON 0: EVENT.BATT_BELOW_LL does not contribute to combined event from BATMON"]
pub type BATT_BELOW_LL_MASK_R = crate::BitReader<bool>;
#[doc = "Field `BATT_BELOW_LL_MASK` writer - 1:1\\]
1: EVENT.BATT_BELOW_LL contributes to combined event from BATMON 0: EVENT.BATT_BELOW_LL does not contribute to combined event from BATMON"]
pub type BATT_BELOW_LL_MASK_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVENTMASK_SPEC, bool, O>;
#[doc = "Field `TEMP_OVER_UL_MASK` reader - 2:2\\]
1: EVENT.TEMP_OVER_UL contributes to combined event from BATMON 0: EVENT.TEMP_OVER_UL does not contribute to combined event from BATMON"]
pub type TEMP_OVER_UL_MASK_R = crate::BitReader<bool>;
#[doc = "Field `TEMP_OVER_UL_MASK` writer - 2:2\\]
1: EVENT.TEMP_OVER_UL contributes to combined event from BATMON 0: EVENT.TEMP_OVER_UL does not contribute to combined event from BATMON"]
pub type TEMP_OVER_UL_MASK_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVENTMASK_SPEC, bool, O>;
#[doc = "Field `TEMP_BELOW_LL_MASK` reader - 3:3\\]
1: EVENT.TEMP_BELOW_LL contributes to combined event from BATMON 0: EVENT.TEMP_BELOW_LL does not contribute to combined event from BATMON"]
pub type TEMP_BELOW_LL_MASK_R = crate::BitReader<bool>;
#[doc = "Field `TEMP_BELOW_LL_MASK` writer - 3:3\\]
1: EVENT.TEMP_BELOW_LL contributes to combined event from BATMON 0: EVENT.TEMP_BELOW_LL does not contribute to combined event from BATMON"]
pub type TEMP_BELOW_LL_MASK_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVENTMASK_SPEC, bool, O>;
#[doc = "Field `BATT_UPDATE_MASK` reader - 4:4\\]
1: EVENT.BATT_UPDATE contributes to combined event from BATMON 0: EVENT.BATT_UPDATE does not contribute to combined event from BATMON"]
pub type BATT_UPDATE_MASK_R = crate::BitReader<bool>;
#[doc = "Field `BATT_UPDATE_MASK` writer - 4:4\\]
1: EVENT.BATT_UPDATE contributes to combined event from BATMON 0: EVENT.BATT_UPDATE does not contribute to combined event from BATMON"]
pub type BATT_UPDATE_MASK_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVENTMASK_SPEC, bool, O>;
#[doc = "Field `TEMP_UPDATE_MASK` reader - 5:5\\]
1: EVENT.TEMP_UPDATE contributes to combined event from BATMON 0: EVENT.TEMP_UPDATE does not contribute to combined event from BATMON"]
pub type TEMP_UPDATE_MASK_R = crate::BitReader<bool>;
#[doc = "Field `TEMP_UPDATE_MASK` writer - 5:5\\]
1: EVENT.TEMP_UPDATE contributes to combined event from BATMON 0: EVENT.TEMP_UPDATE does not contribute to combined event from BATMON"]
pub type TEMP_UPDATE_MASK_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVENTMASK_SPEC, bool, O>;
#[doc = "Field `RESERVED6` reader - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED6_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED6` writer - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED6_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EVENTMASK_SPEC, u32, u32, 26, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
1: EVENT.BATT_OVER_UL contributes to combined event from BATMON 0: EVENT.BATT_OVER_UL does not contribute to combined event from BATMON"]
    #[inline(always)]
    pub fn batt_over_ul_mask(&self) -> BATT_OVER_UL_MASK_R {
        BATT_OVER_UL_MASK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
1: EVENT.BATT_BELOW_LL contributes to combined event from BATMON 0: EVENT.BATT_BELOW_LL does not contribute to combined event from BATMON"]
    #[inline(always)]
    pub fn batt_below_ll_mask(&self) -> BATT_BELOW_LL_MASK_R {
        BATT_BELOW_LL_MASK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
1: EVENT.TEMP_OVER_UL contributes to combined event from BATMON 0: EVENT.TEMP_OVER_UL does not contribute to combined event from BATMON"]
    #[inline(always)]
    pub fn temp_over_ul_mask(&self) -> TEMP_OVER_UL_MASK_R {
        TEMP_OVER_UL_MASK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
1: EVENT.TEMP_BELOW_LL contributes to combined event from BATMON 0: EVENT.TEMP_BELOW_LL does not contribute to combined event from BATMON"]
    #[inline(always)]
    pub fn temp_below_ll_mask(&self) -> TEMP_BELOW_LL_MASK_R {
        TEMP_BELOW_LL_MASK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
1: EVENT.BATT_UPDATE contributes to combined event from BATMON 0: EVENT.BATT_UPDATE does not contribute to combined event from BATMON"]
    #[inline(always)]
    pub fn batt_update_mask(&self) -> BATT_UPDATE_MASK_R {
        BATT_UPDATE_MASK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
1: EVENT.TEMP_UPDATE contributes to combined event from BATMON 0: EVENT.TEMP_UPDATE does not contribute to combined event from BATMON"]
    #[inline(always)]
    pub fn temp_update_mask(&self) -> TEMP_UPDATE_MASK_R {
        TEMP_UPDATE_MASK_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:31 - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved6(&self) -> RESERVED6_R {
        RESERVED6_R::new((self.bits >> 6) & 0x03ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
1: EVENT.BATT_OVER_UL contributes to combined event from BATMON 0: EVENT.BATT_OVER_UL does not contribute to combined event from BATMON"]
    #[inline(always)]
    #[must_use]
    pub fn batt_over_ul_mask(&mut self) -> BATT_OVER_UL_MASK_W<0> {
        BATT_OVER_UL_MASK_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
1: EVENT.BATT_BELOW_LL contributes to combined event from BATMON 0: EVENT.BATT_BELOW_LL does not contribute to combined event from BATMON"]
    #[inline(always)]
    #[must_use]
    pub fn batt_below_ll_mask(&mut self) -> BATT_BELOW_LL_MASK_W<1> {
        BATT_BELOW_LL_MASK_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
1: EVENT.TEMP_OVER_UL contributes to combined event from BATMON 0: EVENT.TEMP_OVER_UL does not contribute to combined event from BATMON"]
    #[inline(always)]
    #[must_use]
    pub fn temp_over_ul_mask(&mut self) -> TEMP_OVER_UL_MASK_W<2> {
        TEMP_OVER_UL_MASK_W::new(self)
    }
    #[doc = "Bit 3 - 3:3\\]
1: EVENT.TEMP_BELOW_LL contributes to combined event from BATMON 0: EVENT.TEMP_BELOW_LL does not contribute to combined event from BATMON"]
    #[inline(always)]
    #[must_use]
    pub fn temp_below_ll_mask(&mut self) -> TEMP_BELOW_LL_MASK_W<3> {
        TEMP_BELOW_LL_MASK_W::new(self)
    }
    #[doc = "Bit 4 - 4:4\\]
1: EVENT.BATT_UPDATE contributes to combined event from BATMON 0: EVENT.BATT_UPDATE does not contribute to combined event from BATMON"]
    #[inline(always)]
    #[must_use]
    pub fn batt_update_mask(&mut self) -> BATT_UPDATE_MASK_W<4> {
        BATT_UPDATE_MASK_W::new(self)
    }
    #[doc = "Bit 5 - 5:5\\]
1: EVENT.TEMP_UPDATE contributes to combined event from BATMON 0: EVENT.TEMP_UPDATE does not contribute to combined event from BATMON"]
    #[inline(always)]
    #[must_use]
    pub fn temp_update_mask(&mut self) -> TEMP_UPDATE_MASK_W<5> {
        TEMP_UPDATE_MASK_W::new(self)
    }
    #[doc = "Bits 6:31 - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved6(&mut self) -> RESERVED6_W<6> {
        RESERVED6_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Event Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eventmask](index.html) module"]
pub struct EVENTMASK_SPEC;
impl crate::RegisterSpec for EVENTMASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eventmask::R](R) reader structure"]
impl crate::Readable for EVENTMASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eventmask::W](W) writer structure"]
impl crate::Writable for EVENTMASK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EVENTMASK to value 0"]
impl crate::Resettable for EVENTMASK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
