#[doc = "Register `RECHARGECFG` reader"]
pub type R = crate::R<RechargecfgSpec>;
#[doc = "Register `RECHARGECFG` writer"]
pub type W = crate::W<RechargecfgSpec>;
#[doc = "Field `PER_E` reader - 2:0\\]
Number of 32 KHz clocks between activation of recharge controller For recharge algorithm, PERIOD is the initial period when entering powerdown mode. The adaptive recharge algorithm will not change this register PERIOD will effectively be a 16 bit value coded in a 5 bit mantissa and 3 bit exponent: This field sets the Exponent of the Period. PERIOD=(PER_M*16+15)*2^PER_E"]
pub type PerER = crate::FieldReader;
#[doc = "Field `PER_E` writer - 2:0\\]
Number of 32 KHz clocks between activation of recharge controller For recharge algorithm, PERIOD is the initial period when entering powerdown mode. The adaptive recharge algorithm will not change this register PERIOD will effectively be a 16 bit value coded in a 5 bit mantissa and 3 bit exponent: This field sets the Exponent of the Period. PERIOD=(PER_M*16+15)*2^PER_E"]
pub type PerEW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PER_M` reader - 7:3\\]
Number of 32 KHz clocks between activation of recharge controller For recharge algorithm, PERIOD is the initial period when entering powerdown mode. The adaptive recharge algorithm will not change this register PERIOD will effectively be a 16 bit value coded in a 5 bit mantissa and 3 bit exponent: This field sets the Mantissa of the Period. PERIOD=(PER_M*16+15)*2^PER_E"]
pub type PerMR = crate::FieldReader;
#[doc = "Field `PER_M` writer - 7:3\\]
Number of 32 KHz clocks between activation of recharge controller For recharge algorithm, PERIOD is the initial period when entering powerdown mode. The adaptive recharge algorithm will not change this register PERIOD will effectively be a 16 bit value coded in a 5 bit mantissa and 3 bit exponent: This field sets the Mantissa of the Period. PERIOD=(PER_M*16+15)*2^PER_E"]
pub type PerMW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `MAX_PER_E` reader - 10:8\\]
This register defines the maximum period that the recharge algorithm can take, i.e. it defines the maximum number of cycles between 2 recharges. The maximum number of cycles is specified with a 5 bit mantissa and 3 bit exponent: MAXCYCLES=(MAX_PER_M*16+15)*2^MAX_PER_E This field sets the exponent MAXCYCLES"]
pub type MaxPerER = crate::FieldReader;
#[doc = "Field `MAX_PER_E` writer - 10:8\\]
This register defines the maximum period that the recharge algorithm can take, i.e. it defines the maximum number of cycles between 2 recharges. The maximum number of cycles is specified with a 5 bit mantissa and 3 bit exponent: MAXCYCLES=(MAX_PER_M*16+15)*2^MAX_PER_E This field sets the exponent MAXCYCLES"]
pub type MaxPerEW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MAX_PER_M` reader - 15:11\\]
This register defines the maximum period that the recharge algorithm can take, i.e. it defines the maximum number of cycles between 2 recharges. The maximum number of cycles is specified with a 5 bit mantissa and 3 bit exponent: MAXCYCLES=(MAX_PER_M*16+15)*2^MAX_PER_E This field sets the mantissa of MAXCYCLES"]
pub type MaxPerMR = crate::FieldReader;
#[doc = "Field `MAX_PER_M` writer - 15:11\\]
This register defines the maximum period that the recharge algorithm can take, i.e. it defines the maximum number of cycles between 2 recharges. The maximum number of cycles is specified with a 5 bit mantissa and 3 bit exponent: MAXCYCLES=(MAX_PER_M*16+15)*2^MAX_PER_E This field sets the mantissa of MAXCYCLES"]
pub type MaxPerMW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `C1` reader - 19:16\\]
Gain factor for adaptive recharge algorithm period_new=period * ( 1+/-(2^-C1+2^-C2) ) Valid values for C1 is 1 to 10 Note: Rounding may cause adaptive recharge not to start for very small values of both Gain and Initial period. Criteria for algorithm to start is MAX(PERIOD*2^-C1,PERIOD*2^-C2) >= 1"]
pub type C1R = crate::FieldReader;
#[doc = "Field `C1` writer - 19:16\\]
Gain factor for adaptive recharge algorithm period_new=period * ( 1+/-(2^-C1+2^-C2) ) Valid values for C1 is 1 to 10 Note: Rounding may cause adaptive recharge not to start for very small values of both Gain and Initial period. Criteria for algorithm to start is MAX(PERIOD*2^-C1,PERIOD*2^-C2) >= 1"]
pub type C1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `C2` reader - 23:20\\]
Gain factor for adaptive recharge algorithm period_new=period * ( 1+/-(2^-C1+2^-C2) ) Valid values for C2 is 2 to 10 Note: Rounding may cause adaptive recharge not to start for very small values of both Gain and Initial period. Criteria for algorithm to start is MAX(PERIOD*2^-C1,PERIOD*2^-C2) >= 1"]
pub type C2R = crate::FieldReader;
#[doc = "Field `C2` writer - 23:20\\]
Gain factor for adaptive recharge algorithm period_new=period * ( 1+/-(2^-C1+2^-C2) ) Valid values for C2 is 2 to 10 Note: Rounding may cause adaptive recharge not to start for very small values of both Gain and Initial period. Criteria for algorithm to start is MAX(PERIOD*2^-C1,PERIOD*2^-C2) >= 1"]
pub type C2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RESERVED24` reader - 30:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved24R = crate::FieldReader;
#[doc = "Field `ADAPTIVE_EN` reader - 31:31\\]
Enable adaptive recharge Note: Recharge can be turned completely of by setting MAX_PER_E=7 and MAX_PER_M=31 and this bitfield to 0"]
pub type AdaptiveEnR = crate::BitReader;
#[doc = "Field `ADAPTIVE_EN` writer - 31:31\\]
Enable adaptive recharge Note: Recharge can be turned completely of by setting MAX_PER_E=7 and MAX_PER_M=31 and this bitfield to 0"]
pub type AdaptiveEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Number of 32 KHz clocks between activation of recharge controller For recharge algorithm, PERIOD is the initial period when entering powerdown mode. The adaptive recharge algorithm will not change this register PERIOD will effectively be a 16 bit value coded in a 5 bit mantissa and 3 bit exponent: This field sets the Exponent of the Period. PERIOD=(PER_M*16+15)*2^PER_E"]
    #[inline(always)]
    pub fn per_e(&self) -> PerER {
        PerER::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:7 - 7:3\\]
Number of 32 KHz clocks between activation of recharge controller For recharge algorithm, PERIOD is the initial period when entering powerdown mode. The adaptive recharge algorithm will not change this register PERIOD will effectively be a 16 bit value coded in a 5 bit mantissa and 3 bit exponent: This field sets the Mantissa of the Period. PERIOD=(PER_M*16+15)*2^PER_E"]
    #[inline(always)]
    pub fn per_m(&self) -> PerMR {
        PerMR::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 8:10 - 10:8\\]
This register defines the maximum period that the recharge algorithm can take, i.e. it defines the maximum number of cycles between 2 recharges. The maximum number of cycles is specified with a 5 bit mantissa and 3 bit exponent: MAXCYCLES=(MAX_PER_M*16+15)*2^MAX_PER_E This field sets the exponent MAXCYCLES"]
    #[inline(always)]
    pub fn max_per_e(&self) -> MaxPerER {
        MaxPerER::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:15 - 15:11\\]
This register defines the maximum period that the recharge algorithm can take, i.e. it defines the maximum number of cycles between 2 recharges. The maximum number of cycles is specified with a 5 bit mantissa and 3 bit exponent: MAXCYCLES=(MAX_PER_M*16+15)*2^MAX_PER_E This field sets the mantissa of MAXCYCLES"]
    #[inline(always)]
    pub fn max_per_m(&self) -> MaxPerMR {
        MaxPerMR::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Gain factor for adaptive recharge algorithm period_new=period * ( 1+/-(2^-C1+2^-C2) ) Valid values for C1 is 1 to 10 Note: Rounding may cause adaptive recharge not to start for very small values of both Gain and Initial period. Criteria for algorithm to start is MAX(PERIOD*2^-C1,PERIOD*2^-C2) >= 1"]
    #[inline(always)]
    pub fn c1(&self) -> C1R {
        C1R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Gain factor for adaptive recharge algorithm period_new=period * ( 1+/-(2^-C1+2^-C2) ) Valid values for C2 is 2 to 10 Note: Rounding may cause adaptive recharge not to start for very small values of both Gain and Initial period. Criteria for algorithm to start is MAX(PERIOD*2^-C1,PERIOD*2^-C2) >= 1"]
    #[inline(always)]
    pub fn c2(&self) -> C2R {
        C2R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:30 - 30:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved24(&self) -> Reserved24R {
        Reserved24R::new(((self.bits >> 24) & 0x7f) as u8)
    }
    #[doc = "Bit 31 - 31:31\\]
Enable adaptive recharge Note: Recharge can be turned completely of by setting MAX_PER_E=7 and MAX_PER_M=31 and this bitfield to 0"]
    #[inline(always)]
    pub fn adaptive_en(&self) -> AdaptiveEnR {
        AdaptiveEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Number of 32 KHz clocks between activation of recharge controller For recharge algorithm, PERIOD is the initial period when entering powerdown mode. The adaptive recharge algorithm will not change this register PERIOD will effectively be a 16 bit value coded in a 5 bit mantissa and 3 bit exponent: This field sets the Exponent of the Period. PERIOD=(PER_M*16+15)*2^PER_E"]
    #[inline(always)]
    #[must_use]
    pub fn per_e(&mut self) -> PerEW<RechargecfgSpec> {
        PerEW::new(self, 0)
    }
    #[doc = "Bits 3:7 - 7:3\\]
Number of 32 KHz clocks between activation of recharge controller For recharge algorithm, PERIOD is the initial period when entering powerdown mode. The adaptive recharge algorithm will not change this register PERIOD will effectively be a 16 bit value coded in a 5 bit mantissa and 3 bit exponent: This field sets the Mantissa of the Period. PERIOD=(PER_M*16+15)*2^PER_E"]
    #[inline(always)]
    #[must_use]
    pub fn per_m(&mut self) -> PerMW<RechargecfgSpec> {
        PerMW::new(self, 3)
    }
    #[doc = "Bits 8:10 - 10:8\\]
This register defines the maximum period that the recharge algorithm can take, i.e. it defines the maximum number of cycles between 2 recharges. The maximum number of cycles is specified with a 5 bit mantissa and 3 bit exponent: MAXCYCLES=(MAX_PER_M*16+15)*2^MAX_PER_E This field sets the exponent MAXCYCLES"]
    #[inline(always)]
    #[must_use]
    pub fn max_per_e(&mut self) -> MaxPerEW<RechargecfgSpec> {
        MaxPerEW::new(self, 8)
    }
    #[doc = "Bits 11:15 - 15:11\\]
This register defines the maximum period that the recharge algorithm can take, i.e. it defines the maximum number of cycles between 2 recharges. The maximum number of cycles is specified with a 5 bit mantissa and 3 bit exponent: MAXCYCLES=(MAX_PER_M*16+15)*2^MAX_PER_E This field sets the mantissa of MAXCYCLES"]
    #[inline(always)]
    #[must_use]
    pub fn max_per_m(&mut self) -> MaxPerMW<RechargecfgSpec> {
        MaxPerMW::new(self, 11)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Gain factor for adaptive recharge algorithm period_new=period * ( 1+/-(2^-C1+2^-C2) ) Valid values for C1 is 1 to 10 Note: Rounding may cause adaptive recharge not to start for very small values of both Gain and Initial period. Criteria for algorithm to start is MAX(PERIOD*2^-C1,PERIOD*2^-C2) >= 1"]
    #[inline(always)]
    #[must_use]
    pub fn c1(&mut self) -> C1W<RechargecfgSpec> {
        C1W::new(self, 16)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Gain factor for adaptive recharge algorithm period_new=period * ( 1+/-(2^-C1+2^-C2) ) Valid values for C2 is 2 to 10 Note: Rounding may cause adaptive recharge not to start for very small values of both Gain and Initial period. Criteria for algorithm to start is MAX(PERIOD*2^-C1,PERIOD*2^-C2) >= 1"]
    #[inline(always)]
    #[must_use]
    pub fn c2(&mut self) -> C2W<RechargecfgSpec> {
        C2W::new(self, 20)
    }
    #[doc = "Bit 31 - 31:31\\]
Enable adaptive recharge Note: Recharge can be turned completely of by setting MAX_PER_E=7 and MAX_PER_M=31 and this bitfield to 0"]
    #[inline(always)]
    #[must_use]
    pub fn adaptive_en(&mut self) -> AdaptiveEnW<RechargecfgSpec> {
        AdaptiveEnW::new(self, 31)
    }
}
#[doc = "Recharge Controller Configuration This register sets all relevant patameters for controlling the recharge algorithm.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rechargecfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rechargecfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RechargecfgSpec;
impl crate::RegisterSpec for RechargecfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rechargecfg::R`](R) reader structure"]
impl crate::Readable for RechargecfgSpec {}
#[doc = "`write(|w| ..)` method takes [`rechargecfg::W`](W) writer structure"]
impl crate::Writable for RechargecfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RECHARGECFG to value 0"]
impl crate::Resettable for RechargecfgSpec {
    const RESET_VALUE: u32 = 0;
}
