#[doc = "Register `MIS` reader"]
pub struct R(crate::R<MIS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MIS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MIS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MIS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MIS` writer"]
pub struct W(crate::W<MIS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MIS_SPEC>;
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
impl From<crate::W<MIS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MIS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TATOMIS` reader - 0:0\\]
0: No interrupt or interrupt not enabled 1: RIS.TATORIS = 1 && IMR.TATOIM = 1"]
pub type TATOMIS_R = crate::BitReader<bool>;
#[doc = "Field `TATOMIS` writer - 0:0\\]
0: No interrupt or interrupt not enabled 1: RIS.TATORIS = 1 && IMR.TATOIM = 1"]
pub type TATOMIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MIS_SPEC, bool, O>;
#[doc = "Field `CAMMIS` reader - 1:1\\]
0: No interrupt or interrupt not enabled 1: RIS.CAMRIS = 1 && IMR.CAMIM = 1"]
pub type CAMMIS_R = crate::BitReader<bool>;
#[doc = "Field `CAMMIS` writer - 1:1\\]
0: No interrupt or interrupt not enabled 1: RIS.CAMRIS = 1 && IMR.CAMIM = 1"]
pub type CAMMIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MIS_SPEC, bool, O>;
#[doc = "Field `CAEMIS` reader - 2:2\\]
0: No interrupt or interrupt not enabled 1: RIS.CAERIS = 1 && IMR.CAEIM = 1"]
pub type CAEMIS_R = crate::BitReader<bool>;
#[doc = "Field `CAEMIS` writer - 2:2\\]
0: No interrupt or interrupt not enabled 1: RIS.CAERIS = 1 && IMR.CAEIM = 1"]
pub type CAEMIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MIS_SPEC, bool, O>;
#[doc = "Field `RESERVED3` reader - 3:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED3_R = crate::BitReader<bool>;
#[doc = "Field `RESERVED3` writer - 3:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED3_W<'a, const O: u8> = crate::BitWriter<'a, u32, MIS_SPEC, bool, O>;
#[doc = "Field `TAMMIS` reader - 4:4\\]
0: No interrupt or interrupt not enabled 1: RIS.TAMRIS = 1 && IMR.TAMIM = 1"]
pub type TAMMIS_R = crate::BitReader<bool>;
#[doc = "Field `TAMMIS` writer - 4:4\\]
0: No interrupt or interrupt not enabled 1: RIS.TAMRIS = 1 && IMR.TAMIM = 1"]
pub type TAMMIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MIS_SPEC, bool, O>;
#[doc = "Field `DMAAMIS` reader - 5:5\\]
0: No interrupt or interrupt not enabled 1: RIS.DMAARIS = 1 && IMR.DMAAIM = 1"]
pub type DMAAMIS_R = crate::BitReader<bool>;
#[doc = "Field `DMAAMIS` writer - 5:5\\]
0: No interrupt or interrupt not enabled 1: RIS.DMAARIS = 1 && IMR.DMAAIM = 1"]
pub type DMAAMIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MIS_SPEC, bool, O>;
#[doc = "Field `RESERVED6` reader - 7:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED6` writer - 7:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED6_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MIS_SPEC, u8, u8, 2, O>;
#[doc = "Field `TBTOMIS` reader - 8:8\\]
0: No interrupt or interrupt not enabled 1: RIS.TBTORIS = 1 && IMR.TBTOIM = 1"]
pub type TBTOMIS_R = crate::BitReader<bool>;
#[doc = "Field `TBTOMIS` writer - 8:8\\]
0: No interrupt or interrupt not enabled 1: RIS.TBTORIS = 1 && IMR.TBTOIM = 1"]
pub type TBTOMIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MIS_SPEC, bool, O>;
#[doc = "Field `CBMMIS` reader - 9:9\\]
0: No interrupt or interrupt not enabled 1: RIS.CBMRIS = 1 && IMR.CBMIM = 1"]
pub type CBMMIS_R = crate::BitReader<bool>;
#[doc = "Field `CBMMIS` writer - 9:9\\]
0: No interrupt or interrupt not enabled 1: RIS.CBMRIS = 1 && IMR.CBMIM = 1"]
pub type CBMMIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MIS_SPEC, bool, O>;
#[doc = "Field `CBEMIS` reader - 10:10\\]
0: No interrupt or interrupt not enabled 1: RIS.CBERIS = 1 && IMR.CBEIM = 1"]
pub type CBEMIS_R = crate::BitReader<bool>;
#[doc = "Field `CBEMIS` writer - 10:10\\]
0: No interrupt or interrupt not enabled 1: RIS.CBERIS = 1 && IMR.CBEIM = 1"]
pub type CBEMIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MIS_SPEC, bool, O>;
#[doc = "Field `TBMMIS` reader - 11:11\\]
0: No interrupt or interrupt not enabled 1: RIS.TBMRIS = 1 && IMR.TBMIM = 1"]
pub type TBMMIS_R = crate::BitReader<bool>;
#[doc = "Field `TBMMIS` writer - 11:11\\]
0: No interrupt or interrupt not enabled 1: RIS.TBMRIS = 1 && IMR.TBMIM = 1"]
pub type TBMMIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MIS_SPEC, bool, O>;
#[doc = "Field `RESERVED12` reader - 12:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED12_R = crate::BitReader<bool>;
#[doc = "Field `RESERVED12` writer - 12:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED12_W<'a, const O: u8> = crate::BitWriter<'a, u32, MIS_SPEC, bool, O>;
#[doc = "Field `DMABMIS` reader - 13:13\\]
0: No interrupt or interrupt not enabled 1: RIS.DMABRIS = 1 && IMR.DMABIM = 1"]
pub type DMABMIS_R = crate::BitReader<bool>;
#[doc = "Field `DMABMIS` writer - 13:13\\]
0: No interrupt or interrupt not enabled 1: RIS.DMABRIS = 1 && IMR.DMABIM = 1"]
pub type DMABMIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MIS_SPEC, bool, O>;
#[doc = "Field `RESERVED14` reader - 31:14\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED14_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED14` writer - 31:14\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED14_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MIS_SPEC, u32, u32, 18, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
0: No interrupt or interrupt not enabled 1: RIS.TATORIS = 1 && IMR.TATOIM = 1"]
    #[inline(always)]
    pub fn tatomis(&self) -> TATOMIS_R {
        TATOMIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
0: No interrupt or interrupt not enabled 1: RIS.CAMRIS = 1 && IMR.CAMIM = 1"]
    #[inline(always)]
    pub fn cammis(&self) -> CAMMIS_R {
        CAMMIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
0: No interrupt or interrupt not enabled 1: RIS.CAERIS = 1 && IMR.CAEIM = 1"]
    #[inline(always)]
    pub fn caemis(&self) -> CAEMIS_R {
        CAEMIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> RESERVED3_R {
        RESERVED3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
0: No interrupt or interrupt not enabled 1: RIS.TAMRIS = 1 && IMR.TAMIM = 1"]
    #[inline(always)]
    pub fn tammis(&self) -> TAMMIS_R {
        TAMMIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
0: No interrupt or interrupt not enabled 1: RIS.DMAARIS = 1 && IMR.DMAAIM = 1"]
    #[inline(always)]
    pub fn dmaamis(&self) -> DMAAMIS_R {
        DMAAMIS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved6(&self) -> RESERVED6_R {
        RESERVED6_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
0: No interrupt or interrupt not enabled 1: RIS.TBTORIS = 1 && IMR.TBTOIM = 1"]
    #[inline(always)]
    pub fn tbtomis(&self) -> TBTOMIS_R {
        TBTOMIS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
0: No interrupt or interrupt not enabled 1: RIS.CBMRIS = 1 && IMR.CBMIM = 1"]
    #[inline(always)]
    pub fn cbmmis(&self) -> CBMMIS_R {
        CBMMIS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
0: No interrupt or interrupt not enabled 1: RIS.CBERIS = 1 && IMR.CBEIM = 1"]
    #[inline(always)]
    pub fn cbemis(&self) -> CBEMIS_R {
        CBEMIS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
0: No interrupt or interrupt not enabled 1: RIS.TBMRIS = 1 && IMR.TBMIM = 1"]
    #[inline(always)]
    pub fn tbmmis(&self) -> TBMMIS_R {
        TBMMIS_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved12(&self) -> RESERVED12_R {
        RESERVED12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
0: No interrupt or interrupt not enabled 1: RIS.DMABRIS = 1 && IMR.DMABIM = 1"]
    #[inline(always)]
    pub fn dmabmis(&self) -> DMABMIS_R {
        DMABMIS_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:31 - 31:14\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved14(&self) -> RESERVED14_R {
        RESERVED14_R::new((self.bits >> 14) & 0x0003_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
0: No interrupt or interrupt not enabled 1: RIS.TATORIS = 1 && IMR.TATOIM = 1"]
    #[inline(always)]
    #[must_use]
    pub fn tatomis(&mut self) -> TATOMIS_W<0> {
        TATOMIS_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
0: No interrupt or interrupt not enabled 1: RIS.CAMRIS = 1 && IMR.CAMIM = 1"]
    #[inline(always)]
    #[must_use]
    pub fn cammis(&mut self) -> CAMMIS_W<1> {
        CAMMIS_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
0: No interrupt or interrupt not enabled 1: RIS.CAERIS = 1 && IMR.CAEIM = 1"]
    #[inline(always)]
    #[must_use]
    pub fn caemis(&mut self) -> CAEMIS_W<2> {
        CAEMIS_W::new(self)
    }
    #[doc = "Bit 3 - 3:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> RESERVED3_W<3> {
        RESERVED3_W::new(self)
    }
    #[doc = "Bit 4 - 4:4\\]
0: No interrupt or interrupt not enabled 1: RIS.TAMRIS = 1 && IMR.TAMIM = 1"]
    #[inline(always)]
    #[must_use]
    pub fn tammis(&mut self) -> TAMMIS_W<4> {
        TAMMIS_W::new(self)
    }
    #[doc = "Bit 5 - 5:5\\]
0: No interrupt or interrupt not enabled 1: RIS.DMAARIS = 1 && IMR.DMAAIM = 1"]
    #[inline(always)]
    #[must_use]
    pub fn dmaamis(&mut self) -> DMAAMIS_W<5> {
        DMAAMIS_W::new(self)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved6(&mut self) -> RESERVED6_W<6> {
        RESERVED6_W::new(self)
    }
    #[doc = "Bit 8 - 8:8\\]
0: No interrupt or interrupt not enabled 1: RIS.TBTORIS = 1 && IMR.TBTOIM = 1"]
    #[inline(always)]
    #[must_use]
    pub fn tbtomis(&mut self) -> TBTOMIS_W<8> {
        TBTOMIS_W::new(self)
    }
    #[doc = "Bit 9 - 9:9\\]
0: No interrupt or interrupt not enabled 1: RIS.CBMRIS = 1 && IMR.CBMIM = 1"]
    #[inline(always)]
    #[must_use]
    pub fn cbmmis(&mut self) -> CBMMIS_W<9> {
        CBMMIS_W::new(self)
    }
    #[doc = "Bit 10 - 10:10\\]
0: No interrupt or interrupt not enabled 1: RIS.CBERIS = 1 && IMR.CBEIM = 1"]
    #[inline(always)]
    #[must_use]
    pub fn cbemis(&mut self) -> CBEMIS_W<10> {
        CBEMIS_W::new(self)
    }
    #[doc = "Bit 11 - 11:11\\]
0: No interrupt or interrupt not enabled 1: RIS.TBMRIS = 1 && IMR.TBMIM = 1"]
    #[inline(always)]
    #[must_use]
    pub fn tbmmis(&mut self) -> TBMMIS_W<11> {
        TBMMIS_W::new(self)
    }
    #[doc = "Bit 12 - 12:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved12(&mut self) -> RESERVED12_W<12> {
        RESERVED12_W::new(self)
    }
    #[doc = "Bit 13 - 13:13\\]
0: No interrupt or interrupt not enabled 1: RIS.DMABRIS = 1 && IMR.DMABIM = 1"]
    #[inline(always)]
    #[must_use]
    pub fn dmabmis(&mut self) -> DMABMIS_W<13> {
        DMABMIS_W::new(self)
    }
    #[doc = "Bits 14:31 - 31:14\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved14(&mut self) -> RESERVED14_W<14> {
        RESERVED14_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Masked Interrupt Status Values are result of bitwise AND operation between RIS and IMR Assosciated clear register: ICLR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mis](index.html) module"]
pub struct MIS_SPEC;
impl crate::RegisterSpec for MIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mis::R](R) reader structure"]
impl crate::Readable for MIS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mis::W](W) writer structure"]
impl crate::Writable for MIS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MIS to value 0"]
impl crate::Resettable for MIS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
