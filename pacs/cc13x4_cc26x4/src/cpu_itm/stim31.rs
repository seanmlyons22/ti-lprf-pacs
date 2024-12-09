#[doc = "Register `STIM31` reader"]
pub type R = crate::R<Stim31Spec>;
#[doc = "Register `STIM31` writer"]
pub type W = crate::W<Stim31Spec>;
#[doc = "Field `FIFOREADY` reader - 0:0\\]
Indicates whether the Stimulus Port can accept data"]
pub type FiforeadyR = crate::BitReader;
#[doc = "Field `DISABLED` reader - 1:1\\]
Indicates whether the Stimulus Port is enabled or disabled"]
pub type DisabledR = crate::BitReader;
#[doc = "Field `RESERVED2` reader - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Indicates whether the Stimulus Port can accept data"]
    #[inline(always)]
    pub fn fifoready(&self) -> FiforeadyR {
        FiforeadyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Indicates whether the Stimulus Port is enabled or disabled"]
    #[inline(always)]
    pub fn disabled(&self) -> DisabledR {
        DisabledR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {}
#[doc = "Provides the interface for generating Instrumentation packets\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stim31::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stim31::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Stim31Spec;
impl crate::RegisterSpec for Stim31Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stim31::R`](R) reader structure"]
impl crate::Readable for Stim31Spec {}
#[doc = "`write(|w| ..)` method takes [`stim31::W`](W) writer structure"]
impl crate::Writable for Stim31Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STIM31 to value 0"]
impl crate::Resettable for Stim31Spec {
    const RESET_VALUE: u32 = 0;
}
