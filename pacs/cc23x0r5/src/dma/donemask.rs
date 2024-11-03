#[doc = "Register `DONEMASK` reader"]
pub type R = crate::R<DonemaskSpec>;
#[doc = "Register `DONEMASK` writer"]
pub type W = crate::W<DonemaskSpec>;
#[doc = "Field `CHNLS` reader - 7:0\\]
Controls the propagation of the uDMA done and active state to the assigned peripheral. Specifically used for software channels. Read as: Bit \\[Ch\\]
= 0: uDMA done and active state for channel Ch is not blocked from reaching to the peripherals. Note that the uDMA done state for channel \\[Ch\\]
is blocked from contributing to generation of combined uDMA done signal Bit \\[Ch\\]
= 1: uDMA done and active state for channel Ch is blocked from reaching to the peripherals. Note that the uDMA done state for channel \\[Ch\\]
is not blocked from contributing to generation of combined uDMA done signal Write as: Bit \\[Ch\\]
= 0: Allows uDMA done and active stat to propagate to the peripherals. Note that this disables uDMA done state for channel \\[Ch\\]
from contributing to generation of combined uDMA done signal Bit \\[Ch\\]
= 1: Blocks uDMA done and active state to propagate to the peripherals. Note that this enables uDMA done for channel \\[Ch\\]
to contribute to generation of combined uDMA done signal."]
pub type ChnlsR = crate::FieldReader;
#[doc = "Field `CHNLS` writer - 7:0\\]
Controls the propagation of the uDMA done and active state to the assigned peripheral. Specifically used for software channels. Read as: Bit \\[Ch\\]
= 0: uDMA done and active state for channel Ch is not blocked from reaching to the peripherals. Note that the uDMA done state for channel \\[Ch\\]
is blocked from contributing to generation of combined uDMA done signal Bit \\[Ch\\]
= 1: uDMA done and active state for channel Ch is blocked from reaching to the peripherals. Note that the uDMA done state for channel \\[Ch\\]
is not blocked from contributing to generation of combined uDMA done signal Write as: Bit \\[Ch\\]
= 0: Allows uDMA done and active stat to propagate to the peripherals. Note that this disables uDMA done state for channel \\[Ch\\]
from contributing to generation of combined uDMA done signal Bit \\[Ch\\]
= 1: Blocks uDMA done and active state to propagate to the peripherals. Note that this enables uDMA done for channel \\[Ch\\]
to contribute to generation of combined uDMA done signal."]
pub type ChnlsW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RESERVED8` reader - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED8` writer - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Controls the propagation of the uDMA done and active state to the assigned peripheral. Specifically used for software channels. Read as: Bit \\[Ch\\]
= 0: uDMA done and active state for channel Ch is not blocked from reaching to the peripherals. Note that the uDMA done state for channel \\[Ch\\]
is blocked from contributing to generation of combined uDMA done signal Bit \\[Ch\\]
= 1: uDMA done and active state for channel Ch is blocked from reaching to the peripherals. Note that the uDMA done state for channel \\[Ch\\]
is not blocked from contributing to generation of combined uDMA done signal Write as: Bit \\[Ch\\]
= 0: Allows uDMA done and active stat to propagate to the peripherals. Note that this disables uDMA done state for channel \\[Ch\\]
from contributing to generation of combined uDMA done signal Bit \\[Ch\\]
= 1: Blocks uDMA done and active state to propagate to the peripherals. Note that this enables uDMA done for channel \\[Ch\\]
to contribute to generation of combined uDMA done signal."]
    #[inline(always)]
    pub fn chnls(&self) -> ChnlsR {
        ChnlsR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&self) -> Reserved8R {
        Reserved8R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Controls the propagation of the uDMA done and active state to the assigned peripheral. Specifically used for software channels. Read as: Bit \\[Ch\\]
= 0: uDMA done and active state for channel Ch is not blocked from reaching to the peripherals. Note that the uDMA done state for channel \\[Ch\\]
is blocked from contributing to generation of combined uDMA done signal Bit \\[Ch\\]
= 1: uDMA done and active state for channel Ch is blocked from reaching to the peripherals. Note that the uDMA done state for channel \\[Ch\\]
is not blocked from contributing to generation of combined uDMA done signal Write as: Bit \\[Ch\\]
= 0: Allows uDMA done and active stat to propagate to the peripherals. Note that this disables uDMA done state for channel \\[Ch\\]
from contributing to generation of combined uDMA done signal Bit \\[Ch\\]
= 1: Blocks uDMA done and active state to propagate to the peripherals. Note that this enables uDMA done for channel \\[Ch\\]
to contribute to generation of combined uDMA done signal."]
    #[inline(always)]
    #[must_use]
    pub fn chnls(&mut self) -> ChnlsW<DonemaskSpec> {
        ChnlsW::new(self, 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved8(&mut self) -> Reserved8W<DonemaskSpec> {
        Reserved8W::new(self, 8)
    }
}
#[doc = "Channel Request Done Mask Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`donemask::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`donemask::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DonemaskSpec;
impl crate::RegisterSpec for DonemaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`donemask::R`](R) reader structure"]
impl crate::Readable for DonemaskSpec {}
#[doc = "`write(|w| ..)` method takes [`donemask::W`](W) writer structure"]
impl crate::Writable for DonemaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DONEMASK to value 0"]
impl crate::Resettable for DonemaskSpec {
    const RESET_VALUE: u32 = 0;
}
