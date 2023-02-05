#[doc = "Register `BATMONTEMP` reader"]
pub struct R(crate::R<BATMONTEMP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BATMONTEMP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BATMONTEMP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BATMONTEMP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BATMONTEMP` writer"]
pub struct W(crate::W<BATMONTEMP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BATMONTEMP_SPEC>;
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
impl From<crate::W<BATMONTEMP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BATMONTEMP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FRAC` reader - 1:0\\]
See AON_BATMON:TEMP.FRAC. Follow this procedure to get the correct value: - Do two dummy reads of FRAC. - Then read FRAC until two consecutive reads are equal."]
pub type FRAC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FRAC` writer - 1:0\\]
See AON_BATMON:TEMP.FRAC. Follow this procedure to get the correct value: - Do two dummy reads of FRAC. - Then read FRAC until two consecutive reads are equal."]
pub type FRAC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BATMONTEMP_SPEC, u8, u8, 2, O>;
#[doc = "Field `INT` reader - 10:2\\]
See AON_BATMON:TEMP.INT. Follow this procedure to get the correct value: - Do two dummy reads of INT. - Then read INT until two consecutive reads are equal."]
pub type INT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `INT` writer - 10:2\\]
See AON_BATMON:TEMP.INT. Follow this procedure to get the correct value: - Do two dummy reads of INT. - Then read INT until two consecutive reads are equal."]
pub type INT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BATMONTEMP_SPEC, u16, u16, 9, O>;
#[doc = "Field `SIGN` reader - 15:11\\]
Sign extension of INT. Follow this procedure to get the correct value: - Do two dummy reads of SIGN. - Then read SIGN until two consecutive reads are equal."]
pub type SIGN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SIGN` writer - 15:11\\]
Sign extension of INT. Follow this procedure to get the correct value: - Do two dummy reads of SIGN. - Then read SIGN until two consecutive reads are equal."]
pub type SIGN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BATMONTEMP_SPEC, u8, u8, 5, O>;
#[doc = "Field `RESERVED16` reader - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED16_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RESERVED16` writer - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED16_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, BATMONTEMP_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
See AON_BATMON:TEMP.FRAC. Follow this procedure to get the correct value: - Do two dummy reads of FRAC. - Then read FRAC until two consecutive reads are equal."]
    #[inline(always)]
    pub fn frac(&self) -> FRAC_R {
        FRAC_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:10 - 10:2\\]
See AON_BATMON:TEMP.INT. Follow this procedure to get the correct value: - Do two dummy reads of INT. - Then read INT until two consecutive reads are equal."]
    #[inline(always)]
    pub fn int(&self) -> INT_R {
        INT_R::new(((self.bits >> 2) & 0x01ff) as u16)
    }
    #[doc = "Bits 11:15 - 15:11\\]
Sign extension of INT. Follow this procedure to get the correct value: - Do two dummy reads of SIGN. - Then read SIGN until two consecutive reads are equal."]
    #[inline(always)]
    pub fn sign(&self) -> SIGN_R {
        SIGN_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved16(&self) -> RESERVED16_R {
        RESERVED16_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
See AON_BATMON:TEMP.FRAC. Follow this procedure to get the correct value: - Do two dummy reads of FRAC. - Then read FRAC until two consecutive reads are equal."]
    #[inline(always)]
    #[must_use]
    pub fn frac(&mut self) -> FRAC_W<0> {
        FRAC_W::new(self)
    }
    #[doc = "Bits 2:10 - 10:2\\]
See AON_BATMON:TEMP.INT. Follow this procedure to get the correct value: - Do two dummy reads of INT. - Then read INT until two consecutive reads are equal."]
    #[inline(always)]
    #[must_use]
    pub fn int(&mut self) -> INT_W<2> {
        INT_W::new(self)
    }
    #[doc = "Bits 11:15 - 15:11\\]
Sign extension of INT. Follow this procedure to get the correct value: - Do two dummy reads of SIGN. - Then read SIGN until two consecutive reads are equal."]
    #[inline(always)]
    #[must_use]
    pub fn sign(&mut self) -> SIGN_W<11> {
        SIGN_W::new(self)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved16(&mut self) -> RESERVED16_W<16> {
        RESERVED16_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AON_BATMON Temperature Value Read access to AON_BATMON:TEMP. System CPU must not access this register. Instead, system CPU must access AON_BATMON:TEMP directly. AON_BATMON:TEMP updates during VDDR recharge or active operational mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [batmontemp](index.html) module"]
pub struct BATMONTEMP_SPEC;
impl crate::RegisterSpec for BATMONTEMP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [batmontemp::R](R) reader structure"]
impl crate::Readable for BATMONTEMP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [batmontemp::W](W) writer structure"]
impl crate::Writable for BATMONTEMP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BATMONTEMP to value 0"]
impl crate::Resettable for BATMONTEMP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
