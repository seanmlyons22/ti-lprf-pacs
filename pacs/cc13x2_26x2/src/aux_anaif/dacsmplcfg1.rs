#[doc = "Register `DACSMPLCFG1` reader"]
pub type R = crate::R<Dacsmplcfg1Spec>;
#[doc = "Register `DACSMPLCFG1` writer"]
pub type W = crate::W<Dacsmplcfg1Spec>;
#[doc = "Field `HOLD_INTERVAL` reader - 7:0\\]
Hold interval. Number of inactive sample clock periods between each active sample clock period during hold phase. The sample clock is low when inactive. The range is 0 to 255."]
pub type HoldIntervalR = crate::FieldReader;
#[doc = "Field `HOLD_INTERVAL` writer - 7:0\\]
Hold interval. Number of inactive sample clock periods between each active sample clock period during hold phase. The sample clock is low when inactive. The range is 0 to 255."]
pub type HoldIntervalW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SETUP_CNT` reader - 11:8\\]
Setup count. Number of active sample clock periods during the setup phase. 0: 1 sample clock period 1: 2 sample clock periods ... 15 : 16 sample clock periods"]
pub type SetupCntR = crate::FieldReader;
#[doc = "Field `SETUP_CNT` writer - 11:8\\]
Setup count. Number of active sample clock periods during the setup phase. 0: 1 sample clock period 1: 2 sample clock periods ... 15 : 16 sample clock periods"]
pub type SetupCntW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `L_PER` reader - 13:12\\]
Low time. The sample clock period is low for this many base periods. 0: 1 period 1: 2 periods 2: 3 periods 3: 4 periods"]
pub type LPerR = crate::FieldReader;
#[doc = "Field `L_PER` writer - 13:12\\]
Low time. The sample clock period is low for this many base periods. 0: 1 period 1: 2 periods 2: 3 periods 3: 4 periods"]
pub type LPerW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `H_PER` reader - 14:14\\]
High time. The sample clock period is high for this many base periods. 0: 2 periods 1: 4 periods"]
pub type HPerR = crate::BitReader;
#[doc = "Field `H_PER` writer - 14:14\\]
High time. The sample clock period is high for this many base periods. 0: 2 periods 1: 4 periods"]
pub type HPerW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED15` reader - 31:15\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved15R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED15` writer - 31:15\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved15W<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Hold interval. Number of inactive sample clock periods between each active sample clock period during hold phase. The sample clock is low when inactive. The range is 0 to 255."]
    #[inline(always)]
    pub fn hold_interval(&self) -> HoldIntervalR {
        HoldIntervalR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Setup count. Number of active sample clock periods during the setup phase. 0: 1 sample clock period 1: 2 sample clock periods ... 15 : 16 sample clock periods"]
    #[inline(always)]
    pub fn setup_cnt(&self) -> SetupCntR {
        SetupCntR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:13 - 13:12\\]
Low time. The sample clock period is low for this many base periods. 0: 1 period 1: 2 periods 2: 3 periods 3: 4 periods"]
    #[inline(always)]
    pub fn l_per(&self) -> LPerR {
        LPerR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - 14:14\\]
High time. The sample clock period is high for this many base periods. 0: 2 periods 1: 4 periods"]
    #[inline(always)]
    pub fn h_per(&self) -> HPerR {
        HPerR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 15:31 - 31:15\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved15(&self) -> Reserved15R {
        Reserved15R::new((self.bits >> 15) & 0x0001_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Hold interval. Number of inactive sample clock periods between each active sample clock period during hold phase. The sample clock is low when inactive. The range is 0 to 255."]
    #[inline(always)]
    #[must_use]
    pub fn hold_interval(&mut self) -> HoldIntervalW<Dacsmplcfg1Spec> {
        HoldIntervalW::new(self, 0)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Setup count. Number of active sample clock periods during the setup phase. 0: 1 sample clock period 1: 2 sample clock periods ... 15 : 16 sample clock periods"]
    #[inline(always)]
    #[must_use]
    pub fn setup_cnt(&mut self) -> SetupCntW<Dacsmplcfg1Spec> {
        SetupCntW::new(self, 8)
    }
    #[doc = "Bits 12:13 - 13:12\\]
Low time. The sample clock period is low for this many base periods. 0: 1 period 1: 2 periods 2: 3 periods 3: 4 periods"]
    #[inline(always)]
    #[must_use]
    pub fn l_per(&mut self) -> LPerW<Dacsmplcfg1Spec> {
        LPerW::new(self, 12)
    }
    #[doc = "Bit 14 - 14:14\\]
High time. The sample clock period is high for this many base periods. 0: 2 periods 1: 4 periods"]
    #[inline(always)]
    #[must_use]
    pub fn h_per(&mut self) -> HPerW<Dacsmplcfg1Spec> {
        HPerW::new(self, 14)
    }
    #[doc = "Bits 15:31 - 31:15\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved15(&mut self) -> Reserved15W<Dacsmplcfg1Spec> {
        Reserved15W::new(self, 15)
    }
}
#[doc = "DAC Sample Configuration 1 The sample clock period equals (high time + low time) * base period. DACSMPLCFG0.CLKDIV determines the base period. Timing requirements (DAC Buffer On / DAC Buffer Off): - (high time + low time) * base period > (4 us / 1 us) - (high time * base period) > (2 us / 0.5 us) - (low time * base period) > (2 us / 0.5 us) - (low time * base period + HOLD_INTERVAL * sample clock period) &lt; 32 us If AUX_SYSIF:OPMODEREQ.REQ equals PDLP, you must set: - H_PER = L_PER = HOLD_INTERVAL = 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dacsmplcfg1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dacsmplcfg1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dacsmplcfg1Spec;
impl crate::RegisterSpec for Dacsmplcfg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dacsmplcfg1::R`](R) reader structure"]
impl crate::Readable for Dacsmplcfg1Spec {}
#[doc = "`write(|w| ..)` method takes [`dacsmplcfg1::W`](W) writer structure"]
impl crate::Writable for Dacsmplcfg1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DACSMPLCFG1 to value 0"]
impl crate::Resettable for Dacsmplcfg1Spec {
    const RESET_VALUE: u32 = 0;
}
