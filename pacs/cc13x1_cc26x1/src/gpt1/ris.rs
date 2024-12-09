#[doc = "Register `RIS` reader"]
pub type R = crate::R<RisSpec>;
#[doc = "Register `RIS` writer"]
pub type W = crate::W<RisSpec>;
#[doc = "Field `TATORIS` reader - 0:0\\]
GPT Timer A Time-out Raw Interrupt 0: Timer A has not timed out 1: Timer A has timed out. This interrupt is asserted when a one-shot or periodic mode timer reaches its count limit. The count limit is 0 or the value loaded into TAILR, depending on the count direction."]
pub type TatorisR = crate::BitReader;
#[doc = "Field `CAMRIS` reader - 1:1\\]
GPT Timer A Capture Mode Match Raw Interrupt 0: The capture mode match for Timer A has not occurred. 1: A capture mode match has occurred for Timer A. This interrupt asserts when the values in the TAR and TAPR match the values in the TAMATCHR and TAPMR when configured in Input Edge-Time mode. This bit is cleared by writing a 1 to the ICLR.CAMCINT bit."]
pub type CamrisR = crate::BitReader;
#[doc = "Field `CAERIS` reader - 2:2\\]
GPT Timer A Capture Mode Event Raw Interrupt 0: The event has not occured. 1: The event has occured. This interrupt asserts when the subtimer is configured in Input Edge-Time mode"]
pub type CaerisR = crate::BitReader;
#[doc = "Field `RESERVED3` reader - 3:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `TAMRIS` reader - 4:4\\]
GPT Timer A Match Raw Interrupt 0: The match value has not been reached 1: The match value is reached. TAMR.TAMIE is set, and the match values in TAMATCHR and optionally TAPMR have been reached when configured in one-shot or periodic mode."]
pub type TamrisR = crate::BitReader;
#[doc = "Field `DMAARIS` reader - 5:5\\]
GPT Timer A DMA Done Raw Interrupt Status 0: Transfer has not completed 1: Transfer has completed"]
pub type DmaarisR = crate::BitReader;
#[doc = "Field `RESERVED6` reader - 7:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved6R = crate::FieldReader;
#[doc = "Field `TBTORIS` reader - 8:8\\]
GPT Timer B Time-out Raw Interrupt 0: Timer B has not timed out 1: Timer B has timed out. This interrupt is asserted when a one-shot or periodic mode timer reaches its count limit. The count limit is 0 or the value loaded into TBILR, depending on the count direction."]
pub type TbtorisR = crate::BitReader;
#[doc = "Field `CBMRIS` reader - 9:9\\]
GPT Timer B Capture Mode Match Raw Interrupt 0: The capture mode match for Timer B has not occurred. 1: A capture mode match has occurred for Timer B. This interrupt asserts when the values in the TBR and TBPR match the values in the TBMATCHR and TBPMR when configured in Input Edge-Time mode. This bit is cleared by writing a 1 to the ICLR.CBMCINT bit."]
pub type CbmrisR = crate::BitReader;
#[doc = "Field `CBERIS` reader - 10:10\\]
GPT Timer B Capture Mode Event Raw Interrupt 0: The event has not occured. 1: The event has occured. This interrupt asserts when the subtimer is configured in Input Edge-Time mode"]
pub type CberisR = crate::BitReader;
#[doc = "Field `TBMRIS` reader - 11:11\\]
GPT Timer B Match Raw Interrupt 0: The match value has not been reached 1: The match value is reached. TBMR.TBMIE is set, and the match values in TBMATCHR and optionally TBPMR have been reached when configured in one-shot or periodic mode."]
pub type TbmrisR = crate::BitReader;
#[doc = "Field `RESERVED12` reader - 12:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved12R = crate::BitReader;
#[doc = "Field `DMABRIS` reader - 13:13\\]
GPT Timer B DMA Done Raw Interrupt Status 0: Transfer has not completed 1: Transfer has completed"]
pub type DmabrisR = crate::BitReader;
#[doc = "Field `RESERVED14` reader - 31:14\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved14R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
GPT Timer A Time-out Raw Interrupt 0: Timer A has not timed out 1: Timer A has timed out. This interrupt is asserted when a one-shot or periodic mode timer reaches its count limit. The count limit is 0 or the value loaded into TAILR, depending on the count direction."]
    #[inline(always)]
    pub fn tatoris(&self) -> TatorisR {
        TatorisR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
GPT Timer A Capture Mode Match Raw Interrupt 0: The capture mode match for Timer A has not occurred. 1: A capture mode match has occurred for Timer A. This interrupt asserts when the values in the TAR and TAPR match the values in the TAMATCHR and TAPMR when configured in Input Edge-Time mode. This bit is cleared by writing a 1 to the ICLR.CAMCINT bit."]
    #[inline(always)]
    pub fn camris(&self) -> CamrisR {
        CamrisR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
GPT Timer A Capture Mode Event Raw Interrupt 0: The event has not occured. 1: The event has occured. This interrupt asserts when the subtimer is configured in Input Edge-Time mode"]
    #[inline(always)]
    pub fn caeris(&self) -> CaerisR {
        CaerisR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
GPT Timer A Match Raw Interrupt 0: The match value has not been reached 1: The match value is reached. TAMR.TAMIE is set, and the match values in TAMATCHR and optionally TAPMR have been reached when configured in one-shot or periodic mode."]
    #[inline(always)]
    pub fn tamris(&self) -> TamrisR {
        TamrisR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
GPT Timer A DMA Done Raw Interrupt Status 0: Transfer has not completed 1: Transfer has completed"]
    #[inline(always)]
    pub fn dmaaris(&self) -> DmaarisR {
        DmaarisR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
GPT Timer B Time-out Raw Interrupt 0: Timer B has not timed out 1: Timer B has timed out. This interrupt is asserted when a one-shot or periodic mode timer reaches its count limit. The count limit is 0 or the value loaded into TBILR, depending on the count direction."]
    #[inline(always)]
    pub fn tbtoris(&self) -> TbtorisR {
        TbtorisR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
GPT Timer B Capture Mode Match Raw Interrupt 0: The capture mode match for Timer B has not occurred. 1: A capture mode match has occurred for Timer B. This interrupt asserts when the values in the TBR and TBPR match the values in the TBMATCHR and TBPMR when configured in Input Edge-Time mode. This bit is cleared by writing a 1 to the ICLR.CBMCINT bit."]
    #[inline(always)]
    pub fn cbmris(&self) -> CbmrisR {
        CbmrisR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
GPT Timer B Capture Mode Event Raw Interrupt 0: The event has not occured. 1: The event has occured. This interrupt asserts when the subtimer is configured in Input Edge-Time mode"]
    #[inline(always)]
    pub fn cberis(&self) -> CberisR {
        CberisR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
GPT Timer B Match Raw Interrupt 0: The match value has not been reached 1: The match value is reached. TBMR.TBMIE is set, and the match values in TBMATCHR and optionally TBPMR have been reached when configured in one-shot or periodic mode."]
    #[inline(always)]
    pub fn tbmris(&self) -> TbmrisR {
        TbmrisR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved12(&self) -> Reserved12R {
        Reserved12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
GPT Timer B DMA Done Raw Interrupt Status 0: Transfer has not completed 1: Transfer has completed"]
    #[inline(always)]
    pub fn dmabris(&self) -> DmabrisR {
        DmabrisR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:31 - 31:14\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved14(&self) -> Reserved14R {
        Reserved14R::new((self.bits >> 14) & 0x0003_ffff)
    }
}
impl W {}
#[doc = "Raw Interrupt Status Associated registers: IMR, MIS, ICLR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ris::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ris::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RisSpec;
impl crate::RegisterSpec for RisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ris::R`](R) reader structure"]
impl crate::Readable for RisSpec {}
#[doc = "`write(|w| ..)` method takes [`ris::W`](W) writer structure"]
impl crate::Writable for RisSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RIS to value 0"]
impl crate::Resettable for RisSpec {
    const RESET_VALUE: u32 = 0;
}
