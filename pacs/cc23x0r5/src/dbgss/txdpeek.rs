#[doc = "Register `TXDPEEK` reader"]
pub type R = crate::R<TxdpeekSpec>;
#[doc = "Register `TXDPEEK` writer"]
pub type W = crate::W<TxdpeekSpec>;
#[doc = "Field `VAL` reader - 31:0\\]
Transmit Data Peek Register. SACI command parameter word. TXCTL.TXDSTA not affected by read of TXDPEEK"]
pub type ValR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Transmit Data Peek Register. SACI command parameter word. TXCTL.TXDSTA not affected by read of TXDPEEK"]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new(self.bits)
    }
}
impl W {}
#[doc = "Transmit data peek register . This register is a read-only version of the TXD register that can be read by host and device without any side-effects. This register is used to peek at the values in TXD without affecting the FULL/EMPTY flag.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txdpeek::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txdpeek::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxdpeekSpec;
impl crate::RegisterSpec for TxdpeekSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txdpeek::R`](R) reader structure"]
impl crate::Readable for TxdpeekSpec {}
#[doc = "`write(|w| ..)` method takes [`txdpeek::W`](W) writer structure"]
impl crate::Writable for TxdpeekSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TXDPEEK to value 0"]
impl crate::Resettable for TxdpeekSpec {
    const RESET_VALUE: u32 = 0;
}
