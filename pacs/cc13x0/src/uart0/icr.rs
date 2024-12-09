#[doc = "Register `ICR` reader"]
pub type R = crate::R<IcrSpec>;
#[doc = "Register `ICR` writer"]
pub type W = crate::W<IcrSpec>;
#[doc = "Field `RESERVED0` writer - 0:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior. Write 0."]
pub type Reserved0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTSMIC` writer - 1:1\\]
Clear to Send (CTS) modem interrupt clear: Writing 1 to this field clears the clear to send interrupt (RIS.CTSRMIS). Writing 0 has no effect."]
pub type CtsmicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED2` writer - 3:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior. Write 0"]
pub type Reserved2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RXIC` writer - 4:4\\]
Receive interrupt clear: Writing 1 to this field clears the receive interrupt (RIS.RXRIS). Writing 0 has no effect."]
pub type RxicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXIC` writer - 5:5\\]
Transmit interrupt clear: Writing 1 to this field clears the transmit interrupt (RIS.TXRIS). Writing 0 has no effect."]
pub type TxicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTIC` writer - 6:6\\]
Receive timeout interrupt clear: Writing 1 to this field clears the receive timeout interrupt (RIS.RTRIS). Writing 0 has no effect."]
pub type RticW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FEIC` writer - 7:7\\]
Framing error interrupt clear: Writing 1 to this field clears the framing error interrupt (RIS.FERIS). Writing 0 has no effect."]
pub type FeicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEIC` writer - 8:8\\]
Parity error interrupt clear: Writing 1 to this field clears the parity error interrupt (RIS.PERIS). Writing 0 has no effect."]
pub type PeicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BEIC` writer - 9:9\\]
Break error interrupt clear: Writing 1 to this field clears the break error interrupt (RIS.BERIS). Writing 0 has no effect."]
pub type BeicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OEIC` writer - 10:10\\]
Overrun error interrupt clear: Writing 1 to this field clears the overrun error interrupt (RIS.OERIS). Writing 0 has no effect."]
pub type OeicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED11` writer - 31:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved11W<'a, REG> = crate::FieldWriter<'a, REG, 21, u32>;
impl W {
    #[doc = "Bit 0 - 0:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior. Write 0."]
    #[inline(always)]
    #[must_use]
    pub fn reserved0(&mut self) -> Reserved0W<IcrSpec> {
        Reserved0W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Clear to Send (CTS) modem interrupt clear: Writing 1 to this field clears the clear to send interrupt (RIS.CTSRMIS). Writing 0 has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn ctsmic(&mut self) -> CtsmicW<IcrSpec> {
        CtsmicW::new(self, 1)
    }
    #[doc = "Bits 2:3 - 3:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior. Write 0"]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> Reserved2W<IcrSpec> {
        Reserved2W::new(self, 2)
    }
    #[doc = "Bit 4 - 4:4\\]
Receive interrupt clear: Writing 1 to this field clears the receive interrupt (RIS.RXRIS). Writing 0 has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn rxic(&mut self) -> RxicW<IcrSpec> {
        RxicW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Transmit interrupt clear: Writing 1 to this field clears the transmit interrupt (RIS.TXRIS). Writing 0 has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn txic(&mut self) -> TxicW<IcrSpec> {
        TxicW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Receive timeout interrupt clear: Writing 1 to this field clears the receive timeout interrupt (RIS.RTRIS). Writing 0 has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn rtic(&mut self) -> RticW<IcrSpec> {
        RticW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Framing error interrupt clear: Writing 1 to this field clears the framing error interrupt (RIS.FERIS). Writing 0 has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn feic(&mut self) -> FeicW<IcrSpec> {
        FeicW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Parity error interrupt clear: Writing 1 to this field clears the parity error interrupt (RIS.PERIS). Writing 0 has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn peic(&mut self) -> PeicW<IcrSpec> {
        PeicW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Break error interrupt clear: Writing 1 to this field clears the break error interrupt (RIS.BERIS). Writing 0 has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn beic(&mut self) -> BeicW<IcrSpec> {
        BeicW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
Overrun error interrupt clear: Writing 1 to this field clears the overrun error interrupt (RIS.OERIS). Writing 0 has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn oeic(&mut self) -> OeicW<IcrSpec> {
        OeicW::new(self, 10)
    }
    #[doc = "Bits 11:31 - 31:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved11(&mut self) -> Reserved11W<IcrSpec> {
        Reserved11W::new(self, 11)
    }
}
#[doc = "Interrupt Clear On a write of 1, the corresponding interrupt is cleared. A write of 0 has no effect.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcrSpec;
impl crate::RegisterSpec for IcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icr::R`](R) reader structure"]
impl crate::Readable for IcrSpec {}
#[doc = "`write(|w| ..)` method takes [`icr::W`](W) writer structure"]
impl crate::Writable for IcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::Resettable for IcrSpec {
    const RESET_VALUE: u32 = 0;
}
