#[doc = "Register `ICR` reader"]
pub struct R(crate::R<ICR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ICR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ICR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ICR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ICR` writer"]
pub struct W(crate::W<ICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICR_SPEC>;
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
impl From<crate::W<ICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED0` reader - 0:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior. Write 0."]
pub type RESERVED0_R = crate::BitReader<bool>;
#[doc = "Field `RESERVED0` writer - 0:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior. Write 0."]
pub type RESERVED0_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `CTSMIC` reader - 1:1\\]
Clear to Send (CTS) modem interrupt clear: Writing 1 to this field clears the clear to send interrupt (RIS.CTSRMIS). Writing 0 has no effect."]
pub type CTSMIC_R = crate::BitReader<bool>;
#[doc = "Field `CTSMIC` writer - 1:1\\]
Clear to Send (CTS) modem interrupt clear: Writing 1 to this field clears the clear to send interrupt (RIS.CTSRMIS). Writing 0 has no effect."]
pub type CTSMIC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `RESERVED2` reader - 3:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior. Write 0"]
pub type RESERVED2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED2` writer - 3:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior. Write 0"]
pub type RESERVED2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ICR_SPEC, u8, u8, 2, O>;
#[doc = "Field `RXIC` reader - 4:4\\]
Receive interrupt clear: Writing 1 to this field clears the receive interrupt (RIS.RXRIS). Writing 0 has no effect."]
pub type RXIC_R = crate::BitReader<bool>;
#[doc = "Field `RXIC` writer - 4:4\\]
Receive interrupt clear: Writing 1 to this field clears the receive interrupt (RIS.RXRIS). Writing 0 has no effect."]
pub type RXIC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `TXIC` reader - 5:5\\]
Transmit interrupt clear: Writing 1 to this field clears the transmit interrupt (RIS.TXRIS). Writing 0 has no effect."]
pub type TXIC_R = crate::BitReader<bool>;
#[doc = "Field `TXIC` writer - 5:5\\]
Transmit interrupt clear: Writing 1 to this field clears the transmit interrupt (RIS.TXRIS). Writing 0 has no effect."]
pub type TXIC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `RTIC` reader - 6:6\\]
Receive timeout interrupt clear: Writing 1 to this field clears the receive timeout interrupt (RIS.RTRIS). Writing 0 has no effect."]
pub type RTIC_R = crate::BitReader<bool>;
#[doc = "Field `RTIC` writer - 6:6\\]
Receive timeout interrupt clear: Writing 1 to this field clears the receive timeout interrupt (RIS.RTRIS). Writing 0 has no effect."]
pub type RTIC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `FEIC` reader - 7:7\\]
Framing error interrupt clear: Writing 1 to this field clears the framing error interrupt (RIS.FERIS). Writing 0 has no effect."]
pub type FEIC_R = crate::BitReader<bool>;
#[doc = "Field `FEIC` writer - 7:7\\]
Framing error interrupt clear: Writing 1 to this field clears the framing error interrupt (RIS.FERIS). Writing 0 has no effect."]
pub type FEIC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `PEIC` reader - 8:8\\]
Parity error interrupt clear: Writing 1 to this field clears the parity error interrupt (RIS.PERIS). Writing 0 has no effect."]
pub type PEIC_R = crate::BitReader<bool>;
#[doc = "Field `PEIC` writer - 8:8\\]
Parity error interrupt clear: Writing 1 to this field clears the parity error interrupt (RIS.PERIS). Writing 0 has no effect."]
pub type PEIC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `BEIC` reader - 9:9\\]
Break error interrupt clear: Writing 1 to this field clears the break error interrupt (RIS.BERIS). Writing 0 has no effect."]
pub type BEIC_R = crate::BitReader<bool>;
#[doc = "Field `BEIC` writer - 9:9\\]
Break error interrupt clear: Writing 1 to this field clears the break error interrupt (RIS.BERIS). Writing 0 has no effect."]
pub type BEIC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `OEIC` reader - 10:10\\]
Overrun error interrupt clear: Writing 1 to this field clears the overrun error interrupt (RIS.OERIS). Writing 0 has no effect."]
pub type OEIC_R = crate::BitReader<bool>;
#[doc = "Field `OEIC` writer - 10:10\\]
Overrun error interrupt clear: Writing 1 to this field clears the overrun error interrupt (RIS.OERIS). Writing 0 has no effect."]
pub type OEIC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `RESERVED11` reader - 31:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED11_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED11` writer - 31:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED11_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ICR_SPEC, u32, u32, 21, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior. Write 0."]
    #[inline(always)]
    pub fn reserved0(&self) -> RESERVED0_R {
        RESERVED0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Clear to Send (CTS) modem interrupt clear: Writing 1 to this field clears the clear to send interrupt (RIS.CTSRMIS). Writing 0 has no effect."]
    #[inline(always)]
    pub fn ctsmic(&self) -> CTSMIC_R {
        CTSMIC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - 3:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior. Write 0"]
    #[inline(always)]
    pub fn reserved2(&self) -> RESERVED2_R {
        RESERVED2_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - 4:4\\]
Receive interrupt clear: Writing 1 to this field clears the receive interrupt (RIS.RXRIS). Writing 0 has no effect."]
    #[inline(always)]
    pub fn rxic(&self) -> RXIC_R {
        RXIC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Transmit interrupt clear: Writing 1 to this field clears the transmit interrupt (RIS.TXRIS). Writing 0 has no effect."]
    #[inline(always)]
    pub fn txic(&self) -> TXIC_R {
        TXIC_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Receive timeout interrupt clear: Writing 1 to this field clears the receive timeout interrupt (RIS.RTRIS). Writing 0 has no effect."]
    #[inline(always)]
    pub fn rtic(&self) -> RTIC_R {
        RTIC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Framing error interrupt clear: Writing 1 to this field clears the framing error interrupt (RIS.FERIS). Writing 0 has no effect."]
    #[inline(always)]
    pub fn feic(&self) -> FEIC_R {
        FEIC_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Parity error interrupt clear: Writing 1 to this field clears the parity error interrupt (RIS.PERIS). Writing 0 has no effect."]
    #[inline(always)]
    pub fn peic(&self) -> PEIC_R {
        PEIC_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Break error interrupt clear: Writing 1 to this field clears the break error interrupt (RIS.BERIS). Writing 0 has no effect."]
    #[inline(always)]
    pub fn beic(&self) -> BEIC_R {
        BEIC_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Overrun error interrupt clear: Writing 1 to this field clears the overrun error interrupt (RIS.OERIS). Writing 0 has no effect."]
    #[inline(always)]
    pub fn oeic(&self) -> OEIC_R {
        OEIC_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:31 - 31:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved11(&self) -> RESERVED11_R {
        RESERVED11_R::new((self.bits >> 11) & 0x001f_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior. Write 0."]
    #[inline(always)]
    #[must_use]
    pub fn reserved0(&mut self) -> RESERVED0_W<0> {
        RESERVED0_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
Clear to Send (CTS) modem interrupt clear: Writing 1 to this field clears the clear to send interrupt (RIS.CTSRMIS). Writing 0 has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn ctsmic(&mut self) -> CTSMIC_W<1> {
        CTSMIC_W::new(self)
    }
    #[doc = "Bits 2:3 - 3:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior. Write 0"]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> RESERVED2_W<2> {
        RESERVED2_W::new(self)
    }
    #[doc = "Bit 4 - 4:4\\]
Receive interrupt clear: Writing 1 to this field clears the receive interrupt (RIS.RXRIS). Writing 0 has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn rxic(&mut self) -> RXIC_W<4> {
        RXIC_W::new(self)
    }
    #[doc = "Bit 5 - 5:5\\]
Transmit interrupt clear: Writing 1 to this field clears the transmit interrupt (RIS.TXRIS). Writing 0 has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn txic(&mut self) -> TXIC_W<5> {
        TXIC_W::new(self)
    }
    #[doc = "Bit 6 - 6:6\\]
Receive timeout interrupt clear: Writing 1 to this field clears the receive timeout interrupt (RIS.RTRIS). Writing 0 has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn rtic(&mut self) -> RTIC_W<6> {
        RTIC_W::new(self)
    }
    #[doc = "Bit 7 - 7:7\\]
Framing error interrupt clear: Writing 1 to this field clears the framing error interrupt (RIS.FERIS). Writing 0 has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn feic(&mut self) -> FEIC_W<7> {
        FEIC_W::new(self)
    }
    #[doc = "Bit 8 - 8:8\\]
Parity error interrupt clear: Writing 1 to this field clears the parity error interrupt (RIS.PERIS). Writing 0 has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn peic(&mut self) -> PEIC_W<8> {
        PEIC_W::new(self)
    }
    #[doc = "Bit 9 - 9:9\\]
Break error interrupt clear: Writing 1 to this field clears the break error interrupt (RIS.BERIS). Writing 0 has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn beic(&mut self) -> BEIC_W<9> {
        BEIC_W::new(self)
    }
    #[doc = "Bit 10 - 10:10\\]
Overrun error interrupt clear: Writing 1 to this field clears the overrun error interrupt (RIS.OERIS). Writing 0 has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn oeic(&mut self) -> OEIC_W<10> {
        OEIC_W::new(self)
    }
    #[doc = "Bits 11:31 - 31:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved11(&mut self) -> RESERVED11_W<11> {
        RESERVED11_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Clear On a write of 1, the corresponding interrupt is cleared. A write of 0 has no effect.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icr](index.html) module"]
pub struct ICR_SPEC;
impl crate::RegisterSpec for ICR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [icr::R](R) reader structure"]
impl crate::Readable for ICR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [icr::W](W) writer structure"]
impl crate::Writable for ICR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::Resettable for ICR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
