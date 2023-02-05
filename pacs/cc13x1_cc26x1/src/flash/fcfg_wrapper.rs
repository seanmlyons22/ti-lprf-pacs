#[doc = "Register `FCFG_WRAPPER` reader"]
pub struct R(crate::R<FCFG_WRAPPER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FCFG_WRAPPER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FCFG_WRAPPER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FCFG_WRAPPER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FCFG_WRAPPER` writer"]
pub struct W(crate::W<FCFG_WRAPPER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FCFG_WRAPPER_SPEC>;
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
impl From<crate::W<FCFG_WRAPPER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FCFG_WRAPPER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CPU_TYPE1` reader - 3:0\\]
Internal. Only to be used through TI provided API."]
pub type CPU_TYPE1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CPU_TYPE1` writer - 3:0\\]
Internal. Only to be used through TI provided API."]
pub type CPU_TYPE1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FCFG_WRAPPER_SPEC, u8, u8, 4, O>;
#[doc = "Field `UERR` reader - 5:4\\]
Internal. Only to be used through TI provided API."]
pub type UERR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `UERR` writer - 5:4\\]
Internal. Only to be used through TI provided API."]
pub type UERR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FCFG_WRAPPER_SPEC, u8, u8, 2, O>;
#[doc = "Field `AUTO_SUSP` reader - 7:6\\]
Internal. Only to be used through TI provided API."]
pub type AUTO_SUSP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AUTO_SUSP` writer - 7:6\\]
Internal. Only to be used through TI provided API."]
pub type AUTO_SUSP_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FCFG_WRAPPER_SPEC, u8, u8, 2, O>;
#[doc = "Field `ECCA` reader - 8:8\\]
Internal. Only to be used through TI provided API."]
pub type ECCA_R = crate::BitReader<bool>;
#[doc = "Field `ECCA` writer - 8:8\\]
Internal. Only to be used through TI provided API."]
pub type ECCA_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCFG_WRAPPER_SPEC, bool, O>;
#[doc = "Field `SIL3` reader - 9:9\\]
Internal. Only to be used through TI provided API."]
pub type SIL3_R = crate::BitReader<bool>;
#[doc = "Field `SIL3` writer - 9:9\\]
Internal. Only to be used through TI provided API."]
pub type SIL3_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCFG_WRAPPER_SPEC, bool, O>;
#[doc = "Field `IFLUSH` reader - 10:10\\]
Internal. Only to be used through TI provided API."]
pub type IFLUSH_R = crate::BitReader<bool>;
#[doc = "Field `IFLUSH` writer - 10:10\\]
Internal. Only to be used through TI provided API."]
pub type IFLUSH_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCFG_WRAPPER_SPEC, bool, O>;
#[doc = "Field `ROM` reader - 11:11\\]
Internal. Only to be used through TI provided API."]
pub type ROM_R = crate::BitReader<bool>;
#[doc = "Field `ROM` writer - 11:11\\]
Internal. Only to be used through TI provided API."]
pub type ROM_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCFG_WRAPPER_SPEC, bool, O>;
#[doc = "Field `EE_IN_MAIN` reader - 15:12\\]
Internal. Only to be used through TI provided API."]
pub type EE_IN_MAIN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EE_IN_MAIN` writer - 15:12\\]
Internal. Only to be used through TI provided API."]
pub type EE_IN_MAIN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FCFG_WRAPPER_SPEC, u8, u8, 4, O>;
#[doc = "Field `CPU2` reader - 19:16\\]
Internal. Only to be used through TI provided API."]
pub type CPU2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CPU2` writer - 19:16\\]
Internal. Only to be used through TI provided API."]
pub type CPU2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FCFG_WRAPPER_SPEC, u8, u8, 4, O>;
#[doc = "Field `MEM_MAP` reader - 20:20\\]
Internal. Only to be used through TI provided API."]
pub type MEM_MAP_R = crate::BitReader<bool>;
#[doc = "Field `MEM_MAP` writer - 20:20\\]
Internal. Only to be used through TI provided API."]
pub type MEM_MAP_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCFG_WRAPPER_SPEC, bool, O>;
#[doc = "Field `RESERVED21` reader - 23:21\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED21_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED21` writer - 23:21\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED21_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FCFG_WRAPPER_SPEC, u8, u8, 3, O>;
#[doc = "Field `FAMILY_TYPE` reader - 31:24\\]
Internal. Only to be used through TI provided API."]
pub type FAMILY_TYPE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FAMILY_TYPE` writer - 31:24\\]
Internal. Only to be used through TI provided API."]
pub type FAMILY_TYPE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FCFG_WRAPPER_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn cpu_type1(&self) -> CPU_TYPE1_R {
        CPU_TYPE1_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - 5:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn uerr(&self) -> UERR_R {
        UERR_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn auto_susp(&self) -> AUTO_SUSP_R {
        AUTO_SUSP_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ecca(&self) -> ECCA_R {
        ECCA_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn sil3(&self) -> SIL3_R {
        SIL3_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn iflush(&self) -> IFLUSH_R {
        IFLUSH_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rom(&self) -> ROM_R {
        ROM_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ee_in_main(&self) -> EE_IN_MAIN_R {
        EE_IN_MAIN_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn cpu2(&self) -> CPU2_R {
        CPU2_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - 20:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn mem_map(&self) -> MEM_MAP_R {
        MEM_MAP_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:23 - 23:21\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved21(&self) -> RESERVED21_R {
        RESERVED21_R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn family_type(&self) -> FAMILY_TYPE_R {
        FAMILY_TYPE_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn cpu_type1(&mut self) -> CPU_TYPE1_W<0> {
        CPU_TYPE1_W::new(self)
    }
    #[doc = "Bits 4:5 - 5:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn uerr(&mut self) -> UERR_W<4> {
        UERR_W::new(self)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn auto_susp(&mut self) -> AUTO_SUSP_W<6> {
        AUTO_SUSP_W::new(self)
    }
    #[doc = "Bit 8 - 8:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn ecca(&mut self) -> ECCA_W<8> {
        ECCA_W::new(self)
    }
    #[doc = "Bit 9 - 9:9\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn sil3(&mut self) -> SIL3_W<9> {
        SIL3_W::new(self)
    }
    #[doc = "Bit 10 - 10:10\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn iflush(&mut self) -> IFLUSH_W<10> {
        IFLUSH_W::new(self)
    }
    #[doc = "Bit 11 - 11:11\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn rom(&mut self) -> ROM_W<11> {
        ROM_W::new(self)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn ee_in_main(&mut self) -> EE_IN_MAIN_W<12> {
        EE_IN_MAIN_W::new(self)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn cpu2(&mut self) -> CPU2_W<16> {
        CPU2_W::new(self)
    }
    #[doc = "Bit 20 - 20:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn mem_map(&mut self) -> MEM_MAP_W<20> {
        MEM_MAP_W::new(self)
    }
    #[doc = "Bits 21:23 - 23:21\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved21(&mut self) -> RESERVED21_W<21> {
        RESERVED21_W::new(self)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn family_type(&mut self) -> FAMILY_TYPE_W<24> {
        FAMILY_TYPE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fcfg_wrapper](index.html) module"]
pub struct FCFG_WRAPPER_SPEC;
impl crate::RegisterSpec for FCFG_WRAPPER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fcfg_wrapper::R](R) reader structure"]
impl crate::Readable for FCFG_WRAPPER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fcfg_wrapper::W](W) writer structure"]
impl crate::Writable for FCFG_WRAPPER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FCFG_WRAPPER to value 0x6000_9001"]
impl crate::Resettable for FCFG_WRAPPER_SPEC {
    const RESET_VALUE: Self::Ux = 0x6000_9001;
}
