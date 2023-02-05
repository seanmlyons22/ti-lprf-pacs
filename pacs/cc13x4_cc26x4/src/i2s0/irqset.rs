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
#[doc = "Field `PTR_ERR` reader - 0:0\\]
1: Sets the interrupt of IRQFLAGS.PTR_ERR"]
pub type PTR_ERR_R = crate::BitReader<bool>;
#[doc = "Field `PTR_ERR` writer - 0:0\\]
1: Sets the interrupt of IRQFLAGS.PTR_ERR"]
pub type PTR_ERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQSET_SPEC, bool, O>;
#[doc = "Field `WCLK_ERR` reader - 1:1\\]
1: Sets the interrupt of IRQFLAGS.WCLK_ERR"]
pub type WCLK_ERR_R = crate::BitReader<bool>;
#[doc = "Field `WCLK_ERR` writer - 1:1\\]
1: Sets the interrupt of IRQFLAGS.WCLK_ERR"]
pub type WCLK_ERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQSET_SPEC, bool, O>;
#[doc = "Field `BUS_ERR` reader - 2:2\\]
1: Sets the interrupt of IRQFLAGS.BUS_ERR"]
pub type BUS_ERR_R = crate::BitReader<bool>;
#[doc = "Field `BUS_ERR` writer - 2:2\\]
1: Sets the interrupt of IRQFLAGS.BUS_ERR"]
pub type BUS_ERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQSET_SPEC, bool, O>;
#[doc = "Field `WCLK_TIMEOUT` reader - 3:3\\]
1: Sets the interrupt of IRQFLAGS.WCLK_TIMEOUT"]
pub type WCLK_TIMEOUT_R = crate::BitReader<bool>;
#[doc = "Field `WCLK_TIMEOUT` writer - 3:3\\]
1: Sets the interrupt of IRQFLAGS.WCLK_TIMEOUT"]
pub type WCLK_TIMEOUT_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQSET_SPEC, bool, O>;
#[doc = "Field `AIF_DMA_OUT` reader - 4:4\\]
1: Sets the interrupt of IRQFLAGS.AIF_DMA_OUT (unless a auto clear criteria was given at the same time, in which the set will be ignored)"]
pub type AIF_DMA_OUT_R = crate::BitReader<bool>;
#[doc = "Field `AIF_DMA_OUT` writer - 4:4\\]
1: Sets the interrupt of IRQFLAGS.AIF_DMA_OUT (unless a auto clear criteria was given at the same time, in which the set will be ignored)"]
pub type AIF_DMA_OUT_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQSET_SPEC, bool, O>;
#[doc = "Field `AIF_DMA_IN` reader - 5:5\\]
1: Sets the interrupt of IRQFLAGS.AIF_DMA_IN (unless a auto clear criteria was given at the same time, in which the set will be ignored)"]
pub type AIF_DMA_IN_R = crate::BitReader<bool>;
#[doc = "Field `AIF_DMA_IN` writer - 5:5\\]
1: Sets the interrupt of IRQFLAGS.AIF_DMA_IN (unless a auto clear criteria was given at the same time, in which the set will be ignored)"]
pub type AIF_DMA_IN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQSET_SPEC, bool, O>;
#[doc = "Field `RESERVED6` reader - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED6_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED6` writer - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED6_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IRQSET_SPEC, u32, u32, 26, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
1: Sets the interrupt of IRQFLAGS.PTR_ERR"]
    #[inline(always)]
    pub fn ptr_err(&self) -> PTR_ERR_R {
        PTR_ERR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
1: Sets the interrupt of IRQFLAGS.WCLK_ERR"]
    #[inline(always)]
    pub fn wclk_err(&self) -> WCLK_ERR_R {
        WCLK_ERR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
1: Sets the interrupt of IRQFLAGS.BUS_ERR"]
    #[inline(always)]
    pub fn bus_err(&self) -> BUS_ERR_R {
        BUS_ERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
1: Sets the interrupt of IRQFLAGS.WCLK_TIMEOUT"]
    #[inline(always)]
    pub fn wclk_timeout(&self) -> WCLK_TIMEOUT_R {
        WCLK_TIMEOUT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
1: Sets the interrupt of IRQFLAGS.AIF_DMA_OUT (unless a auto clear criteria was given at the same time, in which the set will be ignored)"]
    #[inline(always)]
    pub fn aif_dma_out(&self) -> AIF_DMA_OUT_R {
        AIF_DMA_OUT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
1: Sets the interrupt of IRQFLAGS.AIF_DMA_IN (unless a auto clear criteria was given at the same time, in which the set will be ignored)"]
    #[inline(always)]
    pub fn aif_dma_in(&self) -> AIF_DMA_IN_R {
        AIF_DMA_IN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:31 - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved6(&self) -> RESERVED6_R {
        RESERVED6_R::new((self.bits >> 6) & 0x03ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
1: Sets the interrupt of IRQFLAGS.PTR_ERR"]
    #[inline(always)]
    #[must_use]
    pub fn ptr_err(&mut self) -> PTR_ERR_W<0> {
        PTR_ERR_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
1: Sets the interrupt of IRQFLAGS.WCLK_ERR"]
    #[inline(always)]
    #[must_use]
    pub fn wclk_err(&mut self) -> WCLK_ERR_W<1> {
        WCLK_ERR_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
1: Sets the interrupt of IRQFLAGS.BUS_ERR"]
    #[inline(always)]
    #[must_use]
    pub fn bus_err(&mut self) -> BUS_ERR_W<2> {
        BUS_ERR_W::new(self)
    }
    #[doc = "Bit 3 - 3:3\\]
1: Sets the interrupt of IRQFLAGS.WCLK_TIMEOUT"]
    #[inline(always)]
    #[must_use]
    pub fn wclk_timeout(&mut self) -> WCLK_TIMEOUT_W<3> {
        WCLK_TIMEOUT_W::new(self)
    }
    #[doc = "Bit 4 - 4:4\\]
1: Sets the interrupt of IRQFLAGS.AIF_DMA_OUT (unless a auto clear criteria was given at the same time, in which the set will be ignored)"]
    #[inline(always)]
    #[must_use]
    pub fn aif_dma_out(&mut self) -> AIF_DMA_OUT_W<4> {
        AIF_DMA_OUT_W::new(self)
    }
    #[doc = "Bit 5 - 5:5\\]
1: Sets the interrupt of IRQFLAGS.AIF_DMA_IN (unless a auto clear criteria was given at the same time, in which the set will be ignored)"]
    #[inline(always)]
    #[must_use]
    pub fn aif_dma_in(&mut self) -> AIF_DMA_IN_W<5> {
        AIF_DMA_IN_W::new(self)
    }
    #[doc = "Bits 6:31 - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved6(&mut self) -> RESERVED6_W<6> {
        RESERVED6_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Set Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irqset](index.html) module"]
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
