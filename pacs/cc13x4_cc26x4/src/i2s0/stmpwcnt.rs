#[doc = "Register `STMPWCNT` reader"]
pub struct R(crate::R<STMPWCNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STMPWCNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STMPWCNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STMPWCNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STMPWCNT` writer"]
pub struct W(crate::W<STMPWCNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STMPWCNT_SPEC>;
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
impl From<crate::W<STMPWCNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STMPWCNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CURR_VALUE` reader - 15:0\\]
Current value of the WCLK counter"]
pub type CURR_VALUE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CURR_VALUE` writer - 15:0\\]
Current value of the WCLK counter"]
pub type CURR_VALUE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, STMPWCNT_SPEC, u16, u16, 16, O>;
#[doc = "Field `RESERVED16` reader - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED16_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RESERVED16` writer - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED16_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, STMPWCNT_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Current value of the WCLK counter"]
    #[inline(always)]
    pub fn curr_value(&self) -> CURR_VALUE_R {
        CURR_VALUE_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved16(&self) -> RESERVED16_R {
        RESERVED16_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Current value of the WCLK counter"]
    #[inline(always)]
    #[must_use]
    pub fn curr_value(&mut self) -> CURR_VALUE_W<0> {
        CURR_VALUE_W::new(self)
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
#[doc = "Current Value of WCNT\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stmpwcnt](index.html) module"]
pub struct STMPWCNT_SPEC;
impl crate::RegisterSpec for STMPWCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stmpwcnt::R](R) reader structure"]
impl crate::Readable for STMPWCNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [stmpwcnt::W](W) writer structure"]
impl crate::Writable for STMPWCNT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STMPWCNT to value 0"]
impl crate::Resettable for STMPWCNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
