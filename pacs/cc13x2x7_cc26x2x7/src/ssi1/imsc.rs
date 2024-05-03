#[doc = "Register `IMSC` reader"]
pub type R = crate::R<ImscSpec>;
#[doc = "Register `IMSC` writer"]
pub type W = crate::W<ImscSpec>;
#[doc = "Field `RORIM` reader - 0:0\\]
Receive overrun interrupt mask: A read returns the current mask for receive overrun interrupt. On a write of 1, the mask for receive overrun interrupt is set which means the interrupt state will be reflected in MIS.RORMIS. A write of 0 clears the mask which means MIS.RORMIS will not reflect the interrupt."]
pub type RorimR = crate::BitReader;
#[doc = "Field `RORIM` writer - 0:0\\]
Receive overrun interrupt mask: A read returns the current mask for receive overrun interrupt. On a write of 1, the mask for receive overrun interrupt is set which means the interrupt state will be reflected in MIS.RORMIS. A write of 0 clears the mask which means MIS.RORMIS will not reflect the interrupt."]
pub type RorimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTIM` reader - 1:1\\]
Receive timeout interrupt mask: A read returns the current mask for receive timeout interrupt. On a write of 1, the mask for receive timeout interrupt is set which means the interrupt state will be reflected in MIS.RTMIS. A write of 0 clears the mask which means MIS.RTMIS will not reflect the interrupt."]
pub type RtimR = crate::BitReader;
#[doc = "Field `RTIM` writer - 1:1\\]
Receive timeout interrupt mask: A read returns the current mask for receive timeout interrupt. On a write of 1, the mask for receive timeout interrupt is set which means the interrupt state will be reflected in MIS.RTMIS. A write of 0 clears the mask which means MIS.RTMIS will not reflect the interrupt."]
pub type RtimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXIM` reader - 2:2\\]
Receive FIFO interrupt mask: A read returns the current mask for receive FIFO interrupt. On a write of 1, the mask for receive FIFO interrupt is set which means the interrupt state will be reflected in MIS.RXMIS. A write of 0 clears the mask which means MIS.RXMIS will not reflect the interrupt."]
pub type RximR = crate::BitReader;
#[doc = "Field `RXIM` writer - 2:2\\]
Receive FIFO interrupt mask: A read returns the current mask for receive FIFO interrupt. On a write of 1, the mask for receive FIFO interrupt is set which means the interrupt state will be reflected in MIS.RXMIS. A write of 0 clears the mask which means MIS.RXMIS will not reflect the interrupt."]
pub type RximW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXIM` reader - 3:3\\]
Transmit FIFO interrupt mask: A read returns the current mask for transmit FIFO interrupt. On a write of 1, the mask for transmit FIFO interrupt is set which means the interrupt state will be reflected in MIS.TXMIS. A write of 0 clears the mask which means MIS.TXMIS will not reflect the interrupt."]
pub type TximR = crate::BitReader;
#[doc = "Field `TXIM` writer - 3:3\\]
Transmit FIFO interrupt mask: A read returns the current mask for transmit FIFO interrupt. On a write of 1, the mask for transmit FIFO interrupt is set which means the interrupt state will be reflected in MIS.TXMIS. A write of 0 clears the mask which means MIS.TXMIS will not reflect the interrupt."]
pub type TximW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Receive overrun interrupt mask: A read returns the current mask for receive overrun interrupt. On a write of 1, the mask for receive overrun interrupt is set which means the interrupt state will be reflected in MIS.RORMIS. A write of 0 clears the mask which means MIS.RORMIS will not reflect the interrupt."]
    #[inline(always)]
    pub fn rorim(&self) -> RorimR {
        RorimR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Receive timeout interrupt mask: A read returns the current mask for receive timeout interrupt. On a write of 1, the mask for receive timeout interrupt is set which means the interrupt state will be reflected in MIS.RTMIS. A write of 0 clears the mask which means MIS.RTMIS will not reflect the interrupt."]
    #[inline(always)]
    pub fn rtim(&self) -> RtimR {
        RtimR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Receive FIFO interrupt mask: A read returns the current mask for receive FIFO interrupt. On a write of 1, the mask for receive FIFO interrupt is set which means the interrupt state will be reflected in MIS.RXMIS. A write of 0 clears the mask which means MIS.RXMIS will not reflect the interrupt."]
    #[inline(always)]
    pub fn rxim(&self) -> RximR {
        RximR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Transmit FIFO interrupt mask: A read returns the current mask for transmit FIFO interrupt. On a write of 1, the mask for transmit FIFO interrupt is set which means the interrupt state will be reflected in MIS.TXMIS. A write of 0 clears the mask which means MIS.TXMIS will not reflect the interrupt."]
    #[inline(always)]
    pub fn txim(&self) -> TximR {
        TximR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Receive overrun interrupt mask: A read returns the current mask for receive overrun interrupt. On a write of 1, the mask for receive overrun interrupt is set which means the interrupt state will be reflected in MIS.RORMIS. A write of 0 clears the mask which means MIS.RORMIS will not reflect the interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn rorim(&mut self) -> RorimW<ImscSpec> {
        RorimW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Receive timeout interrupt mask: A read returns the current mask for receive timeout interrupt. On a write of 1, the mask for receive timeout interrupt is set which means the interrupt state will be reflected in MIS.RTMIS. A write of 0 clears the mask which means MIS.RTMIS will not reflect the interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn rtim(&mut self) -> RtimW<ImscSpec> {
        RtimW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Receive FIFO interrupt mask: A read returns the current mask for receive FIFO interrupt. On a write of 1, the mask for receive FIFO interrupt is set which means the interrupt state will be reflected in MIS.RXMIS. A write of 0 clears the mask which means MIS.RXMIS will not reflect the interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn rxim(&mut self) -> RximW<ImscSpec> {
        RximW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Transmit FIFO interrupt mask: A read returns the current mask for transmit FIFO interrupt. On a write of 1, the mask for transmit FIFO interrupt is set which means the interrupt state will be reflected in MIS.TXMIS. A write of 0 clears the mask which means MIS.TXMIS will not reflect the interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn txim(&mut self) -> TximW<ImscSpec> {
        TximW::new(self, 3)
    }
}
#[doc = "Interrupt Mask Set and Clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imsc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`imsc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ImscSpec;
impl crate::RegisterSpec for ImscSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imsc::R`](R) reader structure"]
impl crate::Readable for ImscSpec {}
#[doc = "`write(|w| ..)` method takes [`imsc::W`](W) writer structure"]
impl crate::Writable for ImscSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IMSC to value 0"]
impl crate::Resettable for ImscSpec {
    const RESET_VALUE: u32 = 0;
}
