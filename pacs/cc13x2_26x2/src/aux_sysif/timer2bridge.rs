#[doc = "Register `TIMER2BRIDGE` reader"]
pub type R = crate::R<Timer2bridgeSpec>;
#[doc = "Register `TIMER2BRIDGE` writer"]
pub type W = crate::W<Timer2bridgeSpec>;
#[doc = "Field `BUSY` reader - 0:0\\]
Status of bus transactions to AUX_TIMER2. 0: No unfinished bus transactions. 1: A bus transaction is ongoing."]
pub type BusyR = crate::BitReader;
#[doc = "Field `RESERVED1` reader - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Status of bus transactions to AUX_TIMER2. 0: No unfinished bus transactions. 1: A bus transaction is ongoing."]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {}
#[doc = "AUX_TIMER2 Bridge\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer2bridge::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer2bridge::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Timer2bridgeSpec;
impl crate::RegisterSpec for Timer2bridgeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timer2bridge::R`](R) reader structure"]
impl crate::Readable for Timer2bridgeSpec {}
#[doc = "`write(|w| ..)` method takes [`timer2bridge::W`](W) writer structure"]
impl crate::Writable for Timer2bridgeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIMER2BRIDGE to value 0"]
impl crate::Resettable for Timer2bridgeSpec {
    const RESET_VALUE: u32 = 0;
}
