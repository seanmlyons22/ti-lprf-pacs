#[doc = "Register `DMACR` reader"]
pub type R = crate::R<DmacrSpec>;
#[doc = "Register `DMACR` writer"]
pub type W = crate::W<DmacrSpec>;
#[doc = "Field `RXDMAE` reader - 0:0\\]
Receive FIFO DMA enable when set."]
pub type RxdmaeR = crate::BitReader;
#[doc = "Field `RXDMAE` writer - 0:0\\]
Receive FIFO DMA enable when set."]
pub type RxdmaeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXDMAE` reader - 1:1\\]
Transmit FIFO DMA enable when set."]
pub type TxdmaeR = crate::BitReader;
#[doc = "Field `TXDMAE` writer - 1:1\\]
Transmit FIFO DMA enable when set."]
pub type TxdmaeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED2` reader - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Receive FIFO DMA enable when set."]
    #[inline(always)]
    pub fn rxdmae(&self) -> RxdmaeR {
        RxdmaeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Transmit FIFO DMA enable when set."]
    #[inline(always)]
    pub fn txdmae(&self) -> TxdmaeR {
        TxdmaeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Receive FIFO DMA enable when set."]
    #[inline(always)]
    #[must_use]
    pub fn rxdmae(&mut self) -> RxdmaeW<DmacrSpec> {
        RxdmaeW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Transmit FIFO DMA enable when set."]
    #[inline(always)]
    #[must_use]
    pub fn txdmae(&mut self) -> TxdmaeW<DmacrSpec> {
        TxdmaeW::new(self, 1)
    }
}
#[doc = "DMA Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmacr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmacr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmacrSpec;
impl crate::RegisterSpec for DmacrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmacr::R`](R) reader structure"]
impl crate::Readable for DmacrSpec {}
#[doc = "`write(|w| ..)` method takes [`dmacr::W`](W) writer structure"]
impl crate::Writable for DmacrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMACR to value 0"]
impl crate::Resettable for DmacrSpec {
    const RESET_VALUE: u32 = 0;
}
