#[doc = "Register `ERROR` reader"]
pub struct R(crate::R<ERROR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ERROR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ERROR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ERROR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ERROR` writer"]
pub struct W(crate::W<ERROR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ERROR_SPEC>;
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
impl From<crate::W<ERROR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ERROR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STATUS` reader - 0:0\\]
Returns the status of bus error flag in uDMA, or clears this bit Read as: 0: No bus error detected 1: Bus error detected Write as: 0: No effect, status of bus error flag is unchanged. 1: Clears the bus error flag."]
pub type STATUS_R = crate::BitReader<bool>;
#[doc = "Field `STATUS` writer - 0:0\\]
Returns the status of bus error flag in uDMA, or clears this bit Read as: 0: No bus error detected 1: Bus error detected Write as: 0: No effect, status of bus error flag is unchanged. 1: Clears the bus error flag."]
pub type STATUS_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERROR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Returns the status of bus error flag in uDMA, or clears this bit Read as: 0: No bus error detected 1: Bus error detected Write as: 0: No effect, status of bus error flag is unchanged. 1: Clears the bus error flag."]
    #[inline(always)]
    pub fn status(&self) -> STATUS_R {
        STATUS_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Returns the status of bus error flag in uDMA, or clears this bit Read as: 0: No bus error detected 1: Bus error detected Write as: 0: No effect, status of bus error flag is unchanged. 1: Clears the bus error flag."]
    #[inline(always)]
    #[must_use]
    pub fn status(&mut self) -> STATUS_W<0> {
        STATUS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Error Status and Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [error](index.html) module"]
pub struct ERROR_SPEC;
impl crate::RegisterSpec for ERROR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [error::R](R) reader structure"]
impl crate::Readable for ERROR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [error::W](W) writer structure"]
impl crate::Writable for ERROR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ERROR to value 0"]
impl crate::Resettable for ERROR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
