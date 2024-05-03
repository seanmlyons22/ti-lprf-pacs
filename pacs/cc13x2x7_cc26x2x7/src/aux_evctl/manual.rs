#[doc = "Register `MANUAL` reader"]
pub type R = crate::R<ManualSpec>;
#[doc = "Register `MANUAL` writer"]
pub type W = crate::W<ManualSpec>;
#[doc = "Field `EV` reader - 0:0\\]
This bit field sets the value of EVSTAT2.MANUAL_EV."]
pub type EvR = crate::BitReader;
#[doc = "Field `EV` writer - 0:0\\]
This bit field sets the value of EVSTAT2.MANUAL_EV."]
pub type EvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED1` reader - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED1` writer - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
This bit field sets the value of EVSTAT2.MANUAL_EV."]
    #[inline(always)]
    pub fn ev(&self) -> EvR {
        EvR::new((self.bits & 1) != 0)
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
This bit field sets the value of EVSTAT2.MANUAL_EV."]
    #[inline(always)]
    #[must_use]
    pub fn ev(&mut self) -> EvW<ManualSpec> {
        EvW::new(self, 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<ManualSpec> {
        Reserved1W::new(self, 1)
    }
}
#[doc = "Manual Programmable event.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`manual::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`manual::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ManualSpec;
impl crate::RegisterSpec for ManualSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`manual::R`](R) reader structure"]
impl crate::Readable for ManualSpec {}
#[doc = "`write(|w| ..)` method takes [`manual::W`](W) writer structure"]
impl crate::Writable for ManualSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MANUAL to value 0"]
impl crate::Resettable for ManualSpec {
    const RESET_VALUE: u32 = 0;
}
