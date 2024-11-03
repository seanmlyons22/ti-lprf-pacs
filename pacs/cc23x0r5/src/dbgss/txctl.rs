#[doc = "Register `TXCTL` reader"]
pub type R = crate::R<TxctlSpec>;
#[doc = "Register `TXCTL` writer"]
pub type W = crate::W<TxctlSpec>;
#[doc = "0:0\\]
Indicates whether the host has written a word to the TXD register, which can be read by the device: TXDSTA is automatically set upon write to \\[SWD_TAP::SECAP.TXD.*\\]
and automatically cleared upon read from TXD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txdsta {
    #[doc = "1: The TXD register contains a new SACI parameter word from the host, which can be read by the device."]
    Full = 1,
    #[doc = "0: The TXD register does not contain a new SACI parameter word from the host, and should not be read by the device."]
    Empty = 0,
}
impl From<Txdsta> for bool {
    #[inline(always)]
    fn from(variant: Txdsta) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXDSTA` reader - 0:0\\]
Indicates whether the host has written a word to the TXD register, which can be read by the device: TXDSTA is automatically set upon write to \\[SWD_TAP::SECAP.TXD.*\\]
and automatically cleared upon read from TXD"]
pub type TxdstaR = crate::BitReader<Txdsta>;
impl TxdstaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txdsta {
        match self.bits {
            true => Txdsta::Full,
            false => Txdsta::Empty,
        }
    }
    #[doc = "The TXD register contains a new SACI parameter word from the host, which can be read by the device."]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == Txdsta::Full
    }
    #[doc = "The TXD register does not contain a new SACI parameter word from the host, and should not be read by the device."]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == Txdsta::Empty
    }
}
#[doc = "Field `TXDSTA` writer - 0:0\\]
Indicates whether the host has written a word to the TXD register, which can be read by the device: TXDSTA is automatically set upon write to \\[SWD_TAP::SECAP.TXD.*\\]
and automatically cleared upon read from TXD"]
pub type TxdstaW<'a, REG> = crate::BitWriter<'a, REG, Txdsta>;
impl<'a, REG> TxdstaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The TXD register contains a new SACI parameter word from the host, which can be read by the device."]
    #[inline(always)]
    pub fn full(self) -> &'a mut crate::W<REG> {
        self.variant(Txdsta::Full)
    }
    #[doc = "The TXD register does not contain a new SACI parameter word from the host, and should not be read by the device."]
    #[inline(always)]
    pub fn empty(self) -> &'a mut crate::W<REG> {
        self.variant(Txdsta::Empty)
    }
}
#[doc = "Field `FLAGS` reader - 7:1\\]
Software defined flags that are used by the SACI protocol (host to device)."]
pub type FlagsR = crate::FieldReader;
#[doc = "Field `FLAGS` writer - 7:1\\]
Software defined flags that are used by the SACI protocol (host to device)."]
pub type FlagsW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `RESERVED8` reader - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED8` writer - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Indicates whether the host has written a word to the TXD register, which can be read by the device: TXDSTA is automatically set upon write to \\[SWD_TAP::SECAP.TXD.*\\]
and automatically cleared upon read from TXD"]
    #[inline(always)]
    pub fn txdsta(&self) -> TxdstaR {
        TxdstaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - 7:1\\]
Software defined flags that are used by the SACI protocol (host to device)."]
    #[inline(always)]
    pub fn flags(&self) -> FlagsR {
        FlagsR::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&self) -> Reserved8R {
        Reserved8R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Indicates whether the host has written a word to the TXD register, which can be read by the device: TXDSTA is automatically set upon write to \\[SWD_TAP::SECAP.TXD.*\\]
and automatically cleared upon read from TXD"]
    #[inline(always)]
    #[must_use]
    pub fn txdsta(&mut self) -> TxdstaW<TxctlSpec> {
        TxdstaW::new(self, 0)
    }
    #[doc = "Bits 1:7 - 7:1\\]
Software defined flags that are used by the SACI protocol (host to device)."]
    #[inline(always)]
    #[must_use]
    pub fn flags(&mut self) -> FlagsW<TxctlSpec> {
        FlagsW::new(self, 1)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved8(&mut self) -> Reserved8W<TxctlSpec> {
        Reserved8W::new(self, 8)
    }
}
#[doc = "Transmit control register. This register contains status of the TXD register (full/empty), and also software defined flags that are used by the SACI protocol. The host (SWD interface) can write the FLAGS field of the TXCTL register. The host (SWD interface) can read the TXCTL register. The device (boot code) can only read the TXCTL register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxctlSpec;
impl crate::RegisterSpec for TxctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txctl::R`](R) reader structure"]
impl crate::Readable for TxctlSpec {}
#[doc = "`write(|w| ..)` method takes [`txctl::W`](W) writer structure"]
impl crate::Writable for TxctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TXCTL to value 0"]
impl crate::Resettable for TxctlSpec {
    const RESET_VALUE: u32 = 0;
}
