#[doc = "Register `WDTTEST` reader"]
pub type R = crate::R<WdttestSpec>;
#[doc = "Register `WDTTEST` writer"]
pub type W = crate::W<WdttestSpec>;
#[doc = "Field `STALLEN` reader - 0:0\\]
WDT Stall Enable 0: The WDT timer continues counting if the CPU is stopped with a debugger. 1: If the CPU is stopped with a debugger, the WDT stops counting. Once the CPU is restarted, the WDT resumes counting."]
pub type StallenR = crate::BitReader;
#[doc = "Field `STALLEN` writer - 0:0\\]
WDT Stall Enable 0: The WDT timer continues counting if the CPU is stopped with a debugger. 1: If the CPU is stopped with a debugger, the WDT stops counting. Once the CPU is restarted, the WDT resumes counting."]
pub type StallenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED0` reader - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved0R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED0` writer - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved0W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
WDT Stall Enable 0: The WDT timer continues counting if the CPU is stopped with a debugger. 1: If the CPU is stopped with a debugger, the WDT stops counting. Once the CPU is restarted, the WDT resumes counting."]
    #[inline(always)]
    pub fn stallen(&self) -> StallenR {
        StallenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
WDT Stall Enable 0: The WDT timer continues counting if the CPU is stopped with a debugger. 1: If the CPU is stopped with a debugger, the WDT stops counting. Once the CPU is restarted, the WDT resumes counting."]
    #[inline(always)]
    #[must_use]
    pub fn stallen(&mut self) -> StallenW<WdttestSpec> {
        StallenW::new(self, 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved0(&mut self) -> Reserved0W<WdttestSpec> {
        Reserved0W::new(self, 1)
    }
}
#[doc = "Test Mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdttest::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdttest::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WdttestSpec;
impl crate::RegisterSpec for WdttestSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdttest::R`](R) reader structure"]
impl crate::Readable for WdttestSpec {}
#[doc = "`write(|w| ..)` method takes [`wdttest::W`](W) writer structure"]
impl crate::Writable for WdttestSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WDTTEST to value 0"]
impl crate::Resettable for WdttestSpec {
    const RESET_VALUE: u32 = 0;
}
