#[doc = "Register `PWD_CURR_95C` reader"]
pub type R = crate::R<PwdCurr95cSpec>;
#[doc = "Register `PWD_CURR_95C` writer"]
pub type W = crate::W<PwdCurr95cSpec>;
#[doc = "Field `BASELINE` reader - 7:0\\]
Worst-case baseline maximum powerdown current, in units of 0.5uA"]
pub type BaselineR = crate::FieldReader;
#[doc = "Field `BASELINE` writer - 7:0\\]
Worst-case baseline maximum powerdown current, in units of 0.5uA"]
pub type BaselineW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DELTA_XOSC_LPM` reader - 15:8\\]
Additional maximum current, in units of 1uA, with XOSC_HF on in low-power mode"]
pub type DeltaXoscLpmR = crate::FieldReader;
#[doc = "Field `DELTA_XOSC_LPM` writer - 15:8\\]
Additional maximum current, in units of 1uA, with XOSC_HF on in low-power mode"]
pub type DeltaXoscLpmW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DELTA_RFMEM_RET` reader - 23:16\\]
Additional maximum current, in 1uA units, with RF memory retention"]
pub type DeltaRfmemRetR = crate::FieldReader;
#[doc = "Field `DELTA_RFMEM_RET` writer - 23:16\\]
Additional maximum current, in 1uA units, with RF memory retention"]
pub type DeltaRfmemRetW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DELTA_CACHE_REF` reader - 31:24\\]
Additional maximum current, in units of 1uA, with cache retention"]
pub type DeltaCacheRefR = crate::FieldReader;
#[doc = "Field `DELTA_CACHE_REF` writer - 31:24\\]
Additional maximum current, in units of 1uA, with cache retention"]
pub type DeltaCacheRefW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Worst-case baseline maximum powerdown current, in units of 0.5uA"]
    #[inline(always)]
    pub fn baseline(&self) -> BaselineR {
        BaselineR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Additional maximum current, in units of 1uA, with XOSC_HF on in low-power mode"]
    #[inline(always)]
    pub fn delta_xosc_lpm(&self) -> DeltaXoscLpmR {
        DeltaXoscLpmR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Additional maximum current, in 1uA units, with RF memory retention"]
    #[inline(always)]
    pub fn delta_rfmem_ret(&self) -> DeltaRfmemRetR {
        DeltaRfmemRetR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Additional maximum current, in units of 1uA, with cache retention"]
    #[inline(always)]
    pub fn delta_cache_ref(&self) -> DeltaCacheRefR {
        DeltaCacheRefR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Worst-case baseline maximum powerdown current, in units of 0.5uA"]
    #[inline(always)]
    #[must_use]
    pub fn baseline(&mut self) -> BaselineW<PwdCurr95cSpec> {
        BaselineW::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Additional maximum current, in units of 1uA, with XOSC_HF on in low-power mode"]
    #[inline(always)]
    #[must_use]
    pub fn delta_xosc_lpm(&mut self) -> DeltaXoscLpmW<PwdCurr95cSpec> {
        DeltaXoscLpmW::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Additional maximum current, in 1uA units, with RF memory retention"]
    #[inline(always)]
    #[must_use]
    pub fn delta_rfmem_ret(&mut self) -> DeltaRfmemRetW<PwdCurr95cSpec> {
        DeltaRfmemRetW::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Additional maximum current, in units of 1uA, with cache retention"]
    #[inline(always)]
    #[must_use]
    pub fn delta_cache_ref(&mut self) -> DeltaCacheRefW<PwdCurr95cSpec> {
        DeltaCacheRefW::new(self, 24)
    }
}
#[doc = "Power Down Current Control 95C\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwd_curr_95c::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwd_curr_95c::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwdCurr95cSpec;
impl crate::RegisterSpec for PwdCurr95cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwd_curr_95c::R`](R) reader structure"]
impl crate::Readable for PwdCurr95cSpec {}
#[doc = "`write(|w| ..)` method takes [`pwd_curr_95c::W`](W) writer structure"]
impl crate::Writable for PwdCurr95cSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PWD_CURR_95C to value 0x4c62_7a3b"]
impl crate::Resettable for PwdCurr95cSpec {
    const RESET_VALUE: u32 = 0x4c62_7a3b;
}
