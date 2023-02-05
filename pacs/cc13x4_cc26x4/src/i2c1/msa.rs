#[doc = "Register `MSA` reader"]
pub struct R(crate::R<MSA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MSA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MSA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MSA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MSA` writer"]
pub struct W(crate::W<MSA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MSA_SPEC>;
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
impl From<crate::W<MSA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MSA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RS` reader - 0:0\\]
Receive or Send This bit-field specifies if the next operation is a receive (high) or a transmit/send (low) from the addressed slave SA."]
pub type RS_R = crate::BitReader<RS_A>;
#[doc = "0:0\\]
Receive or Send This bit-field specifies if the next operation is a receive (high) or a transmit/send (low) from the addressed slave SA.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RS_A {
    #[doc = "1: Receive data from slave"]
    RX = 1,
    #[doc = "0: Transmit/send data to slave"]
    TX = 0,
}
impl From<RS_A> for bool {
    #[inline(always)]
    fn from(variant: RS_A) -> Self {
        variant as u8 != 0
    }
}
impl RS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RS_A {
        match self.bits {
            true => RS_A::RX,
            false => RS_A::TX,
        }
    }
    #[doc = "Checks if the value of the field is `RX`"]
    #[inline(always)]
    pub fn is_rx(&self) -> bool {
        *self == RS_A::RX
    }
    #[doc = "Checks if the value of the field is `TX`"]
    #[inline(always)]
    pub fn is_tx(&self) -> bool {
        *self == RS_A::TX
    }
}
#[doc = "Field `RS` writer - 0:0\\]
Receive or Send This bit-field specifies if the next operation is a receive (high) or a transmit/send (low) from the addressed slave SA."]
pub type RS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MSA_SPEC, RS_A, O>;
impl<'a, const O: u8> RS_W<'a, O> {
    #[doc = "Receive data from slave"]
    #[inline(always)]
    pub fn rx(self) -> &'a mut W {
        self.variant(RS_A::RX)
    }
    #[doc = "Transmit/send data to slave"]
    #[inline(always)]
    pub fn tx(self) -> &'a mut W {
        self.variant(RS_A::TX)
    }
}
#[doc = "Field `SA` reader - 7:1\\]
I2C master slave address Defines which slave is addressed for the transaction in master mode"]
pub type SA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SA` writer - 7:1\\]
I2C master slave address Defines which slave is addressed for the transaction in master mode"]
pub type SA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MSA_SPEC, u8, u8, 7, O>;
#[doc = "Field `RESERVED8` reader - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED8_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED8` writer - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED8_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MSA_SPEC, u32, u32, 24, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Receive or Send This bit-field specifies if the next operation is a receive (high) or a transmit/send (low) from the addressed slave SA."]
    #[inline(always)]
    pub fn rs(&self) -> RS_R {
        RS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - 7:1\\]
I2C master slave address Defines which slave is addressed for the transaction in master mode"]
    #[inline(always)]
    pub fn sa(&self) -> SA_R {
        SA_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&self) -> RESERVED8_R {
        RESERVED8_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Receive or Send This bit-field specifies if the next operation is a receive (high) or a transmit/send (low) from the addressed slave SA."]
    #[inline(always)]
    #[must_use]
    pub fn rs(&mut self) -> RS_W<0> {
        RS_W::new(self)
    }
    #[doc = "Bits 1:7 - 7:1\\]
I2C master slave address Defines which slave is addressed for the transaction in master mode"]
    #[inline(always)]
    #[must_use]
    pub fn sa(&mut self) -> SA_W<1> {
        SA_W::new(self)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved8(&mut self) -> RESERVED8_W<8> {
        RESERVED8_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Master Salve Address This register contains seven address bits of the slave to be accessed by the master (a6-a0), and an RS bit determining if the next operation is a receive or transmit.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [msa](index.html) module"]
pub struct MSA_SPEC;
impl crate::RegisterSpec for MSA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [msa::R](R) reader structure"]
impl crate::Readable for MSA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [msa::W](W) writer structure"]
impl crate::Writable for MSA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MSA to value 0"]
impl crate::Resettable for MSA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
