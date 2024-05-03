#[doc = "Register `CTL1` reader"]
pub type R = crate::R<Ctl1Spec>;
#[doc = "Register `CTL1` writer"]
pub type W = crate::W<Ctl1Spec>;
#[doc = "Field `XOSC_HF_FAST_START` reader - 1:0\\]
Internal. Only to be used through TI provided API."]
pub type XoscHfFastStartR = crate::FieldReader;
#[doc = "Field `XOSC_HF_FAST_START` writer - 1:0\\]
Internal. Only to be used through TI provided API."]
pub type XoscHfFastStartW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SPARE2` reader - 7:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Spare2R = crate::FieldReader;
#[doc = "Field `SPARE2` writer - 7:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Spare2W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `CLK_LF_LOSS_EN` reader - 8:8\\]
Enable LF clock loss detection and hence the indicators to the system controller. Checks SCLK_LF clock loss indicators. 0: Disable 1: Enable Clock loss detection must be disabled when changing the sclk_lf source. STAT0.SCLK_LF_SRC can be polled to determine when a change to a new sclk_lf source has completed."]
pub type ClkLfLossEnR = crate::BitReader;
#[doc = "Field `CLK_LF_LOSS_EN` writer - 8:8\\]
Enable LF clock loss detection and hence the indicators to the system controller. Checks SCLK_LF clock loss indicators. 0: Disable 1: Enable Clock loss detection must be disabled when changing the sclk_lf source. STAT0.SCLK_LF_SRC can be polled to determine when a change to a new sclk_lf source has completed."]
pub type ClkLfLossEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_RCOSC_LF` reader - 9:9\\]
Force rcosc_lf to be enabled 0: Disabled 1: Enabled"]
pub type ForceRcoscLfR = crate::BitReader;
#[doc = "Field `FORCE_RCOSC_LF` writer - 9:9\\]
Force rcosc_lf to be enabled 0: Disabled 1: Enabled"]
pub type ForceRcoscLfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPARE10` reader - 16:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Spare10R = crate::FieldReader;
#[doc = "Field `SPARE10` writer - 16:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Spare10W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `RCOSCHFCTRIMFRACT_EN` reader - 17:17\\]
Internal. Only to be used through TI provided API."]
pub type RcoschfctrimfractEnR = crate::BitReader;
#[doc = "Field `RCOSCHFCTRIMFRACT_EN` writer - 17:17\\]
Internal. Only to be used through TI provided API."]
pub type RcoschfctrimfractEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RCOSCHFCTRIMFRACT` reader - 22:18\\]
Internal. Only to be used through TI provided API."]
pub type RcoschfctrimfractR = crate::FieldReader;
#[doc = "Field `RCOSCHFCTRIMFRACT` writer - 22:18\\]
Internal. Only to be used through TI provided API."]
pub type RcoschfctrimfractW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `RESERVED23` reader - 31:23\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved23R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED23` writer - 31:23\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved23W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn xosc_hf_fast_start(&self) -> XoscHfFastStartR {
        XoscHfFastStartR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:7 - 7:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn spare2(&self) -> Spare2R {
        Spare2R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Enable LF clock loss detection and hence the indicators to the system controller. Checks SCLK_LF clock loss indicators. 0: Disable 1: Enable Clock loss detection must be disabled when changing the sclk_lf source. STAT0.SCLK_LF_SRC can be polled to determine when a change to a new sclk_lf source has completed."]
    #[inline(always)]
    pub fn clk_lf_loss_en(&self) -> ClkLfLossEnR {
        ClkLfLossEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Force rcosc_lf to be enabled 0: Disabled 1: Enabled"]
    #[inline(always)]
    pub fn force_rcosc_lf(&self) -> ForceRcoscLfR {
        ForceRcoscLfR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:16 - 16:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn spare10(&self) -> Spare10R {
        Spare10R::new(((self.bits >> 10) & 0x7f) as u8)
    }
    #[doc = "Bit 17 - 17:17\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rcoschfctrimfract_en(&self) -> RcoschfctrimfractEnR {
        RcoschfctrimfractEnR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:22 - 22:18\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rcoschfctrimfract(&self) -> RcoschfctrimfractR {
        RcoschfctrimfractR::new(((self.bits >> 18) & 0x1f) as u8)
    }
    #[doc = "Bits 23:31 - 31:23\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved23(&self) -> Reserved23R {
        Reserved23R::new(((self.bits >> 23) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn xosc_hf_fast_start(&mut self) -> XoscHfFastStartW<Ctl1Spec> {
        XoscHfFastStartW::new(self, 0)
    }
    #[doc = "Bits 2:7 - 7:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn spare2(&mut self) -> Spare2W<Ctl1Spec> {
        Spare2W::new(self, 2)
    }
    #[doc = "Bit 8 - 8:8\\]
Enable LF clock loss detection and hence the indicators to the system controller. Checks SCLK_LF clock loss indicators. 0: Disable 1: Enable Clock loss detection must be disabled when changing the sclk_lf source. STAT0.SCLK_LF_SRC can be polled to determine when a change to a new sclk_lf source has completed."]
    #[inline(always)]
    #[must_use]
    pub fn clk_lf_loss_en(&mut self) -> ClkLfLossEnW<Ctl1Spec> {
        ClkLfLossEnW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Force rcosc_lf to be enabled 0: Disabled 1: Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn force_rcosc_lf(&mut self) -> ForceRcoscLfW<Ctl1Spec> {
        ForceRcoscLfW::new(self, 9)
    }
    #[doc = "Bits 10:16 - 16:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn spare10(&mut self) -> Spare10W<Ctl1Spec> {
        Spare10W::new(self, 10)
    }
    #[doc = "Bit 17 - 17:17\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn rcoschfctrimfract_en(&mut self) -> RcoschfctrimfractEnW<Ctl1Spec> {
        RcoschfctrimfractEnW::new(self, 17)
    }
    #[doc = "Bits 18:22 - 22:18\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn rcoschfctrimfract(&mut self) -> RcoschfctrimfractW<Ctl1Spec> {
        RcoschfctrimfractW::new(self, 18)
    }
    #[doc = "Bits 23:31 - 31:23\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved23(&mut self) -> Reserved23W<Ctl1Spec> {
        Reserved23W::new(self, 23)
    }
}
#[doc = "Control 1 This register contains OSC_DIG configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctl1Spec;
impl crate::RegisterSpec for Ctl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl1::R`](R) reader structure"]
impl crate::Readable for Ctl1Spec {}
#[doc = "`write(|w| ..)` method takes [`ctl1::W`](W) writer structure"]
impl crate::Writable for Ctl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTL1 to value 0"]
impl crate::Resettable for Ctl1Spec {
    const RESET_VALUE: u32 = 0;
}
