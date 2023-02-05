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
#[doc = "Field `PRI_34` reader - 23:16\\]
Priority of interrupt 34 (See EVENT:CPUIRQSEL34.EV for details)."]
pub type PRI_34_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRI_34` writer - 23:16\\]
Priority of interrupt 34 (See EVENT:CPUIRQSEL34.EV for details)."]
pub type PRI_34_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NVIC_IPR8_SPEC, u8, u8, 8, O>;
#[doc = "Field `PRI_35` reader - 31:24\\]
Priority of interrupt 35 (See EVENT:CPUIRQSEL35.EV for details)."]
pub type PRI_35_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRI_35` writer - 31:24\\]
Priority of interrupt 35 (See EVENT:CPUIRQSEL35.EV for details)."]
pub type PRI_35_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NVIC_IPR8_SPEC, u8, u8, 8, O>;
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
    #[doc = "Bits 16:23 - 23:16\\]
Priority of interrupt 34 (See EVENT:CPUIRQSEL34.EV for details)."]
    #[inline(always)]
    pub fn pri_34(&self) -> PRI_34_R {
        PRI_34_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Priority of interrupt 35 (See EVENT:CPUIRQSEL35.EV for details)."]
    #[inline(always)]
    pub fn pri_35(&self) -> PRI_35_R {
        PRI_35_R::new(((self.bits >> 24) & 0xff) as u8)
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
    #[doc = "Bits 16:23 - 23:16\\]
Priority of interrupt 34 (See EVENT:CPUIRQSEL34.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn pri_34(&mut self) -> PRI_34_W<16> {
        PRI_34_W::new(self)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Priority of interrupt 35 (See EVENT:CPUIRQSEL35.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn pri_35(&mut self) -> PRI_35_W<24> {
        PRI_35_W::new(self)
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
