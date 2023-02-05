#[doc = "Register `STMPCTL` reader"]
pub struct R(crate::R<STMPCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STMPCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STMPCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STMPCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STMPCTL` writer"]
pub struct W(crate::W<STMPCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STMPCTL_SPEC>;
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
impl From<crate::W<STMPCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STMPCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STMP_EN` reader - 0:0\\]
Enables the samplestamp generator. The samplestamp generator must only be enabled after it has been properly configured. When cleared, all samplestamp generator counters and capture values are cleared."]
pub type STMP_EN_R = crate::BitReader<bool>;
#[doc = "Field `STMP_EN` writer - 0:0\\]
Enables the samplestamp generator. The samplestamp generator must only be enabled after it has been properly configured. When cleared, all samplestamp generator counters and capture values are cleared."]
pub type STMP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, STMPCTL_SPEC, bool, O>;
#[doc = "Field `IN_RDY` reader - 1:1\\]
Low until the input pins are ready to be started by the samplestamp generator. When started (that is STMPINTRIG equals the WCLK counter) the bit goes back low."]
pub type IN_RDY_R = crate::BitReader<bool>;
#[doc = "Field `IN_RDY` writer - 1:1\\]
Low until the input pins are ready to be started by the samplestamp generator. When started (that is STMPINTRIG equals the WCLK counter) the bit goes back low."]
pub type IN_RDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, STMPCTL_SPEC, bool, O>;
#[doc = "Field `OUT_RDY` reader - 2:2\\]
Low until the output pins are ready to be started by the samplestamp generator. When started (that is STMPOUTTRIG equals the WCLK counter) the bit goes back low."]
pub type OUT_RDY_R = crate::BitReader<bool>;
#[doc = "Field `OUT_RDY` writer - 2:2\\]
Low until the output pins are ready to be started by the samplestamp generator. When started (that is STMPOUTTRIG equals the WCLK counter) the bit goes back low."]
pub type OUT_RDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, STMPCTL_SPEC, bool, O>;
#[doc = "Field `RESERVED3` reader - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED3_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED3` writer - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, STMPCTL_SPEC, u32, u32, 29, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Enables the samplestamp generator. The samplestamp generator must only be enabled after it has been properly configured. When cleared, all samplestamp generator counters and capture values are cleared."]
    #[inline(always)]
    pub fn stmp_en(&self) -> STMP_EN_R {
        STMP_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Low until the input pins are ready to be started by the samplestamp generator. When started (that is STMPINTRIG equals the WCLK counter) the bit goes back low."]
    #[inline(always)]
    pub fn in_rdy(&self) -> IN_RDY_R {
        IN_RDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Low until the output pins are ready to be started by the samplestamp generator. When started (that is STMPOUTTRIG equals the WCLK counter) the bit goes back low."]
    #[inline(always)]
    pub fn out_rdy(&self) -> OUT_RDY_R {
        OUT_RDY_R::new(((self.bits >> 2) & 1) != 0)
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
Enables the samplestamp generator. The samplestamp generator must only be enabled after it has been properly configured. When cleared, all samplestamp generator counters and capture values are cleared."]
    #[inline(always)]
    #[must_use]
    pub fn stmp_en(&mut self) -> STMP_EN_W<0> {
        STMP_EN_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
Low until the input pins are ready to be started by the samplestamp generator. When started (that is STMPINTRIG equals the WCLK counter) the bit goes back low."]
    #[inline(always)]
    #[must_use]
    pub fn in_rdy(&mut self) -> IN_RDY_W<1> {
        IN_RDY_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
Low until the output pins are ready to be started by the samplestamp generator. When started (that is STMPOUTTRIG equals the WCLK counter) the bit goes back low."]
    #[inline(always)]
    #[must_use]
    pub fn out_rdy(&mut self) -> OUT_RDY_W<2> {
        OUT_RDY_W::new(self)
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
#[doc = "Samplestamp Generator Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stmpctl](index.html) module"]
pub struct STMPCTL_SPEC;
impl crate::RegisterSpec for STMPCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stmpctl::R](R) reader structure"]
impl crate::Readable for STMPCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [stmpctl::W](W) writer structure"]
impl crate::Writable for STMPCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STMPCTL to value 0"]
impl crate::Resettable for STMPCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
