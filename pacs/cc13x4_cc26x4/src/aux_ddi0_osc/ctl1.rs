#[doc = "Register `CTL1` reader"]
pub struct R(crate::R<CTL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTL1` writer"]
pub struct W(crate::W<CTL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTL1_SPEC>;
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
impl From<crate::W<CTL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XOSC_HF_FAST_START` reader - 1:0\\]
Internal. Only to be used through TI provided API."]
pub type XOSC_HF_FAST_START_R = crate::FieldReader<u8, u8>;
#[doc = "Field `XOSC_HF_FAST_START` writer - 1:0\\]
Internal. Only to be used through TI provided API."]
pub type XOSC_HF_FAST_START_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CTL1_SPEC, u8, u8, 2, O>;
#[doc = "Field `SPARE2` reader - 7:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type SPARE2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SPARE2` writer - 7:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type SPARE2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTL1_SPEC, u8, u8, 6, O>;
#[doc = "Field `CLK_LF_LOSS_EN` reader - 8:8\\]
Enable LF clock loss detection and hence the indicators to the system controller. Checks SCLK_LF clock loss indicators. 0: Disable 1: Enable Clock loss detection must be disabled when changing the sclk_lf source. STAT0.SCLK_LF_SRC can be polled to determine when a change to a new sclk_lf source has completed."]
pub type CLK_LF_LOSS_EN_R = crate::BitReader<bool>;
#[doc = "Field `CLK_LF_LOSS_EN` writer - 8:8\\]
Enable LF clock loss detection and hence the indicators to the system controller. Checks SCLK_LF clock loss indicators. 0: Disable 1: Enable Clock loss detection must be disabled when changing the sclk_lf source. STAT0.SCLK_LF_SRC can be polled to determine when a change to a new sclk_lf source has completed."]
pub type CLK_LF_LOSS_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL1_SPEC, bool, O>;
#[doc = "Field `FORCE_RCOSC_LF` reader - 9:9\\]
Force rcosc_lf to be enabled 0: Disabled 1: Enabled"]
pub type FORCE_RCOSC_LF_R = crate::BitReader<bool>;
#[doc = "Field `FORCE_RCOSC_LF` writer - 9:9\\]
Force rcosc_lf to be enabled 0: Disabled 1: Enabled"]
pub type FORCE_RCOSC_LF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL1_SPEC, bool, O>;
#[doc = "Field `SPARE10` reader - 16:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type SPARE10_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SPARE10` writer - 16:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type SPARE10_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTL1_SPEC, u8, u8, 7, O>;
#[doc = "Field `RCOSCHFCTRIMFRACT_EN` reader - 17:17\\]
Internal. Only to be used through TI provided API."]
pub type RCOSCHFCTRIMFRACT_EN_R = crate::BitReader<bool>;
#[doc = "Field `RCOSCHFCTRIMFRACT_EN` writer - 17:17\\]
Internal. Only to be used through TI provided API."]
pub type RCOSCHFCTRIMFRACT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL1_SPEC, bool, O>;
#[doc = "Field `RCOSCHFCTRIMFRACT` reader - 22:18\\]
Internal. Only to be used through TI provided API."]
pub type RCOSCHFCTRIMFRACT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RCOSCHFCTRIMFRACT` writer - 22:18\\]
Internal. Only to be used through TI provided API."]
pub type RCOSCHFCTRIMFRACT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CTL1_SPEC, u8, u8, 5, O>;
#[doc = "Field `RESERVED23` reader - 31:23\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED23_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RESERVED23` writer - 31:23\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED23_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTL1_SPEC, u16, u16, 9, O>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn xosc_hf_fast_start(&self) -> XOSC_HF_FAST_START_R {
        XOSC_HF_FAST_START_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:7 - 7:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn spare2(&self) -> SPARE2_R {
        SPARE2_R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Enable LF clock loss detection and hence the indicators to the system controller. Checks SCLK_LF clock loss indicators. 0: Disable 1: Enable Clock loss detection must be disabled when changing the sclk_lf source. STAT0.SCLK_LF_SRC can be polled to determine when a change to a new sclk_lf source has completed."]
    #[inline(always)]
    pub fn clk_lf_loss_en(&self) -> CLK_LF_LOSS_EN_R {
        CLK_LF_LOSS_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Force rcosc_lf to be enabled 0: Disabled 1: Enabled"]
    #[inline(always)]
    pub fn force_rcosc_lf(&self) -> FORCE_RCOSC_LF_R {
        FORCE_RCOSC_LF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:16 - 16:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn spare10(&self) -> SPARE10_R {
        SPARE10_R::new(((self.bits >> 10) & 0x7f) as u8)
    }
    #[doc = "Bit 17 - 17:17\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rcoschfctrimfract_en(&self) -> RCOSCHFCTRIMFRACT_EN_R {
        RCOSCHFCTRIMFRACT_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:22 - 22:18\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rcoschfctrimfract(&self) -> RCOSCHFCTRIMFRACT_R {
        RCOSCHFCTRIMFRACT_R::new(((self.bits >> 18) & 0x1f) as u8)
    }
    #[doc = "Bits 23:31 - 31:23\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved23(&self) -> RESERVED23_R {
        RESERVED23_R::new(((self.bits >> 23) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn xosc_hf_fast_start(&mut self) -> XOSC_HF_FAST_START_W<0> {
        XOSC_HF_FAST_START_W::new(self)
    }
    #[doc = "Bits 2:7 - 7:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn spare2(&mut self) -> SPARE2_W<2> {
        SPARE2_W::new(self)
    }
    #[doc = "Bit 8 - 8:8\\]
Enable LF clock loss detection and hence the indicators to the system controller. Checks SCLK_LF clock loss indicators. 0: Disable 1: Enable Clock loss detection must be disabled when changing the sclk_lf source. STAT0.SCLK_LF_SRC can be polled to determine when a change to a new sclk_lf source has completed."]
    #[inline(always)]
    #[must_use]
    pub fn clk_lf_loss_en(&mut self) -> CLK_LF_LOSS_EN_W<8> {
        CLK_LF_LOSS_EN_W::new(self)
    }
    #[doc = "Bit 9 - 9:9\\]
Force rcosc_lf to be enabled 0: Disabled 1: Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn force_rcosc_lf(&mut self) -> FORCE_RCOSC_LF_W<9> {
        FORCE_RCOSC_LF_W::new(self)
    }
    #[doc = "Bits 10:16 - 16:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn spare10(&mut self) -> SPARE10_W<10> {
        SPARE10_W::new(self)
    }
    #[doc = "Bit 17 - 17:17\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn rcoschfctrimfract_en(&mut self) -> RCOSCHFCTRIMFRACT_EN_W<17> {
        RCOSCHFCTRIMFRACT_EN_W::new(self)
    }
    #[doc = "Bits 18:22 - 22:18\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn rcoschfctrimfract(&mut self) -> RCOSCHFCTRIMFRACT_W<18> {
        RCOSCHFCTRIMFRACT_W::new(self)
    }
    #[doc = "Bits 23:31 - 31:23\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved23(&mut self) -> RESERVED23_W<23> {
        RESERVED23_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control 1 This register contains OSC_DIG configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl1](index.html) module"]
pub struct CTL1_SPEC;
impl crate::RegisterSpec for CTL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctl1::R](R) reader structure"]
impl crate::Readable for CTL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctl1::W](W) writer structure"]
impl crate::Writable for CTL1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTL1 to value 0"]
impl crate::Resettable for CTL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
