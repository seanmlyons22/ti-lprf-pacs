#[doc = "Register `SHDW_ANA_TRIM` reader"]
pub struct R(crate::R<SHDW_ANA_TRIM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SHDW_ANA_TRIM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SHDW_ANA_TRIM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SHDW_ANA_TRIM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SHDW_ANA_TRIM` writer"]
pub struct W(crate::W<SHDW_ANA_TRIM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SHDW_ANA_TRIM_SPEC>;
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
impl From<crate::W<SHDW_ANA_TRIM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SHDW_ANA_TRIM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRIMTEMP` reader - 5:0\\]
Internal. Only to be used through TI provided API."]
pub type TRIMTEMP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRIMTEMP` writer - 5:0\\]
Internal. Only to be used through TI provided API."]
pub type TRIMTEMP_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SHDW_ANA_TRIM_SPEC, u8, u8, 6, O>;
#[doc = "Field `TRIMBOD_EXTMODE` reader - 10:6\\]
Internal. Only to be used through TI provided API."]
pub type TRIMBOD_EXTMODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRIMBOD_EXTMODE` writer - 10:6\\]
Internal. Only to be used through TI provided API."]
pub type TRIMBOD_EXTMODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SHDW_ANA_TRIM_SPEC, u8, u8, 5, O>;
#[doc = "Field `TRIMBOD_INTMODE` reader - 15:11\\]
Internal. Only to be used through TI provided API."]
pub type TRIMBOD_INTMODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRIMBOD_INTMODE` writer - 15:11\\]
Internal. Only to be used through TI provided API."]
pub type TRIMBOD_INTMODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SHDW_ANA_TRIM_SPEC, u8, u8, 5, O>;
#[doc = "Field `VDDR_TRIM` reader - 20:16\\]
Internal. Only to be used through TI provided API."]
pub type VDDR_TRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VDDR_TRIM` writer - 20:16\\]
Internal. Only to be used through TI provided API."]
pub type VDDR_TRIM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SHDW_ANA_TRIM_SPEC, u8, u8, 5, O>;
#[doc = "Field `IPTAT_TRIM` reader - 22:21\\]
Internal. Only to be used through TI provided API."]
pub type IPTAT_TRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IPTAT_TRIM` writer - 22:21\\]
Internal. Only to be used through TI provided API."]
pub type IPTAT_TRIM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SHDW_ANA_TRIM_SPEC, u8, u8, 2, O>;
#[doc = "Field `VDDR_OK_HYS` reader - 23:23\\]
Internal. Only to be used through TI provided API."]
pub type VDDR_OK_HYS_R = crate::BitReader<bool>;
#[doc = "Field `VDDR_OK_HYS` writer - 23:23\\]
Internal. Only to be used through TI provided API."]
pub type VDDR_OK_HYS_W<'a, const O: u8> = crate::BitWriter<'a, u32, SHDW_ANA_TRIM_SPEC, bool, O>;
#[doc = "Field `VDDR_ENABLE_PG1` reader - 24:24\\]
Internal. Only to be used through TI provided API."]
pub type VDDR_ENABLE_PG1_R = crate::BitReader<bool>;
#[doc = "Field `VDDR_ENABLE_PG1` writer - 24:24\\]
Internal. Only to be used through TI provided API."]
pub type VDDR_ENABLE_PG1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SHDW_ANA_TRIM_SPEC, bool, O>;
#[doc = "Field `BOD_BANDGAP_TRIM_CNF` reader - 26:25\\]
Internal. Only to be used through TI provided API."]
pub type BOD_BANDGAP_TRIM_CNF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BOD_BANDGAP_TRIM_CNF` writer - 26:25\\]
Internal. Only to be used through TI provided API."]
pub type BOD_BANDGAP_TRIM_CNF_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SHDW_ANA_TRIM_SPEC, u8, u8, 2, O>;
#[doc = "Field `BOD_BANDGAP_TRIM_CNF_EXT` reader - 28:27\\]
Internal. Only to be used through TI provided API."]
pub type BOD_BANDGAP_TRIM_CNF_EXT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BOD_BANDGAP_TRIM_CNF_EXT` writer - 28:27\\]
Internal. Only to be used through TI provided API."]
pub type BOD_BANDGAP_TRIM_CNF_EXT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SHDW_ANA_TRIM_SPEC, u8, u8, 2, O>;
#[doc = "Field `DET_LOGIC_DIS` reader - 29:29\\]
Internal. Only to be used through TI provided API."]
pub type DET_LOGIC_DIS_R = crate::BitReader<bool>;
#[doc = "Field `DET_LOGIC_DIS` writer - 29:29\\]
Internal. Only to be used through TI provided API."]
pub type DET_LOGIC_DIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, SHDW_ANA_TRIM_SPEC, bool, O>;
#[doc = "Field `ALT_VDDR_TRIM` reader - 30:30\\]
Internal. Only to be used through TI provided API."]
pub type ALT_VDDR_TRIM_R = crate::BitReader<bool>;
#[doc = "Field `ALT_VDDR_TRIM` writer - 30:30\\]
Internal. Only to be used through TI provided API."]
pub type ALT_VDDR_TRIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, SHDW_ANA_TRIM_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn trimtemp(&self) -> TRIMTEMP_R {
        TRIMTEMP_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:10 - 10:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn trimbod_extmode(&self) -> TRIMBOD_EXTMODE_R {
        TRIMBOD_EXTMODE_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bits 11:15 - 15:11\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn trimbod_intmode(&self) -> TRIMBOD_INTMODE_R {
        TRIMBOD_INTMODE_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vddr_trim(&self) -> VDDR_TRIM_R {
        VDDR_TRIM_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:22 - 22:21\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn iptat_trim(&self) -> IPTAT_TRIM_R {
        IPTAT_TRIM_R::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bit 23 - 23:23\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vddr_ok_hys(&self) -> VDDR_OK_HYS_R {
        VDDR_OK_HYS_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vddr_enable_pg1(&self) -> VDDR_ENABLE_PG1_R {
        VDDR_ENABLE_PG1_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:26 - 26:25\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn bod_bandgap_trim_cnf(&self) -> BOD_BANDGAP_TRIM_CNF_R {
        BOD_BANDGAP_TRIM_CNF_R::new(((self.bits >> 25) & 3) as u8)
    }
    #[doc = "Bits 27:28 - 28:27\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn bod_bandgap_trim_cnf_ext(&self) -> BOD_BANDGAP_TRIM_CNF_EXT_R {
        BOD_BANDGAP_TRIM_CNF_EXT_R::new(((self.bits >> 27) & 3) as u8)
    }
    #[doc = "Bit 29 - 29:29\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn det_logic_dis(&self) -> DET_LOGIC_DIS_R {
        DET_LOGIC_DIS_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 30:30\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn alt_vddr_trim(&self) -> ALT_VDDR_TRIM_R {
        ALT_VDDR_TRIM_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn trimtemp(&mut self) -> TRIMTEMP_W<0> {
        TRIMTEMP_W::new(self)
    }
    #[doc = "Bits 6:10 - 10:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn trimbod_extmode(&mut self) -> TRIMBOD_EXTMODE_W<6> {
        TRIMBOD_EXTMODE_W::new(self)
    }
    #[doc = "Bits 11:15 - 15:11\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn trimbod_intmode(&mut self) -> TRIMBOD_INTMODE_W<11> {
        TRIMBOD_INTMODE_W::new(self)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn vddr_trim(&mut self) -> VDDR_TRIM_W<16> {
        VDDR_TRIM_W::new(self)
    }
    #[doc = "Bits 21:22 - 22:21\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn iptat_trim(&mut self) -> IPTAT_TRIM_W<21> {
        IPTAT_TRIM_W::new(self)
    }
    #[doc = "Bit 23 - 23:23\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn vddr_ok_hys(&mut self) -> VDDR_OK_HYS_W<23> {
        VDDR_OK_HYS_W::new(self)
    }
    #[doc = "Bit 24 - 24:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn vddr_enable_pg1(&mut self) -> VDDR_ENABLE_PG1_W<24> {
        VDDR_ENABLE_PG1_W::new(self)
    }
    #[doc = "Bits 25:26 - 26:25\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn bod_bandgap_trim_cnf(&mut self) -> BOD_BANDGAP_TRIM_CNF_W<25> {
        BOD_BANDGAP_TRIM_CNF_W::new(self)
    }
    #[doc = "Bits 27:28 - 28:27\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn bod_bandgap_trim_cnf_ext(&mut self) -> BOD_BANDGAP_TRIM_CNF_EXT_W<27> {
        BOD_BANDGAP_TRIM_CNF_EXT_W::new(self)
    }
    #[doc = "Bit 29 - 29:29\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn det_logic_dis(&mut self) -> DET_LOGIC_DIS_W<29> {
        DET_LOGIC_DIS_W::new(self)
    }
    #[doc = "Bit 30 - 30:30\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn alt_vddr_trim(&mut self) -> ALT_VDDR_TRIM_W<30> {
        ALT_VDDR_TRIM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shdw_ana_trim](index.html) module"]
pub struct SHDW_ANA_TRIM_SPEC;
impl crate::RegisterSpec for SHDW_ANA_TRIM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [shdw_ana_trim::R](R) reader structure"]
impl crate::Readable for SHDW_ANA_TRIM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [shdw_ana_trim::W](W) writer structure"]
impl crate::Writable for SHDW_ANA_TRIM_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SHDW_ANA_TRIM to value 0"]
impl crate::Resettable for SHDW_ANA_TRIM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
