#[doc = "Register `CHCTL` reader"]
pub struct R(crate::R<CHCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHCTL` writer"]
pub struct W(crate::W<CHCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHCTL_SPEC>;
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
impl From<crate::W<CHCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH0_EN` reader - 0:0\\]
RTC Channel 0 Enable 0: Disable RTC Channel 0 1: Enable RTC Channel 0"]
pub type CH0_EN_R = crate::BitReader<bool>;
#[doc = "Field `CH0_EN` writer - 0:0\\]
RTC Channel 0 Enable 0: Disable RTC Channel 0 1: Enable RTC Channel 0"]
pub type CH0_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHCTL_SPEC, bool, O>;
#[doc = "Field `RESERVED1` reader - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED1` writer - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CHCTL_SPEC, u8, u8, 7, O>;
#[doc = "Field `CH1_EN` reader - 8:8\\]
RTC Channel 1 Enable 0: Disable RTC Channel 1 1: Enable RTC Channel 1"]
pub type CH1_EN_R = crate::BitReader<bool>;
#[doc = "Field `CH1_EN` writer - 8:8\\]
RTC Channel 1 Enable 0: Disable RTC Channel 1 1: Enable RTC Channel 1"]
pub type CH1_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHCTL_SPEC, bool, O>;
#[doc = "Field `CH1_CAPT_EN` reader - 9:9\\]
Set Channel 1 mode 0: Compare mode (default) 1: Capture mode"]
pub type CH1_CAPT_EN_R = crate::BitReader<bool>;
#[doc = "Field `CH1_CAPT_EN` writer - 9:9\\]
Set Channel 1 mode 0: Compare mode (default) 1: Capture mode"]
pub type CH1_CAPT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHCTL_SPEC, bool, O>;
#[doc = "Field `RESERVED10` reader - 15:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED10_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED10` writer - 15:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED10_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CHCTL_SPEC, u8, u8, 6, O>;
#[doc = "Field `CH2_EN` reader - 16:16\\]
RTC Channel 2 Enable 0: Disable RTC Channel 2 1: Enable RTC Channel 2"]
pub type CH2_EN_R = crate::BitReader<bool>;
#[doc = "Field `CH2_EN` writer - 16:16\\]
RTC Channel 2 Enable 0: Disable RTC Channel 2 1: Enable RTC Channel 2"]
pub type CH2_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHCTL_SPEC, bool, O>;
#[doc = "Field `RESERVED17` reader - 17:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED17_R = crate::BitReader<bool>;
#[doc = "Field `RESERVED17` writer - 17:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED17_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHCTL_SPEC, bool, O>;
#[doc = "Field `CH2_CONT_EN` reader - 18:18\\]
Set to enable continuous operation of Channel 2"]
pub type CH2_CONT_EN_R = crate::BitReader<bool>;
#[doc = "Field `CH2_CONT_EN` writer - 18:18\\]
Set to enable continuous operation of Channel 2"]
pub type CH2_CONT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHCTL_SPEC, bool, O>;
#[doc = "Field `RESERVED19` reader - 31:19\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED19_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RESERVED19` writer - 31:19\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED19_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CHCTL_SPEC, u16, u16, 13, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
RTC Channel 0 Enable 0: Disable RTC Channel 0 1: Enable RTC Channel 0"]
    #[inline(always)]
    pub fn ch0_en(&self) -> CH0_EN_R {
        CH0_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
RTC Channel 1 Enable 0: Disable RTC Channel 1 1: Enable RTC Channel 1"]
    #[inline(always)]
    pub fn ch1_en(&self) -> CH1_EN_R {
        CH1_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Set Channel 1 mode 0: Compare mode (default) 1: Capture mode"]
    #[inline(always)]
    pub fn ch1_capt_en(&self) -> CH1_CAPT_EN_R {
        CH1_CAPT_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:15 - 15:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved10(&self) -> RESERVED10_R {
        RESERVED10_R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
RTC Channel 2 Enable 0: Disable RTC Channel 2 1: Enable RTC Channel 2"]
    #[inline(always)]
    pub fn ch2_en(&self) -> CH2_EN_R {
        CH2_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved17(&self) -> RESERVED17_R {
        RESERVED17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Set to enable continuous operation of Channel 2"]
    #[inline(always)]
    pub fn ch2_cont_en(&self) -> CH2_CONT_EN_R {
        CH2_CONT_EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:31 - 31:19\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved19(&self) -> RESERVED19_R {
        RESERVED19_R::new(((self.bits >> 19) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
RTC Channel 0 Enable 0: Disable RTC Channel 0 1: Enable RTC Channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn ch0_en(&mut self) -> CH0_EN_W<0> {
        CH0_EN_W::new(self)
    }
    #[doc = "Bits 1:7 - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> RESERVED1_W<1> {
        RESERVED1_W::new(self)
    }
    #[doc = "Bit 8 - 8:8\\]
RTC Channel 1 Enable 0: Disable RTC Channel 1 1: Enable RTC Channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_en(&mut self) -> CH1_EN_W<8> {
        CH1_EN_W::new(self)
    }
    #[doc = "Bit 9 - 9:9\\]
Set Channel 1 mode 0: Compare mode (default) 1: Capture mode"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_capt_en(&mut self) -> CH1_CAPT_EN_W<9> {
        CH1_CAPT_EN_W::new(self)
    }
    #[doc = "Bits 10:15 - 15:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved10(&mut self) -> RESERVED10_W<10> {
        RESERVED10_W::new(self)
    }
    #[doc = "Bit 16 - 16:16\\]
RTC Channel 2 Enable 0: Disable RTC Channel 2 1: Enable RTC Channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn ch2_en(&mut self) -> CH2_EN_W<16> {
        CH2_EN_W::new(self)
    }
    #[doc = "Bit 17 - 17:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved17(&mut self) -> RESERVED17_W<17> {
        RESERVED17_W::new(self)
    }
    #[doc = "Bit 18 - 18:18\\]
Set to enable continuous operation of Channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn ch2_cont_en(&mut self) -> CH2_CONT_EN_W<18> {
        CH2_CONT_EN_W::new(self)
    }
    #[doc = "Bits 19:31 - 31:19\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved19(&mut self) -> RESERVED19_W<19> {
        RESERVED19_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chctl](index.html) module"]
pub struct CHCTL_SPEC;
impl crate::RegisterSpec for CHCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chctl::R](R) reader structure"]
impl crate::Readable for CHCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chctl::W](W) writer structure"]
impl crate::Writable for CHCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CHCTL to value 0"]
impl crate::Resettable for CHCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
