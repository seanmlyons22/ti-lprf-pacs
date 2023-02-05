#[doc = "Register `HFXTTARG` reader"]
pub type R = crate::R<HfxttargSpec>;
#[doc = "Register `HFXTTARG` writer"]
pub type W = crate::W<HfxttargSpec>;
#[doc = "Field `Q1CAP` reader - 5:0\\]
Target HFXT Q1 cap trim"]
pub type Q1capR = crate::FieldReader;
#[doc = "Field `Q1CAP` writer - 5:0\\]
Target HFXT Q1 cap trim"]
pub type Q1capW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `Q2CAP` reader - 11:6\\]
Target HFXT Q2 cap trim"]
pub type Q2capR = crate::FieldReader;
#[doc = "Field `Q2CAP` writer - 11:6\\]
Target HFXT Q2 cap trim"]
pub type Q2capW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `IREF` reader - 15:12\\]
Target HFXT IREF current"]
pub type IrefR = crate::FieldReader;
#[doc = "Field `IREF` writer - 15:12\\]
Target HFXT IREF current"]
pub type IrefW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `IDAC` reader - 22:16\\]
Minimum IDAC current"]
pub type IdacR = crate::FieldReader;
#[doc = "Field `IDAC` writer - 22:16\\]
Minimum IDAC current"]
pub type IdacW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `AMPTHR` reader - 29:23\\]
Minimum HFXT amplitude"]
pub type AmpthrR = crate::FieldReader;
#[doc = "Field `AMPTHR` writer - 29:23\\]
Minimum HFXT amplitude"]
pub type AmpthrW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `AMPHYST` reader - 31:30\\]
ADC hysteresis used during IDAC updates. Every AMPCFG1.INTERVAL, IDAC will be regulated - up as long as ADC $lt; AMPTHR - down as long as ADC $gt; AMPTHR+AMPHYST"]
pub type AmphystR = crate::FieldReader;
#[doc = "Field `AMPHYST` writer - 31:30\\]
ADC hysteresis used during IDAC updates. Every AMPCFG1.INTERVAL, IDAC will be regulated - up as long as ADC $lt; AMPTHR - down as long as ADC $gt; AMPTHR+AMPHYST"]
pub type AmphystW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Target HFXT Q1 cap trim"]
    #[inline(always)]
    pub fn q1cap(&self) -> Q1capR {
        Q1capR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:11 - 11:6\\]
Target HFXT Q2 cap trim"]
    #[inline(always)]
    pub fn q2cap(&self) -> Q2capR {
        Q2capR::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Target HFXT IREF current"]
    #[inline(always)]
    pub fn iref(&self) -> IrefR {
        IrefR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:22 - 22:16\\]
Minimum IDAC current"]
    #[inline(always)]
    pub fn idac(&self) -> IdacR {
        IdacR::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 23:29 - 29:23\\]
Minimum HFXT amplitude"]
    #[inline(always)]
    pub fn ampthr(&self) -> AmpthrR {
        AmpthrR::new(((self.bits >> 23) & 0x7f) as u8)
    }
    #[doc = "Bits 30:31 - 31:30\\]
ADC hysteresis used during IDAC updates. Every AMPCFG1.INTERVAL, IDAC will be regulated - up as long as ADC $lt; AMPTHR - down as long as ADC $gt; AMPTHR+AMPHYST"]
    #[inline(always)]
    pub fn amphyst(&self) -> AmphystR {
        AmphystR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Target HFXT Q1 cap trim"]
    #[inline(always)]
    #[must_use]
    pub fn q1cap(&mut self) -> Q1capW<HfxttargSpec> {
        Q1capW::new(self, 0)
    }
    #[doc = "Bits 6:11 - 11:6\\]
Target HFXT Q2 cap trim"]
    #[inline(always)]
    #[must_use]
    pub fn q2cap(&mut self) -> Q2capW<HfxttargSpec> {
        Q2capW::new(self, 6)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Target HFXT IREF current"]
    #[inline(always)]
    #[must_use]
    pub fn iref(&mut self) -> IrefW<HfxttargSpec> {
        IrefW::new(self, 12)
    }
    #[doc = "Bits 16:22 - 22:16\\]
Minimum IDAC current"]
    #[inline(always)]
    #[must_use]
    pub fn idac(&mut self) -> IdacW<HfxttargSpec> {
        IdacW::new(self, 16)
    }
    #[doc = "Bits 23:29 - 29:23\\]
Minimum HFXT amplitude"]
    #[inline(always)]
    #[must_use]
    pub fn ampthr(&mut self) -> AmpthrW<HfxttargSpec> {
        AmpthrW::new(self, 23)
    }
    #[doc = "Bits 30:31 - 31:30\\]
ADC hysteresis used during IDAC updates. Every AMPCFG1.INTERVAL, IDAC will be regulated - up as long as ADC $lt; AMPTHR - down as long as ADC $gt; AMPTHR+AMPHYST"]
    #[inline(always)]
    #[must_use]
    pub fn amphyst(&mut self) -> AmphystW<HfxttargSpec> {
        AmphystW::new(self, 30)
    }
}
#[doc = "Target values for HFXT ramping\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hfxttarg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hfxttarg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HfxttargSpec;
impl crate::RegisterSpec for HfxttargSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hfxttarg::R`](R) reader structure"]
impl crate::Readable for HfxttargSpec {}
#[doc = "`write(|w| ..)` method takes [`hfxttarg::W`](W) writer structure"]
impl crate::Writable for HfxttargSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HFXTTARG to value 0x5446_4b6d"]
impl crate::Resettable for HfxttargSpec {
    const RESET_VALUE: u32 = 0x5446_4b6d;
}
