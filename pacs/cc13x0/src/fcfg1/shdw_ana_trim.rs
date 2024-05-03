#[doc = "Register `SHDW_ANA_TRIM` reader"]
pub type R = crate::R<ShdwAnaTrimSpec>;
#[doc = "Register `SHDW_ANA_TRIM` writer"]
pub type W = crate::W<ShdwAnaTrimSpec>;
#[doc = "Field `TRIMTEMP` reader - 5:0\\]
Internal. Only to be used through TI provided API."]
pub type TrimtempR = crate::FieldReader;
#[doc = "Field `TRIMTEMP` writer - 5:0\\]
Internal. Only to be used through TI provided API."]
pub type TrimtempW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `TRIMBOD_EXTMODE` reader - 10:6\\]
Internal. Only to be used through TI provided API."]
pub type TrimbodExtmodeR = crate::FieldReader;
#[doc = "Field `TRIMBOD_EXTMODE` writer - 10:6\\]
Internal. Only to be used through TI provided API."]
pub type TrimbodExtmodeW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `TRIMBOD_INTMODE` reader - 15:11\\]
Internal. Only to be used through TI provided API."]
pub type TrimbodIntmodeR = crate::FieldReader;
#[doc = "Field `TRIMBOD_INTMODE` writer - 15:11\\]
Internal. Only to be used through TI provided API."]
pub type TrimbodIntmodeW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `VDDR_TRIM` reader - 20:16\\]
Internal. Only to be used through TI provided API."]
pub type VddrTrimR = crate::FieldReader;
#[doc = "Field `VDDR_TRIM` writer - 20:16\\]
Internal. Only to be used through TI provided API."]
pub type VddrTrimW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `IPTAT_TRIM` reader - 22:21\\]
Internal. Only to be used through TI provided API."]
pub type IptatTrimR = crate::FieldReader;
#[doc = "Field `IPTAT_TRIM` writer - 22:21\\]
Internal. Only to be used through TI provided API."]
pub type IptatTrimW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `VDDR_OK_HYS` reader - 23:23\\]
Internal. Only to be used through TI provided API."]
pub type VddrOkHysR = crate::BitReader;
#[doc = "Field `VDDR_OK_HYS` writer - 23:23\\]
Internal. Only to be used through TI provided API."]
pub type VddrOkHysW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VDDR_ENABLE_PG1` reader - 24:24\\]
Internal. Only to be used through TI provided API."]
pub type VddrEnablePg1R = crate::BitReader;
#[doc = "Field `VDDR_ENABLE_PG1` writer - 24:24\\]
Internal. Only to be used through TI provided API."]
pub type VddrEnablePg1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOD_BANDGAP_TRIM_CNF` reader - 26:25\\]
Internal. Only to be used through TI provided API."]
pub type BodBandgapTrimCnfR = crate::FieldReader;
#[doc = "Field `BOD_BANDGAP_TRIM_CNF` writer - 26:25\\]
Internal. Only to be used through TI provided API."]
pub type BodBandgapTrimCnfW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn trimtemp(&self) -> TrimtempR {
        TrimtempR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:10 - 10:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn trimbod_extmode(&self) -> TrimbodExtmodeR {
        TrimbodExtmodeR::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bits 11:15 - 15:11\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn trimbod_intmode(&self) -> TrimbodIntmodeR {
        TrimbodIntmodeR::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vddr_trim(&self) -> VddrTrimR {
        VddrTrimR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:22 - 22:21\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn iptat_trim(&self) -> IptatTrimR {
        IptatTrimR::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bit 23 - 23:23\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vddr_ok_hys(&self) -> VddrOkHysR {
        VddrOkHysR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vddr_enable_pg1(&self) -> VddrEnablePg1R {
        VddrEnablePg1R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:26 - 26:25\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn bod_bandgap_trim_cnf(&self) -> BodBandgapTrimCnfR {
        BodBandgapTrimCnfR::new(((self.bits >> 25) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn trimtemp(&mut self) -> TrimtempW<ShdwAnaTrimSpec> {
        TrimtempW::new(self, 0)
    }
    #[doc = "Bits 6:10 - 10:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn trimbod_extmode(&mut self) -> TrimbodExtmodeW<ShdwAnaTrimSpec> {
        TrimbodExtmodeW::new(self, 6)
    }
    #[doc = "Bits 11:15 - 15:11\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn trimbod_intmode(&mut self) -> TrimbodIntmodeW<ShdwAnaTrimSpec> {
        TrimbodIntmodeW::new(self, 11)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn vddr_trim(&mut self) -> VddrTrimW<ShdwAnaTrimSpec> {
        VddrTrimW::new(self, 16)
    }
    #[doc = "Bits 21:22 - 22:21\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn iptat_trim(&mut self) -> IptatTrimW<ShdwAnaTrimSpec> {
        IptatTrimW::new(self, 21)
    }
    #[doc = "Bit 23 - 23:23\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn vddr_ok_hys(&mut self) -> VddrOkHysW<ShdwAnaTrimSpec> {
        VddrOkHysW::new(self, 23)
    }
    #[doc = "Bit 24 - 24:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn vddr_enable_pg1(&mut self) -> VddrEnablePg1W<ShdwAnaTrimSpec> {
        VddrEnablePg1W::new(self, 24)
    }
    #[doc = "Bits 25:26 - 26:25\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn bod_bandgap_trim_cnf(&mut self) -> BodBandgapTrimCnfW<ShdwAnaTrimSpec> {
        BodBandgapTrimCnfW::new(self, 25)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`shdw_ana_trim::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`shdw_ana_trim::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ShdwAnaTrimSpec;
impl crate::RegisterSpec for ShdwAnaTrimSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`shdw_ana_trim::R`](R) reader structure"]
impl crate::Readable for ShdwAnaTrimSpec {}
#[doc = "`write(|w| ..)` method takes [`shdw_ana_trim::W`](W) writer structure"]
impl crate::Writable for ShdwAnaTrimSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SHDW_ANA_TRIM to value 0"]
impl crate::Resettable for ShdwAnaTrimSpec {
    const RESET_VALUE: u32 = 0;
}
