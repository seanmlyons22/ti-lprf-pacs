#[doc = "Register `FMSTAT` reader"]
pub struct R(crate::R<FMSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FMSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FMSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FMSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FMSTAT` writer"]
pub struct W(crate::W<FMSTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FMSTAT_SPEC>;
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
impl From<crate::W<FMSTAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FMSTAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLOCK` reader - 0:0\\]
Internal. Only to be used through TI provided API."]
pub type SLOCK_R = crate::BitReader<bool>;
#[doc = "Field `SLOCK` writer - 0:0\\]
Internal. Only to be used through TI provided API."]
pub type SLOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, FMSTAT_SPEC, bool, O>;
#[doc = "Field `PSUSP` reader - 1:1\\]
Internal. Only to be used through TI provided API."]
pub type PSUSP_R = crate::BitReader<bool>;
#[doc = "Field `PSUSP` writer - 1:1\\]
Internal. Only to be used through TI provided API."]
pub type PSUSP_W<'a, const O: u8> = crate::BitWriter<'a, u32, FMSTAT_SPEC, bool, O>;
#[doc = "Field `ESUSP` reader - 2:2\\]
Internal. Only to be used through TI provided API."]
pub type ESUSP_R = crate::BitReader<bool>;
#[doc = "Field `ESUSP` writer - 2:2\\]
Internal. Only to be used through TI provided API."]
pub type ESUSP_W<'a, const O: u8> = crate::BitWriter<'a, u32, FMSTAT_SPEC, bool, O>;
#[doc = "Field `VOLSTAT` reader - 3:3\\]
Internal. Only to be used through TI provided API."]
pub type VOLSTAT_R = crate::BitReader<bool>;
#[doc = "Field `VOLSTAT` writer - 3:3\\]
Internal. Only to be used through TI provided API."]
pub type VOLSTAT_W<'a, const O: u8> = crate::BitWriter<'a, u32, FMSTAT_SPEC, bool, O>;
#[doc = "Field `CSTAT` reader - 4:4\\]
Internal. Only to be used through TI provided API."]
pub type CSTAT_R = crate::BitReader<bool>;
#[doc = "Field `CSTAT` writer - 4:4\\]
Internal. Only to be used through TI provided API."]
pub type CSTAT_W<'a, const O: u8> = crate::BitWriter<'a, u32, FMSTAT_SPEC, bool, O>;
#[doc = "Field `INVDAT` reader - 5:5\\]
Internal. Only to be used through TI provided API."]
pub type INVDAT_R = crate::BitReader<bool>;
#[doc = "Field `INVDAT` writer - 5:5\\]
Internal. Only to be used through TI provided API."]
pub type INVDAT_W<'a, const O: u8> = crate::BitWriter<'a, u32, FMSTAT_SPEC, bool, O>;
#[doc = "Field `PGM` reader - 6:6\\]
Internal. Only to be used through TI provided API."]
pub type PGM_R = crate::BitReader<bool>;
#[doc = "Field `PGM` writer - 6:6\\]
Internal. Only to be used through TI provided API."]
pub type PGM_W<'a, const O: u8> = crate::BitWriter<'a, u32, FMSTAT_SPEC, bool, O>;
#[doc = "Field `ERS` reader - 7:7\\]
Internal. Only to be used through TI provided API."]
pub type ERS_R = crate::BitReader<bool>;
#[doc = "Field `ERS` writer - 7:7\\]
Internal. Only to be used through TI provided API."]
pub type ERS_W<'a, const O: u8> = crate::BitWriter<'a, u32, FMSTAT_SPEC, bool, O>;
#[doc = "Field `BUSY` reader - 8:8\\]
Internal. Only to be used through TI provided API."]
pub type BUSY_R = crate::BitReader<bool>;
#[doc = "Field `BUSY` writer - 8:8\\]
Internal. Only to be used through TI provided API."]
pub type BUSY_W<'a, const O: u8> = crate::BitWriter<'a, u32, FMSTAT_SPEC, bool, O>;
#[doc = "Field `CV` reader - 9:9\\]
Internal. Only to be used through TI provided API."]
pub type CV_R = crate::BitReader<bool>;
#[doc = "Field `CV` writer - 9:9\\]
Internal. Only to be used through TI provided API."]
pub type CV_W<'a, const O: u8> = crate::BitWriter<'a, u32, FMSTAT_SPEC, bool, O>;
#[doc = "Field `EV` reader - 10:10\\]
Internal. Only to be used through TI provided API."]
pub type EV_R = crate::BitReader<bool>;
#[doc = "Field `EV` writer - 10:10\\]
Internal. Only to be used through TI provided API."]
pub type EV_W<'a, const O: u8> = crate::BitWriter<'a, u32, FMSTAT_SPEC, bool, O>;
#[doc = "Field `PCV` reader - 11:11\\]
Internal. Only to be used through TI provided API."]
pub type PCV_R = crate::BitReader<bool>;
#[doc = "Field `PCV` writer - 11:11\\]
Internal. Only to be used through TI provided API."]
pub type PCV_W<'a, const O: u8> = crate::BitWriter<'a, u32, FMSTAT_SPEC, bool, O>;
#[doc = "Field `PGV` reader - 12:12\\]
Internal. Only to be used through TI provided API."]
pub type PGV_R = crate::BitReader<bool>;
#[doc = "Field `PGV` writer - 12:12\\]
Internal. Only to be used through TI provided API."]
pub type PGV_W<'a, const O: u8> = crate::BitWriter<'a, u32, FMSTAT_SPEC, bool, O>;
#[doc = "Field `DBF` reader - 13:13\\]
Internal. Only to be used through TI provided API."]
pub type DBF_R = crate::BitReader<bool>;
#[doc = "Field `DBF` writer - 13:13\\]
Internal. Only to be used through TI provided API."]
pub type DBF_W<'a, const O: u8> = crate::BitWriter<'a, u32, FMSTAT_SPEC, bool, O>;
#[doc = "Field `ILA` reader - 14:14\\]
Internal. Only to be used through TI provided API."]
pub type ILA_R = crate::BitReader<bool>;
#[doc = "Field `ILA` writer - 14:14\\]
Internal. Only to be used through TI provided API."]
pub type ILA_W<'a, const O: u8> = crate::BitWriter<'a, u32, FMSTAT_SPEC, bool, O>;
#[doc = "Field `RVF` reader - 15:15\\]
Internal. Only to be used through TI provided API."]
pub type RVF_R = crate::BitReader<bool>;
#[doc = "Field `RVF` writer - 15:15\\]
Internal. Only to be used through TI provided API."]
pub type RVF_W<'a, const O: u8> = crate::BitWriter<'a, u32, FMSTAT_SPEC, bool, O>;
#[doc = "Field `RDVER` reader - 16:16\\]
Internal. Only to be used through TI provided API."]
pub type RDVER_R = crate::BitReader<bool>;
#[doc = "Field `RDVER` writer - 16:16\\]
Internal. Only to be used through TI provided API."]
pub type RDVER_W<'a, const O: u8> = crate::BitWriter<'a, u32, FMSTAT_SPEC, bool, O>;
#[doc = "Field `RVSUSP` reader - 17:17\\]
Internal. Only to be used through TI provided API."]
pub type RVSUSP_R = crate::BitReader<bool>;
#[doc = "Field `RVSUSP` writer - 17:17\\]
Internal. Only to be used through TI provided API."]
pub type RVSUSP_W<'a, const O: u8> = crate::BitWriter<'a, u32, FMSTAT_SPEC, bool, O>;
#[doc = "Field `RESERVED18` reader - 31:18\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED18_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RESERVED18` writer - 31:18\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED18_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FMSTAT_SPEC, u16, u16, 14, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn slock(&self) -> SLOCK_R {
        SLOCK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn psusp(&self) -> PSUSP_R {
        PSUSP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn esusp(&self) -> ESUSP_R {
        ESUSP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn volstat(&self) -> VOLSTAT_R {
        VOLSTAT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn cstat(&self) -> CSTAT_R {
        CSTAT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn invdat(&self) -> INVDAT_R {
        INVDAT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn pgm(&self) -> PGM_R {
        PGM_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ers(&self) -> ERS_R {
        ERS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn cv(&self) -> CV_R {
        CV_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ev(&self) -> EV_R {
        EV_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn pcv(&self) -> PCV_R {
        PCV_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn pgv(&self) -> PGV_R {
        PGV_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dbf(&self) -> DBF_R {
        DBF_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ila(&self) -> ILA_R {
        ILA_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rvf(&self) -> RVF_R {
        RVF_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rdver(&self) -> RDVER_R {
        RDVER_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rvsusp(&self) -> RVSUSP_R {
        RVSUSP_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:31 - 31:18\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved18(&self) -> RESERVED18_R {
        RESERVED18_R::new(((self.bits >> 18) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn slock(&mut self) -> SLOCK_W<0> {
        SLOCK_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn psusp(&mut self) -> PSUSP_W<1> {
        PSUSP_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn esusp(&mut self) -> ESUSP_W<2> {
        ESUSP_W::new(self)
    }
    #[doc = "Bit 3 - 3:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn volstat(&mut self) -> VOLSTAT_W<3> {
        VOLSTAT_W::new(self)
    }
    #[doc = "Bit 4 - 4:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn cstat(&mut self) -> CSTAT_W<4> {
        CSTAT_W::new(self)
    }
    #[doc = "Bit 5 - 5:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn invdat(&mut self) -> INVDAT_W<5> {
        INVDAT_W::new(self)
    }
    #[doc = "Bit 6 - 6:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn pgm(&mut self) -> PGM_W<6> {
        PGM_W::new(self)
    }
    #[doc = "Bit 7 - 7:7\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn ers(&mut self) -> ERS_W<7> {
        ERS_W::new(self)
    }
    #[doc = "Bit 8 - 8:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn busy(&mut self) -> BUSY_W<8> {
        BUSY_W::new(self)
    }
    #[doc = "Bit 9 - 9:9\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn cv(&mut self) -> CV_W<9> {
        CV_W::new(self)
    }
    #[doc = "Bit 10 - 10:10\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn ev(&mut self) -> EV_W<10> {
        EV_W::new(self)
    }
    #[doc = "Bit 11 - 11:11\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn pcv(&mut self) -> PCV_W<11> {
        PCV_W::new(self)
    }
    #[doc = "Bit 12 - 12:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn pgv(&mut self) -> PGV_W<12> {
        PGV_W::new(self)
    }
    #[doc = "Bit 13 - 13:13\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn dbf(&mut self) -> DBF_W<13> {
        DBF_W::new(self)
    }
    #[doc = "Bit 14 - 14:14\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn ila(&mut self) -> ILA_W<14> {
        ILA_W::new(self)
    }
    #[doc = "Bit 15 - 15:15\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn rvf(&mut self) -> RVF_W<15> {
        RVF_W::new(self)
    }
    #[doc = "Bit 16 - 16:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn rdver(&mut self) -> RDVER_W<16> {
        RDVER_W::new(self)
    }
    #[doc = "Bit 17 - 17:17\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn rvsusp(&mut self) -> RVSUSP_W<17> {
        RVSUSP_W::new(self)
    }
    #[doc = "Bits 18:31 - 31:18\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved18(&mut self) -> RESERVED18_W<18> {
        RESERVED18_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmstat](index.html) module"]
pub struct FMSTAT_SPEC;
impl crate::RegisterSpec for FMSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fmstat::R](R) reader structure"]
impl crate::Readable for FMSTAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fmstat::W](W) writer structure"]
impl crate::Writable for FMSTAT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FMSTAT to value 0"]
impl crate::Resettable for FMSTAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
