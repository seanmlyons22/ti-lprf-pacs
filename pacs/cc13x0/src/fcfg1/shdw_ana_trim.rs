#[doc = "Register `SHDW_ANA_TRIM` reader"]
pub type R = crate::R<ShdwAnaTrimSpec>;
#[doc = "Register `SHDW_ANA_TRIM` writer"]
pub type W = crate::W<ShdwAnaTrimSpec>;
#[doc = "Field `TRIMTEMP` reader - 5:0\\]
Internal. Only to be used through TI provided API."]
pub type TrimtempR = crate::FieldReader;
#[doc = "Field `TRIMBOD_EXTMODE` reader - 10:6\\]
Internal. Only to be used through TI provided API."]
pub type TrimbodExtmodeR = crate::FieldReader;
#[doc = "Field `TRIMBOD_INTMODE` reader - 15:11\\]
Internal. Only to be used through TI provided API."]
pub type TrimbodIntmodeR = crate::FieldReader;
#[doc = "Field `VDDR_TRIM` reader - 20:16\\]
Internal. Only to be used through TI provided API."]
pub type VddrTrimR = crate::FieldReader;
#[doc = "Field `IPTAT_TRIM` reader - 22:21\\]
Internal. Only to be used through TI provided API."]
pub type IptatTrimR = crate::FieldReader;
#[doc = "Field `VDDR_OK_HYS` reader - 23:23\\]
Internal. Only to be used through TI provided API."]
pub type VddrOkHysR = crate::BitReader;
#[doc = "Field `VDDR_ENABLE_PG1` reader - 24:24\\]
Internal. Only to be used through TI provided API."]
pub type VddrEnablePg1R = crate::BitReader;
#[doc = "Field `BOD_BANDGAP_TRIM_CNF` reader - 26:25\\]
Internal. Only to be used through TI provided API."]
pub type BodBandgapTrimCnfR = crate::FieldReader;
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
impl W {}
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
