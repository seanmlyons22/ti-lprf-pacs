#[doc = "Register `DMACTL` reader"]
pub struct R(crate::R<DMACTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMACTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMACTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMACTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMACTL` writer"]
pub struct W(crate::W<DMACTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMACTL_SPEC>;
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
impl From<crate::W<DMACTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMACTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXDMAE` reader - 0:0\\]
Receive DMA enable. If this bit is set to 1, DMA for the receive FIFO is enabled."]
pub type RXDMAE_R = crate::BitReader<bool>;
#[doc = "Field `RXDMAE` writer - 0:0\\]
Receive DMA enable. If this bit is set to 1, DMA for the receive FIFO is enabled."]
pub type RXDMAE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMACTL_SPEC, bool, O>;
#[doc = "Field `TXDMAE` reader - 1:1\\]
Transmit DMA enable. If this bit is set to 1, DMA for the transmit FIFO is enabled."]
pub type TXDMAE_R = crate::BitReader<bool>;
#[doc = "Field `TXDMAE` writer - 1:1\\]
Transmit DMA enable. If this bit is set to 1, DMA for the transmit FIFO is enabled."]
pub type TXDMAE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMACTL_SPEC, bool, O>;
#[doc = "Field `DMAONERR` reader - 2:2\\]
DMA on error. If this bit is set to 1, the DMA receive request outputs (for single and burst requests) are disabled when the UART error interrupt is asserted (more specifically if any of the error interrupts RIS.PERIS, RIS.BERIS, RIS.FERIS or RIS.OERIS are asserted)."]
pub type DMAONERR_R = crate::BitReader<bool>;
#[doc = "Field `DMAONERR` writer - 2:2\\]
DMA on error. If this bit is set to 1, the DMA receive request outputs (for single and burst requests) are disabled when the UART error interrupt is asserted (more specifically if any of the error interrupts RIS.PERIS, RIS.BERIS, RIS.FERIS or RIS.OERIS are asserted)."]
pub type DMAONERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMACTL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Receive DMA enable. If this bit is set to 1, DMA for the receive FIFO is enabled."]
    #[inline(always)]
    pub fn rxdmae(&self) -> RXDMAE_R {
        RXDMAE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Transmit DMA enable. If this bit is set to 1, DMA for the transmit FIFO is enabled."]
    #[inline(always)]
    pub fn txdmae(&self) -> TXDMAE_R {
        TXDMAE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
DMA on error. If this bit is set to 1, the DMA receive request outputs (for single and burst requests) are disabled when the UART error interrupt is asserted (more specifically if any of the error interrupts RIS.PERIS, RIS.BERIS, RIS.FERIS or RIS.OERIS are asserted)."]
    #[inline(always)]
    pub fn dmaonerr(&self) -> DMAONERR_R {
        DMAONERR_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Receive DMA enable. If this bit is set to 1, DMA for the receive FIFO is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn rxdmae(&mut self) -> RXDMAE_W<0> {
        RXDMAE_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
Transmit DMA enable. If this bit is set to 1, DMA for the transmit FIFO is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn txdmae(&mut self) -> TXDMAE_W<1> {
        TXDMAE_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
DMA on error. If this bit is set to 1, the DMA receive request outputs (for single and burst requests) are disabled when the UART error interrupt is asserted (more specifically if any of the error interrupts RIS.PERIS, RIS.BERIS, RIS.FERIS or RIS.OERIS are asserted)."]
    #[inline(always)]
    #[must_use]
    pub fn dmaonerr(&mut self) -> DMAONERR_W<2> {
        DMAONERR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmactl](index.html) module"]
pub struct DMACTL_SPEC;
impl crate::RegisterSpec for DMACTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmactl::R](R) reader structure"]
impl crate::Readable for DMACTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmactl::W](W) writer structure"]
impl crate::Writable for DMACTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMACTL to value 0"]
impl crate::Resettable for DMACTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
