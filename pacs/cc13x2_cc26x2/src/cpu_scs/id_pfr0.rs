#[doc = "Register `ID_PFR0` reader"]
pub struct R(crate::R<ID_PFR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ID_PFR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ID_PFR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ID_PFR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ID_PFR0` writer"]
pub struct W(crate::W<ID_PFR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ID_PFR0_SPEC>;
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
impl From<crate::W<ID_PFR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ID_PFR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STATE0` reader - 3:0\\]
State0 (T-bit == 0) 0x0: No ARM encoding 0x1: N/A"]
pub type STATE0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `STATE0` writer - 3:0\\]
State0 (T-bit == 0) 0x0: No ARM encoding 0x1: N/A"]
pub type STATE0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ID_PFR0_SPEC, u8, u8, 4, O>;
#[doc = "Field `STATE1` reader - 7:4\\]
State1 (T-bit == 1) 0x0: N/A 0x1: N/A 0x2: Thumb-2 encoding with the 16-bit basic instructions plus 32-bit Buncond/BL but no other 32-bit basic instructions (Note non-basic 32-bit instructions can be added using the appropriate instruction attribute, but other 32-bit basic instructions cannot.) 0x3: Thumb-2 encoding with all Thumb-2 basic instructions"]
pub type STATE1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `STATE1` writer - 7:4\\]
State1 (T-bit == 1) 0x0: N/A 0x1: N/A 0x2: Thumb-2 encoding with the 16-bit basic instructions plus 32-bit Buncond/BL but no other 32-bit basic instructions (Note non-basic 32-bit instructions can be added using the appropriate instruction attribute, but other 32-bit basic instructions cannot.) 0x3: Thumb-2 encoding with all Thumb-2 basic instructions"]
pub type STATE1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ID_PFR0_SPEC, u8, u8, 4, O>;
#[doc = "Field `RESERVED8` reader - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED8_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED8` writer - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED8_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ID_PFR0_SPEC, u32, u32, 24, O>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
State0 (T-bit == 0) 0x0: No ARM encoding 0x1: N/A"]
    #[inline(always)]
    pub fn state0(&self) -> STATE0_R {
        STATE0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
State1 (T-bit == 1) 0x0: N/A 0x1: N/A 0x2: Thumb-2 encoding with the 16-bit basic instructions plus 32-bit Buncond/BL but no other 32-bit basic instructions (Note non-basic 32-bit instructions can be added using the appropriate instruction attribute, but other 32-bit basic instructions cannot.) 0x3: Thumb-2 encoding with all Thumb-2 basic instructions"]
    #[inline(always)]
    pub fn state1(&self) -> STATE1_R {
        STATE1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&self) -> RESERVED8_R {
        RESERVED8_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
State0 (T-bit == 0) 0x0: No ARM encoding 0x1: N/A"]
    #[inline(always)]
    #[must_use]
    pub fn state0(&mut self) -> STATE0_W<0> {
        STATE0_W::new(self)
    }
    #[doc = "Bits 4:7 - 7:4\\]
State1 (T-bit == 1) 0x0: N/A 0x1: N/A 0x2: Thumb-2 encoding with the 16-bit basic instructions plus 32-bit Buncond/BL but no other 32-bit basic instructions (Note non-basic 32-bit instructions can be added using the appropriate instruction attribute, but other 32-bit basic instructions cannot.) 0x3: Thumb-2 encoding with all Thumb-2 basic instructions"]
    #[inline(always)]
    #[must_use]
    pub fn state1(&mut self) -> STATE1_W<4> {
        STATE1_W::new(self)
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
#[doc = "Processor Feature 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [id_pfr0](index.html) module"]
pub struct ID_PFR0_SPEC;
impl crate::RegisterSpec for ID_PFR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [id_pfr0::R](R) reader structure"]
impl crate::Readable for ID_PFR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [id_pfr0::W](W) writer structure"]
impl crate::Writable for ID_PFR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ID_PFR0 to value 0x30"]
impl crate::Resettable for ID_PFR0_SPEC {
    const RESET_VALUE: Self::Ux = 0x30;
}
