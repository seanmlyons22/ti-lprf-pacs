#[doc = "Register `NVIC_IPR8` reader"]
pub struct R(crate::R<NVIC_IPR8_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NVIC_IPR8_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NVIC_IPR8_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NVIC_IPR8_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NVIC_IPR8` writer"]
pub struct W(crate::W<NVIC_IPR8_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NVIC_IPR8_SPEC>;
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
impl From<crate::W<NVIC_IPR8_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NVIC_IPR8_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRI_32` reader - 7:0\\]
Priority of interrupt 32 (See EVENT:CPUIRQSEL32.EV for details)."]
pub type PRI_32_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRI_32` writer - 7:0\\]
Priority of interrupt 32 (See EVENT:CPUIRQSEL32.EV for details)."]
pub type PRI_32_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NVIC_IPR8_SPEC, u8, u8, 8, O>;
#[doc = "Field `PRI_33` reader - 15:8\\]
Priority of interrupt 33 (See EVENT:CPUIRQSEL33.EV for details)."]
pub type PRI_33_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRI_33` writer - 15:8\\]
Priority of interrupt 33 (See EVENT:CPUIRQSEL33.EV for details)."]
pub type PRI_33_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NVIC_IPR8_SPEC, u8, u8, 8, O>;
#[doc = "Field `RESERVED16` reader - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED16_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RESERVED16` writer - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED16_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, NVIC_IPR8_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Priority of interrupt 32 (See EVENT:CPUIRQSEL32.EV for details)."]
    #[inline(always)]
    pub fn pri_32(&self) -> PRI_32_R {
        PRI_32_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Priority of interrupt 33 (See EVENT:CPUIRQSEL33.EV for details)."]
    #[inline(always)]
    pub fn pri_33(&self) -> PRI_33_R {
        PRI_33_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved16(&self) -> RESERVED16_R {
        RESERVED16_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Priority of interrupt 32 (See EVENT:CPUIRQSEL32.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn pri_32(&mut self) -> PRI_32_W<0> {
        PRI_32_W::new(self)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Priority of interrupt 33 (See EVENT:CPUIRQSEL33.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn pri_33(&mut self) -> PRI_33_W<8> {
        PRI_33_W::new(self)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved16(&mut self) -> RESERVED16_W<16> {
        RESERVED16_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Irq 32 to 35 Priority This register is used to assign a priority from 0 to 255 to each of the available interrupts. 0 is the highest priority, and 255 is the lowest. The interpretation of the Interrupt Priority Registers changes based on the setting in AIRCR.PRIGROUP.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nvic_ipr8](index.html) module"]
pub struct NVIC_IPR8_SPEC;
impl crate::RegisterSpec for NVIC_IPR8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nvic_ipr8::R](R) reader structure"]
impl crate::Readable for NVIC_IPR8_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nvic_ipr8::W](W) writer structure"]
impl crate::Writable for NVIC_IPR8_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NVIC_IPR8 to value 0"]
impl crate::Resettable for NVIC_IPR8_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
