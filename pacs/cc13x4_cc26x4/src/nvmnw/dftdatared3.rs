#[doc = "Register `DFTDATARED3` reader"]
pub struct R(crate::R<DFTDATARED3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DFTDATARED3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DFTDATARED3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DFTDATARED3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DFTDATARED3` writer"]
pub struct W(crate::W<DFTDATARED3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DFTDATARED3_SPEC>;
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
impl From<crate::W<DFTDATARED3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DFTDATARED3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VAL` reader - 3:0\\]
Data for redundant bits"]
pub type VAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VAL` writer - 3:0\\]
Data for redundant bits"]
pub type VAL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DFTDATARED3_SPEC, u8, u8, 4, O>;
#[doc = "Field `RESERVED4` reader - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED4_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED4` writer - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED4_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DFTDATARED3_SPEC, u32, u32, 28, O>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Data for redundant bits"]
    #[inline(always)]
    pub fn val(&self) -> VAL_R {
        VAL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:31 - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved4(&self) -> RESERVED4_R {
        RESERVED4_R::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Data for redundant bits"]
    #[inline(always)]
    #[must_use]
    pub fn val(&mut self) -> VAL_W<0> {
        VAL_W::new(self)
    }
    #[doc = "Bits 4:31 - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved4(&mut self) -> RESERVED4_W<4> {
        RESERVED4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DFT Redundancy Data Register 3 This register is used when testing the redundant columns in the flash. It acts as an extension of the CMDDATA* registers. The bits in this register correspond to flash data word register 3. In addition, this register is used to aggregate masking for bits that do not require additional program pulses during program operations. The original data written to this register will be lost during program command execution. Use cases for this register are as follows: 1)Program - Contains the data to be programmed. 2)Erase - Not used. 3)Read Verify - Contains data to be verified. This register is only writable when DFTEN.ENABLE is set. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dftdatared3](index.html) module"]
pub struct DFTDATARED3_SPEC;
impl crate::RegisterSpec for DFTDATARED3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dftdatared3::R](R) reader structure"]
impl crate::Readable for DFTDATARED3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dftdatared3::W](W) writer structure"]
impl crate::Writable for DFTDATARED3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DFTDATARED3 to value 0x0f"]
impl crate::Resettable for DFTDATARED3_SPEC {
    const RESET_VALUE: Self::Ux = 0x0f;
}
