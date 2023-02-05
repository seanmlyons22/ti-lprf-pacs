#[doc = "Register `SWEVSET` reader"]
pub struct R(crate::R<SWEVSET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SWEVSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SWEVSET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SWEVSET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SWEVSET` writer"]
pub struct W(crate::W<SWEVSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SWEVSET_SPEC>;
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
impl From<crate::W<SWEVSET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SWEVSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SWEV0` reader - 0:0\\]
Software event flag 0. 0: No effect. 1: Set software event flag 0."]
pub type SWEV0_R = crate::BitReader<bool>;
#[doc = "Field `SWEV0` writer - 0:0\\]
Software event flag 0. 0: No effect. 1: Set software event flag 0."]
pub type SWEV0_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWEVSET_SPEC, bool, O>;
#[doc = "Field `SWEV1` reader - 1:1\\]
Software event flag 1. 0: No effect. 1: Set software event flag 1."]
pub type SWEV1_R = crate::BitReader<bool>;
#[doc = "Field `SWEV1` writer - 1:1\\]
Software event flag 1. 0: No effect. 1: Set software event flag 1."]
pub type SWEV1_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWEVSET_SPEC, bool, O>;
#[doc = "Field `SWEV2` reader - 2:2\\]
Software event flag 2. 0: No effect. 1: Set software event flag 2."]
pub type SWEV2_R = crate::BitReader<bool>;
#[doc = "Field `SWEV2` writer - 2:2\\]
Software event flag 2. 0: No effect. 1: Set software event flag 2."]
pub type SWEV2_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWEVSET_SPEC, bool, O>;
#[doc = "Field `RESERVED3` reader - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED3_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED3` writer - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SWEVSET_SPEC, u32, u32, 29, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Software event flag 0. 0: No effect. 1: Set software event flag 0."]
    #[inline(always)]
    pub fn swev0(&self) -> SWEV0_R {
        SWEV0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Software event flag 1. 0: No effect. 1: Set software event flag 1."]
    #[inline(always)]
    pub fn swev1(&self) -> SWEV1_R {
        SWEV1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Software event flag 2. 0: No effect. 1: Set software event flag 2."]
    #[inline(always)]
    pub fn swev2(&self) -> SWEV2_R {
        SWEV2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:31 - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> RESERVED3_R {
        RESERVED3_R::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Software event flag 0. 0: No effect. 1: Set software event flag 0."]
    #[inline(always)]
    #[must_use]
    pub fn swev0(&mut self) -> SWEV0_W<0> {
        SWEV0_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
Software event flag 1. 0: No effect. 1: Set software event flag 1."]
    #[inline(always)]
    #[must_use]
    pub fn swev1(&mut self) -> SWEV1_W<1> {
        SWEV1_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
Software event flag 2. 0: No effect. 1: Set software event flag 2."]
    #[inline(always)]
    #[must_use]
    pub fn swev2(&mut self) -> SWEV2_W<2> {
        SWEV2_W::new(self)
    }
    #[doc = "Bits 3:31 - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> RESERVED3_W<3> {
        RESERVED3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Software Event Set Set software event flags from AUX domain to AON and MCU domains. CPUs in MCU domain can read the event flags from EVTOAONFLAGS and clear them in EVTOAONFLAGSCLR. Use of these event flags is software-defined.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swevset](index.html) module"]
pub struct SWEVSET_SPEC;
impl crate::RegisterSpec for SWEVSET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [swevset::R](R) reader structure"]
impl crate::Readable for SWEVSET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [swevset::W](W) writer structure"]
impl crate::Writable for SWEVSET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SWEVSET to value 0"]
impl crate::Resettable for SWEVSET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
