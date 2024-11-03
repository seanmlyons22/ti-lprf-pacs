#[doc = "Register `TXD` reader"]
pub type R = crate::R<TxdSpec>;
#[doc = "Register `TXD` writer"]
pub type W = crate::W<TxdSpec>;
#[doc = "Field `VAL` reader - 31:0\\]
SACI command/parameter word. Valid value when TXCTL.TXDSTA=1. TXCTL.TXDSTA gets automatically cleared upon read."]
pub type ValR = crate::FieldReader<u32>;
#[doc = "Field `VAL` writer - 31:0\\]
SACI command/parameter word. Valid value when TXCTL.TXDSTA=1. TXCTL.TXDSTA gets automatically cleared upon read."]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
SACI command/parameter word. Valid value when TXCTL.TXDSTA=1. TXCTL.TXDSTA gets automatically cleared upon read."]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
SACI command/parameter word. Valid value when TXCTL.TXDSTA=1. TXCTL.TXDSTA gets automatically cleared upon read."]
    #[inline(always)]
    #[must_use]
    pub fn val(&mut self) -> ValW<TxdSpec> {
        ValW::new(self, 0)
    }
}
#[doc = "Transmit data register. This register is used for sending SACI command parameter data from the host to the device. The host (SWD interface) can write this register. This updates the value of TXD, and sets TXCTL.TXDSTA = FULL The host should only write TXD while TXCTL.TXDSTA = EMPTY. If the host incorrectly writes TXD while TXCTL.TXDSTA = FULL, this will just update the value of TXD. The host (SWD interface) can read the TXD register. This does not affect TXCTL.TXDSTA. The device (boot code) can only read the TXD register. This sets TXCTL.TXDSTA = EMPTY. The device should only read TXD while TXCTL.TXDSTA = FULL. If the device incorrectly reads TXD while TXCTL.TXDSTA = EMPTY, this will just return the value of TXD. If the host writes TXD on the same clock cycle as the device reads TXD: The device reads the old TXD value. TXD is updated with the new value, and TXCTL.TXDSTA is set to FULL.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxdSpec;
impl crate::RegisterSpec for TxdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txd::R`](R) reader structure"]
impl crate::Readable for TxdSpec {}
#[doc = "`write(|w| ..)` method takes [`txd::W`](W) writer structure"]
impl crate::Writable for TxdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TXD to value 0"]
impl crate::Resettable for TxdSpec {
    const RESET_VALUE: u32 = 0;
}
