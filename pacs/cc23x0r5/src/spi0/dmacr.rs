#[doc = "Register `DMACR` reader"]
pub type R = crate::R<DmacrSpec>;
#[doc = "Register `DMACR` writer"]
pub type W = crate::W<DmacrSpec>;
#[doc = "0:0\\]
Receive DMA enable. If this bit is set to 1, DMA for the receive FIFO is enabled.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxen {
    #[doc = "1: Enable RX DMA"]
    Ena = 1,
    #[doc = "0: Disable RX DMA"]
    Dis = 0,
}
impl From<Rxen> for bool {
    #[inline(always)]
    fn from(variant: Rxen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXEN` reader - 0:0\\]
Receive DMA enable. If this bit is set to 1, DMA for the receive FIFO is enabled."]
pub type RxenR = crate::BitReader<Rxen>;
impl RxenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxen {
        match self.bits {
            true => Rxen::Ena,
            false => Rxen::Dis,
        }
    }
    #[doc = "Enable RX DMA"]
    #[inline(always)]
    pub fn is_ena(&self) -> bool {
        *self == Rxen::Ena
    }
    #[doc = "Disable RX DMA"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Rxen::Dis
    }
}
#[doc = "Field `RXEN` writer - 0:0\\]
Receive DMA enable. If this bit is set to 1, DMA for the receive FIFO is enabled."]
pub type RxenW<'a, REG> = crate::BitWriter<'a, REG, Rxen>;
impl<'a, REG> RxenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable RX DMA"]
    #[inline(always)]
    pub fn ena(self) -> &'a mut crate::W<REG> {
        self.variant(Rxen::Ena)
    }
    #[doc = "Disable RX DMA"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Rxen::Dis)
    }
}
#[doc = "Field `RESERVED1` reader - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `RESERVED1` writer - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "8:8\\]
Transmit DMA enable. If this bit is set to 1, DMA for the trasmit FIFO is enabled.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txen {
    #[doc = "1: Enable TX DMA"]
    Ena = 1,
    #[doc = "0: Disable TX DMA"]
    Dis = 0,
}
impl From<Txen> for bool {
    #[inline(always)]
    fn from(variant: Txen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXEN` reader - 8:8\\]
Transmit DMA enable. If this bit is set to 1, DMA for the trasmit FIFO is enabled."]
pub type TxenR = crate::BitReader<Txen>;
impl TxenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txen {
        match self.bits {
            true => Txen::Ena,
            false => Txen::Dis,
        }
    }
    #[doc = "Enable TX DMA"]
    #[inline(always)]
    pub fn is_ena(&self) -> bool {
        *self == Txen::Ena
    }
    #[doc = "Disable TX DMA"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Txen::Dis
    }
}
#[doc = "Field `TXEN` writer - 8:8\\]
Transmit DMA enable. If this bit is set to 1, DMA for the trasmit FIFO is enabled."]
pub type TxenW<'a, REG> = crate::BitWriter<'a, REG, Txen>;
impl<'a, REG> TxenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable TX DMA"]
    #[inline(always)]
    pub fn ena(self) -> &'a mut crate::W<REG> {
        self.variant(Txen::Ena)
    }
    #[doc = "Disable TX DMA"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Txen::Dis)
    }
}
#[doc = "Field `RESERVED9` reader - 31:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved9R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED9` writer - 31:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved9W<'a, REG> = crate::FieldWriter<'a, REG, 23, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Receive DMA enable. If this bit is set to 1, DMA for the receive FIFO is enabled."]
    #[inline(always)]
    pub fn rxen(&self) -> RxenR {
        RxenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Transmit DMA enable. If this bit is set to 1, DMA for the trasmit FIFO is enabled."]
    #[inline(always)]
    pub fn txen(&self) -> TxenR {
        TxenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:31 - 31:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved9(&self) -> Reserved9R {
        Reserved9R::new((self.bits >> 9) & 0x007f_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Receive DMA enable. If this bit is set to 1, DMA for the receive FIFO is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn rxen(&mut self) -> RxenW<DmacrSpec> {
        RxenW::new(self, 0)
    }
    #[doc = "Bits 1:7 - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<DmacrSpec> {
        Reserved1W::new(self, 1)
    }
    #[doc = "Bit 8 - 8:8\\]
Transmit DMA enable. If this bit is set to 1, DMA for the trasmit FIFO is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn txen(&mut self) -> TxenW<DmacrSpec> {
        TxenW::new(self, 8)
    }
    #[doc = "Bits 9:31 - 31:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved9(&mut self) -> Reserved9W<DmacrSpec> {
        Reserved9W::new(self, 9)
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
