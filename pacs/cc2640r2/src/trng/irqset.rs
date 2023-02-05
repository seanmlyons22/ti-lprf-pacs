#[doc = "Register `IRQSET` reader"]
pub struct R(crate::R<IRQSET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IRQSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IRQSET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IRQSET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IRQSET` writer"]
pub struct W(crate::W<IRQSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IRQSET_SPEC>;
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
impl From<crate::W<IRQSET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IRQSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RDY` reader - 31:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RDY_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RDY` writer - 31:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RDY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IRQSET_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn rdy(&self) -> RDY_R {
        RDY_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn rdy(&mut self) -> RDY_W<0> {
        RDY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Set\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irqset](index.html) module"]
pub struct IRQSET_SPEC;
impl crate::RegisterSpec for IRQSET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [irqset::R](R) reader structure"]
impl crate::Readable for IRQSET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [irqset::W](W) writer structure"]
impl crate::Writable for IRQSET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IRQSET to value 0"]
impl crate::Resettable for IRQSET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
