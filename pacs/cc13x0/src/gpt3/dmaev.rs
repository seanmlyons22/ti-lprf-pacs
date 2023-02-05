#[doc = "Register `DMAEV` reader"]
pub struct R(crate::R<DMAEV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMAEV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMAEV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMAEV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMAEV` writer"]
pub struct W(crate::W<DMAEV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMAEV_SPEC>;
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
impl From<crate::W<DMAEV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMAEV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TATODMAEN` reader - 0:0\\]
GPT Timer A Time-Out DMA Trigger Enable"]
pub type TATODMAEN_R = crate::BitReader<bool>;
#[doc = "Field `TATODMAEN` writer - 0:0\\]
GPT Timer A Time-Out DMA Trigger Enable"]
pub type TATODMAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAEV_SPEC, bool, O>;
#[doc = "Field `CAMDMAEN` reader - 1:1\\]
GPT Timer A Capture Match DMA Trigger Enable"]
pub type CAMDMAEN_R = crate::BitReader<bool>;
#[doc = "Field `CAMDMAEN` writer - 1:1\\]
GPT Timer A Capture Match DMA Trigger Enable"]
pub type CAMDMAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAEV_SPEC, bool, O>;
#[doc = "Field `CAEDMAEN` reader - 2:2\\]
GPT Timer A Capture Event DMA Trigger Enable"]
pub type CAEDMAEN_R = crate::BitReader<bool>;
#[doc = "Field `CAEDMAEN` writer - 2:2\\]
GPT Timer A Capture Event DMA Trigger Enable"]
pub type CAEDMAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAEV_SPEC, bool, O>;
#[doc = "Field `RESERVED3` reader - 3:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED3_R = crate::BitReader<bool>;
#[doc = "Field `RESERVED3` writer - 3:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED3_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAEV_SPEC, bool, O>;
#[doc = "Field `TAMDMAEN` reader - 4:4\\]
GPT Timer A Match DMA Trigger Enable"]
pub type TAMDMAEN_R = crate::BitReader<bool>;
#[doc = "Field `TAMDMAEN` writer - 4:4\\]
GPT Timer A Match DMA Trigger Enable"]
pub type TAMDMAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAEV_SPEC, bool, O>;
#[doc = "Field `RESERVED5` reader - 7:5\\]
Software should not rely on the value of a reserved field. Writing any other value may result in undefined behavior."]
pub type RESERVED5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED5` writer - 7:5\\]
Software should not rely on the value of a reserved field. Writing any other value may result in undefined behavior."]
pub type RESERVED5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMAEV_SPEC, u8, u8, 3, O>;
#[doc = "Field `TBTODMAEN` reader - 8:8\\]
GPT Timer B Time-Out DMA Trigger Enable"]
pub type TBTODMAEN_R = crate::BitReader<bool>;
#[doc = "Field `TBTODMAEN` writer - 8:8\\]
GPT Timer B Time-Out DMA Trigger Enable"]
pub type TBTODMAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAEV_SPEC, bool, O>;
#[doc = "Field `CBMDMAEN` reader - 9:9\\]
GPT Timer B Capture Match DMA Trigger Enable"]
pub type CBMDMAEN_R = crate::BitReader<bool>;
#[doc = "Field `CBMDMAEN` writer - 9:9\\]
GPT Timer B Capture Match DMA Trigger Enable"]
pub type CBMDMAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAEV_SPEC, bool, O>;
#[doc = "Field `CBEDMAEN` reader - 10:10\\]
GPT Timer B Capture Event DMA Trigger Enable"]
pub type CBEDMAEN_R = crate::BitReader<bool>;
#[doc = "Field `CBEDMAEN` writer - 10:10\\]
GPT Timer B Capture Event DMA Trigger Enable"]
pub type CBEDMAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAEV_SPEC, bool, O>;
#[doc = "Field `TBMDMAEN` reader - 11:11\\]
GPT Timer B Match DMA Trigger Enable"]
pub type TBMDMAEN_R = crate::BitReader<bool>;
#[doc = "Field `TBMDMAEN` writer - 11:11\\]
GPT Timer B Match DMA Trigger Enable"]
pub type TBMDMAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAEV_SPEC, bool, O>;
#[doc = "Field `RESERVED12` reader - 31:12\\]
Software should not rely on the value of a reserved field. Writing any other value may result in undefined behavior."]
pub type RESERVED12_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED12` writer - 31:12\\]
Software should not rely on the value of a reserved field. Writing any other value may result in undefined behavior."]
pub type RESERVED12_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMAEV_SPEC, u32, u32, 20, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
GPT Timer A Time-Out DMA Trigger Enable"]
    #[inline(always)]
    pub fn tatodmaen(&self) -> TATODMAEN_R {
        TATODMAEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
GPT Timer A Capture Match DMA Trigger Enable"]
    #[inline(always)]
    pub fn camdmaen(&self) -> CAMDMAEN_R {
        CAMDMAEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
GPT Timer A Capture Event DMA Trigger Enable"]
    #[inline(always)]
    pub fn caedmaen(&self) -> CAEDMAEN_R {
        CAEDMAEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> RESERVED3_R {
        RESERVED3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
GPT Timer A Match DMA Trigger Enable"]
    #[inline(always)]
    pub fn tamdmaen(&self) -> TAMDMAEN_R {
        TAMDMAEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - 7:5\\]
Software should not rely on the value of a reserved field. Writing any other value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved5(&self) -> RESERVED5_R {
        RESERVED5_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
GPT Timer B Time-Out DMA Trigger Enable"]
    #[inline(always)]
    pub fn tbtodmaen(&self) -> TBTODMAEN_R {
        TBTODMAEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
GPT Timer B Capture Match DMA Trigger Enable"]
    #[inline(always)]
    pub fn cbmdmaen(&self) -> CBMDMAEN_R {
        CBMDMAEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
GPT Timer B Capture Event DMA Trigger Enable"]
    #[inline(always)]
    pub fn cbedmaen(&self) -> CBEDMAEN_R {
        CBEDMAEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
GPT Timer B Match DMA Trigger Enable"]
    #[inline(always)]
    pub fn tbmdmaen(&self) -> TBMDMAEN_R {
        TBMDMAEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:31 - 31:12\\]
Software should not rely on the value of a reserved field. Writing any other value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved12(&self) -> RESERVED12_R {
        RESERVED12_R::new((self.bits >> 12) & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
GPT Timer A Time-Out DMA Trigger Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tatodmaen(&mut self) -> TATODMAEN_W<0> {
        TATODMAEN_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
GPT Timer A Capture Match DMA Trigger Enable"]
    #[inline(always)]
    #[must_use]
    pub fn camdmaen(&mut self) -> CAMDMAEN_W<1> {
        CAMDMAEN_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
GPT Timer A Capture Event DMA Trigger Enable"]
    #[inline(always)]
    #[must_use]
    pub fn caedmaen(&mut self) -> CAEDMAEN_W<2> {
        CAEDMAEN_W::new(self)
    }
    #[doc = "Bit 3 - 3:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> RESERVED3_W<3> {
        RESERVED3_W::new(self)
    }
    #[doc = "Bit 4 - 4:4\\]
GPT Timer A Match DMA Trigger Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tamdmaen(&mut self) -> TAMDMAEN_W<4> {
        TAMDMAEN_W::new(self)
    }
    #[doc = "Bits 5:7 - 7:5\\]
Software should not rely on the value of a reserved field. Writing any other value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved5(&mut self) -> RESERVED5_W<5> {
        RESERVED5_W::new(self)
    }
    #[doc = "Bit 8 - 8:8\\]
GPT Timer B Time-Out DMA Trigger Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tbtodmaen(&mut self) -> TBTODMAEN_W<8> {
        TBTODMAEN_W::new(self)
    }
    #[doc = "Bit 9 - 9:9\\]
GPT Timer B Capture Match DMA Trigger Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cbmdmaen(&mut self) -> CBMDMAEN_W<9> {
        CBMDMAEN_W::new(self)
    }
    #[doc = "Bit 10 - 10:10\\]
GPT Timer B Capture Event DMA Trigger Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cbedmaen(&mut self) -> CBEDMAEN_W<10> {
        CBEDMAEN_W::new(self)
    }
    #[doc = "Bit 11 - 11:11\\]
GPT Timer B Match DMA Trigger Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tbmdmaen(&mut self) -> TBMDMAEN_W<11> {
        TBMDMAEN_W::new(self)
    }
    #[doc = "Bits 12:31 - 31:12\\]
Software should not rely on the value of a reserved field. Writing any other value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved12(&mut self) -> RESERVED12_W<12> {
        RESERVED12_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Event This register allows software to enable/disable GPT DMA trigger events.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmaev](index.html) module"]
pub struct DMAEV_SPEC;
impl crate::RegisterSpec for DMAEV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmaev::R](R) reader structure"]
impl crate::Readable for DMAEV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmaev::W](W) writer structure"]
impl crate::Writable for DMAEV_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMAEV to value 0"]
impl crate::Resettable for DMAEV_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
