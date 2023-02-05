#[doc = "Register `CKEY2` reader"]
pub struct R(crate::R<CKEY2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CKEY2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CKEY2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CKEY2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CKEY2` writer"]
pub struct W(crate::W<CKEY2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CKEY2_SPEC>;
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
impl From<crate::W<CKEY2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CKEY2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `KEY` reader - 31:0\\]
Bit\\[95:64\\]
of customer key used for XOR of TI unlock code when CCFG_TI_OPTIONS.C_FA_DIS != 0xC5."]
pub type KEY_R = crate::FieldReader<u32, u32>;
#[doc = "Field `KEY` writer - 31:0\\]
Bit\\[95:64\\]
of customer key used for XOR of TI unlock code when CCFG_TI_OPTIONS.C_FA_DIS != 0xC5."]
pub type KEY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CKEY2_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Bit\\[95:64\\]
of customer key used for XOR of TI unlock code when CCFG_TI_OPTIONS.C_FA_DIS != 0xC5."]
    #[inline(always)]
    pub fn key(&self) -> KEY_R {
        KEY_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Bit\\[95:64\\]
of customer key used for XOR of TI unlock code when CCFG_TI_OPTIONS.C_FA_DIS != 0xC5."]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KEY_W<0> {
        KEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Customer key\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ckey2](index.html) module"]
pub struct CKEY2_SPEC;
impl crate::RegisterSpec for CKEY2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ckey2::R](R) reader structure"]
impl crate::Readable for CKEY2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ckey2::W](W) writer structure"]
impl crate::Writable for CKEY2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CKEY2 to value 0x0fff_ffff"]
impl crate::Resettable for CKEY2_SPEC {
    const RESET_VALUE: Self::Ux = 0x0fff_ffff;
}
