#[doc = "Register `FCFG1_REVISION` reader"]
pub struct R(crate::R<FCFG1_REVISION_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FCFG1_REVISION_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FCFG1_REVISION_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FCFG1_REVISION_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FCFG1_REVISION` writer"]
pub struct W(crate::W<FCFG1_REVISION_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FCFG1_REVISION_SPEC>;
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
impl From<crate::W<FCFG1_REVISION_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FCFG1_REVISION_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REV` reader - 31:0\\]
The revision number of the FCFG1 layout. This value will be read by application SW in order to determine which FCFG1 parameters that have valid values. This revision number must be incremented by 1 before any devices are to be produced if the FCFG1 layout has changed since the previous production of devices. Value migth change without warning."]
pub type REV_R = crate::FieldReader<u32, u32>;
#[doc = "Field `REV` writer - 31:0\\]
The revision number of the FCFG1 layout. This value will be read by application SW in order to determine which FCFG1 parameters that have valid values. This revision number must be incremented by 1 before any devices are to be produced if the FCFG1 layout has changed since the previous production of devices. Value migth change without warning."]
pub type REV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FCFG1_REVISION_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
The revision number of the FCFG1 layout. This value will be read by application SW in order to determine which FCFG1 parameters that have valid values. This revision number must be incremented by 1 before any devices are to be produced if the FCFG1 layout has changed since the previous production of devices. Value migth change without warning."]
    #[inline(always)]
    pub fn rev(&self) -> REV_R {
        REV_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
The revision number of the FCFG1 layout. This value will be read by application SW in order to determine which FCFG1 parameters that have valid values. This revision number must be incremented by 1 before any devices are to be produced if the FCFG1 layout has changed since the previous production of devices. Value migth change without warning."]
    #[inline(always)]
    #[must_use]
    pub fn rev(&mut self) -> REV_W<0> {
        REV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Factory Configuration (FCFG1) Revision\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fcfg1_revision](index.html) module"]
pub struct FCFG1_REVISION_SPEC;
impl crate::RegisterSpec for FCFG1_REVISION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fcfg1_revision::R](R) reader structure"]
impl crate::Readable for FCFG1_REVISION_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fcfg1_revision::W](W) writer structure"]
impl crate::Writable for FCFG1_REVISION_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FCFG1_REVISION to value 0x26"]
impl crate::Resettable for FCFG1_REVISION_SPEC {
    const RESET_VALUE: Self::Ux = 0x26;
}
