#[doc = "Register `PRECFG` reader"]
pub type R = crate::R<PrecfgSpec>;
#[doc = "Register `PRECFG` writer"]
pub type W = crate::W<PrecfgSpec>;
#[doc = "Field `CLKDIV` reader - 7:0\\]
Clock division. CLKDIV determines the timer clock frequency for counter, synchronization, and timer event updates. The timer clock frequency is the clock selected by AUX_SYSIF:TIMER2CLKCTL.SRC divided by (CLKDIV + 1). This inverse is the timer clock period. 0x00: Divide by 1. 0x01: Divide by 2. ... 0xFF: Divide by 256."]
pub type ClkdivR = crate::FieldReader;
#[doc = "Field `CLKDIV` writer - 7:0\\]
Clock division. CLKDIV determines the timer clock frequency for counter, synchronization, and timer event updates. The timer clock frequency is the clock selected by AUX_SYSIF:TIMER2CLKCTL.SRC divided by (CLKDIV + 1). This inverse is the timer clock period. 0x00: Divide by 1. 0x01: Divide by 2. ... 0xFF: Divide by 256."]
pub type ClkdivW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RESERVED8` reader - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED8` writer - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Clock division. CLKDIV determines the timer clock frequency for counter, synchronization, and timer event updates. The timer clock frequency is the clock selected by AUX_SYSIF:TIMER2CLKCTL.SRC divided by (CLKDIV + 1). This inverse is the timer clock period. 0x00: Divide by 1. 0x01: Divide by 2. ... 0xFF: Divide by 256."]
    #[inline(always)]
    pub fn clkdiv(&self) -> ClkdivR {
        ClkdivR::new((self.bits & 0xff) as u8)
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
Clock division. CLKDIV determines the timer clock frequency for counter, synchronization, and timer event updates. The timer clock frequency is the clock selected by AUX_SYSIF:TIMER2CLKCTL.SRC divided by (CLKDIV + 1). This inverse is the timer clock period. 0x00: Divide by 1. 0x01: Divide by 2. ... 0xFF: Divide by 256."]
    #[inline(always)]
    #[must_use]
    pub fn clkdiv(&mut self) -> ClkdivW<PrecfgSpec> {
        ClkdivW::new(self, 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved8(&mut self) -> Reserved8W<PrecfgSpec> {
        Reserved8W::new(self, 8)
    }
}
#[doc = "Clock Prescaler Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`precfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`precfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PrecfgSpec;
impl crate::RegisterSpec for PrecfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`precfg::R`](R) reader structure"]
impl crate::Readable for PrecfgSpec {}
#[doc = "`write(|w| ..)` method takes [`precfg::W`](W) writer structure"]
impl crate::Writable for PrecfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRECFG to value 0"]
impl crate::Resettable for PrecfgSpec {
    const RESET_VALUE: u32 = 0;
}
