#[doc = "Register `BATMONBAT` reader"]
pub struct R(crate::R<BATMONBAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BATMONBAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BATMONBAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BATMONBAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BATMONBAT` writer"]
pub struct W(crate::W<BATMONBAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BATMONBAT_SPEC>;
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
impl From<crate::W<BATMONBAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BATMONBAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FRAC` reader - 7:0\\]
See AON_BATMON:BAT.FRAC. Follow this procedure to get the correct value: - Do two dummy reads of FRAC. - Then read FRAC until two consecutive reads are equal."]
pub type FRAC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FRAC` writer - 7:0\\]
See AON_BATMON:BAT.FRAC. Follow this procedure to get the correct value: - Do two dummy reads of FRAC. - Then read FRAC until two consecutive reads are equal."]
pub type FRAC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BATMONBAT_SPEC, u8, u8, 8, O>;
#[doc = "Field `INT` reader - 10:8\\]
See AON_BATMON:BAT.INT. Follow this procedure to get the correct value: - Do two dummy reads of INT. - Then read INT until two consecutive reads are equal."]
pub type INT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT` writer - 10:8\\]
See AON_BATMON:BAT.INT. Follow this procedure to get the correct value: - Do two dummy reads of INT. - Then read INT until two consecutive reads are equal."]
pub type INT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BATMONBAT_SPEC, u8, u8, 3, O>;
#[doc = "Field `RESERVED11` reader - 31:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED11_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED11` writer - 31:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED11_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, BATMONBAT_SPEC, u32, u32, 21, O>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
See AON_BATMON:BAT.FRAC. Follow this procedure to get the correct value: - Do two dummy reads of FRAC. - Then read FRAC until two consecutive reads are equal."]
    #[inline(always)]
    pub fn frac(&self) -> FRAC_R {
        FRAC_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10 - 10:8\\]
See AON_BATMON:BAT.INT. Follow this procedure to get the correct value: - Do two dummy reads of INT. - Then read INT until two consecutive reads are equal."]
    #[inline(always)]
    pub fn int(&self) -> INT_R {
        INT_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:31 - 31:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved11(&self) -> RESERVED11_R {
        RESERVED11_R::new((self.bits >> 11) & 0x001f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
See AON_BATMON:BAT.FRAC. Follow this procedure to get the correct value: - Do two dummy reads of FRAC. - Then read FRAC until two consecutive reads are equal."]
    #[inline(always)]
    #[must_use]
    pub fn frac(&mut self) -> FRAC_W<0> {
        FRAC_W::new(self)
    }
    #[doc = "Bits 8:10 - 10:8\\]
See AON_BATMON:BAT.INT. Follow this procedure to get the correct value: - Do two dummy reads of INT. - Then read INT until two consecutive reads are equal."]
    #[inline(always)]
    #[must_use]
    pub fn int(&mut self) -> INT_W<8> {
        INT_W::new(self)
    }
    #[doc = "Bits 11:31 - 31:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved11(&mut self) -> RESERVED11_W<11> {
        RESERVED11_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AON_BATMON Battery Voltage Value Read access to AON_BATMON:BAT. System CPU must not access this register. Instead, system CPU must access AON_BATMON:BAT directly. AON_BATMON:BAT updates during VDDR recharge or active operational mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [batmonbat](index.html) module"]
pub struct BATMONBAT_SPEC;
impl crate::RegisterSpec for BATMONBAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [batmonbat::R](R) reader structure"]
impl crate::Readable for BATMONBAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [batmonbat::W](W) writer structure"]
impl crate::Writable for BATMONBAT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BATMONBAT to value 0"]
impl crate::Resettable for BATMONBAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
