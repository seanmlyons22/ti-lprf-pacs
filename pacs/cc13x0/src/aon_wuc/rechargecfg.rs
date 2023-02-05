#[doc = "Register `RECHARGECFG` reader"]
pub struct R(crate::R<RECHARGECFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RECHARGECFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RECHARGECFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RECHARGECFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RECHARGECFG` writer"]
pub struct W(crate::W<RECHARGECFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RECHARGECFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<RECHARGECFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RECHARGECFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PER_E` reader - 2:0\\]
Number of 32 KHz clocks between activation of recharge controller For recharge algorithm, PERIOD is the initial period when entering powerdown mode. The adaptive recharge algorithm will not change this register PERIOD will effectively be a 16 bit value coded in a 5 bit mantissa and 3 bit exponent: This field sets the Exponent of the Period. PERIOD=(PER_M*16+15)*2^PER_E"]
pub type PER_E_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PER_E` writer - 2:0\\]
Number of 32 KHz clocks between activation of recharge controller For recharge algorithm, PERIOD is the initial period when entering powerdown mode. The adaptive recharge algorithm will not change this register PERIOD will effectively be a 16 bit value coded in a 5 bit mantissa and 3 bit exponent: This field sets the Exponent of the Period. PERIOD=(PER_M*16+15)*2^PER_E"]
pub type PER_E_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RECHARGECFG_SPEC, u8, u8, 3, O>;
#[doc = "Field `PER_M` reader - 7:3\\]
Number of 32 KHz clocks between activation of recharge controller For recharge algorithm, PERIOD is the initial period when entering powerdown mode. The adaptive recharge algorithm will not change this register PERIOD will effectively be a 16 bit value coded in a 5 bit mantissa and 3 bit exponent: This field sets the Mantissa of the Period. PERIOD=(PER_M*16+15)*2^PER_E"]
pub type PER_M_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PER_M` writer - 7:3\\]
Number of 32 KHz clocks between activation of recharge controller For recharge algorithm, PERIOD is the initial period when entering powerdown mode. The adaptive recharge algorithm will not change this register PERIOD will effectively be a 16 bit value coded in a 5 bit mantissa and 3 bit exponent: This field sets the Mantissa of the Period. PERIOD=(PER_M*16+15)*2^PER_E"]
pub type PER_M_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RECHARGECFG_SPEC, u8, u8, 5, O>;
#[doc = "Field `MAX_PER_E` reader - 10:8\\]
This register defines the maximum period that the recharge algorithm can take, i.e. it defines the maximum number of cycles between 2 recharges. The maximum number of cycles is specified with a 5 bit mantissa and 3 bit exponent: MAXCYCLES=(MAX_PER_M*16+15)*2^MAX_PER_E This field sets the exponent MAXCYCLES"]
pub type MAX_PER_E_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MAX_PER_E` writer - 10:8\\]
This register defines the maximum period that the recharge algorithm can take, i.e. it defines the maximum number of cycles between 2 recharges. The maximum number of cycles is specified with a 5 bit mantissa and 3 bit exponent: MAXCYCLES=(MAX_PER_M*16+15)*2^MAX_PER_E This field sets the exponent MAXCYCLES"]
pub type MAX_PER_E_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RECHARGECFG_SPEC, u8, u8, 3, O>;
#[doc = "Field `MAX_PER_M` reader - 15:11\\]
This register defines the maximum period that the recharge algorithm can take, i.e. it defines the maximum number of cycles between 2 recharges. The maximum number of cycles is specified with a 5 bit mantissa and 3 bit exponent: MAXCYCLES=(MAX_PER_M*16+15)*2^MAX_PER_E This field sets the mantissa of MAXCYCLES"]
pub type MAX_PER_M_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MAX_PER_M` writer - 15:11\\]
This register defines the maximum period that the recharge algorithm can take, i.e. it defines the maximum number of cycles between 2 recharges. The maximum number of cycles is specified with a 5 bit mantissa and 3 bit exponent: MAXCYCLES=(MAX_PER_M*16+15)*2^MAX_PER_E This field sets the mantissa of MAXCYCLES"]
pub type MAX_PER_M_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RECHARGECFG_SPEC, u8, u8, 5, O>;
#[doc = "Field `C1` reader - 19:16\\]
Gain factor for adaptive recharge algorithm period_new=period * ( 1+/-(2^-C1+2^-C2) ) Valid values for C1 is 1 to 10 Note: Rounding may cause adaptive recharge not to start for very small values of both Gain and Initial period. Criteria for algorithm to start is MAX(PERIOD*2^-C1,PERIOD*2^-C2) >= 1"]
pub type C1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `C1` writer - 19:16\\]
Gain factor for adaptive recharge algorithm period_new=period * ( 1+/-(2^-C1+2^-C2) ) Valid values for C1 is 1 to 10 Note: Rounding may cause adaptive recharge not to start for very small values of both Gain and Initial period. Criteria for algorithm to start is MAX(PERIOD*2^-C1,PERIOD*2^-C2) >= 1"]
pub type C1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RECHARGECFG_SPEC, u8, u8, 4, O>;
#[doc = "Field `C2` reader - 23:20\\]
Gain factor for adaptive recharge algorithm period_new=period * ( 1+/-(2^-C1+2^-C2) ) Valid values for C2 is 2 to 10 Note: Rounding may cause adaptive recharge not to start for very small values of both Gain and Initial period. Criteria for algorithm to start is MAX(PERIOD*2^-C1,PERIOD*2^-C2) >= 1"]
pub type C2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `C2` writer - 23:20\\]
Gain factor for adaptive recharge algorithm period_new=period * ( 1+/-(2^-C1+2^-C2) ) Valid values for C2 is 2 to 10 Note: Rounding may cause adaptive recharge not to start for very small values of both Gain and Initial period. Criteria for algorithm to start is MAX(PERIOD*2^-C1,PERIOD*2^-C2) >= 1"]
pub type C2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RECHARGECFG_SPEC, u8, u8, 4, O>;
#[doc = "Field `RESERVED24` reader - 30:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED24_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED24` writer - 30:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED24_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RECHARGECFG_SPEC, u8, u8, 7, O>;
#[doc = "Field `ADAPTIVE_EN` reader - 31:31\\]
Enable adaptive recharge Note: Recharge can be turned completely of by setting MAX_PER_E=7 and MAX_PER_M=31 and this bitfield to 0"]
pub type ADAPTIVE_EN_R = crate::BitReader<bool>;
#[doc = "Field `ADAPTIVE_EN` writer - 31:31\\]
Enable adaptive recharge Note: Recharge can be turned completely of by setting MAX_PER_E=7 and MAX_PER_M=31 and this bitfield to 0"]
pub type ADAPTIVE_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RECHARGECFG_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Number of 32 KHz clocks between activation of recharge controller For recharge algorithm, PERIOD is the initial period when entering powerdown mode. The adaptive recharge algorithm will not change this register PERIOD will effectively be a 16 bit value coded in a 5 bit mantissa and 3 bit exponent: This field sets the Exponent of the Period. PERIOD=(PER_M*16+15)*2^PER_E"]
    #[inline(always)]
    pub fn per_e(&self) -> PER_E_R {
        PER_E_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:7 - 7:3\\]
