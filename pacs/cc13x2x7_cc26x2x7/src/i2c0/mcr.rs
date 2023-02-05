#[doc = "Register `MCR` reader"]
pub struct R(crate::R<MCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCR` writer"]
pub struct W(crate::W<MCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCR_SPEC>;
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
impl From<crate::W<MCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LPBK` reader - 0:0\\]
I2C loopback 0: Normal operation 1: Loopback operation (test mode)"]
pub type LPBK_R = crate::BitReader<LPBK_A>;
#[doc = "0:0\\]
I2C loopback 0: Normal operation 1: Loopback operation (test mode)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPBK_A {
    #[doc = "1: Enable Test Mode"]
    EN = 1,
    #[doc = "0: Disable Test Mode"]
    DIS = 0,
}
impl From<LPBK_A> for bool {
    #[inline(always)]
    fn from(variant: LPBK_A) -> Self {
        variant as u8 != 0
    }
}
impl LPBK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPBK_A {
        match self.bits {
            true => LPBK_A::EN,
            false => LPBK_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == LPBK_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == LPBK_A::DIS
    }
}
#[doc = "Field `LPBK` writer - 0:0\\]
I2C loopback 0: Normal operation 1: Loopback operation (test mode)"]
pub type LPBK_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, LPBK_A, O>;
impl<'a, const O: u8> LPBK_W<'a, O> {
    #[doc = "Enable Test Mode"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(LPBK_A::EN)
    }
    #[doc = "Disable Test Mode"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(LPBK_A::DIS)
    }
}
#[doc = "Field `RESERVED1` reader - 3:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED1` writer - 3:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MCR_SPEC, u8, u8, 3, O>;
#[doc = "Field `MFE` reader - 4:4\\]
I2C master function enable"]
pub type MFE_R = crate::BitReader<MFE_A>;
#[doc = "4:4\\]
I2C master function enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MFE_A {
    #[doc = "1: Master mode is enabled."]
    EN = 1,
    #[doc = "0: Master mode is disabled."]
    DIS = 0,
}
impl From<MFE_A> for bool {
    #[inline(always)]
    fn from(variant: MFE_A) -> Self {
        variant as u8 != 0
    }
}
impl MFE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MFE_A {
        match self.bits {
            true => MFE_A::EN,
            false => MFE_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == MFE_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == MFE_A::DIS
    }
}
#[doc = "Field `MFE` writer - 4:4\\]
I2C master function enable"]
pub type MFE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, MFE_A, O>;
impl<'a, const O: u8> MFE_W<'a, O> {
    #[doc = "Master mode is enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(MFE_A::EN)
    }
    #[doc = "Master mode is disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(MFE_A::DIS)
    }
}
#[doc = "Field `SFE` reader - 5:5\\]
I2C slave function enable"]
pub type SFE_R = crate::BitReader<SFE_A>;
#[doc = "5:5\\]
I2C slave function enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SFE_A {
    #[doc = "1: Slave mode is enabled."]
    EN = 1,
    #[doc = "0: Slave mode is disabled."]
    DIS = 0,
}
impl From<SFE_A> for bool {
    #[inline(always)]
    fn from(variant: SFE_A) -> Self {
        variant as u8 != 0
    }
}
impl SFE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SFE_A {
        match self.bits {
            true => SFE_A::EN,
            false => SFE_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == SFE_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == SFE_A::DIS
    }
}
#[doc = "Field `SFE` writer - 5:5\\]
I2C slave function enable"]
pub type SFE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, SFE_A, O>;
impl<'a, const O: u8> SFE_W<'a, O> {
    #[doc = "Slave mode is enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(SFE_A::EN)
    }
    #[doc = "Slave mode is disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(SFE_A::DIS)
    }
}
#[doc = "Field `RESERVED6` reader - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED6_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED6` writer - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED6_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MCR_SPEC, u32, u32, 26, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
I2C loopback 0: Normal operation 1: Loopback operation (test mode)"]
    #[inline(always)]
    pub fn lpbk(&self) -> LPBK_R {
        LPBK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - 3:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bit 4 - 4:4\\]
I2C master function enable"]
    #[inline(always)]
    pub fn mfe(&self) -> MFE_R {
        MFE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
I2C slave function enable"]
    #[inline(always)]
    pub fn sfe(&self) -> SFE_R {
        SFE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:31 - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved6(&self) -> RESERVED6_R {
        RESERVED6_R::new((self.bits >> 6) & 0x03ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
I2C loopback 0: Normal operation 1: Loopback operation (test mode)"]
    #[inline(always)]
    #[must_use]
    pub fn lpbk(&mut self) -> LPBK_W<0> {
        LPBK_W::new(self)
    }
    #[doc = "Bits 1:3 - 3:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> RESERVED1_W<1> {
        RESERVED1_W::new(self)
    }
    #[doc = "Bit 4 - 4:4\\]
I2C master function enable"]
    #[inline(always)]
    #[must_use]
    pub fn mfe(&mut self) -> MFE_W<4> {
        MFE_W::new(self)
    }
    #[doc = "Bit 5 - 5:5\\]
I2C slave function enable"]
    #[inline(always)]
    #[must_use]
    pub fn sfe(&mut self) -> SFE_W<5> {
        SFE_W::new(self)
    }
    #[doc = "Bits 6:31 - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved6(&mut self) -> RESERVED6_W<6> {
        RESERVED6_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Master Configuration This register configures the mode (Master or Slave) and sets the interface for test mode loopback.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcr](index.html) module"]
pub struct MCR_SPEC;
impl crate::RegisterSpec for MCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mcr::R](R) reader structure"]
impl crate::Readable for MCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mcr::W](W) writer structure"]
impl crate::Writable for MCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MCR to value 0"]
impl crate::Resettable for MCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
