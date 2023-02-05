#[doc = "Register `PER_CHK` reader"]
pub struct R(crate::R<PER_CHK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PER_CHK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PER_CHK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PER_CHK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PER_CHK` writer"]
pub struct W(crate::W<PER_CHK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PER_CHK_SPEC>;
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
impl From<crate::W<PER_CHK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PER_CHK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PER_ADDR` reader - 23:0\\]
Parity Error Address Offset Returns the last address offset which resulted in a parity error during an SRAM read. The address offset returned is always the word-aligned address that contains the location with the parity error. For parity faults on non word-aligned accesses, CPU_SCS:BFAR.ADDRESS will hold the address of the location that resulted in parity error."]
pub type PER_ADDR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `PER_ADDR` writer - 23:0\\]
Parity Error Address Offset Returns the last address offset which resulted in a parity error during an SRAM read. The address offset returned is always the word-aligned address that contains the location with the parity error. For parity faults on non word-aligned accesses, CPU_SCS:BFAR.ADDRESS will hold the address of the location that resulted in parity error."]
pub type PER_ADDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PER_CHK_SPEC, u32, u32, 24, O>;
#[doc = "Field `RESERVED24` reader - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED24_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED24` writer - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED24_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PER_CHK_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:23 - 23:0\\]
Parity Error Address Offset Returns the last address offset which resulted in a parity error during an SRAM read. The address offset returned is always the word-aligned address that contains the location with the parity error. For parity faults on non word-aligned accesses, CPU_SCS:BFAR.ADDRESS will hold the address of the location that resulted in parity error."]
    #[inline(always)]
    pub fn per_addr(&self) -> PER_ADDR_R {
        PER_ADDR_R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved24(&self) -> RESERVED24_R {
        RESERVED24_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - 23:0\\]
Parity Error Address Offset Returns the last address offset which resulted in a parity error during an SRAM read. The address offset returned is always the word-aligned address that contains the location with the parity error. For parity faults on non word-aligned accesses, CPU_SCS:BFAR.ADDRESS will hold the address of the location that resulted in parity error."]
    #[inline(always)]
    #[must_use]
    pub fn per_addr(&mut self) -> PER_ADDR_W<0> {
        PER_ADDR_W::new(self)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved24(&mut self) -> RESERVED24_W<24> {
        RESERVED24_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Parity Error Check Parity error check results\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [per_chk](index.html) module"]
pub struct PER_CHK_SPEC;
impl crate::RegisterSpec for PER_CHK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [per_chk::R](R) reader structure"]
impl crate::Readable for PER_CHK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [per_chk::W](W) writer structure"]
impl crate::Writable for PER_CHK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PER_CHK to value 0"]
impl crate::Resettable for PER_CHK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
