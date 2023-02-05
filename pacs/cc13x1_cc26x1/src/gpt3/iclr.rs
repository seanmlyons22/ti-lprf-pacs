#[doc = "Register `ICLR` reader"]
pub struct R(crate::R<ICLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ICLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ICLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ICLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ICLR` writer"]
pub struct W(crate::W<ICLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICLR_SPEC>;
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
impl From<crate::W<ICLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TATOCINT` reader - 0:0\\]
0: Do nothing. 1: Clear RIS.TATORIS and MIS.TATOMIS"]
pub type TATOCINT_R = crate::BitReader<bool>;
#[doc = "Field `TATOCINT` writer - 0:0\\]
0: Do nothing. 1: Clear RIS.TATORIS and MIS.TATOMIS"]
pub type TATOCINT_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICLR_SPEC, bool, O>;
#[doc = "Field `CAMCINT` reader - 1:1\\]
0: Do nothing. 1: Clear RIS.CAMRIS and MIS.CAMMIS"]
pub type CAMCINT_R = crate::BitReader<bool>;
#[doc = "Field `CAMCINT` writer - 1:1\\]
0: Do nothing. 1: Clear RIS.CAMRIS and MIS.CAMMIS"]
pub type CAMCINT_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICLR_SPEC, bool, O>;
#[doc = "Field `CAECINT` reader - 2:2\\]
0: Do nothing. 1: Clear RIS.CAERIS and MIS.CAEMIS"]
pub type CAECINT_R = crate::BitReader<bool>;
#[doc = "Field `CAECINT` writer - 2:2\\]
0: Do nothing. 1: Clear RIS.CAERIS and MIS.CAEMIS"]
pub type CAECINT_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICLR_SPEC, bool, O>;
#[doc = "Field `RESERVED3` reader - 3:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED3_R = crate::BitReader<bool>;
#[doc = "Field `RESERVED3` writer - 3:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED3_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICLR_SPEC, bool, O>;
#[doc = "Field `TAMCINT` reader - 4:4\\]
0: Do nothing. 1: Clear RIS.TAMRIS and MIS.TAMMIS"]
pub type TAMCINT_R = crate::BitReader<bool>;
#[doc = "Field `TAMCINT` writer - 4:4\\]
0: Do nothing. 1: Clear RIS.TAMRIS and MIS.TAMMIS"]
pub type TAMCINT_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICLR_SPEC, bool, O>;
#[doc = "Field `DMAAINT` reader - 5:5\\]
0: Do nothing. 1: Clear RIS.DMAARIS and MIS.DMAAMIS"]
pub type DMAAINT_R = crate::BitReader<bool>;
#[doc = "Field `DMAAINT` writer - 5:5\\]
0: Do nothing. 1: Clear RIS.DMAARIS and MIS.DMAAMIS"]
pub type DMAAINT_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICLR_SPEC, bool, O>;
#[doc = "Field `RESERVED6` reader - 7:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED6` writer - 7:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED6_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ICLR_SPEC, u8, u8, 2, O>;
#[doc = "Field `TBTOCINT` reader - 8:8\\]
0: Do nothing. 1: Clear RIS.TBTORIS and MIS.TBTOMIS"]
pub type TBTOCINT_R = crate::BitReader<bool>;
#[doc = "Field `TBTOCINT` writer - 8:8\\]
0: Do nothing. 1: Clear RIS.TBTORIS and MIS.TBTOMIS"]
pub type TBTOCINT_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICLR_SPEC, bool, O>;
#[doc = "Field `CBMCINT` reader - 9:9\\]
0: Do nothing. 1: Clear RIS.CBMRIS and MIS.CBMMIS"]
pub type CBMCINT_R = crate::BitReader<bool>;
#[doc = "Field `CBMCINT` writer - 9:9\\]
0: Do nothing. 1: Clear RIS.CBMRIS and MIS.CBMMIS"]
pub type CBMCINT_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICLR_SPEC, bool, O>;
#[doc = "Field `CBECINT` reader - 10:10\\]
0: Do nothing. 1: Clear RIS.CBERIS and MIS.CBEMIS"]
pub type CBECINT_R = crate::BitReader<bool>;
#[doc = "Field `CBECINT` writer - 10:10\\]
0: Do nothing. 1: Clear RIS.CBERIS and MIS.CBEMIS"]
pub type CBECINT_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICLR_SPEC, bool, O>;
#[doc = "Field `TBMCINT` reader - 11:11\\]
0: Do nothing. 1: Clear RIS.TBMRIS and MIS.TBMMIS"]
pub type TBMCINT_R = crate::BitReader<bool>;
#[doc = "Field `TBMCINT` writer - 11:11\\]
0: Do nothing. 1: Clear RIS.TBMRIS and MIS.TBMMIS"]
pub type TBMCINT_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICLR_SPEC, bool, O>;
#[doc = "Field `RESERVED12` reader - 12:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED12_R = crate::BitReader<bool>;
#[doc = "Field `RESERVED12` writer - 12:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED12_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICLR_SPEC, bool, O>;
#[doc = "Field `DMABINT` reader - 13:13\\]
0: Do nothing. 1: Clear RIS.DMABRIS and MIS.DMABMIS"]
pub type DMABINT_R = crate::BitReader<bool>;
#[doc = "Field `DMABINT` writer - 13:13\\]
0: Do nothing. 1: Clear RIS.DMABRIS and MIS.DMABMIS"]
pub type DMABINT_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICLR_SPEC, bool, O>;
#[doc = "Field `RESERVED14` reader - 31:14\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED14_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED14` writer - 31:14\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED14_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ICLR_SPEC, u32, u32, 18, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
0: Do nothing. 1: Clear RIS.TATORIS and MIS.TATOMIS"]
    #[inline(always)]
    pub fn tatocint(&self) -> TATOCINT_R {
        TATOCINT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
0: Do nothing. 1: Clear RIS.CAMRIS and MIS.CAMMIS"]
    #[inline(always)]
    pub fn camcint(&self) -> CAMCINT_R {
        CAMCINT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
0: Do nothing. 1: Clear RIS.CAERIS and MIS.CAEMIS"]
    #[inline(always)]
    pub fn caecint(&self) -> CAECINT_R {
        CAECINT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> RESERVED3_R {
        RESERVED3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
0: Do nothing. 1: Clear RIS.TAMRIS and MIS.TAMMIS"]
    #[inline(always)]
    pub fn tamcint(&self) -> TAMCINT_R {
        TAMCINT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
0: Do nothing. 1: Clear RIS.DMAARIS and MIS.DMAAMIS"]
    #[inline(always)]
    pub fn dmaaint(&self) -> DMAAINT_R {
        DMAAINT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved6(&self) -> RESERVED6_R {
        RESERVED6_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
0: Do nothing. 1: Clear RIS.TBTORIS and MIS.TBTOMIS"]
    #[inline(always)]
    pub fn tbtocint(&self) -> TBTOCINT_R {
        TBTOCINT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
0: Do nothing. 1: Clear RIS.CBMRIS and MIS.CBMMIS"]
    #[inline(always)]
    pub fn cbmcint(&self) -> CBMCINT_R {
        CBMCINT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
0: Do nothing. 1: Clear RIS.CBERIS and MIS.CBEMIS"]
    #[inline(always)]
    pub fn cbecint(&self) -> CBECINT_R {
        CBECINT_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
0: Do nothing. 1: Clear RIS.TBMRIS and MIS.TBMMIS"]
    #[inline(always)]
    pub fn tbmcint(&self) -> TBMCINT_R {
        TBMCINT_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved12(&self) -> RESERVED12_R {
        RESERVED12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
0: Do nothing. 1: Clear RIS.DMABRIS and MIS.DMABMIS"]
    #[inline(always)]
    pub fn dmabint(&self) -> DMABINT_R {
        DMABINT_R::new(((self.bits >> 13) & 1) != 0)
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
0: Do nothing. 1: Clear RIS.TATORIS and MIS.TATOMIS"]
    #[inline(always)]
    #[must_use]
    pub fn tatocint(&mut self) -> TATOCINT_W<0> {
        TATOCINT_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
0: Do nothing. 1: Clear RIS.CAMRIS and MIS.CAMMIS"]
    #[inline(always)]
    #[must_use]
    pub fn camcint(&mut self) -> CAMCINT_W<1> {
        CAMCINT_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
0: Do nothing. 1: Clear RIS.CAERIS and MIS.CAEMIS"]
    #[inline(always)]
    #[must_use]
    pub fn caecint(&mut self) -> CAECINT_W<2> {
        CAECINT_W::new(self)
    }
    #[doc = "Bit 3 - 3:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> RESERVED3_W<3> {
        RESERVED3_W::new(self)
    }
    #[doc = "Bit 4 - 4:4\\]
0: Do nothing. 1: Clear RIS.TAMRIS and MIS.TAMMIS"]
    #[inline(always)]
    #[must_use]
    pub fn tamcint(&mut self) -> TAMCINT_W<4> {
        TAMCINT_W::new(self)
    }
    #[doc = "Bit 5 - 5:5\\]
0: Do nothing. 1: Clear RIS.DMAARIS and MIS.DMAAMIS"]
    #[inline(always)]
    #[must_use]
    pub fn dmaaint(&mut self) -> DMAAINT_W<5> {
        DMAAINT_W::new(self)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved6(&mut self) -> RESERVED6_W<6> {
        RESERVED6_W::new(self)
    }
    #[doc = "Bit 8 - 8:8\\]
0: Do nothing. 1: Clear RIS.TBTORIS and MIS.TBTOMIS"]
    #[inline(always)]
    #[must_use]
    pub fn tbtocint(&mut self) -> TBTOCINT_W<8> {
        TBTOCINT_W::new(self)
    }
    #[doc = "Bit 9 - 9:9\\]
0: Do nothing. 1: Clear RIS.CBMRIS and MIS.CBMMIS"]
    #[inline(always)]
    #[must_use]
    pub fn cbmcint(&mut self) -> CBMCINT_W<9> {
        CBMCINT_W::new(self)
    }
    #[doc = "Bit 10 - 10:10\\]
0: Do nothing. 1: Clear RIS.CBERIS and MIS.CBEMIS"]
    #[inline(always)]
    #[must_use]
    pub fn cbecint(&mut self) -> CBECINT_W<10> {
        CBECINT_W::new(self)
    }
    #[doc = "Bit 11 - 11:11\\]
0: Do nothing. 1: Clear RIS.TBMRIS and MIS.TBMMIS"]
    #[inline(always)]
    #[must_use]
    pub fn tbmcint(&mut self) -> TBMCINT_W<11> {
        TBMCINT_W::new(self)
    }
    #[doc = "Bit 12 - 12:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved12(&mut self) -> RESERVED12_W<12> {
        RESERVED12_W::new(self)
    }
    #[doc = "Bit 13 - 13:13\\]
0: Do nothing. 1: Clear RIS.DMABRIS and MIS.DMABMIS"]
    #[inline(always)]
    #[must_use]
    pub fn dmabint(&mut self) -> DMABINT_W<13> {
        DMABINT_W::new(self)
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
#[doc = "Interrupt Clear This register is used to clear status bits in the RIS and MIS registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iclr](index.html) module"]
pub struct ICLR_SPEC;
impl crate::RegisterSpec for ICLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iclr::R](R) reader structure"]
impl crate::Readable for ICLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iclr::W](W) writer structure"]
impl crate::Writable for ICLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ICLR to value 0"]
impl crate::Resettable for ICLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
