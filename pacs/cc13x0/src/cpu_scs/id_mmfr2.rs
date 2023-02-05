#[doc = "Register `ID_MMFR2` reader"]
pub struct R(crate::R<ID_MMFR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ID_MMFR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ID_MMFR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ID_MMFR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ID_MMFR2` writer"]
pub struct W(crate::W<ID_MMFR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ID_MMFR2_SPEC>;
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
impl From<crate::W<ID_MMFR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ID_MMFR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED0` reader - 23:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED0_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED0` writer - 23:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ID_MMFR2_SPEC, u32, u32, 24, O>;
#[doc = "Field `WAIT_FOR_INTERRUPT_STALLING` reader - 24:24\\]
wait for interrupt stalling 0x0: Not supported 0x1: Wait for interrupt supported"]
pub type WAIT_FOR_INTERRUPT_STALLING_R = crate::BitReader<bool>;
#[doc = "Field `WAIT_FOR_INTERRUPT_STALLING` writer - 24:24\\]
wait for interrupt stalling 0x0: Not supported 0x1: Wait for interrupt supported"]
pub type WAIT_FOR_INTERRUPT_STALLING_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ID_MMFR2_SPEC, bool, O>;
#[doc = "Field `RESERVED28` reader - 31:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED28_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED28` writer - 31:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED28_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ID_MMFR2_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:23 - 23:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&self) -> RESERVED0_R {
        RESERVED0_R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bit 24 - 24:24\\]
wait for interrupt stalling 0x0: Not supported 0x1: Wait for interrupt supported"]
    #[inline(always)]
    pub fn wait_for_interrupt_stalling(&self) -> WAIT_FOR_INTERRUPT_STALLING_R {
        WAIT_FOR_INTERRUPT_STALLING_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:31 - 31:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved28(&self) -> RESERVED28_R {
        RESERVED28_R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - 23:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved0(&mut self) -> RESERVED0_W<0> {
        RESERVED0_W::new(self)
    }
    #[doc = "Bit 24 - 24:24\\]
wait for interrupt stalling 0x0: Not supported 0x1: Wait for interrupt supported"]
    #[inline(always)]
    #[must_use]
    pub fn wait_for_interrupt_stalling(&mut self) -> WAIT_FOR_INTERRUPT_STALLING_W<24> {
        WAIT_FOR_INTERRUPT_STALLING_W::new(self)
    }
    #[doc = "Bits 25:31 - 31:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved28(&mut self) -> RESERVED28_W<25> {
        RESERVED28_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Memory Model Feature 2 General information on the memory model and memory management support.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [id_mmfr2](index.html) module"]
pub struct ID_MMFR2_SPEC;
impl crate::RegisterSpec for ID_MMFR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [id_mmfr2::R](R) reader structure"]
impl crate::Readable for ID_MMFR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [id_mmfr2::W](W) writer structure"]
impl crate::Writable for ID_MMFR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ID_MMFR2 to value 0x0100_0000"]
impl crate::Resettable for ID_MMFR2_SPEC {
    const RESET_VALUE: Self::Ux = 0x0100_0000;
}
