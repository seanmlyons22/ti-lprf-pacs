#[doc = "Register `RXDPEEK` reader"]
pub type R = crate::R<RxdpeekSpec>;
#[doc = "Register `RXDPEEK` writer"]
pub type W = crate::W<RxdpeekSpec>;
#[doc = "Field `VAL` reader - 31:0\\]
Receive Data Peek Register. SACI command response word. RXCTL.RXDSTA not affected by read of RXDPEEK"]
pub type ValR = crate::FieldReader<u32>;
#[doc = "Field `VAL` writer - 31:0\\]
Receive Data Peek Register. SACI command response word. RXCTL.RXDSTA not affected by read of RXDPEEK"]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Receive Data Peek Register. SACI command response word. RXCTL.RXDSTA not affected by read of RXDPEEK"]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Receive Data Peek Register. SACI command response word. RXCTL.RXDSTA not affected by read of RXDPEEK"]
    #[inline(always)]
    #[must_use]
    pub fn val(&mut self) -> ValW<RxdpeekSpec> {
        ValW::new(self, 0)
    }
}
#[doc = "Receive data peek register. The RXDPEEK register is a read-only version of the RXD register that can be read by host and device without any side-effects This register is used to peek at the values in Receive Data Register without affecting the FULL/EMPTY flag.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxdpeek::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxdpeek::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxdpeekSpec;
impl crate::RegisterSpec for RxdpeekSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxdpeek::R`](R) reader structure"]
impl crate::Readable for RxdpeekSpec {}
#[doc = "`write(|w| ..)` method takes [`rxdpeek::W`](W) writer structure"]
impl crate::Writable for RxdpeekSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RXDPEEK to value 0"]
impl crate::Resettable for RxdpeekSpec {
    const RESET_VALUE: u32 = 0;
}
