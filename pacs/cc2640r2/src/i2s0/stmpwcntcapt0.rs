#[doc = "Register `STMPWCNTCAPT0` reader"]
pub type R = crate::R<Stmpwcntcapt0Spec>;
#[doc = "Register `STMPWCNTCAPT0` writer"]
pub type W = crate::W<Stmpwcntcapt0Spec>;
#[doc = "Field `CAPT_VALUE` reader - 15:0\\]
The value of the samplestamp WCLK counter (STMPWCNT.CURR_VALUE) last time an event was pulsed (event source selected in EVENT:I2SSTMPSEL0.EV for channel 0). This number corresponds to the number of positive WCLK edges since the samplestamp generator was enabled (not taking modification through STMPWADD/STMPWSET into account). The value is cleared when STMPCTL.STMP_EN = 0."]
pub type CaptValueR = crate::FieldReader<u16>;
#[doc = "Field `CAPT_VALUE` writer - 15:0\\]
The value of the samplestamp WCLK counter (STMPWCNT.CURR_VALUE) last time an event was pulsed (event source selected in EVENT:I2SSTMPSEL0.EV for channel 0). This number corresponds to the number of positive WCLK edges since the samplestamp generator was enabled (not taking modification through STMPWADD/STMPWSET into account). The value is cleared when STMPCTL.STMP_EN = 0."]
pub type CaptValueW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `RESERVED16` reader - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved16R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED16` writer - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved16W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
The value of the samplestamp WCLK counter (STMPWCNT.CURR_VALUE) last time an event was pulsed (event source selected in EVENT:I2SSTMPSEL0.EV for channel 0). This number corresponds to the number of positive WCLK edges since the samplestamp generator was enabled (not taking modification through STMPWADD/STMPWSET into account). The value is cleared when STMPCTL.STMP_EN = 0."]
    #[inline(always)]
    pub fn capt_value(&self) -> CaptValueR {
        CaptValueR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved16(&self) -> Reserved16R {
        Reserved16R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
The value of the samplestamp WCLK counter (STMPWCNT.CURR_VALUE) last time an event was pulsed (event source selected in EVENT:I2SSTMPSEL0.EV for channel 0). This number corresponds to the number of positive WCLK edges since the samplestamp generator was enabled (not taking modification through STMPWADD/STMPWSET into account). The value is cleared when STMPCTL.STMP_EN = 0."]
    #[inline(always)]
    #[must_use]
    pub fn capt_value(&mut self) -> CaptValueW<Stmpwcntcapt0Spec> {
        CaptValueW::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved16(&mut self) -> Reserved16W<Stmpwcntcapt0Spec> {
        Reserved16W::new(self, 16)
    }
}
#[doc = "Captured WCLK Counter Value, Capture Channel 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stmpwcntcapt0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stmpwcntcapt0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Stmpwcntcapt0Spec;
impl crate::RegisterSpec for Stmpwcntcapt0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stmpwcntcapt0::R`](R) reader structure"]
impl crate::Readable for Stmpwcntcapt0Spec {}
#[doc = "`write(|w| ..)` method takes [`stmpwcntcapt0::W`](W) writer structure"]
impl crate::Writable for Stmpwcntcapt0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STMPWCNTCAPT0 to value 0"]
impl crate::Resettable for Stmpwcntcapt0Spec {
    const RESET_VALUE: u32 = 0;
}
