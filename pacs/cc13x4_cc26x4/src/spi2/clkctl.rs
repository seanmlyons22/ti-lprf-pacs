#[doc = "Register `CLKCTL` reader"]
pub struct R(crate::R<CLKCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLKCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLKCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLKCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLKCTL` writer"]
pub struct W(crate::W<CLKCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLKCTL_SPEC>;
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
impl From<crate::W<CLKCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLKCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCR` reader - 9:0\\]
Serial clock divider: This is used to generate the transmit and receive bit rate of the SPI. The SPI bit rate is (SPI's functional clock frequency)/((SCR+1)*2). SCR is a value from 0-1023."]
pub type SCR_R = crate::FieldReader<u16, SCR_A>;
#[doc = "9:0\\]
Serial clock divider: This is used to generate the transmit and receive bit rate of the SPI. The SPI bit rate is (SPI's functional clock frequency)/((SCR+1)*2). SCR is a value from 0-1023.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum SCR_A {
    #[doc = "1023: Highest possible value"]
    MAXIMUM = 1023,
    #[doc = "0: Smallest value"]
    MINIMUM = 0,
}
impl From<SCR_A> for u16 {
    #[inline(always)]
    fn from(variant: SCR_A) -> Self {
        variant as _
    }
}
impl SCR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SCR_A> {
        match self.bits {
            1023 => Some(SCR_A::MAXIMUM),
            0 => Some(SCR_A::MINIMUM),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MAXIMUM`"]
    #[inline(always)]
    pub fn is_maximum(&self) -> bool {
        *self == SCR_A::MAXIMUM
    }
    #[doc = "Checks if the value of the field is `MINIMUM`"]
    #[inline(always)]
    pub fn is_minimum(&self) -> bool {
        *self == SCR_A::MINIMUM
    }
}
#[doc = "Field `SCR` writer - 9:0\\]
Serial clock divider: This is used to generate the transmit and receive bit rate of the SPI. The SPI bit rate is (SPI's functional clock frequency)/((SCR+1)*2). SCR is a value from 0-1023."]
pub type SCR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CLKCTL_SPEC, u16, SCR_A, 10, O>;
impl<'a, const O: u8> SCR_W<'a, O> {
    #[doc = "Highest possible value"]
    #[inline(always)]
    pub fn maximum(self) -> &'a mut W {
        self.variant(SCR_A::MAXIMUM)
    }
    #[doc = "Smallest value"]
    #[inline(always)]
    pub fn minimum(self) -> &'a mut W {
        self.variant(SCR_A::MINIMUM)
    }
}
#[doc = "Field `RESERVED10` reader - 27:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED10_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED10` writer - 27:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED10_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CLKCTL_SPEC, u32, u32, 18, O>;
#[doc = "Field `DSAMPLE` reader - 31:28\\]
Delayed sampling. In master mode the data on the input pin will be sampled after the defined clock cycles. Note: As an example, if the SPI transmit frequency is set to 12 MHz in the master mode, DSAMPLE should be set to a value of 2"]
pub type DSAMPLE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DSAMPLE` writer - 31:28\\]
Delayed sampling. In master mode the data on the input pin will be sampled after the defined clock cycles. Note: As an example, if the SPI transmit frequency is set to 12 MHz in the master mode, DSAMPLE should be set to a value of 2"]
pub type DSAMPLE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CLKCTL_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:9 - 9:0\\]
Serial clock divider: This is used to generate the transmit and receive bit rate of the SPI. The SPI bit rate is (SPI's functional clock frequency)/((SCR+1)*2). SCR is a value from 0-1023."]
    #[inline(always)]
    pub fn scr(&self) -> SCR_R {
        SCR_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:27 - 27:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved10(&self) -> RESERVED10_R {
        RESERVED10_R::new((self.bits >> 10) & 0x0003_ffff)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Delayed sampling. In master mode the data on the input pin will be sampled after the defined clock cycles. Note: As an example, if the SPI transmit frequency is set to 12 MHz in the master mode, DSAMPLE should be set to a value of 2"]
    #[inline(always)]
    pub fn dsample(&self) -> DSAMPLE_R {
        DSAMPLE_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9 - 9:0\\]
Serial clock divider: This is used to generate the transmit and receive bit rate of the SPI. The SPI bit rate is (SPI's functional clock frequency)/((SCR+1)*2). SCR is a value from 0-1023."]
    #[inline(always)]
    #[must_use]
    pub fn scr(&mut self) -> SCR_W<0> {
        SCR_W::new(self)
    }
    #[doc = "Bits 10:27 - 27:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved10(&mut self) -> RESERVED10_W<10> {
        RESERVED10_W::new(self)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Delayed sampling. In master mode the data on the input pin will be sampled after the defined clock cycles. Note: As an example, if the SPI transmit frequency is set to 12 MHz in the master mode, DSAMPLE should be set to a value of 2"]
    #[inline(always)]
    #[must_use]
    pub fn dsample(&mut self) -> DSAMPLE_W<28> {
        DSAMPLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock prescaler and divider register. This register contains the settings for the Clock prescaler and divider settings.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkctl](index.html) module"]
pub struct CLKCTL_SPEC;
impl crate::RegisterSpec for CLKCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clkctl::R](R) reader structure"]
impl crate::Readable for CLKCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clkctl::W](W) writer structure"]
impl crate::Writable for CLKCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLKCTL to value 0"]
impl crate::Resettable for CLKCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
