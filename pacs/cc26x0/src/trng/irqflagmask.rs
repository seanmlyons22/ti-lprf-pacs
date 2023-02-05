#[doc = "Register `IRQFLAGMASK` reader"]
pub struct R(crate::R<IRQFLAGMASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IRQFLAGMASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IRQFLAGMASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IRQFLAGMASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IRQFLAGMASK` writer"]
pub struct W(crate::W<IRQFLAGMASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IRQFLAGMASK_SPEC>;
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
impl From<crate::W<IRQFLAGMASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IRQFLAGMASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RDY` reader - 0:0\\]
1: Allow IRQFLAGSTAT.RDY to activate the interrupt from this module."]
pub type RDY_R = crate::BitReader<bool>;
#[doc = "Field `RDY` writer - 0:0\\]
1: Allow IRQFLAGSTAT.RDY to activate the interrupt from this module."]
pub type RDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQFLAGMASK_SPEC, bool, O>;
#[doc = "Field `SHUTDOWN_OVF` reader - 1:1\\]
1: Allow IRQFLAGSTAT.SHUTDOWN_OVF to activate the interrupt from this module."]
pub type SHUTDOWN_OVF_R = crate::BitReader<bool>;
#[doc = "Field `SHUTDOWN_OVF` writer - 1:1\\]
1: Allow IRQFLAGSTAT.SHUTDOWN_OVF to activate the interrupt from this module."]
pub type SHUTDOWN_OVF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQFLAGMASK_SPEC, bool, O>;
#[doc = "Field `RESERVED2` reader - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED2_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED2` writer - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, IRQFLAGMASK_SPEC, u32, u32, 30, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
1: Allow IRQFLAGSTAT.RDY to activate the interrupt from this module."]
    #[inline(always)]
    pub fn rdy(&self) -> RDY_R {
        RDY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
1: Allow IRQFLAGSTAT.SHUTDOWN_OVF to activate the interrupt from this module."]
    #[inline(always)]
    pub fn shutdown_ovf(&self) -> SHUTDOWN_OVF_R {
        SHUTDOWN_OVF_R::new(((self.bits >> 1) & 1) != 0)
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
1: Allow IRQFLAGSTAT.RDY to activate the interrupt from this module."]
    #[inline(always)]
    #[must_use]
    pub fn rdy(&mut self) -> RDY_W<0> {
        RDY_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
1: Allow IRQFLAGSTAT.SHUTDOWN_OVF to activate the interrupt from this module."]
    #[inline(always)]
    #[must_use]
    pub fn shutdown_ovf(&mut self) -> SHUTDOWN_OVF_W<1> {
        SHUTDOWN_OVF_W::new(self)
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
#[doc = "Interrupt Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irqflagmask](index.html) module"]
pub struct IRQFLAGMASK_SPEC;
impl crate::RegisterSpec for IRQFLAGMASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [irqflagmask::R](R) reader structure"]
impl crate::Readable for IRQFLAGMASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [irqflagmask::W](W) writer structure"]
impl crate::Writable for IRQFLAGMASK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IRQFLAGMASK to value 0"]
impl crate::Resettable for IRQFLAGMASK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
