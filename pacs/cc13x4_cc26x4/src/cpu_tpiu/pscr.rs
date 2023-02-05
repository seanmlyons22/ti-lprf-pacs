#[doc = "Register `PSCR` reader"]
pub struct R(crate::R<PSCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PSCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PSCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PSCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PSCR` writer"]
pub struct W(crate::W<PSCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PSCR_SPEC>;
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
impl From<crate::W<PSCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PSCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PSCOUNT` reader - 4:0\\]
Periodic Synchronization Count. Determines the reload value of the Periodic Synchronization Counter. The reload value takes effect the next time the counter reaches zero. Reads from this register return the reload value programmed into this register 0b00000 Synchronization disabled. 0b00111 128 bytes. 0b01000 256 bytes. 0b11111 2^31 bytes."]
pub type PSCOUNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PSCOUNT` writer - 4:0\\]
Periodic Synchronization Count. Determines the reload value of the Periodic Synchronization Counter. The reload value takes effect the next time the counter reaches zero. Reads from this register return the reload value programmed into this register 0b00000 Synchronization disabled. 0b00111 128 bytes. 0b01000 256 bytes. 0b11111 2^31 bytes."]
pub type PSCOUNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PSCR_SPEC, u8, u8, 5, O>;
#[doc = "Field `RESERVED5` reader - 31:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED5_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED5` writer - 31:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PSCR_SPEC, u32, u32, 27, O>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Periodic Synchronization Count. Determines the reload value of the Periodic Synchronization Counter. The reload value takes effect the next time the counter reaches zero. Reads from this register return the reload value programmed into this register 0b00000 Synchronization disabled. 0b00111 128 bytes. 0b01000 256 bytes. 0b11111 2^31 bytes."]
    #[inline(always)]
    pub fn pscount(&self) -> PSCOUNT_R {
        PSCOUNT_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:31 - 31:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved5(&self) -> RESERVED5_R {
        RESERVED5_R::new((self.bits >> 5) & 0x07ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Periodic Synchronization Count. Determines the reload value of the Periodic Synchronization Counter. The reload value takes effect the next time the counter reaches zero. Reads from this register return the reload value programmed into this register 0b00000 Synchronization disabled. 0b00111 128 bytes. 0b01000 256 bytes. 0b11111 2^31 bytes."]
    #[inline(always)]
    #[must_use]
    pub fn pscount(&mut self) -> PSCOUNT_W<0> {
        PSCOUNT_W::new(self)
    }
    #[doc = "Bits 5:31 - 31:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved5(&mut self) -> RESERVED5_W<5> {
        RESERVED5_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Formatter Synchronization Counter\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pscr](index.html) module"]
pub struct PSCR_SPEC;
impl crate::RegisterSpec for PSCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pscr::R](R) reader structure"]
impl crate::Readable for PSCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pscr::W](W) writer structure"]
impl crate::Writable for PSCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PSCR to value 0"]
impl crate::Resettable for PSCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
