#[doc = "Register `SOAR` reader"]
pub struct R(crate::R<SOAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SOAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SOAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SOAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SOAR` writer"]
pub struct W(crate::W<SOAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SOAR_SPEC>;
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
impl From<crate::W<SOAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SOAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OAR` reader - 6:0\\]
I2C slave own address This field specifies bits a6 through a0 of the slave address."]
pub type OAR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OAR` writer - 6:0\\]
I2C slave own address This field specifies bits a6 through a0 of the slave address."]
pub type OAR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SOAR_SPEC, u8, u8, 7, O>;
#[doc = "Field `RESERVED7` reader - 31:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED7_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED7` writer - 31:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED7_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SOAR_SPEC, u32, u32, 25, O>;
impl R {
    #[doc = "Bits 0:6 - 6:0\\]
I2C slave own address This field specifies bits a6 through a0 of the slave address."]
    #[inline(always)]
    pub fn oar(&self) -> OAR_R {
        OAR_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 7:31 - 31:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved7(&self) -> RESERVED7_R {
        RESERVED7_R::new((self.bits >> 7) & 0x01ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:6 - 6:0\\]
I2C slave own address This field specifies bits a6 through a0 of the slave address."]
    #[inline(always)]
    #[must_use]
    pub fn oar(&mut self) -> OAR_W<0> {
        OAR_W::new(self)
    }
    #[doc = "Bits 7:31 - 31:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved7(&mut self) -> RESERVED7_W<7> {
        RESERVED7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Slave Own Address This register consists of seven address bits that identify this I2C device on the I2C bus.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [soar](index.html) module"]
pub struct SOAR_SPEC;
impl crate::RegisterSpec for SOAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [soar::R](R) reader structure"]
impl crate::Readable for SOAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [soar::W](W) writer structure"]
impl crate::Writable for SOAR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SOAR to value 0"]
impl crate::Resettable for SOAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
