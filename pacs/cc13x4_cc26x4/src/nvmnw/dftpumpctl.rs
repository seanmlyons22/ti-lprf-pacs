#[doc = "Register `DFTPUMPCTL` reader"]
pub struct R(crate::R<DFTPUMPCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DFTPUMPCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DFTPUMPCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DFTPUMPCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DFTPUMPCTL` writer"]
pub struct W(crate::W<DFTPUMPCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DFTPUMPCTL_SPEC>;
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
impl From<crate::W<DFTPUMPCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DFTPUMPCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TCR` reader - 6:0\\]
TCR test mode to be applied to the pump"]
pub type TCR_R = crate::FieldReader<u8, TCR_A>;
#[doc = "6:0\\]
TCR test mode to be applied to the pump\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TCR_A {
    #[doc = "127: Maximum value"]
    MAXIMUM = 127,
    #[doc = "0: Minimum value"]
    MINIMUM = 0,
}
impl From<TCR_A> for u8 {
    #[inline(always)]
    fn from(variant: TCR_A) -> Self {
        variant as _
    }
}
impl TCR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TCR_A> {
        match self.bits {
            127 => Some(TCR_A::MAXIMUM),
            0 => Some(TCR_A::MINIMUM),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MAXIMUM`"]
    #[inline(always)]
    pub fn is_maximum(&self) -> bool {
        *self == TCR_A::MAXIMUM
    }
    #[doc = "Checks if the value of the field is `MINIMUM`"]
    #[inline(always)]
    pub fn is_minimum(&self) -> bool {
        *self == TCR_A::MINIMUM
    }
}
#[doc = "Field `TCR` writer - 6:0\\]
TCR test mode to be applied to the pump"]
pub type TCR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DFTPUMPCTL_SPEC, u8, TCR_A, 7, O>;
impl<'a, const O: u8> TCR_W<'a, O> {
    #[doc = "Maximum value"]
    #[inline(always)]
    pub fn maximum(self) -> &'a mut W {
        self.variant(TCR_A::MAXIMUM)
    }
    #[doc = "Minimum value"]
    #[inline(always)]
    pub fn minimum(self) -> &'a mut W {
        self.variant(TCR_A::MINIMUM)
    }
}
#[doc = "Field `RESERVED7` reader - 7:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED7_R = crate::BitReader<bool>;
#[doc = "Field `RESERVED7` writer - 7:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED7_W<'a, const O: u8> = crate::BitWriter<'a, u32, DFTPUMPCTL_SPEC, bool, O>;
#[doc = "Field `PUMPCLKEN` reader - 8:8\\]
Allows direct control of the pump oscillator which is used to generate pumpclk. Normally, enable/disable of pumpclk is under NoWrapper state machine control. This bit allows system to enable the clock independently."]
pub type PUMPCLKEN_R = crate::BitReader<PUMPCLKEN_A>;
#[doc = "8:8\\]
Allows direct control of the pump oscillator which is used to generate pumpclk. Normally, enable/disable of pumpclk is under NoWrapper state machine control. This bit allows system to enable the clock independently.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PUMPCLKEN_A {
    #[doc = "1: Force pump clock oscillator to be enabled."]
    ENABLE = 1,
    #[doc = "0: Allow pump clock oscillator to be controlled by hardware."]
    HWCTL = 0,
}
impl From<PUMPCLKEN_A> for bool {
    #[inline(always)]
    fn from(variant: PUMPCLKEN_A) -> Self {
        variant as u8 != 0
    }
}
impl PUMPCLKEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PUMPCLKEN_A {
        match self.bits {
            true => PUMPCLKEN_A::ENABLE,
            false => PUMPCLKEN_A::HWCTL,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PUMPCLKEN_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `HWCTL`"]
    #[inline(always)]
    pub fn is_hwctl(&self) -> bool {
        *self == PUMPCLKEN_A::HWCTL
    }
}
#[doc = "Field `PUMPCLKEN` writer - 8:8\\]
Allows direct control of the pump oscillator which is used to generate pumpclk. Normally, enable/disable of pumpclk is under NoWrapper state machine control. This bit allows system to enable the clock independently."]
pub type PUMPCLKEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DFTPUMPCTL_SPEC, PUMPCLKEN_A, O>;
impl<'a, const O: u8> PUMPCLKEN_W<'a, O> {
    #[doc = "Force pump clock oscillator to be enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PUMPCLKEN_A::ENABLE)
    }
    #[doc = "Allow pump clock oscillator to be controlled by hardware."]
    #[inline(always)]
    pub fn hwctl(self) -> &'a mut W {
        self.variant(PUMPCLKEN_A::HWCTL)
    }
}
#[doc = "Field `SSEN` reader - 9:9\\]
Dither control for oscillator Enumeration: 0: Disable Dither 1: Enable Dither"]
pub type SSEN_R = crate::BitReader<SSEN_A>;
#[doc = "9:9\\]
Dither control for oscillator Enumeration: 0: Disable Dither 1: Enable Dither\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSEN_A {
    #[doc = "1: Enable"]
    ENABLE = 1,
    #[doc = "0: Disable"]
    DISABLE = 0,
}
impl From<SSEN_A> for bool {
    #[inline(always)]
    fn from(variant: SSEN_A) -> Self {
        variant as u8 != 0
    }
}
impl SSEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSEN_A {
        match self.bits {
            true => SSEN_A::ENABLE,
            false => SSEN_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SSEN_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SSEN_A::DISABLE
    }
}
#[doc = "Field `SSEN` writer - 9:9\\]
Dither control for oscillator Enumeration: 0: Disable Dither 1: Enable Dither"]
pub type SSEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DFTPUMPCTL_SPEC, SSEN_A, O>;
impl<'a, const O: u8> SSEN_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SSEN_A::ENABLE)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SSEN_A::DISABLE)
    }
}
#[doc = "Field `RESERVED10` reader - 11:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED10_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED10` writer - 11:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED10_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DFTPUMPCTL_SPEC, u8, u8, 2, O>;
#[doc = "Field `CONFIGPMP` reader - 15:12\\]
Pump configuration control. LP, HP operation"]
pub type CONFIGPMP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CONFIGPMP` writer - 15:12\\]
Pump configuration control. LP, HP operation"]
pub type CONFIGPMP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DFTPUMPCTL_SPEC, u8, u8, 4, O>;
#[doc = "Field `IREFEVCTL` reader - 18:16\\]
IREFEV control IREFVRD, REFTC, IREFCONST, IREFCCOR blocks in IREFEV"]
pub type IREFEVCTL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IREFEVCTL` writer - 18:16\\]
IREFEV control IREFVRD, REFTC, IREFCONST, IREFCCOR blocks in IREFEV"]
pub type IREFEVCTL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DFTPUMPCTL_SPEC, u8, u8, 3, O>;
#[doc = "Field `RESERVED19` reader - 31:19\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED19_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RESERVED19` writer - 31:19\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED19_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DFTPUMPCTL_SPEC, u16, u16, 13, O>;
impl R {
    #[doc = "Bits 0:6 - 6:0\\]
TCR test mode to be applied to the pump"]
    #[inline(always)]
    pub fn tcr(&self) -> TCR_R {
        TCR_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - 7:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved7(&self) -> RESERVED7_R {
        RESERVED7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Allows direct control of the pump oscillator which is used to generate pumpclk. Normally, enable/disable of pumpclk is under NoWrapper state machine control. This bit allows system to enable the clock independently."]
    #[inline(always)]
    pub fn pumpclken(&self) -> PUMPCLKEN_R {
        PUMPCLKEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Dither control for oscillator Enumeration: 0: Disable Dither 1: Enable Dither"]
    #[inline(always)]
    pub fn ssen(&self) -> SSEN_R {
        SSEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - 11:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved10(&self) -> RESERVED10_R {
        RESERVED10_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Pump configuration control. LP, HP operation"]
    #[inline(always)]
    pub fn configpmp(&self) -> CONFIGPMP_R {
        CONFIGPMP_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:18 - 18:16\\]
IREFEV control IREFVRD, REFTC, IREFCONST, IREFCCOR blocks in IREFEV"]
    #[inline(always)]
    pub fn irefevctl(&self) -> IREFEVCTL_R {
        IREFEVCTL_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 19:31 - 31:19\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved19(&self) -> RESERVED19_R {
        RESERVED19_R::new(((self.bits >> 19) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:6 - 6:0\\]
TCR test mode to be applied to the pump"]
    #[inline(always)]
    #[must_use]
    pub fn tcr(&mut self) -> TCR_W<0> {
        TCR_W::new(self)
    }
    #[doc = "Bit 7 - 7:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved7(&mut self) -> RESERVED7_W<7> {
        RESERVED7_W::new(self)
    }
    #[doc = "Bit 8 - 8:8\\]
Allows direct control of the pump oscillator which is used to generate pumpclk. Normally, enable/disable of pumpclk is under NoWrapper state machine control. This bit allows system to enable the clock independently."]
    #[inline(always)]
    #[must_use]
    pub fn pumpclken(&mut self) -> PUMPCLKEN_W<8> {
        PUMPCLKEN_W::new(self)
    }
    #[doc = "Bit 9 - 9:9\\]
Dither control for oscillator Enumeration: 0: Disable Dither 1: Enable Dither"]
    #[inline(always)]
    #[must_use]
    pub fn ssen(&mut self) -> SSEN_W<9> {
        SSEN_W::new(self)
    }
    #[doc = "Bits 10:11 - 11:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved10(&mut self) -> RESERVED10_W<10> {
        RESERVED10_W::new(self)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Pump configuration control. LP, HP operation"]
    #[inline(always)]
    #[must_use]
    pub fn configpmp(&mut self) -> CONFIGPMP_W<12> {
        CONFIGPMP_W::new(self)
    }
    #[doc = "Bits 16:18 - 18:16\\]
IREFEV control IREFVRD, REFTC, IREFCONST, IREFCCOR blocks in IREFEV"]
    #[inline(always)]
    #[must_use]
    pub fn irefevctl(&mut self) -> IREFEVCTL_W<16> {
        IREFEVCTL_W::new(self)
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
#[doc = "DFT Pump Control Register This allows some configuration of pump parameters during test. This register is only writable when DFTEN.ENABLE is set. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dftpumpctl](index.html) module"]
pub struct DFTPUMPCTL_SPEC;
impl crate::RegisterSpec for DFTPUMPCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dftpumpctl::R](R) reader structure"]
impl crate::Readable for DFTPUMPCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dftpumpctl::W](W) writer structure"]
impl crate::Writable for DFTPUMPCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DFTPUMPCTL to value 0x1000"]
impl crate::Resettable for DFTPUMPCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x1000;
}
