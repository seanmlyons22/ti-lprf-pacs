#[doc = "Register `SWRESET` reader"]
pub struct R(crate::R<SWRESET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SWRESET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SWRESET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SWRESET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SWRESET` writer"]
pub struct W(crate::W<SWRESET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SWRESET_SPEC>;
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
impl From<crate::W<SWRESET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SWRESET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SW_RESET` reader - 0:0\\]
If this bit is set to 1, the following modules are reset: - Master control internal state is reset. That includes interrupt, error status register, and result available interrupt generation FSM. - Key store module state is reset. That includes clearing the written area flags; therefore, the keys must be reloaded to the key store module. Writing 0 has no effect. The bit is self cleared after executing the reset."]
pub type SW_RESET_R = crate::BitReader<bool>;
#[doc = "Field `SW_RESET` writer - 0:0\\]
If this bit is set to 1, the following modules are reset: - Master control internal state is reset. That includes interrupt, error status register, and result available interrupt generation FSM. - Key store module state is reset. That includes clearing the written area flags; therefore, the keys must be reloaded to the key store module. Writing 0 has no effect. The bit is self cleared after executing the reset."]
pub type SW_RESET_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWRESET_SPEC, bool, O>;
#[doc = "Field `RESERVED1` reader - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED1_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED1` writer - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SWRESET_SPEC, u32, u32, 31, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
If this bit is set to 1, the following modules are reset: - Master control internal state is reset. That includes interrupt, error status register, and result available interrupt generation FSM. - Key store module state is reset. That includes clearing the written area flags; therefore, the keys must be reloaded to the key store module. Writing 0 has no effect. The bit is self cleared after executing the reset."]
    #[inline(always)]
    pub fn sw_reset(&self) -> SW_RESET_R {
        SW_RESET_R::new((self.bits & 1) != 0)
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
If this bit is set to 1, the following modules are reset: - Master control internal state is reset. That includes interrupt, error status register, and result available interrupt generation FSM. - Key store module state is reset. That includes clearing the written area flags; therefore, the keys must be reloaded to the key store module. Writing 0 has no effect. The bit is self cleared after executing the reset."]
    #[inline(always)]
    #[must_use]
    pub fn sw_reset(&mut self) -> SW_RESET_W<0> {
        SW_RESET_W::new(self)
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
#[doc = "Software Reset\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swreset](index.html) module"]
pub struct SWRESET_SPEC;
impl crate::RegisterSpec for SWRESET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [swreset::R](R) reader structure"]
impl crate::Readable for SWRESET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [swreset::W](W) writer structure"]
impl crate::Writable for SWRESET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SWRESET to value 0"]
impl crate::Resettable for SWRESET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
