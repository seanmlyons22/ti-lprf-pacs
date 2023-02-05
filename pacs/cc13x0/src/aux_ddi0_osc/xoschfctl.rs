#[doc = "Register `XOSCHFCTL` reader"]
pub struct R(crate::R<XOSCHFCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<XOSCHFCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<XOSCHFCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<XOSCHFCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `XOSCHFCTL` writer"]
pub struct W(crate::W<XOSCHFCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<XOSCHFCTL_SPEC>;
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
impl From<crate::W<XOSCHFCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<XOSCHFCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LP_BUF_ITRIM` reader - 1:0\\]
Internal. Only to be used through TI provided API."]
pub type LP_BUF_ITRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LP_BUF_ITRIM` writer - 1:0\\]
Internal. Only to be used through TI provided API."]
pub type LP_BUF_ITRIM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, XOSCHFCTL_SPEC, u8, u8, 2, O>;
#[doc = "Field `HP_BUF_ITRIM` reader - 4:2\\]
Internal. Only to be used through TI provided API."]
pub type HP_BUF_ITRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HP_BUF_ITRIM` writer - 4:2\\]
Internal. Only to be used through TI provided API."]
pub type HP_BUF_ITRIM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, XOSCHFCTL_SPEC, u8, u8, 3, O>;
#[doc = "Field `RESERVED5` reader - 5:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED5_R = crate::BitReader<bool>;
#[doc = "Field `RESERVED5` writer - 5:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED5_W<'a, const O: u8> = crate::BitWriter<'a, u32, XOSCHFCTL_SPEC, bool, O>;
#[doc = "Field `BYPASS` reader - 6:6\\]
Internal. Only to be used through TI provided API."]
pub type BYPASS_R = crate::BitReader<bool>;
#[doc = "Field `BYPASS` writer - 6:6\\]
Internal. Only to be used through TI provided API."]
pub type BYPASS_W<'a, const O: u8> = crate::BitWriter<'a, u32, XOSCHFCTL_SPEC, bool, O>;
#[doc = "Field `RESERVED7` reader - 7:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED7_R = crate::BitReader<bool>;
#[doc = "Field `RESERVED7` writer - 7:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED7_W<'a, const O: u8> = crate::BitWriter<'a, u32, XOSCHFCTL_SPEC, bool, O>;
#[doc = "Field `PEAK_DET_ITRIM` reader - 9:8\\]
Internal. Only to be used through TI provided API."]
pub type PEAK_DET_ITRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PEAK_DET_ITRIM` writer - 9:8\\]
Internal. Only to be used through TI provided API."]
pub type PEAK_DET_ITRIM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, XOSCHFCTL_SPEC, u8, u8, 2, O>;
#[doc = "Field `RESERVED10` reader - 31:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED10_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED10` writer - 31:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED10_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, XOSCHFCTL_SPEC, u32, u32, 22, O>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn lp_buf_itrim(&self) -> LP_BUF_ITRIM_R {
        LP_BUF_ITRIM_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:4 - 4:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn hp_buf_itrim(&self) -> HP_BUF_ITRIM_R {
        HP_BUF_ITRIM_R::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bit 5 - 5:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved5(&self) -> RESERVED5_R {
        RESERVED5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn bypass(&self) -> BYPASS_R {
        BYPASS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved7(&self) -> RESERVED7_R {
        RESERVED7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn peak_det_itrim(&self) -> PEAK_DET_ITRIM_R {
        PEAK_DET_ITRIM_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:31 - 31:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved10(&self) -> RESERVED10_R {
        RESERVED10_R::new((self.bits >> 10) & 0x003f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn lp_buf_itrim(&mut self) -> LP_BUF_ITRIM_W<0> {
        LP_BUF_ITRIM_W::new(self)
    }
    #[doc = "Bits 2:4 - 4:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn hp_buf_itrim(&mut self) -> HP_BUF_ITRIM_W<2> {
        HP_BUF_ITRIM_W::new(self)
    }
    #[doc = "Bit 5 - 5:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved5(&mut self) -> RESERVED5_W<5> {
        RESERVED5_W::new(self)
    }
    #[doc = "Bit 6 - 6:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn bypass(&mut self) -> BYPASS_W<6> {
        BYPASS_W::new(self)
    }
    #[doc = "Bit 7 - 7:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved7(&mut self) -> RESERVED7_W<7> {
        RESERVED7_W::new(self)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn peak_det_itrim(&mut self) -> PEAK_DET_ITRIM_W<8> {
        PEAK_DET_ITRIM_W::new(self)
    }
    #[doc = "Bits 10:31 - 31:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved10(&mut self) -> RESERVED10_W<10> {
        RESERVED10_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "XOSCHF Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xoschfctl](index.html) module"]
pub struct XOSCHFCTL_SPEC;
impl crate::RegisterSpec for XOSCHFCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [xoschfctl::R](R) reader structure"]
impl crate::Readable for XOSCHFCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [xoschfctl::W](W) writer structure"]
impl crate::Writable for XOSCHFCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets XOSCHFCTL to value 0"]
impl crate::Resettable for XOSCHFCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
