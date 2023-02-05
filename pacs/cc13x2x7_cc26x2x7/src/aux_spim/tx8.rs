#[doc = "Register `TX8` reader"]
pub struct R(crate::R<TX8_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TX8_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TX8_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TX8_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TX8` writer"]
pub struct W(crate::W<TX8_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TX8_SPEC>;
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
impl From<crate::W<TX8_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TX8_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA` reader - 7:0\\]
8 bit data transfer. Write DATA to start transfer, MSB first. When transfer completes, MOSI stays at the value of LSB."]
pub type DATA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA` writer - 7:0\\]
8 bit data transfer. Write DATA to start transfer, MSB first. When transfer completes, MOSI stays at the value of LSB."]
pub type DATA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TX8_SPEC, u8, u8, 8, O>;
#[doc = "Field `RESERVED8` reader - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED8_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED8` writer - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED8_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TX8_SPEC, u32, u32, 24, O>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
8 bit data transfer. Write DATA to start transfer, MSB first. When transfer completes, MOSI stays at the value of LSB."]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&self) -> RESERVED8_R {
        RESERVED8_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
8 bit data transfer. Write DATA to start transfer, MSB first. When transfer completes, MOSI stays at the value of LSB."]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DATA_W<0> {
        DATA_W::new(self)
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
#[doc = "Transmit 8 Bit Write operation stalls until current transfer completes.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx8](index.html) module"]
pub struct TX8_SPEC;
impl crate::RegisterSpec for TX8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tx8::R](R) reader structure"]
impl crate::Readable for TX8_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tx8::W](W) writer structure"]
impl crate::Writable for TX8_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TX8 to value 0"]
impl crate::Resettable for TX8_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
