#[doc = "Register `AMPCFG1` reader"]
pub type R = crate::R<Ampcfg1Spec>;
#[doc = "Register `AMPCFG1` writer"]
pub type W = crate::W<Ampcfg1Spec>;
#[doc = "Field `INTERVAL` reader - 11:0\\]
Interval for amplitude adjustments. Set to 0 to disable periodic adjustments. This value specifies the number of clock cycles between adjustments. The clock speed is defined in AMPCFG0.FSMRATE."]
pub type IntervalR = crate::FieldReader<u16>;
#[doc = "Field `INTERVAL` writer - 11:0\\]
Interval for amplitude adjustments. Set to 0 to disable periodic adjustments. This value specifies the number of clock cycles between adjustments. The clock speed is defined in AMPCFG0.FSMRATE."]
pub type IntervalW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `BIASLT` reader - 23:12\\]
Lifetime of the amplitude ADC bias value. This value specifies the number of adjustment intervals, until the ADC bias value has to be measured again. Set to 0 to disable automatic bias measurements."]
pub type BiasltR = crate::FieldReader<u16>;
#[doc = "Field `BIASLT` writer - 23:12\\]
Lifetime of the amplitude ADC bias value. This value specifies the number of adjustment intervals, until the ADC bias value has to be measured again. Set to 0 to disable automatic bias measurements."]
pub type BiasltW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `IREFDLY` reader - 27:24\\]
IREF change delay. Number of clock cycles to wait before changing IREF by one step. Clock frequency defined in AMPCFG0.FSMRATE."]
pub type IrefdlyR = crate::FieldReader;
#[doc = "Field `IREFDLY` writer - 27:24\\]
IREF change delay. Number of clock cycles to wait before changing IREF by one step. Clock frequency defined in AMPCFG0.FSMRATE."]
pub type IrefdlyW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `IDACDLY` reader - 31:28\\]
IDAC change delay. Time to wait before changing IDAC by one step. This time needs to be long enough for the crystal to settle. The number of clock cycles to wait is IDACDLY$lt;$lt;4 + 15. Clock frequency defined in AMPCFG0.FSMRATE."]
pub type IdacdlyR = crate::FieldReader;
#[doc = "Field `IDACDLY` writer - 31:28\\]
IDAC change delay. Time to wait before changing IDAC by one step. This time needs to be long enough for the crystal to settle. The number of clock cycles to wait is IDACDLY$lt;$lt;4 + 15. Clock frequency defined in AMPCFG0.FSMRATE."]
pub type IdacdlyW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:11 - 11:0\\]
Interval for amplitude adjustments. Set to 0 to disable periodic adjustments. This value specifies the number of clock cycles between adjustments. The clock speed is defined in AMPCFG0.FSMRATE."]
    #[inline(always)]
    pub fn interval(&self) -> IntervalR {
        IntervalR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:23 - 23:12\\]
Lifetime of the amplitude ADC bias value. This value specifies the number of adjustment intervals, until the ADC bias value has to be measured again. Set to 0 to disable automatic bias measurements."]
    #[inline(always)]
    pub fn biaslt(&self) -> BiasltR {
        BiasltR::new(((self.bits >> 12) & 0x0fff) as u16)
    }
    #[doc = "Bits 24:27 - 27:24\\]
IREF change delay. Number of clock cycles to wait before changing IREF by one step. Clock frequency defined in AMPCFG0.FSMRATE."]
    #[inline(always)]
    pub fn irefdly(&self) -> IrefdlyR {
        IrefdlyR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - 31:28\\]
IDAC change delay. Time to wait before changing IDAC by one step. This time needs to be long enough for the crystal to settle. The number of clock cycles to wait is IDACDLY$lt;$lt;4 + 15. Clock frequency defined in AMPCFG0.FSMRATE."]
    #[inline(always)]
    pub fn idacdly(&self) -> IdacdlyR {
        IdacdlyR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - 11:0\\]
Interval for amplitude adjustments. Set to 0 to disable periodic adjustments. This value specifies the number of clock cycles between adjustments. The clock speed is defined in AMPCFG0.FSMRATE."]
    #[inline(always)]
    #[must_use]
    pub fn interval(&mut self) -> IntervalW<Ampcfg1Spec> {
        IntervalW::new(self, 0)
    }
    #[doc = "Bits 12:23 - 23:12\\]
Lifetime of the amplitude ADC bias value. This value specifies the number of adjustment intervals, until the ADC bias value has to be measured again. Set to 0 to disable automatic bias measurements."]
    #[inline(always)]
    #[must_use]
    pub fn biaslt(&mut self) -> BiasltW<Ampcfg1Spec> {
        BiasltW::new(self, 12)
    }
    #[doc = "Bits 24:27 - 27:24\\]
IREF change delay. Number of clock cycles to wait before changing IREF by one step. Clock frequency defined in AMPCFG0.FSMRATE."]
    #[inline(always)]
    #[must_use]
    pub fn irefdly(&mut self) -> IrefdlyW<Ampcfg1Spec> {
        IrefdlyW::new(self, 24)
    }
    #[doc = "Bits 28:31 - 31:28\\]
IDAC change delay. Time to wait before changing IDAC by one step. This time needs to be long enough for the crystal to settle. The number of clock cycles to wait is IDACDLY$lt;$lt;4 + 15. Clock frequency defined in AMPCFG0.FSMRATE."]
    #[inline(always)]
    #[must_use]
    pub fn idacdly(&mut self) -> IdacdlyW<Ampcfg1Spec> {
        IdacdlyW::new(self, 28)
    }
}
#[doc = "Amplitude Compensation Configuration 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ampcfg1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ampcfg1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ampcfg1Spec;
impl crate::RegisterSpec for Ampcfg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ampcfg1::R`](R) reader structure"]
impl crate::Readable for Ampcfg1Spec {}
#[doc = "`write(|w| ..)` method takes [`ampcfg1::W`](W) writer structure"]
impl crate::Writable for Ampcfg1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AMPCFG1 to value 0x260f_f0ff"]
impl crate::Resettable for Ampcfg1Spec {
    const RESET_VALUE: u32 = 0x260f_f0ff;
}
