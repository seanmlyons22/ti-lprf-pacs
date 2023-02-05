#[doc = "Register `DMASTAT` reader"]
pub struct R(crate::R<DMASTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMASTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMASTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMASTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMASTAT` writer"]
pub struct W(crate::W<DMASTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMASTAT_SPEC>;
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
impl From<crate::W<DMASTAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMASTAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH0_ACT` reader - 0:0\\]
A value of 1 indicates that channel 0 is active (DMA transfer on-going)."]
pub type CH0_ACT_R = crate::BitReader<bool>;
#[doc = "Field `CH0_ACT` writer - 0:0\\]
A value of 1 indicates that channel 0 is active (DMA transfer on-going)."]
pub type CH0_ACT_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMASTAT_SPEC, bool, O>;
#[doc = "Field `CH1_ACT` reader - 1:1\\]
A value of 1 indicates that channel 1 is active (DMA transfer on-going)."]
pub type CH1_ACT_R = crate::BitReader<bool>;
#[doc = "Field `CH1_ACT` writer - 1:1\\]
A value of 1 indicates that channel 1 is active (DMA transfer on-going)."]
pub type CH1_ACT_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMASTAT_SPEC, bool, O>;
#[doc = "Field `RESERVED2` reader - 16:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED2_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RESERVED2` writer - 16:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMASTAT_SPEC, u16, u16, 15, O>;
#[doc = "Field `PORT_ERR` reader - 17:17\\]
Reflects possible transfer errors on the AHB port."]
pub type PORT_ERR_R = crate::BitReader<bool>;
#[doc = "Field `PORT_ERR` writer - 17:17\\]
Reflects possible transfer errors on the AHB port."]
pub type PORT_ERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMASTAT_SPEC, bool, O>;
#[doc = "Field `RESERVED18` reader - 31:18\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED18_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RESERVED18` writer - 31:18\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED18_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMASTAT_SPEC, u16, u16, 14, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
A value of 1 indicates that channel 0 is active (DMA transfer on-going)."]
    #[inline(always)]
    pub fn ch0_act(&self) -> CH0_ACT_R {
        CH0_ACT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
A value of 1 indicates that channel 1 is active (DMA transfer on-going)."]
    #[inline(always)]
    pub fn ch1_act(&self) -> CH1_ACT_R {
        CH1_ACT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:16 - 16:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> RESERVED2_R {
        RESERVED2_R::new(((self.bits >> 2) & 0x7fff) as u16)
    }
    #[doc = "Bit 17 - 17:17\\]
Reflects possible transfer errors on the AHB port."]
    #[inline(always)]
    pub fn port_err(&self) -> PORT_ERR_R {
        PORT_ERR_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:31 - 31:18\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved18(&self) -> RESERVED18_R {
        RESERVED18_R::new(((self.bits >> 18) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
A value of 1 indicates that channel 0 is active (DMA transfer on-going)."]
    #[inline(always)]
    #[must_use]
    pub fn ch0_act(&mut self) -> CH0_ACT_W<0> {
        CH0_ACT_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
A value of 1 indicates that channel 1 is active (DMA transfer on-going)."]
    #[inline(always)]
    #[must_use]
    pub fn ch1_act(&mut self) -> CH1_ACT_W<1> {
        CH1_ACT_W::new(self)
    }
    #[doc = "Bits 2:16 - 16:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> RESERVED2_W<2> {
        RESERVED2_W::new(self)
    }
    #[doc = "Bit 17 - 17:17\\]
Reflects possible transfer errors on the AHB port."]
    #[inline(always)]
    #[must_use]
    pub fn port_err(&mut self) -> PORT_ERR_W<17> {
        PORT_ERR_W::new(self)
    }
    #[doc = "Bits 18:31 - 31:18\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved18(&mut self) -> RESERVED18_W<18> {
        RESERVED18_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMAC Status This register provides the actual state of each DMA channel. It also reports port errors in case these were received by the master interface module during the data transfer.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmastat](index.html) module"]
pub struct DMASTAT_SPEC;
impl crate::RegisterSpec for DMASTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmastat::R](R) reader structure"]
impl crate::Readable for DMASTAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmastat::W](W) writer structure"]
impl crate::Writable for DMASTAT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMASTAT to value 0"]
impl crate::Resettable for DMASTAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