Number of 32 KHz clocks between activation of recharge controller For recharge algorithm, PERIOD is the initial period when entering powerdown mode. The adaptive recharge algorithm will not change this register PERIOD will effectively be a 16 bit value coded in a 5 bit mantissa and 3 bit exponent: This field sets the Mantissa of the Period. PERIOD=(PER_M*16+15)*2^PER_E"]
    #[inline(always)]
    pub fn per_m(&self) -> PER_M_R {
        PER_M_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 8:10 - 10:8\\]
This register defines the maximum period that the recharge algorithm can take, i.e. it defines the maximum number of cycles between 2 recharges. The maximum number of cycles is specified with a 5 bit mantissa and 3 bit exponent: MAXCYCLES=(MAX_PER_M*16+15)*2^MAX_PER_E This field sets the exponent MAXCYCLES"]
    #[inline(always)]
    pub fn max_per_e(&self) -> MAX_PER_E_R {
        MAX_PER_E_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:15 - 15:11\\]
This register defines the maximum period that the recharge algorithm can take, i.e. it defines the maximum number of cycles between 2 recharges. The maximum number of cycles is specified with a 5 bit mantissa and 3 bit exponent: MAXCYCLES=(MAX_PER_M*16+15)*2^MAX_PER_E This field sets the mantissa of MAXCYCLES"]
    #[inline(always)]
    pub fn max_per_m(&self) -> MAX_PER_M_R {
        MAX_PER_M_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Gain factor for adaptive recharge algorithm period_new=period * ( 1+/-(2^-C1+2^-C2) ) Valid values for C1 is 1 to 10 Note: Rounding may cause adaptive recharge not to start for very small values of both Gain and Initial period. Criteria for algorithm to start is MAX(PERIOD*2^-C1,PERIOD*2^-C2) >= 1"]
    #[inline(always)]
    pub fn c1(&self) -> C1_R {
        C1_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Gain factor for adaptive recharge algorithm period_new=period * ( 1+/-(2^-C1+2^-C2) ) Valid values for C2 is 2 to 10 Note: Rounding may cause adaptive recharge not to start for very small values of both Gain and Initial period. Criteria for algorithm to start is MAX(PERIOD*2^-C1,PERIOD*2^-C2) >= 1"]
    #[inline(always)]
    pub fn c2(&self) -> C2_R {
        C2_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:30 - 30:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved24(&self) -> RESERVED24_R {
        RESERVED24_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
    #[doc = "Bit 31 - 31:31\\]
Enable adaptive recharge Note: Recharge can be turned completely of by setting MAX_PER_E=7 and MAX_PER_M=31 and this bitfield to 0"]
    #[inline(always)]
    pub fn adaptive_en(&self) -> ADAPTIVE_EN_R {
        ADAPTIVE_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Number of 32 KHz clocks between activation of recharge controller For recharge algorithm, PERIOD is the initial period when entering powerdown mode. The adaptive recharge algorithm will not change this register PERIOD will effectively be a 16 bit value coded in a 5 bit mantissa and 3 bit exponent: This field sets the Exponent of the Period. PERIOD=(PER_M*16+15)*2^PER_E"]
    #[inline(always)]
    #[must_use]
    pub fn per_e(&mut self) -> PER_E_W<0> {
        PER_E_W::new(self)
    }
    #[doc = "Bits 3:7 - 7:3\\]
Number of 32 KHz clocks between activation of recharge controller For recharge algorithm, PERIOD is the initial period when entering powerdown mode. The adaptive recharge algorithm will not change this register PERIOD will effectively be a 16 bit value coded in a 5 bit mantissa and 3 bit exponent: This field sets the Mantissa of the Period. PERIOD=(PER_M*16+15)*2^PER_E"]
    #[inline(always)]
    #[must_use]
    pub fn per_m(&mut self) -> PER_M_W<3> {
        PER_M_W::new(self)
    }
    #[doc = "Bits 8:10 - 10:8\\]
This register defines the maximum period that the recharge algorithm can take, i.e. it defines the maximum number of cycles between 2 recharges. The maximum number of cycles is specified with a 5 bit mantissa and 3 bit exponent: MAXCYCLES=(MAX_PER_M*16+15)*2^MAX_PER_E This field sets the exponent MAXCYCLES"]
    #[inline(always)]
    #[must_use]
    pub fn max_per_e(&mut self) -> MAX_PER_E_W<8> {
        MAX_PER_E_W::new(self)
    }
    #[doc = "Bits 11:15 - 15:11\\]
This register defines the maximum period that the recharge algorithm can take, i.e. it defines the maximum number of cycles between 2 recharges. The maximum number of cycles is specified with a 5 bit mantissa and 3 bit exponent: MAXCYCLES=(MAX_PER_M*16+15)*2^MAX_PER_E This field sets the mantissa of MAXCYCLES"]
    #[inline(always)]
    #[must_use]
    pub fn max_per_m(&mut self) -> MAX_PER_M_W<11> {
        MAX_PER_M_W::new(self)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Gain factor for adaptive recharge algorithm period_new=period * ( 1+/-(2^-C1+2^-C2) ) Valid values for C1 is 1 to 10 Note: Rounding may cause adaptive recharge not to start for very small values of both Gain and Initial period. Criteria for algorithm to start is MAX(PERIOD*2^-C1,PERIOD*2^-C2) >= 1"]
    #[inline(always)]
    #[must_use]
    pub fn c1(&mut self) -> C1_W<16> {
        C1_W::new(self)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Gain factor for adaptive recharge algorithm period_new=period * ( 1+/-(2^-C1+2^-C2) ) Valid values for C2 is 2 to 10 Note: Rounding may cause adaptive recharge not to start for very small values of both Gain and Initial period. Criteria for algorithm to start is MAX(PERIOD*2^-C1,PERIOD*2^-C2) >= 1"]
    #[inline(always)]
    #[must_use]
    pub fn c2(&mut self) -> C2_W<20> {
        C2_W::new(self)
    }
    #[doc = "Bits 24:30 - 30:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved24(&mut self) -> RESERVED24_W<24> {
        RESERVED24_W::new(self)
    }
    #[doc = "Bit 31 - 31:31\\]
Enable adaptive recharge Note: Recharge can be turned completely of by setting MAX_PER_E=7 and MAX_PER_M=31 and this bitfield to 0"]
    #[inline(always)]
    #[must_use]
    pub fn adaptive_en(&mut self) -> ADAPTIVE_EN_W<31> {
        ADAPTIVE_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Recharge Controller Configuration This register sets all relevant patameters for controlling the recharge algorithm.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rechargecfg](index.html) module"]
pub struct RECHARGECFG_SPEC;
impl crate::RegisterSpec for RECHARGECFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rechargecfg::R](R) reader structure"]
impl crate::Readable for RECHARGECFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rechargecfg::W](W) writer structure"]
impl crate::Writable for RECHARGECFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RECHARGECFG to value 0"]
impl crate::Resettable for RECHARGECFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
