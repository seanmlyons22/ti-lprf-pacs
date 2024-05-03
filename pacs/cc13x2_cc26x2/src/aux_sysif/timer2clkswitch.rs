#[doc = "Register `TIMER2CLKSWITCH` reader"]
pub type R = crate::R<Timer2clkswitchSpec>;
#[doc = "Register `TIMER2CLKSWITCH` writer"]
pub type W = crate::W<Timer2clkswitchSpec>;
#[doc = "Field `RDY` reader - 0:0\\]
Status of clock switcher. 0: TIMER2CLKCTL.SRC is different from TIMER2CLKSTAT.STAT. 1: TIMER2CLKCTL.SRC equals TIMER2CLKSTAT.STAT. RDY connects to AUX_EVCTL:EVSTAT3.AUX_TIMER2_CLKSWITCH_RDY."]
pub type RdyR = crate::BitReader;
#[doc = "Field `RDY` writer - 0:0\\]
Status of clock switcher. 0: TIMER2CLKCTL.SRC is different from TIMER2CLKSTAT.STAT. 1: TIMER2CLKCTL.SRC equals TIMER2CLKSTAT.STAT. RDY connects to AUX_EVCTL:EVSTAT3.AUX_TIMER2_CLKSWITCH_RDY."]
pub type RdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED1` reader - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED1` writer - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Status of clock switcher. 0: TIMER2CLKCTL.SRC is different from TIMER2CLKSTAT.STAT. 1: TIMER2CLKCTL.SRC equals TIMER2CLKSTAT.STAT. RDY connects to AUX_EVCTL:EVSTAT3.AUX_TIMER2_CLKSWITCH_RDY."]
    #[inline(always)]
    pub fn rdy(&self) -> RdyR {
        RdyR::new((self.bits & 1) != 0)
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
Status of clock switcher. 0: TIMER2CLKCTL.SRC is different from TIMER2CLKSTAT.STAT. 1: TIMER2CLKCTL.SRC equals TIMER2CLKSTAT.STAT. RDY connects to AUX_EVCTL:EVSTAT3.AUX_TIMER2_CLKSWITCH_RDY."]
    #[inline(always)]
    #[must_use]
    pub fn rdy(&mut self) -> RdyW<Timer2clkswitchSpec> {
        RdyW::new(self, 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<Timer2clkswitchSpec> {
        Reserved1W::new(self, 1)
    }
}
#[doc = "AUX_TIMER2 Clock Switch\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer2clkswitch::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer2clkswitch::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Timer2clkswitchSpec;
impl crate::RegisterSpec for Timer2clkswitchSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timer2clkswitch::R`](R) reader structure"]
impl crate::Readable for Timer2clkswitchSpec {}
#[doc = "`write(|w| ..)` method takes [`timer2clkswitch::W`](W) writer structure"]
impl crate::Writable for Timer2clkswitchSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIMER2CLKSWITCH to value 0x01"]
impl crate::Resettable for Timer2clkswitchSpec {
    const RESET_VALUE: u32 = 0x01;
}
