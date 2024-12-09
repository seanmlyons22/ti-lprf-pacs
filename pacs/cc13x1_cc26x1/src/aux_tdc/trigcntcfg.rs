#[doc = "Register `TRIGCNTCFG` reader"]
pub type R = crate::R<TrigcntcfgSpec>;
#[doc = "Register `TRIGCNTCFG` writer"]
pub type W = crate::W<TrigcntcfgSpec>;
#[doc = "Field `EN` reader - 0:0\\]
Enable stop-counter. 0: Disable stop-counter. 1: Enable stop-counter. Change only while STAT.STATE is IDLE."]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - 0:0\\]
Enable stop-counter. 0: Disable stop-counter. 1: Enable stop-counter. Change only while STAT.STATE is IDLE."]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED1` reader - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Enable stop-counter. 0: Disable stop-counter. 1: Enable stop-counter. Change only while STAT.STATE is IDLE."]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Enable stop-counter. 0: Disable stop-counter. 1: Enable stop-counter. Change only while STAT.STATE is IDLE."]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EnW<TrigcntcfgSpec> {
        EnW::new(self, 0)
    }
}
#[doc = "Trigger Counter Configuration Stop-counter configuration.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`trigcntcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`trigcntcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TrigcntcfgSpec;
impl crate::RegisterSpec for TrigcntcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trigcntcfg::R`](R) reader structure"]
impl crate::Readable for TrigcntcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`trigcntcfg::W`](W) writer structure"]
impl crate::Writable for TrigcntcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TRIGCNTCFG to value 0"]
impl crate::Resettable for TrigcntcfgSpec {
    const RESET_VALUE: u32 = 0;
}
