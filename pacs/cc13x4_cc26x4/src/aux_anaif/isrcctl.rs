#[doc = "Register `ISRCCTL` reader"]
pub struct R(crate::R<ISRCCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ISRCCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ISRCCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ISRCCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ISRCCTL` writer"]
pub struct W(crate::W<ISRCCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ISRCCTL_SPEC>;
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
impl From<crate::W<ISRCCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ISRCCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESET_N` reader - 0:0\\]
ISRC reset control. 0: ISRC drives 0 uA. 1: ISRC drives current ADI_4_AUX:ISRC.TRIM to COMPA_IN."]
pub type RESET_N_R = crate::BitReader<bool>;
#[doc = "Field `RESET_N` writer - 0:0\\]
ISRC reset control. 0: ISRC drives 0 uA. 1: ISRC drives current ADI_4_AUX:ISRC.TRIM to COMPA_IN."]
pub type RESET_N_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISRCCTL_SPEC, bool, O>;
#[doc = "Field `RESERVED1` reader - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED1_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED1` writer - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ISRCCTL_SPEC, u32, u32, 31, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
ISRC reset control. 0: ISRC drives 0 uA. 1: ISRC drives current ADI_4_AUX:ISRC.TRIM to COMPA_IN."]
    #[inline(always)]
    pub fn reset_n(&self) -> RESET_N_R {
        RESET_N_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
ISRC reset control. 0: ISRC drives 0 uA. 1: ISRC drives current ADI_4_AUX:ISRC.TRIM to COMPA_IN."]
    #[inline(always)]
    #[must_use]
    pub fn reset_n(&mut self) -> RESET_N_W<0> {
        RESET_N_W::new(self)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> RESERVED1_W<1> {
        RESERVED1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Current Source Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isrcctl](index.html) module"]
pub struct ISRCCTL_SPEC;
impl crate::RegisterSpec for ISRCCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [isrcctl::R](R) reader structure"]
impl crate::Readable for ISRCCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [isrcctl::W](W) writer structure"]
impl crate::Writable for ISRCCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ISRCCTL to value 0x01"]
impl crate::Resettable for ISRCCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
