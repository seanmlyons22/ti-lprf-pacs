#[doc = "Register `ADCREF0` reader"]
pub type R = crate::R<Adcref0Spec>;
#[doc = "Register `ADCREF0` writer"]
pub type W = crate::W<Adcref0Spec>;
#[doc = "Field `EN` reader - 0:0\\]
ADC reference module enable: 0: ADC reference module powered down 1: ADC reference module enabled"]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - 0:0\\]
ADC reference module enable: 0: ADC reference module powered down 1: ADC reference module enabled"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED1` reader - 2:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `RESERVED1` writer - 2:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SRC` reader - 3:3\\]
ADC reference source: 0: Fixed reference = 4.3V 1: Relative reference = VDDS"]
pub type SrcR = crate::BitReader;
#[doc = "Field `SRC` writer - 3:3\\]
ADC reference source: 0: Fixed reference = 4.3V 1: Relative reference = VDDS"]
pub type SrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXT` reader - 4:4\\]
Internal. Only to be used through TI provided API."]
pub type ExtR = crate::BitReader;
#[doc = "Field `EXT` writer - 4:4\\]
Internal. Only to be used through TI provided API."]
pub type ExtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOMUX` reader - 5:5\\]
Internal. Only to be used through TI provided API."]
pub type IomuxR = crate::BitReader;
#[doc = "Field `IOMUX` writer - 5:5\\]
Internal. Only to be used through TI provided API."]
pub type IomuxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REF_ON_IDLE` reader - 6:6\\]
Enable ADCREF in IDLE state. 0: Disabled in IDLE state 1: Enabled in IDLE state Keep ADCREF enabled when ADC0.SMPL_MODE = 0. Recommendation: Enable ADCREF always when ADC0.SMPL_CYCLE_EXP is less than 0x6 (21.3us sampling time)."]
pub type RefOnIdleR = crate::BitReader;
#[doc = "Field `REF_ON_IDLE` writer - 6:6\\]
Enable ADCREF in IDLE state. 0: Disabled in IDLE state 1: Enabled in IDLE state Keep ADCREF enabled when ADC0.SMPL_MODE = 0. Recommendation: Enable ADCREF always when ADC0.SMPL_CYCLE_EXP is less than 0x6 (21.3us sampling time)."]
pub type RefOnIdleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPARE7` reader - 7:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Spare7R = crate::BitReader;
#[doc = "Field `SPARE7` writer - 7:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Spare7W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
ADC reference module enable: 0: ADC reference module powered down 1: ADC reference module enabled"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - 2:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new((self.bits >> 1) & 3)
    }
    #[doc = "Bit 3 - 3:3\\]
ADC reference source: 0: Fixed reference = 4.3V 1: Relative reference = VDDS"]
    #[inline(always)]
    pub fn src(&self) -> SrcR {
        SrcR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ext(&self) -> ExtR {
        ExtR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn iomux(&self) -> IomuxR {
        IomuxR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Enable ADCREF in IDLE state. 0: Disabled in IDLE state 1: Enabled in IDLE state Keep ADCREF enabled when ADC0.SMPL_MODE = 0. Recommendation: Enable ADCREF always when ADC0.SMPL_CYCLE_EXP is less than 0x6 (21.3us sampling time)."]
    #[inline(always)]
    pub fn ref_on_idle(&self) -> RefOnIdleR {
        RefOnIdleR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn spare7(&self) -> Spare7R {
        Spare7R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
ADC reference module enable: 0: ADC reference module powered down 1: ADC reference module enabled"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EnW<Adcref0Spec> {
        EnW::new(self, 0)
    }
    #[doc = "Bits 1:2 - 2:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<Adcref0Spec> {
        Reserved1W::new(self, 1)
    }
    #[doc = "Bit 3 - 3:3\\]
ADC reference source: 0: Fixed reference = 4.3V 1: Relative reference = VDDS"]
    #[inline(always)]
    #[must_use]
    pub fn src(&mut self) -> SrcW<Adcref0Spec> {
        SrcW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn ext(&mut self) -> ExtW<Adcref0Spec> {
        ExtW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn iomux(&mut self) -> IomuxW<Adcref0Spec> {
        IomuxW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Enable ADCREF in IDLE state. 0: Disabled in IDLE state 1: Enabled in IDLE state Keep ADCREF enabled when ADC0.SMPL_MODE = 0. Recommendation: Enable ADCREF always when ADC0.SMPL_CYCLE_EXP is less than 0x6 (21.3us sampling time)."]
    #[inline(always)]
    #[must_use]
    pub fn ref_on_idle(&mut self) -> RefOnIdleW<Adcref0Spec> {
        RefOnIdleW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn spare7(&mut self) -> Spare7W<Adcref0Spec> {
        Spare7W::new(self, 7)
    }
}
#[doc = "ADC Reference 0 Control reference used by the ADC. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcref0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcref0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adcref0Spec;
impl crate::RegisterSpec for Adcref0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`adcref0::R`](R) reader structure"]
impl crate::Readable for Adcref0Spec {}
#[doc = "`write(|w| ..)` method takes [`adcref0::W`](W) writer structure"]
impl crate::Writable for Adcref0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets ADCREF0 to value 0"]
impl crate::Resettable for Adcref0Spec {
    const RESET_VALUE: u8 = 0;
}
