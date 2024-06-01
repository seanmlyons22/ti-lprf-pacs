#[doc = "Register `IMSC` reader"]
pub type R = crate::R<ImscSpec>;
#[doc = "Register `IMSC` writer"]
pub type W = crate::W<ImscSpec>;
#[doc = "Field `RESERVED0` reader - 0:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved0R = crate::BitReader;
#[doc = "Field `RESERVED0` writer - 0:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTSMIM` reader - 1:1\\]
Clear to Send (CTS) modem interrupt mask. A read returns the current mask for UART's clear to send interrupt. On a write of 1, the mask of the overrun error interrupt is set which means the interrupt state will be reflected in MIS.CTSMMIS. A write of 0 clears the mask which means MIS.CTSMMIS will not reflect the interrupt."]
pub type CtsmimR = crate::BitReader;
#[doc = "Field `CTSMIM` writer - 1:1\\]
Clear to Send (CTS) modem interrupt mask. A read returns the current mask for UART's clear to send interrupt. On a write of 1, the mask of the overrun error interrupt is set which means the interrupt state will be reflected in MIS.CTSMMIS. A write of 0 clears the mask which means MIS.CTSMMIS will not reflect the interrupt."]
pub type CtsmimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED2` reader - 3:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `RESERVED2` writer - 3:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RXIM` reader - 4:4\\]
Receive interrupt mask. A read returns the current mask for UART's receive interrupt. On a write of 1, the mask of the overrun error interrupt is set which means the interrupt state will be reflected in MIS.RXMIS. A write of 0 clears the mask which means MIS.RXMIS will not reflect the interrupt."]
pub type RximR = crate::BitReader;
#[doc = "Field `RXIM` writer - 4:4\\]
Receive interrupt mask. A read returns the current mask for UART's receive interrupt. On a write of 1, the mask of the overrun error interrupt is set which means the interrupt state will be reflected in MIS.RXMIS. A write of 0 clears the mask which means MIS.RXMIS will not reflect the interrupt."]
pub type RximW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXIM` reader - 5:5\\]
Transmit interrupt mask. A read returns the current mask for UART's transmit interrupt. On a write of 1, the mask of the overrun error interrupt is set which means the interrupt state will be reflected in MIS.TXMIS. A write of 0 clears the mask which means MIS.TXMIS will not reflect the interrupt."]
pub type TximR = crate::BitReader;
#[doc = "Field `TXIM` writer - 5:5\\]
Transmit interrupt mask. A read returns the current mask for UART's transmit interrupt. On a write of 1, the mask of the overrun error interrupt is set which means the interrupt state will be reflected in MIS.TXMIS. A write of 0 clears the mask which means MIS.TXMIS will not reflect the interrupt."]
pub type TximW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTIM` reader - 6:6\\]
Receive timeout interrupt mask. A read returns the current mask for UART's receive timeout interrupt. On a write of 1, the mask of the overrun error interrupt is set which means the interrupt state will be reflected in MIS.RTMIS. A write of 0 clears the mask which means this bitfield will not reflect the interrupt. The raw interrupt for receive timeout RIS.RTRIS cannot be set unless the mask is set (RTIM = 1). This is because the mask acts as an enable for power saving. That is, the same status can be read from MIS.RTMIS and RIS.RTRIS."]
pub type RtimR = crate::BitReader;
#[doc = "Field `RTIM` writer - 6:6\\]
Receive timeout interrupt mask. A read returns the current mask for UART's receive timeout interrupt. On a write of 1, the mask of the overrun error interrupt is set which means the interrupt state will be reflected in MIS.RTMIS. A write of 0 clears the mask which means this bitfield will not reflect the interrupt. The raw interrupt for receive timeout RIS.RTRIS cannot be set unless the mask is set (RTIM = 1). This is because the mask acts as an enable for power saving. That is, the same status can be read from MIS.RTMIS and RIS.RTRIS."]
pub type RtimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FEIM` reader - 7:7\\]
Framing error interrupt mask. A read returns the current mask for UART's framing error interrupt. On a write of 1, the mask of the overrun error interrupt is set which means the interrupt state will be reflected in MIS.FEMIS. A write of 0 clears the mask which means MIS.FEMIS will not reflect the interrupt."]
pub type FeimR = crate::BitReader;
#[doc = "Field `FEIM` writer - 7:7\\]
Framing error interrupt mask. A read returns the current mask for UART's framing error interrupt. On a write of 1, the mask of the overrun error interrupt is set which means the interrupt state will be reflected in MIS.FEMIS. A write of 0 clears the mask which means MIS.FEMIS will not reflect the interrupt."]
pub type FeimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEIM` reader - 8:8\\]
Parity error interrupt mask. A read returns the current mask for UART's parity error interrupt. On a write of 1, the mask of the overrun error interrupt is set which means the interrupt state will be reflected in MIS.PEMIS. A write of 0 clears the mask which means MIS.PEMIS will not reflect the interrupt."]
pub type PeimR = crate::BitReader;
#[doc = "Field `PEIM` writer - 8:8\\]
Parity error interrupt mask. A read returns the current mask for UART's parity error interrupt. On a write of 1, the mask of the overrun error interrupt is set which means the interrupt state will be reflected in MIS.PEMIS. A write of 0 clears the mask which means MIS.PEMIS will not reflect the interrupt."]
pub type PeimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BEIM` reader - 9:9\\]
Break error interrupt mask. A read returns the current mask for UART's break error interrupt. On a write of 1, the mask of the overrun error interrupt is set which means the interrupt state will be reflected in MIS.BEMIS. A write of 0 clears the mask which means MIS.BEMIS will not reflect the interrupt."]
pub type BeimR = crate::BitReader;
#[doc = "Field `BEIM` writer - 9:9\\]
Break error interrupt mask. A read returns the current mask for UART's break error interrupt. On a write of 1, the mask of the overrun error interrupt is set which means the interrupt state will be reflected in MIS.BEMIS. A write of 0 clears the mask which means MIS.BEMIS will not reflect the interrupt."]
pub type BeimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OEIM` reader - 10:10\\]
Overrun error interrupt mask. A read returns the current mask for UART's overrun error interrupt. On a write of 1, the mask of the overrun error interrupt is set which means the interrupt state will be reflected in MIS.OEMIS. A write of 0 clears the mask which means MIS.OEMIS will not reflect the interrupt."]
pub type OeimR = crate::BitReader;
#[doc = "Field `OEIM` writer - 10:10\\]
Overrun error interrupt mask. A read returns the current mask for UART's overrun error interrupt. On a write of 1, the mask of the overrun error interrupt is set which means the interrupt state will be reflected in MIS.OEMIS. A write of 0 clears the mask which means MIS.OEMIS will not reflect the interrupt."]
pub type OeimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOTIM` reader - 11:11\\]
End of Transmission interrupt mask. A read returns the current mask for UART's EoT interrupt. On a write of 1, the mask of the EoT interrupt is set which means the interrupt state will be reflected in MIS.EOTMIS. A write of 0 clears the mask which means MIS.EOTMIS will not reflect the interrupt."]
pub type EotimR = crate::BitReader;
#[doc = "Field `EOTIM` writer - 11:11\\]
End of Transmission interrupt mask. A read returns the current mask for UART's EoT interrupt. On a write of 1, the mask of the EoT interrupt is set which means the interrupt state will be reflected in MIS.EOTMIS. A write of 0 clears the mask which means MIS.EOTMIS will not reflect the interrupt."]
pub type EotimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED12` reader - 31:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved12R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED12` writer - 31:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved12W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Clear to Send (CTS) modem interrupt mask. A read returns the current mask for UART's clear to send interrupt. On a write of 1, the mask of the overrun error interrupt is set which means the interrupt state will be reflected in MIS.CTSMMIS. A write of 0 clears the mask which means MIS.CTSMMIS will not reflect the interrupt."]
    #[inline(always)]
    pub fn ctsmim(&self) -> CtsmimR {
        CtsmimR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - 3:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - 4:4\\]
Receive interrupt mask. A read returns the current mask for UART's receive interrupt. On a write of 1, the mask of the overrun error interrupt is set which means the interrupt state will be reflected in MIS.RXMIS. A write of 0 clears the mask which means MIS.RXMIS will not reflect the interrupt."]
    #[inline(always)]
    pub fn rxim(&self) -> RximR {
        RximR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Transmit interrupt mask. A read returns the current mask for UART's transmit interrupt. On a write of 1, the mask of the overrun error interrupt is set which means the interrupt state will be reflected in MIS.TXMIS. A write of 0 clears the mask which means MIS.TXMIS will not reflect the interrupt."]
    #[inline(always)]
    pub fn txim(&self) -> TximR {
        TximR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Receive timeout interrupt mask. A read returns the current mask for UART's receive timeout interrupt. On a write of 1, the mask of the overrun error interrupt is set which means the interrupt state will be reflected in MIS.RTMIS. A write of 0 clears the mask which means this bitfield will not reflect the interrupt. The raw interrupt for receive timeout RIS.RTRIS cannot be set unless the mask is set (RTIM = 1). This is because the mask acts as an enable for power saving. That is, the same status can be read from MIS.RTMIS and RIS.RTRIS."]
    #[inline(always)]
    pub fn rtim(&self) -> RtimR {
        RtimR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Framing error interrupt mask. A read returns the current mask for UART's framing error interrupt. On a write of 1, the mask of the overrun error interrupt is set which means the interrupt state will be reflected in MIS.FEMIS. A write of 0 clears the mask which means MIS.FEMIS will not reflect the interrupt."]
    #[inline(always)]
    pub fn feim(&self) -> FeimR {
        FeimR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Parity error interrupt mask. A read returns the current mask for UART's parity error interrupt. On a write of 1, the mask of the overrun error interrupt is set which means the interrupt state will be reflected in MIS.PEMIS. A write of 0 clears the mask which means MIS.PEMIS will not reflect the interrupt."]
    #[inline(always)]
    pub fn peim(&self) -> PeimR {
        PeimR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Break error interrupt mask. A read returns the current mask for UART's break error interrupt. On a write of 1, the mask of the overrun error interrupt is set which means the interrupt state will be reflected in MIS.BEMIS. A write of 0 clears the mask which means MIS.BEMIS will not reflect the interrupt."]
    #[inline(always)]
    pub fn beim(&self) -> BeimR {
        BeimR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Overrun error interrupt mask. A read returns the current mask for UART's overrun error interrupt. On a write of 1, the mask of the overrun error interrupt is set which means the interrupt state will be reflected in MIS.OEMIS. A write of 0 clears the mask which means MIS.OEMIS will not reflect the interrupt."]
    #[inline(always)]
    pub fn oeim(&self) -> OeimR {
        OeimR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
End of Transmission interrupt mask. A read returns the current mask for UART's EoT interrupt. On a write of 1, the mask of the EoT interrupt is set which means the interrupt state will be reflected in MIS.EOTMIS. A write of 0 clears the mask which means MIS.EOTMIS will not reflect the interrupt."]
    #[inline(always)]
    pub fn eotim(&self) -> EotimR {
        EotimR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:31 - 31:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved12(&self) -> Reserved12R {
        Reserved12R::new((self.bits >> 12) & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved0(&mut self) -> Reserved0W<ImscSpec> {
        Reserved0W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Clear to Send (CTS) modem interrupt mask. A read returns the current mask for UART's clear to send interrupt. On a write of 1, the mask of the overrun error interrupt is set which means the interrupt state will be reflected in MIS.CTSMMIS. A write of 0 clears the mask which means MIS.CTSMMIS will not reflect the interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ctsmim(&mut self) -> CtsmimW<ImscSpec> {
        CtsmimW::new(self, 1)
    }
    #[doc = "Bits 2:3 - 3:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> Reserved2W<ImscSpec> {
        Reserved2W::new(self, 2)
    }
    #[doc = "Bit 4 - 4:4\\]
Receive interrupt mask. A read returns the current mask for UART's receive interrupt. On a write of 1, the mask of the overrun error interrupt is set which means the interrupt state will be reflected in MIS.RXMIS. A write of 0 clears the mask which means MIS.RXMIS will not reflect the interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn rxim(&mut self) -> RximW<ImscSpec> {
        RximW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Transmit interrupt mask. A read returns the current mask for UART's transmit interrupt. On a write of 1, the mask of the overrun error interrupt is set which means the interrupt state will be reflected in MIS.TXMIS. A write of 0 clears the mask which means MIS.TXMIS will not reflect the interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn txim(&mut self) -> TximW<ImscSpec> {
        TximW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Receive timeout interrupt mask. A read returns the current mask for UART's receive timeout interrupt. On a write of 1, the mask of the overrun error interrupt is set which means the interrupt state will be reflected in MIS.RTMIS. A write of 0 clears the mask which means this bitfield will not reflect the interrupt. The raw interrupt for receive timeout RIS.RTRIS cannot be set unless the mask is set (RTIM = 1). This is because the mask acts as an enable for power saving. That is, the same status can be read from MIS.RTMIS and RIS.RTRIS."]
    #[inline(always)]
    #[must_use]
    pub fn rtim(&mut self) -> RtimW<ImscSpec> {
        RtimW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Framing error interrupt mask. A read returns the current mask for UART's framing error interrupt. On a write of 1, the mask of the overrun error interrupt is set which means the interrupt state will be reflected in MIS.FEMIS. A write of 0 clears the mask which means MIS.FEMIS will not reflect the interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn feim(&mut self) -> FeimW<ImscSpec> {
        FeimW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Parity error interrupt mask. A read returns the current mask for UART's parity error interrupt. On a write of 1, the mask of the overrun error interrupt is set which means the interrupt state will be reflected in MIS.PEMIS. A write of 0 clears the mask which means MIS.PEMIS will not reflect the interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn peim(&mut self) -> PeimW<ImscSpec> {
        PeimW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Break error interrupt mask. A read returns the current mask for UART's break error interrupt. On a write of 1, the mask of the overrun error interrupt is set which means the interrupt state will be reflected in MIS.BEMIS. A write of 0 clears the mask which means MIS.BEMIS will not reflect the interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn beim(&mut self) -> BeimW<ImscSpec> {
        BeimW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
Overrun error interrupt mask. A read returns the current mask for UART's overrun error interrupt. On a write of 1, the mask of the overrun error interrupt is set which means the interrupt state will be reflected in MIS.OEMIS. A write of 0 clears the mask which means MIS.OEMIS will not reflect the interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn oeim(&mut self) -> OeimW<ImscSpec> {
        OeimW::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
End of Transmission interrupt mask. A read returns the current mask for UART's EoT interrupt. On a write of 1, the mask of the EoT interrupt is set which means the interrupt state will be reflected in MIS.EOTMIS. A write of 0 clears the mask which means MIS.EOTMIS will not reflect the interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn eotim(&mut self) -> EotimW<ImscSpec> {
        EotimW::new(self, 11)
    }
    #[doc = "Bits 12:31 - 31:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved12(&mut self) -> Reserved12W<ImscSpec> {
        Reserved12W::new(self, 12)
    }
}
#[doc = "Interrupt Mask Set/Clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imsc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`imsc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
