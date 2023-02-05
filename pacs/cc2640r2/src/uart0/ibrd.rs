#[doc = "Register `IBRD` reader"]
pub struct R(crate::R<IBRD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IBRD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IBRD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IBRD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IBRD` writer"]
pub struct W(crate::W<IBRD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IBRD_SPEC>;
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
impl From<crate::W<IBRD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IBRD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIVINT` reader - 15:0\\]
The integer baud rate divisor: The baud rate divisor is calculated using the formula below: Baud rate divisor = (UART reference clock frequency) / (16 * Baud rate) Baud rate divisor must be minimum 1 and maximum 65535. That is, DIVINT=0 does not give a valid baud rate. Similarly, if DIVINT=0xFFFF, any non-zero values in FBRD.DIVFRAC will be illegal. A valid value must be written to this field before the UART can be used for RX or TX operations."]
pub type DIVINT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DIVINT` writer - 15:0\\]
The integer baud rate divisor: The baud rate divisor is calculated using the formula below: Baud rate divisor = (UART reference clock frequency) / (16 * Baud rate) Baud rate divisor must be minimum 1 and maximum 65535. That is, DIVINT=0 does not give a valid baud rate. Similarly, if DIVINT=0xFFFF, any non-zero values in FBRD.DIVFRAC will be illegal. A valid value must be written to this field before the UART can be used for RX or TX operations."]
pub type DIVINT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IBRD_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
The integer baud rate divisor: The baud rate divisor is calculated using the formula below: Baud rate divisor = (UART reference clock frequency) / (16 * Baud rate) Baud rate divisor must be minimum 1 and maximum 65535. That is, DIVINT=0 does not give a valid baud rate. Similarly, if DIVINT=0xFFFF, any non-zero values in FBRD.DIVFRAC will be illegal. A valid value must be written to this field before the UART can be used for RX or TX operations."]
    #[inline(always)]
    pub fn divint(&self) -> DIVINT_R {
        DIVINT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
The integer baud rate divisor: The baud rate divisor is calculated using the formula below: Baud rate divisor = (UART reference clock frequency) / (16 * Baud rate) Baud rate divisor must be minimum 1 and maximum 65535. That is, DIVINT=0 does not give a valid baud rate. Similarly, if DIVINT=0xFFFF, any non-zero values in FBRD.DIVFRAC will be illegal. A valid value must be written to this field before the UART can be used for RX or TX operations."]
    #[inline(always)]
    #[must_use]
    pub fn divint(&mut self) -> DIVINT_W<0> {
        DIVINT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Integer Baud-Rate Divisor If this register is modified while trasmission or reception is on-going, the baudrate will not be updated until transmission or reception of the current character is complete.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ibrd](index.html) module"]
pub struct IBRD_SPEC;
impl crate::RegisterSpec for IBRD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ibrd::R](R) reader structure"]
impl crate::Readable for IBRD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ibrd::W](W) writer structure"]
impl crate::Writable for IBRD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IBRD to value 0"]
impl crate::Resettable for IBRD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
