#[doc = "Register `STIM30` reader"]
pub struct R(crate::R<STIM30_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STIM30_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STIM30_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STIM30_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STIM30` writer"]
pub struct W(crate::W<STIM30_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STIM30_SPEC>;
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
impl From<crate::W<STIM30_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STIM30_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FIFOREADY` reader - 0:0\\]
Indicates whether the Stimulus Port can accept data"]
pub type FIFOREADY_R = crate::BitReader<bool>;
#[doc = "Field `FIFOREADY` writer - 0:0\\]
Indicates whether the Stimulus Port can accept data"]
pub type FIFOREADY_W<'a, const O: u8> = crate::BitWriter<'a, u32, STIM30_SPEC, bool, O>;
#[doc = "Field `DISABLED` reader - 1:1\\]
Indicates whether the Stimulus Port is enabled or disabled"]
pub type DISABLED_R = crate::BitReader<bool>;
#[doc = "Field `DISABLED` writer - 1:1\\]
Indicates whether the Stimulus Port is enabled or disabled"]
pub type DISABLED_W<'a, const O: u8> = crate::BitWriter<'a, u32, STIM30_SPEC, bool, O>;
#[doc = "Field `RESERVED2` reader - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED2_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED2` writer - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, STIM30_SPEC, u32, u32, 30, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Indicates whether the Stimulus Port can accept data"]
    #[inline(always)]
    pub fn fifoready(&self) -> FIFOREADY_R {
        FIFOREADY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Indicates whether the Stimulus Port is enabled or disabled"]
    #[inline(always)]
    pub fn disabled(&self) -> DISABLED_R {
        DISABLED_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> RESERVED2_R {
        RESERVED2_R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Indicates whether the Stimulus Port can accept data"]
    #[inline(always)]
    #[must_use]
    pub fn fifoready(&mut self) -> FIFOREADY_W<0> {
        FIFOREADY_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
Indicates whether the Stimulus Port is enabled or disabled"]
    #[inline(always)]
    #[must_use]
    pub fn disabled(&mut self) -> DISABLED_W<1> {
        DISABLED_W::new(self)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> RESERVED2_W<2> {
        RESERVED2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Provides the interface for generating Instrumentation packets\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stim30](index.html) module"]
pub struct STIM30_SPEC;
impl crate::RegisterSpec for STIM30_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stim30::R](R) reader structure"]
impl crate::Readable for STIM30_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [stim30::W](W) writer structure"]
impl crate::Writable for STIM30_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STIM30 to value 0"]
impl crate::Resettable for STIM30_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
