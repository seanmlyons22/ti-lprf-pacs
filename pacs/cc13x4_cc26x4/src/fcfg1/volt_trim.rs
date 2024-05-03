#[doc = "Register `VOLT_TRIM` reader"]
pub type R = crate::R<VoltTrimSpec>;
#[doc = "Register `VOLT_TRIM` writer"]
pub type W = crate::W<VoltTrimSpec>;
#[doc = "Field `TRIMBOD_H` reader - 4:0\\]
Internal. Only to be used through TI provided API."]
pub type TrimbodHR = crate::FieldReader;
#[doc = "Field `TRIMBOD_H` writer - 4:0\\]
Internal. Only to be used through TI provided API."]
pub type TrimbodHW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `RESERVED0` reader - 7:5\\]
Internal. Only to be used through TI provided API."]
pub type Reserved0R = crate::FieldReader;
#[doc = "Field `RESERVED0` writer - 7:5\\]
Internal. Only to be used through TI provided API."]
pub type Reserved0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `VDDR_TRIM_SLEEP_H` reader - 12:8\\]
Internal. Only to be used through TI provided API."]
pub type VddrTrimSleepHR = crate::FieldReader;
#[doc = "Field `VDDR_TRIM_SLEEP_H` writer - 12:8\\]
Internal. Only to be used through TI provided API."]
pub type VddrTrimSleepHW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `RESERVED1` reader - 15:13\\]
Internal. Only to be used through TI provided API."]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `RESERVED1` writer - 15:13\\]
Internal. Only to be used through TI provided API."]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `VDDR_TRIM_H` reader - 20:16\\]
Internal. Only to be used through TI provided API."]
pub type VddrTrimHR = crate::FieldReader;
#[doc = "Field `VDDR_TRIM_H` writer - 20:16\\]
Internal. Only to be used through TI provided API."]
pub type VddrTrimHW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `RESERVED2` reader - 23:21\\]
Internal. Only to be used through TI provided API."]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `RESERVED2` writer - 23:21\\]
Internal. Only to be used through TI provided API."]
pub type Reserved2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `VDDR_TRIM_HH` reader - 28:24\\]
Internal. Only to be used through TI provided API."]
pub type VddrTrimHhR = crate::FieldReader;
#[doc = "Field `VDDR_TRIM_HH` writer - 28:24\\]
Internal. Only to be used through TI provided API."]
pub type VddrTrimHhW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `RESERVED3` reader - 31:29\\]
Internal. Only to be used through TI provided API."]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `RESERVED3` writer - 31:29\\]
Internal. Only to be used through TI provided API."]
pub type Reserved3W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn trimbod_h(&self) -> TrimbodHR {
        TrimbodHR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:7 - 7:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vddr_trim_sleep_h(&self) -> VddrTrimSleepHR {
        VddrTrimSleepHR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 13:15 - 15:13\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vddr_trim_h(&self) -> VddrTrimHR {
        VddrTrimHR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:23 - 23:21\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vddr_trim_hh(&self) -> VddrTrimHhR {
        VddrTrimHhR::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bits 29:31 - 31:29\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn trimbod_h(&mut self) -> TrimbodHW<VoltTrimSpec> {
        TrimbodHW::new(self, 0)
    }
    #[doc = "Bits 5:7 - 7:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved0(&mut self) -> Reserved0W<VoltTrimSpec> {
        Reserved0W::new(self, 5)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn vddr_trim_sleep_h(&mut self) -> VddrTrimSleepHW<VoltTrimSpec> {
        VddrTrimSleepHW::new(self, 8)
    }
    #[doc = "Bits 13:15 - 15:13\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<VoltTrimSpec> {
        Reserved1W::new(self, 13)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn vddr_trim_h(&mut self) -> VddrTrimHW<VoltTrimSpec> {
        VddrTrimHW::new(self, 16)
    }
    #[doc = "Bits 21:23 - 23:21\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> Reserved2W<VoltTrimSpec> {
        Reserved2W::new(self, 21)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn vddr_trim_hh(&mut self) -> VddrTrimHhW<VoltTrimSpec> {
        VddrTrimHhW::new(self, 24)
    }
    #[doc = "Bits 29:31 - 31:29\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> Reserved3W<VoltTrimSpec> {
        Reserved3W::new(self, 29)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`volt_trim::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`volt_trim::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VoltTrimSpec;
impl crate::RegisterSpec for VoltTrimSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`volt_trim::R`](R) reader structure"]
impl crate::Readable for VoltTrimSpec {}
#[doc = "`write(|w| ..)` method takes [`volt_trim::W`](W) writer structure"]
impl crate::Writable for VoltTrimSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VOLT_TRIM to value 0xe0e0_e0e0"]
impl crate::Resettable for VoltTrimSpec {
    const RESET_VALUE: u32 = 0xe0e0_e0e0;
}
