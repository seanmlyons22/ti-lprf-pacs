#[doc = "Register `IIDX` reader"]
pub type R = crate::R<IidxSpec>;
#[doc = "Register `IIDX` writer"]
pub type W = crate::W<IidxSpec>;
#[doc = "7:0\\]
Interrupt index status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Stat {
    #[doc = "9: DMA Done for Transmit Event/interrupt pending"]
    DmaDoneTxEvt = 9,
    #[doc = "8: DMA Done for Receive Event/interrupt pending"]
    DmaDoneRxEvt = 8,
    #[doc = "7: End of Transmit Event/interrupt pending"]
    IdleEvt = 7,
    #[doc = "6: Transmit Buffer Empty Event/interrupt pending"]
    TxEmpty = 6,
    #[doc = "5: Transmit Event/interrupt pending"]
    TxEvt = 5,
    #[doc = "4: Receive Event/interrupt pending"]
    RxEvt = 4,
    #[doc = "3: SPI Receive Time-Out Event/interrupt pending"]
    RtoutEvt = 3,
    #[doc = "2: Transmit Parity Event/interrupt pending"]
    PerEvt = 2,
    #[doc = "1: RX FIFO Overflow Event/interrupt pending"]
    RxfifoOfvEvt = 1,
    #[doc = "0: No interrupt pending"]
    NoIntr = 0,
}
impl From<Stat> for u8 {
    #[inline(always)]
    fn from(variant: Stat) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Stat {
    type Ux = u8;
}
impl crate::IsEnum for Stat {}
#[doc = "Field `STAT` reader - 7:0\\]
Interrupt index status"]
pub type StatR = crate::FieldReader<Stat>;
impl StatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Stat> {
        match self.bits {
            9 => Some(Stat::DmaDoneTxEvt),
            8 => Some(Stat::DmaDoneRxEvt),
            7 => Some(Stat::IdleEvt),
            6 => Some(Stat::TxEmpty),
            5 => Some(Stat::TxEvt),
            4 => Some(Stat::RxEvt),
            3 => Some(Stat::RtoutEvt),
            2 => Some(Stat::PerEvt),
            1 => Some(Stat::RxfifoOfvEvt),
            0 => Some(Stat::NoIntr),
            _ => None,
        }
    }
    #[doc = "DMA Done for Transmit Event/interrupt pending"]
    #[inline(always)]
    pub fn is_dma_done_tx_evt(&self) -> bool {
        *self == Stat::DmaDoneTxEvt
    }
    #[doc = "DMA Done for Receive Event/interrupt pending"]
    #[inline(always)]
    pub fn is_dma_done_rx_evt(&self) -> bool {
        *self == Stat::DmaDoneRxEvt
    }
    #[doc = "End of Transmit Event/interrupt pending"]
    #[inline(always)]
    pub fn is_idle_evt(&self) -> bool {
        *self == Stat::IdleEvt
    }
    #[doc = "Transmit Buffer Empty Event/interrupt pending"]
    #[inline(always)]
    pub fn is_tx_empty(&self) -> bool {
        *self == Stat::TxEmpty
    }
    #[doc = "Transmit Event/interrupt pending"]
    #[inline(always)]
    pub fn is_tx_evt(&self) -> bool {
        *self == Stat::TxEvt
    }
    #[doc = "Receive Event/interrupt pending"]
    #[inline(always)]
    pub fn is_rx_evt(&self) -> bool {
        *self == Stat::RxEvt
    }
    #[doc = "SPI Receive Time-Out Event/interrupt pending"]
    #[inline(always)]
    pub fn is_rtout_evt(&self) -> bool {
        *self == Stat::RtoutEvt
    }
    #[doc = "Transmit Parity Event/interrupt pending"]
    #[inline(always)]
    pub fn is_per_evt(&self) -> bool {
        *self == Stat::PerEvt
    }
    #[doc = "RX FIFO Overflow Event/interrupt pending"]
    #[inline(always)]
    pub fn is_rxfifo_ofv_evt(&self) -> bool {
        *self == Stat::RxfifoOfvEvt
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_no_intr(&self) -> bool {
        *self == Stat::NoIntr
    }
}
#[doc = "Field `RESERVED8` reader - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Interrupt index status"]
    #[inline(always)]
    pub fn stat(&self) -> StatR {
        StatR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&self) -> Reserved8R {
        Reserved8R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {}
#[doc = "This register provides the highest priority enabled interrupt index. Value 0x00 means no event pending. Interrupt 1 is the highest priority, and 31 is the least priority. That is, the least bit position that is set to 1 denotes the highest priority pending interrupt. The priority order is fixed. However, users can implement their own prioritization schemes using other registers that expose the full set of interrupts that have occurred. On each read, only one interrupt is indicated. On a read, the current interrupt (highest priority) is automatically cleared by the hardware and the corresponding interrupt flag in RIS and MIS are cleared as well. After a read from the CPU (not from the debug interface), the register is updated with the next highest priority interrupt, if none are pending, then it would display 0x0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iidx::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iidx::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IidxSpec;
impl crate::RegisterSpec for IidxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iidx::R`](R) reader structure"]
impl crate::Readable for IidxSpec {}
#[doc = "`write(|w| ..)` method takes [`iidx::W`](W) writer structure"]
impl crate::Writable for IidxSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IIDX to value 0"]
impl crate::Resettable for IidxSpec {
    const RESET_VALUE: u32 = 0;
}
