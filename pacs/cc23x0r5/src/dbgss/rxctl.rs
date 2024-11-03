#[doc = "Register `RXCTL` reader"]
pub type R = crate::R<RxctlSpec>;
#[doc = "Register `RXCTL` writer"]
pub type W = crate::W<RxctlSpec>;
#[doc = "0:0\\]
Indicates whether the device has written a word to the RXD register, which can be read by the host: RXDSTA is automatically set upon write to RXD and automatically cleared upon read from \\[SWD_TAP::SECAP.RXD.*\\]
or RXD.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxdsta {
    #[doc = "1: The RXD register contains a new SACI response word from the device, which can be read by the host."]
    Full = 1,
    #[doc = "0: The RXD register does not contain a new SACI response word from the device, and should not be read by the host."]
    Empty = 0,
}
impl From<Rxdsta> for bool {
    #[inline(always)]
    fn from(variant: Rxdsta) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXDSTA` reader - 0:0\\]
Indicates whether the device has written a word to the RXD register, which can be read by the host: RXDSTA is automatically set upon write to RXD and automatically cleared upon read from \\[SWD_TAP::SECAP.RXD.*\\]
or RXD."]
pub type RxdstaR = crate::BitReader<Rxdsta>;
impl RxdstaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxdsta {
        match self.bits {
            true => Rxdsta::Full,
            false => Rxdsta::Empty,
        }
    }
    #[doc = "The RXD register contains a new SACI response word from the device, which can be read by the host."]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == Rxdsta::Full
    }
    #[doc = "The RXD register does not contain a new SACI response word from the device, and should not be read by the host."]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == Rxdsta::Empty
    }
}
#[doc = "Field `RXDSTA` writer - 0:0\\]
Indicates whether the device has written a word to the RXD register, which can be read by the host: RXDSTA is automatically set upon write to RXD and automatically cleared upon read from \\[SWD_TAP::SECAP.RXD.*\\]
or RXD."]
pub type RxdstaW<'a, REG> = crate::BitWriter<'a, REG, Rxdsta>;
impl<'a, REG> RxdstaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The RXD register contains a new SACI response word from the device, which can be read by the host."]
    #[inline(always)]
    pub fn full(self) -> &'a mut crate::W<REG> {
        self.variant(Rxdsta::Full)
    }
    #[doc = "The RXD register does not contain a new SACI response word from the device, and should not be read by the host."]
    #[inline(always)]
    pub fn empty(self) -> &'a mut crate::W<REG> {
        self.variant(Rxdsta::Empty)
    }
}
#[doc = "Field `FLAGS` reader - 7:1\\]
Software defined flags that are used by the SACI protocol (device to host)."]
pub type FlagsR = crate::FieldReader;
#[doc = "Field `FLAGS` writer - 7:1\\]
Software defined flags that are used by the SACI protocol (device to host)."]
pub type FlagsW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `RESERVED8` reader - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED8` writer - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Indicates whether the device has written a word to the RXD register, which can be read by the host: RXDSTA is automatically set upon write to RXD and automatically cleared upon read from \\[SWD_TAP::SECAP.RXD.*\\]
or RXD."]
    #[inline(always)]
    pub fn rxdsta(&self) -> RxdstaR {
        RxdstaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - 7:1\\]
Software defined flags that are used by the SACI protocol (device to host)."]
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
Indicates whether the device has written a word to the RXD register, which can be read by the host: RXDSTA is automatically set upon write to RXD and automatically cleared upon read from \\[SWD_TAP::SECAP.RXD.*\\]
or RXD."]
    #[inline(always)]
    #[must_use]
    pub fn rxdsta(&mut self) -> RxdstaW<RxctlSpec> {
        RxdstaW::new(self, 0)
    }
    #[doc = "Bits 1:7 - 7:1\\]
Software defined flags that are used by the SACI protocol (device to host)."]
    #[inline(always)]
    #[must_use]
    pub fn flags(&mut self) -> FlagsW<RxctlSpec> {
        FlagsW::new(self, 1)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved8(&mut self) -> Reserved8W<RxctlSpec> {
        Reserved8W::new(self, 8)
    }
}
#[doc = "Receive control register. This register contains status of the RXD register (full/empty), and also software defined flags that are used by the SACI protocol. The device (boot code) can write the FLAGS field of the RXCTL register. The device (boot code) can read the RXCTL register. The host (SWD interface) can only read the RXCTL register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxctlSpec;
impl crate::RegisterSpec for RxctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxctl::R`](R) reader structure"]
impl crate::Readable for RxctlSpec {}
#[doc = "`write(|w| ..)` method takes [`rxctl::W`](W) writer structure"]
impl crate::Writable for RxctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RXCTL to value 0"]
impl crate::Resettable for RxctlSpec {
    const RESET_VALUE: u32 = 0;
}
