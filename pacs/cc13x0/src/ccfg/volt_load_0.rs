#[doc = "Register `VOLT_LOAD_0` reader"]
pub struct R(crate::R<VOLT_LOAD_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VOLT_LOAD_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VOLT_LOAD_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VOLT_LOAD_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VOLT_LOAD_0` writer"]
pub struct W(crate::W<VOLT_LOAD_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VOLT_LOAD_0_SPEC>;
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
impl From<crate::W<VOLT_LOAD_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VOLT_LOAD_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VDDR_EXT_TM15` reader - 7:0\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
pub type VDDR_EXT_TM15_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VDDR_EXT_TM15` writer - 7:0\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
pub type VDDR_EXT_TM15_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, VOLT_LOAD_0_SPEC, u8, u8, 8, O>;
#[doc = "Field `VDDR_EXT_TP5` reader - 15:8\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
pub type VDDR_EXT_TP5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VDDR_EXT_TP5` writer - 15:8\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
pub type VDDR_EXT_TP5_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, VOLT_LOAD_0_SPEC, u8, u8, 8, O>;
#[doc = "Field `VDDR_EXT_TP25` reader - 23:16\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
pub type VDDR_EXT_TP25_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VDDR_EXT_TP25` writer - 23:16\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
pub type VDDR_EXT_TP25_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, VOLT_LOAD_0_SPEC, u8, u8, 8, O>;
#[doc = "Field `VDDR_EXT_TP45` reader - 31:24\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
pub type VDDR_EXT_TP45_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VDDR_EXT_TP45` writer - 31:24\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
pub type VDDR_EXT_TP45_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, VOLT_LOAD_0_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline(always)]
    pub fn vddr_ext_tm15(&self) -> VDDR_EXT_TM15_R {
        VDDR_EXT_TM15_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline(always)]
    pub fn vddr_ext_tp5(&self) -> VDDR_EXT_TP5_R {
        VDDR_EXT_TP5_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline(always)]
    pub fn vddr_ext_tp25(&self) -> VDDR_EXT_TP25_R {
        VDDR_EXT_TP25_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline(always)]
    pub fn vddr_ext_tp45(&self) -> VDDR_EXT_TP45_R {
        VDDR_EXT_TP45_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn vddr_ext_tm15(&mut self) -> VDDR_EXT_TM15_W<0> {
        VDDR_EXT_TM15_W::new(self)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn vddr_ext_tp5(&mut self) -> VDDR_EXT_TP5_W<8> {
        VDDR_EXT_TP5_W::new(self)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn vddr_ext_tp25(&mut self) -> VDDR_EXT_TP25_W<16> {
        VDDR_EXT_TP25_W::new(self)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn vddr_ext_tp45(&mut self) -> VDDR_EXT_TP45_W<24> {
        VDDR_EXT_TP45_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Voltage Load 0 Enabled by MODE_CONF.VDDR_EXT_LOAD.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [volt_load_0](index.html) module"]
pub struct VOLT_LOAD_0_SPEC;
impl crate::RegisterSpec for VOLT_LOAD_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [volt_load_0::R](R) reader structure"]
impl crate::Readable for VOLT_LOAD_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [volt_load_0::W](W) writer structure"]
impl crate::Writable for VOLT_LOAD_0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets VOLT_LOAD_0 to value 0xffff_ffff"]
impl crate::Resettable for VOLT_LOAD_0_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
