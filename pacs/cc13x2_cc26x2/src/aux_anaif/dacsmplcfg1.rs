#[doc = "Register `DACSMPLCFG1` reader"]
pub struct R(crate::R<DACSMPLCFG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DACSMPLCFG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DACSMPLCFG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DACSMPLCFG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DACSMPLCFG1` writer"]
pub struct W(crate::W<DACSMPLCFG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DACSMPLCFG1_SPEC>;
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
impl From<crate::W<DACSMPLCFG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DACSMPLCFG1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HOLD_INTERVAL` reader - 7:0\\]
Hold interval. Number of inactive sample clock periods between each active sample clock period during hold phase. The sample clock is low when inactive. The range is 0 to 255."]
pub type HOLD_INTERVAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HOLD_INTERVAL` writer - 7:0\\]
Hold interval. Number of inactive sample clock periods between each active sample clock period during hold phase. The sample clock is low when inactive. The range is 0 to 255."]
pub type HOLD_INTERVAL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DACSMPLCFG1_SPEC, u8, u8, 8, O>;
#[doc = "Field `SETUP_CNT` reader - 11:8\\]
Setup count. Number of active sample clock periods during the setup phase. 0: 1 sample clock period 1: 2 sample clock periods ... 15 : 16 sample clock periods"]
pub type SETUP_CNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SETUP_CNT` writer - 11:8\\]
Setup count. Number of active sample clock periods during the setup phase. 0: 1 sample clock period 1: 2 sample clock periods ... 15 : 16 sample clock periods"]
pub type SETUP_CNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DACSMPLCFG1_SPEC, u8, u8, 4, O>;
#[doc = "Field `L_PER` reader - 13:12\\]
Low time. The sample clock period is low for this many base periods. 0: 1 period 1: 2 periods 2: 3 periods 3: 4 periods"]
pub type L_PER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `L_PER` writer - 13:12\\]
Low time. The sample clock period is low for this many base periods. 0: 1 period 1: 2 periods 2: 3 periods 3: 4 periods"]
pub type L_PER_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DACSMPLCFG1_SPEC, u8, u8, 2, O>;
#[doc = "Field `H_PER` reader - 14:14\\]
High time. The sample clock period is high for this many base periods. 0: 2 periods 1: 4 periods"]
pub type H_PER_R = crate::BitReader<bool>;
#[doc = "Field `H_PER` writer - 14:14\\]
High time. The sample clock period is high for this many base periods. 0: 2 periods 1: 4 periods"]
pub type H_PER_W<'a, const O: u8> = crate::BitWriter<'a, u32, DACSMPLCFG1_SPEC, bool, O>;
#[doc = "Field `RESERVED15` reader - 31:15\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED15_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED15` writer - 31:15\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED15_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DACSMPLCFG1_SPEC, u32, u32, 17, O>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Hold interval. Number of inactive sample clock periods between each active sample clock period during hold phase. The sample clock is low when inactive. The range is 0 to 255."]
    #[inline(always)]
    pub fn hold_interval(&self) -> HOLD_INTERVAL_R {
        HOLD_INTERVAL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Setup count. Number of active sample clock periods during the setup phase. 0: 1 sample clock period 1: 2 sample clock periods ... 15 : 16 sample clock periods"]
    #[inline(always)]
    pub fn setup_cnt(&self) -> SETUP_CNT_R {
        SETUP_CNT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:13 - 13:12\\]
Low time. The sample clock period is low for this many base periods. 0: 1 period 1: 2 periods 2: 3 periods 3: 4 periods"]
    #[inline(always)]
    pub fn l_per(&self) -> L_PER_R {
        L_PER_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - 14:14\\]
High time. The sample clock period is high for this many base periods. 0: 2 periods 1: 4 periods"]
    #[inline(always)]
    pub fn h_per(&self) -> H_PER_R {
        H_PER_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 15:31 - 31:15\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved15(&self) -> RESERVED15_R {
        RESERVED15_R::new((self.bits >> 15) & 0x0001_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Hold interval. Number of inactive sample clock periods between each active sample clock period during hold phase. The sample clock is low when inactive. The range is 0 to 255."]
    #[inline(always)]
    #[must_use]
    pub fn hold_interval(&mut self) -> HOLD_INTERVAL_W<0> {
        HOLD_INTERVAL_W::new(self)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Setup count. Number of active sample clock periods during the setup phase. 0: 1 sample clock period 1: 2 sample clock periods ... 15 : 16 sample clock periods"]
    #[inline(always)]
    #[must_use]
    pub fn setup_cnt(&mut self) -> SETUP_CNT_W<8> {
        SETUP_CNT_W::new(self)
    }
    #[doc = "Bits 12:13 - 13:12\\]
Low time. The sample clock period is low for this many base periods. 0: 1 period 1: 2 periods 2: 3 periods 3: 4 periods"]
    #[inline(always)]
    #[must_use]
    pub fn l_per(&mut self) -> L_PER_W<12> {
        L_PER_W::new(self)
    }
    #[doc = "Bit 14 - 14:14\\]
High time. The sample clock period is high for this many base periods. 0: 2 periods 1: 4 periods"]
    #[inline(always)]
    #[must_use]
    pub fn h_per(&mut self) -> H_PER_W<14> {
        H_PER_W::new(self)
    }
    #[doc = "Bits 15:31 - 31:15\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved15(&mut self) -> RESERVED15_W<15> {
        RESERVED15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DAC Sample Configuration 1 The sample clock period equals (high time + low time) * base period. DACSMPLCFG0.CLKDIV determines the base period. Timing requirements (DAC Buffer On / DAC Buffer Off): - (high time + low time) * base period > (4 us / 1 us) - (high time * base period) > (2 us / 0.5 us) - (low time * base period) > (2 us / 0.5 us) - (low time * base period + HOLD_INTERVAL * sample clock period) < 32 us If AUX_SYSIF:OPMODEREQ.REQ equals PDLP, you must set: - H_PER = L_PER = HOLD_INTERVAL = 0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dacsmplcfg1](index.html) module"]
pub struct DACSMPLCFG1_SPEC;
impl crate::RegisterSpec for DACSMPLCFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dacsmplcfg1::R](R) reader structure"]
impl crate::Readable for DACSMPLCFG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dacsmplcfg1::W](W) writer structure"]
impl crate::Writable for DACSMPLCFG1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DACSMPLCFG1 to value 0"]
impl crate::Resettable for DACSMPLCFG1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
