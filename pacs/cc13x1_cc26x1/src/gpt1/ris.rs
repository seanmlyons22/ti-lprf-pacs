#[doc = "Register `RIS` reader"]
pub struct R(crate::R<RIS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RIS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RIS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RIS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RIS` writer"]
pub struct W(crate::W<RIS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RIS_SPEC>;
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
impl From<crate::W<RIS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RIS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TATORIS` reader - 0:0\\]
GPT Timer A Time-out Raw Interrupt 0: Timer A has not timed out 1: Timer A has timed out. This interrupt is asserted when a one-shot or periodic mode timer reaches its count limit. The count limit is 0 or the value loaded into TAILR, depending on the count direction."]
pub type TATORIS_R = crate::BitReader<bool>;
#[doc = "Field `TATORIS` writer - 0:0\\]
GPT Timer A Time-out Raw Interrupt 0: Timer A has not timed out 1: Timer A has timed out. This interrupt is asserted when a one-shot or periodic mode timer reaches its count limit. The count limit is 0 or the value loaded into TAILR, depending on the count direction."]
pub type TATORIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, RIS_SPEC, bool, O>;
#[doc = "Field `CAMRIS` reader - 1:1\\]
GPT Timer A Capture Mode Match Raw Interrupt 0: The capture mode match for Timer A has not occurred. 1: A capture mode match has occurred for Timer A. This interrupt asserts when the values in the TAR and TAPR match the values in the TAMATCHR and TAPMR when configured in Input Edge-Time mode. This bit is cleared by writing a 1 to the ICLR.CAMCINT bit."]
pub type CAMRIS_R = crate::BitReader<bool>;
#[doc = "Field `CAMRIS` writer - 1:1\\]
GPT Timer A Capture Mode Match Raw Interrupt 0: The capture mode match for Timer A has not occurred. 1: A capture mode match has occurred for Timer A. This interrupt asserts when the values in the TAR and TAPR match the values in the TAMATCHR and TAPMR when configured in Input Edge-Time mode. This bit is cleared by writing a 1 to the ICLR.CAMCINT bit."]
pub type CAMRIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, RIS_SPEC, bool, O>;
#[doc = "Field `CAERIS` reader - 2:2\\]
GPT Timer A Capture Mode Event Raw Interrupt 0: The event has not occured. 1: The event has occured. This interrupt asserts when the subtimer is configured in Input Edge-Time mode"]
pub type CAERIS_R = crate::BitReader<bool>;
#[doc = "Field `CAERIS` writer - 2:2\\]
GPT Timer A Capture Mode Event Raw Interrupt 0: The event has not occured. 1: The event has occured. This interrupt asserts when the subtimer is configured in Input Edge-Time mode"]
pub type CAERIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, RIS_SPEC, bool, O>;
#[doc = "Field `RESERVED3` reader - 3:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED3_R = crate::BitReader<bool>;
#[doc = "Field `RESERVED3` writer - 3:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED3_W<'a, const O: u8> = crate::BitWriter<'a, u32, RIS_SPEC, bool, O>;
#[doc = "Field `TAMRIS` reader - 4:4\\]
GPT Timer A Match Raw Interrupt 0: The match value has not been reached 1: The match value is reached. TAMR.TAMIE is set, and the match values in TAMATCHR and optionally TAPMR have been reached when configured in one-shot or periodic mode."]
pub type TAMRIS_R = crate::BitReader<bool>;
#[doc = "Field `TAMRIS` writer - 4:4\\]
GPT Timer A Match Raw Interrupt 0: The match value has not been reached 1: The match value is reached. TAMR.TAMIE is set, and the match values in TAMATCHR and optionally TAPMR have been reached when configured in one-shot or periodic mode."]
pub type TAMRIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, RIS_SPEC, bool, O>;
#[doc = "Field `DMAARIS` reader - 5:5\\]
GPT Timer A DMA Done Raw Interrupt Status 0: Transfer has not completed 1: Transfer has completed"]
pub type DMAARIS_R = crate::BitReader<bool>;
#[doc = "Field `DMAARIS` writer - 5:5\\]
GPT Timer A DMA Done Raw Interrupt Status 0: Transfer has not completed 1: Transfer has completed"]
pub type DMAARIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, RIS_SPEC, bool, O>;
#[doc = "Field `RESERVED6` reader - 7:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED6` writer - 7:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED6_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RIS_SPEC, u8, u8, 2, O>;
#[doc = "Field `TBTORIS` reader - 8:8\\]
GPT Timer B Time-out Raw Interrupt 0: Timer B has not timed out 1: Timer B has timed out. This interrupt is asserted when a one-shot or periodic mode timer reaches its count limit. The count limit is 0 or the value loaded into TBILR, depending on the count direction."]
pub type TBTORIS_R = crate::BitReader<bool>;
#[doc = "Field `TBTORIS` writer - 8:8\\]
GPT Timer B Time-out Raw Interrupt 0: Timer B has not timed out 1: Timer B has timed out. This interrupt is asserted when a one-shot or periodic mode timer reaches its count limit. The count limit is 0 or the value loaded into TBILR, depending on the count direction."]
pub type TBTORIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, RIS_SPEC, bool, O>;
#[doc = "Field `CBMRIS` reader - 9:9\\]
GPT Timer B Capture Mode Match Raw Interrupt 0: The capture mode match for Timer B has not occurred. 1: A capture mode match has occurred for Timer B. This interrupt asserts when the values in the TBR and TBPR match the values in the TBMATCHR and TBPMR when configured in Input Edge-Time mode. This bit is cleared by writing a 1 to the ICLR.CBMCINT bit."]
pub type CBMRIS_R = crate::BitReader<bool>;
#[doc = "Field `CBMRIS` writer - 9:9\\]
GPT Timer B Capture Mode Match Raw Interrupt 0: The capture mode match for Timer B has not occurred. 1: A capture mode match has occurred for Timer B. This interrupt asserts when the values in the TBR and TBPR match the values in the TBMATCHR and TBPMR when configured in Input Edge-Time mode. This bit is cleared by writing a 1 to the ICLR.CBMCINT bit."]
pub type CBMRIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, RIS_SPEC, bool, O>;
#[doc = "Field `CBERIS` reader - 10:10\\]
GPT Timer B Capture Mode Event Raw Interrupt 0: The event has not occured. 1: The event has occured. This interrupt asserts when the subtimer is configured in Input Edge-Time mode"]
pub type CBERIS_R = crate::BitReader<bool>;
#[doc = "Field `CBERIS` writer - 10:10\\]
GPT Timer B Capture Mode Event Raw Interrupt 0: The event has not occured. 1: The event has occured. This interrupt asserts when the subtimer is configured in Input Edge-Time mode"]
pub type CBERIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, RIS_SPEC, bool, O>;
#[doc = "Field `TBMRIS` reader - 11:11\\]
GPT Timer B Match Raw Interrupt 0: The match value has not been reached 1: The match value is reached. TBMR.TBMIE is set, and the match values in TBMATCHR and optionally TBPMR have been reached when configured in one-shot or periodic mode."]
pub type TBMRIS_R = crate::BitReader<bool>;
#[doc = "Field `TBMRIS` writer - 11:11\\]
GPT Timer B Match Raw Interrupt 0: The match value has not been reached 1: The match value is reached. TBMR.TBMIE is set, and the match values in TBMATCHR and optionally TBPMR have been reached when configured in one-shot or periodic mode."]
pub type TBMRIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, RIS_SPEC, bool, O>;
#[doc = "Field `RESERVED12` reader - 12:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED12_R = crate::BitReader<bool>;
#[doc = "Field `RESERVED12` writer - 12:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED12_W<'a, const O: u8> = crate::BitWriter<'a, u32, RIS_SPEC, bool, O>;
#[doc = "Field `DMABRIS` reader - 13:13\\]
GPT Timer B DMA Done Raw Interrupt Status 0: Transfer has not completed 1: Transfer has completed"]
pub type DMABRIS_R = crate::BitReader<bool>;
#[doc = "Field `DMABRIS` writer - 13:13\\]
GPT Timer B DMA Done Raw Interrupt Status 0: Transfer has not completed 1: Transfer has completed"]
pub type DMABRIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, RIS_SPEC, bool, O>;
#[doc = "Field `RESERVED14` reader - 31:14\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED14_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED14` writer - 31:14\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED14_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RIS_SPEC, u32, u32, 18, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
GPT Timer A Time-out Raw Interrupt 0: Timer A has not timed out 1: Timer A has timed out. This interrupt is asserted when a one-shot or periodic mode timer reaches its count limit. The count limit is 0 or the value loaded into TAILR, depending on the count direction."]
    #[inline(always)]
    pub fn tatoris(&self) -> TATORIS_R {
        TATORIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
GPT Timer A Capture Mode Match Raw Interrupt 0: The capture mode match for Timer A has not occurred. 1: A capture mode match has occurred for Timer A. This interrupt asserts when the values in the TAR and TAPR match the values in the TAMATCHR and TAPMR when configured in Input Edge-Time mode. This bit is cleared by writing a 1 to the ICLR.CAMCINT bit."]
    #[inline(always)]
    pub fn camris(&self) -> CAMRIS_R {
        CAMRIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
GPT Timer A Capture Mode Event Raw Interrupt 0: The event has not occured. 1: The event has occured. This interrupt asserts when the subtimer is configured in Input Edge-Time mode"]
    #[inline(always)]
    pub fn caeris(&self) -> CAERIS_R {
        CAERIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> RESERVED3_R {
        RESERVED3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
GPT Timer A Match Raw Interrupt 0: The match value has not been reached 1: The match value is reached. TAMR.TAMIE is set, and the match values in TAMATCHR and optionally TAPMR have been reached when configured in one-shot or periodic mode."]
    #[inline(always)]
    pub fn tamris(&self) -> TAMRIS_R {
        TAMRIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
GPT Timer A DMA Done Raw Interrupt Status 0: Transfer has not completed 1: Transfer has completed"]
    #[inline(always)]
    pub fn dmaaris(&self) -> DMAARIS_R {
        DMAARIS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved6(&self) -> RESERVED6_R {
        RESERVED6_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
GPT Timer B Time-out Raw Interrupt 0: Timer B has not timed out 1: Timer B has timed out. This interrupt is asserted when a one-shot or periodic mode timer reaches its count limit. The count limit is 0 or the value loaded into TBILR, depending on the count direction."]
    #[inline(always)]
    pub fn tbtoris(&self) -> TBTORIS_R {
        TBTORIS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
GPT Timer B Capture Mode Match Raw Interrupt 0: The capture mode match for Timer B has not occurred. 1: A capture mode match has occurred for Timer B. This interrupt asserts when the values in the TBR and TBPR match the values in the TBMATCHR and TBPMR when configured in Input Edge-Time mode. This bit is cleared by writing a 1 to the ICLR.CBMCINT bit."]
    #[inline(always)]
    pub fn cbmris(&self) -> CBMRIS_R {
        CBMRIS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
GPT Timer B Capture Mode Event Raw Interrupt 0: The event has not occured. 1: The event has occured. This interrupt asserts when the subtimer is configured in Input Edge-Time mode"]
    #[inline(always)]
    pub fn cberis(&self) -> CBERIS_R {
        CBERIS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
GPT Timer B Match Raw Interrupt 0: The match value has not been reached 1: The match value is reached. TBMR.TBMIE is set, and the match values in TBMATCHR and optionally TBPMR have been reached when configured in one-shot or periodic mode."]
    #[inline(always)]
    pub fn tbmris(&self) -> TBMRIS_R {
        TBMRIS_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved12(&self) -> RESERVED12_R {
        RESERVED12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
GPT Timer B DMA Done Raw Interrupt Status 0: Transfer has not completed 1: Transfer has completed"]
    #[inline(always)]
    pub fn dmabris(&self) -> DMABRIS_R {
        DMABRIS_R::new(((self.bits >> 13) & 1) != 0)
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
GPT Timer A Time-out Raw Interrupt 0: Timer A has not timed out 1: Timer A has timed out. This interrupt is asserted when a one-shot or periodic mode timer reaches its count limit. The count limit is 0 or the value loaded into TAILR, depending on the count direction."]
    #[inline(always)]
    #[must_use]
    pub fn tatoris(&mut self) -> TATORIS_W<0> {
        TATORIS_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
GPT Timer A Capture Mode Match Raw Interrupt 0: The capture mode match for Timer A has not occurred. 1: A capture mode match has occurred for Timer A. This interrupt asserts when the values in the TAR and TAPR match the values in the TAMATCHR and TAPMR when configured in Input Edge-Time mode. This bit is cleared by writing a 1 to the ICLR.CAMCINT bit."]
    #[inline(always)]
    #[must_use]
    pub fn camris(&mut self) -> CAMRIS_W<1> {
        CAMRIS_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
GPT Timer A Capture Mode Event Raw Interrupt 0: The event has not occured. 1: The event has occured. This interrupt asserts when the subtimer is configured in Input Edge-Time mode"]
    #[inline(always)]
    #[must_use]
    pub fn caeris(&mut self) -> CAERIS_W<2> {
        CAERIS_W::new(self)
    }
    #[doc = "Bit 3 - 3:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> RESERVED3_W<3> {
        RESERVED3_W::new(self)
    }
    #[doc = "Bit 4 - 4:4\\]
GPT Timer A Match Raw Interrupt 0: The match value has not been reached 1: The match value is reached. TAMR.TAMIE is set, and the match values in TAMATCHR and optionally TAPMR have been reached when configured in one-shot or periodic mode."]
    #[inline(always)]
    #[must_use]
    pub fn tamris(&mut self) -> TAMRIS_W<4> {
        TAMRIS_W::new(self)
    }
    #[doc = "Bit 5 - 5:5\\]
GPT Timer A DMA Done Raw Interrupt Status 0: Transfer has not completed 1: Transfer has completed"]
    #[inline(always)]
    #[must_use]
    pub fn dmaaris(&mut self) -> DMAARIS_W<5> {
        DMAARIS_W::new(self)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved6(&mut self) -> RESERVED6_W<6> {
        RESERVED6_W::new(self)
    }
    #[doc = "Bit 8 - 8:8\\]
GPT Timer B Time-out Raw Interrupt 0: Timer B has not timed out 1: Timer B has timed out. This interrupt is asserted when a one-shot or periodic mode timer reaches its count limit. The count limit is 0 or the value loaded into TBILR, depending on the count direction."]
    #[inline(always)]
    #[must_use]
    pub fn tbtoris(&mut self) -> TBTORIS_W<8> {
        TBTORIS_W::new(self)
    }
    #[doc = "Bit 9 - 9:9\\]
GPT Timer B Capture Mode Match Raw Interrupt 0: The capture mode match for Timer B has not occurred. 1: A capture mode match has occurred for Timer B. This interrupt asserts when the values in the TBR and TBPR match the values in the TBMATCHR and TBPMR when configured in Input Edge-Time mode. This bit is cleared by writing a 1 to the ICLR.CBMCINT bit."]
    #[inline(always)]
    #[must_use]
    pub fn cbmris(&mut self) -> CBMRIS_W<9> {
        CBMRIS_W::new(self)
    }
    #[doc = "Bit 10 - 10:10\\]
GPT Timer B Capture Mode Event Raw Interrupt 0: The event has not occured. 1: The event has occured. This interrupt asserts when the subtimer is configured in Input Edge-Time mode"]
    #[inline(always)]
    #[must_use]
    pub fn cberis(&mut self) -> CBERIS_W<10> {
        CBERIS_W::new(self)
    }
    #[doc = "Bit 11 - 11:11\\]
GPT Timer B Match Raw Interrupt 0: The match value has not been reached 1: The match value is reached. TBMR.TBMIE is set, and the match values in TBMATCHR and optionally TBPMR have been reached when configured in one-shot or periodic mode."]
    #[inline(always)]
    #[must_use]
    pub fn tbmris(&mut self) -> TBMRIS_W<11> {
        TBMRIS_W::new(self)
    }
    #[doc = "Bit 12 - 12:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved12(&mut self) -> RESERVED12_W<12> {
        RESERVED12_W::new(self)
    }
    #[doc = "Bit 13 - 13:13\\]
GPT Timer B DMA Done Raw Interrupt Status 0: Transfer has not completed 1: Transfer has completed"]
    #[inline(always)]
    #[must_use]
    pub fn dmabris(&mut self) -> DMABRIS_W<13> {
        DMABRIS_W::new(self)
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
#[doc = "Raw Interrupt Status Associated registers: IMR, MIS, ICLR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ris](index.html) module"]
pub struct RIS_SPEC;
impl crate::RegisterSpec for RIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ris::R](R) reader structure"]
impl crate::Readable for RIS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ris::W](W) writer structure"]
impl crate::Writable for RIS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RIS to value 0"]
impl crate::Resettable for RIS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
