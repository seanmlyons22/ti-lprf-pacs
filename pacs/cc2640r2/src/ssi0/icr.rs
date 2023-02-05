#[doc = "Register `ICR` reader"]
pub struct R(crate::R<ICR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ICR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ICR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ICR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ICR` writer"]
pub struct W(crate::W<ICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICR_SPEC>;
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
impl From<crate::W<ICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RORIC` reader - 0:0\\]
Clear the receive overrun interrupt: Writing 1 to this field clears the overrun error interrupt (RIS.RORRIS). Writing 0 has no effect."]
pub type RORIC_R = crate::BitReader<bool>;
#[doc = "Field `RORIC` writer - 0:0\\]
Clear the receive overrun interrupt: Writing 1 to this field clears the overrun error interrupt (RIS.RORRIS). Writing 0 has no effect."]
pub type RORIC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `RTIC` reader - 1:1\\]
Clear the receive timeout interrupt: Writing 1 to this field clears the timeout interrupt (RIS.RTRIS). Writing 0 has no effect."]
pub type RTIC_R = crate::BitReader<bool>;
#[doc = "Field `RTIC` writer - 1:1\\]
Clear the receive timeout interrupt: Writing 1 to this field clears the timeout interrupt (RIS.RTRIS). Writing 0 has no effect."]
pub type RTIC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Clear the receive overrun interrupt: Writing 1 to this field clears the overrun error interrupt (RIS.RORRIS). Writing 0 has no effect."]
    #[inline(always)]
    pub fn roric(&self) -> RORIC_R {
        RORIC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Clear the receive timeout interrupt: Writing 1 to this field clears the timeout interrupt (RIS.RTRIS). Writing 0 has no effect."]
    #[inline(always)]
    pub fn rtic(&self) -> RTIC_R {
        RTIC_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Clear the receive overrun interrupt: Writing 1 to this field clears the overrun error interrupt (RIS.RORRIS). Writing 0 has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn roric(&mut self) -> RORIC_W<0> {
        RORIC_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
Clear the receive timeout interrupt: Writing 1 to this field clears the timeout interrupt (RIS.RTRIS). Writing 0 has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn rtic(&mut self) -> RTIC_W<1> {
        RTIC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Clear On a write of 1, the corresponding interrupt is cleared. A write of 0 has no effect.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icr](index.html) module"]
pub struct ICR_SPEC;
impl crate::RegisterSpec for ICR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [icr::R](R) reader structure"]
impl crate::Readable for ICR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [icr::W](W) writer structure"]
impl crate::Writable for ICR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::Resettable for ICR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
