#[doc = "Register `RXD` reader"]
pub type R = crate::R<RxdSpec>;
#[doc = "Register `RXD` writer"]
pub type W = crate::W<RxdSpec>;
#[doc = "Field `VAL` reader - 31:0\\]
SACI command response word. RXCTL.RXDSTA automatically set upon write. RXCTL.RXDSTA automatically cleared upon read (flush operation)."]
pub type ValR = crate::FieldReader<u32>;
#[doc = "Field `VAL` writer - 31:0\\]
SACI command response word. RXCTL.RXDSTA automatically set upon write. RXCTL.RXDSTA automatically cleared upon read (flush operation)."]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
SACI command response word. RXCTL.RXDSTA automatically set upon write. RXCTL.RXDSTA automatically cleared upon read (flush operation)."]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
SACI command response word. RXCTL.RXDSTA automatically set upon write. RXCTL.RXDSTA automatically cleared upon read (flush operation)."]
    #[inline(always)]
    #[must_use]
    pub fn val(&mut self) -> ValW<RxdSpec> {
        ValW::new(self, 0)
    }
}
#[doc = "Receive data register. This register is used to send SACI command response data from the device to the host. The device (boot code) can write the RXD register. This updates the value of RXD, and sets RXCTL.RXDSTA = FULL. The device should only write RXD while RXCTL.RXDSTA = EMPTY. If the device incorrectly writes RXD while RXCTL.RXDSTA = FULL, this will just update the value of RXD. The device (boot code) can read the RXD register in order to flush it. This sets RXCTL.RXDSTA = EMPTY. The host (SWD interface) can only read the RXD register. This sets RXCTL.RXDSTA = EMPTY. The host should only read RXD while RXCTL.RXDSTA = FULL. If the host incorrectly reads RXD while RXCTL.RXDSTA = EMPTY, this will just return the value of RXD. If the device writes RXD on the same clock cycle as the host reads RXD: The host reads the old RXD value. RXD is updated with the new value, and RXCTL.RXDSTA is set to FULL.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxdSpec;
impl crate::RegisterSpec for RxdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxd::R`](R) reader structure"]
impl crate::Readable for RxdSpec {}
#[doc = "`write(|w| ..)` method takes [`rxd::W`](W) writer structure"]
impl crate::Writable for RxdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RXD to value 0"]
impl crate::Resettable for RxdSpec {
    const RESET_VALUE: u32 = 0;
}
