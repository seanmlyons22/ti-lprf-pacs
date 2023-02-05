#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLE` reader - 0:0\\]
Flash patch unit enable bit 0x0: Flash patch unit disabled 0x1: Flash patch unit enabled"]
pub type ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE` writer - 0:0\\]
Flash patch unit enable bit 0x0: Flash patch unit disabled 0x1: Flash patch unit enabled"]
pub type ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `KEY` reader - 1:1\\]
Key field. In order to write to this register, this bit-field must be written to '1'. This bit always reads 0."]
pub type KEY_R = crate::BitReader<bool>;
#[doc = "Field `KEY` writer - 1:1\\]
Key field. In order to write to this register, this bit-field must be written to '1'. This bit always reads 0."]
pub type KEY_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `RESERVED2` reader - 3:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED2` writer - 3:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 2, O>;
#[doc = "Field `NUM_CODE1` reader - 7:4\\]
Number of code slots field. 0x0: No code slots 0x2: Two code slots 0x6: Six code slots"]
pub type NUM_CODE1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NUM_CODE1` writer - 7:4\\]
Number of code slots field. 0x0: No code slots 0x2: Two code slots 0x6: Six code slots"]
pub type NUM_CODE1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 4, O>;
#[doc = "Field `NUM_LIT` reader - 11:8\\]
Number of literal slots field. 0x0: No literal slots 0x2: Two literal slots"]
pub type NUM_LIT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NUM_LIT` writer - 11:8\\]
Number of literal slots field. 0x0: No literal slots 0x2: Two literal slots"]
pub type NUM_LIT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 4, O>;
#[doc = "Field `NUM_CODE2` reader - 13:12\\]
Number of full banks of code comparators, sixteen comparators per bank. Where less than sixteen code comparators are provided, the bank count is zero, and the number present indicated by NUM_CODE1. This read only field contains 3'b000 to indicate 0 banks for Cortex-M processor."]
pub type NUM_CODE2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NUM_CODE2` writer - 13:12\\]
Number of full banks of code comparators, sixteen comparators per bank. Where less than sixteen code comparators are provided, the bank count is zero, and the number present indicated by NUM_CODE1. This read only field contains 3'b000 to indicate 0 banks for Cortex-M processor."]
pub type NUM_CODE2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 2, O>;
#[doc = "Field `RESERVED14` reader - 31:14\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED14_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED14` writer - 31:14\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED14_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u32, u32, 18, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Flash patch unit enable bit 0x0: Flash patch unit disabled 0x1: Flash patch unit enabled"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Key field. In order to write to this register, this bit-field must be written to '1'. This bit always reads 0."]
    #[inline(always)]
    pub fn key(&self) -> KEY_R {
        KEY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - 3:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> RESERVED2_R {
        RESERVED2_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Number of code slots field. 0x0: No code slots 0x2: Two code slots 0x6: Six code slots"]
    #[inline(always)]
    pub fn num_code1(&self) -> NUM_CODE1_R {
        NUM_CODE1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Number of literal slots field. 0x0: No literal slots 0x2: Two literal slots"]
    #[inline(always)]
    pub fn num_lit(&self) -> NUM_LIT_R {
        NUM_LIT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:13 - 13:12\\]
Number of full banks of code comparators, sixteen comparators per bank. Where less than sixteen code comparators are provided, the bank count is zero, and the number present indicated by NUM_CODE1. This read only field contains 3'b000 to indicate 0 banks for Cortex-M processor."]
    #[inline(always)]
    pub fn num_code2(&self) -> NUM_CODE2_R {
        NUM_CODE2_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:31 - 31:14\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved14(&self) -> RESERVED14_R {
        RESERVED14_R::new((self.bits >> 14) & 0x0003_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Flash patch unit enable bit 0x0: Flash patch unit disabled 0x1: Flash patch unit enabled"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<0> {
        ENABLE_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
Key field. In order to write to this register, this bit-field must be written to '1'. This bit always reads 0."]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KEY_W<1> {
        KEY_W::new(self)
    }
    #[doc = "Bits 2:3 - 3:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> RESERVED2_W<2> {
        RESERVED2_W::new(self)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Number of code slots field. 0x0: No code slots 0x2: Two code slots 0x6: Six code slots"]
    #[inline(always)]
    #[must_use]
    pub fn num_code1(&mut self) -> NUM_CODE1_W<4> {
        NUM_CODE1_W::new(self)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Number of literal slots field. 0x0: No literal slots 0x2: Two literal slots"]
    #[inline(always)]
    #[must_use]
    pub fn num_lit(&mut self) -> NUM_LIT_W<8> {
        NUM_LIT_W::new(self)
    }
    #[doc = "Bits 12:13 - 13:12\\]
Number of full banks of code comparators, sixteen comparators per bank. Where less than sixteen code comparators are provided, the bank count is zero, and the number present indicated by NUM_CODE1. This read only field contains 3'b000 to indicate 0 banks for Cortex-M processor."]
    #[inline(always)]
    #[must_use]
    pub fn num_code2(&mut self) -> NUM_CODE2_W<12> {
        NUM_CODE2_W::new(self)
    }
    #[doc = "Bits 14:31 - 31:14\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved14(&mut self) -> RESERVED14_W<14> {
        RESERVED14_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control This register is used to enable the flash patch block.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL to value 0x0260"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0260;
}
