#[doc = "Register `NVIC_IPR4` reader"]
pub struct R(crate::R<NVIC_IPR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NVIC_IPR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NVIC_IPR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NVIC_IPR4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NVIC_IPR4` writer"]
pub struct W(crate::W<NVIC_IPR4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NVIC_IPR4_SPEC>;
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
impl From<crate::W<NVIC_IPR4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NVIC_IPR4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRI_16` reader - 7:0\\]
Priority of interrupt 16 (See EVENT:CPUIRQSEL16.EV for details)."]
pub type PRI_16_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRI_16` writer - 7:0\\]
Priority of interrupt 16 (See EVENT:CPUIRQSEL16.EV for details)."]
pub type PRI_16_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NVIC_IPR4_SPEC, u8, u8, 8, O>;
#[doc = "Field `PRI_17` reader - 15:8\\]
Priority of interrupt 17 (See EVENT:CPUIRQSEL17.EV for details)."]
pub type PRI_17_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRI_17` writer - 15:8\\]
Priority of interrupt 17 (See EVENT:CPUIRQSEL17.EV for details)."]
pub type PRI_17_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NVIC_IPR4_SPEC, u8, u8, 8, O>;
#[doc = "Field `PRI_18` reader - 23:16\\]
Priority of interrupt 18 (See EVENT:CPUIRQSEL18.EV for details)."]
pub type PRI_18_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRI_18` writer - 23:16\\]
Priority of interrupt 18 (See EVENT:CPUIRQSEL18.EV for details)."]
pub type PRI_18_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NVIC_IPR4_SPEC, u8, u8, 8, O>;
#[doc = "Field `PRI_19` reader - 31:24\\]
Priority of interrupt 19 (See EVENT:CPUIRQSEL19.EV for details)."]
pub type PRI_19_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRI_19` writer - 31:24\\]
Priority of interrupt 19 (See EVENT:CPUIRQSEL19.EV for details)."]
pub type PRI_19_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NVIC_IPR4_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Priority of interrupt 16 (See EVENT:CPUIRQSEL16.EV for details)."]
    #[inline(always)]
    pub fn pri_16(&self) -> PRI_16_R {
        PRI_16_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Priority of interrupt 17 (See EVENT:CPUIRQSEL17.EV for details)."]
    #[inline(always)]
    pub fn pri_17(&self) -> PRI_17_R {
        PRI_17_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Priority of interrupt 18 (See EVENT:CPUIRQSEL18.EV for details)."]
    #[inline(always)]
    pub fn pri_18(&self) -> PRI_18_R {
        PRI_18_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Priority of interrupt 19 (See EVENT:CPUIRQSEL19.EV for details)."]
    #[inline(always)]
    pub fn pri_19(&self) -> PRI_19_R {
        PRI_19_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Priority of interrupt 16 (See EVENT:CPUIRQSEL16.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn pri_16(&mut self) -> PRI_16_W<0> {
        PRI_16_W::new(self)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Priority of interrupt 17 (See EVENT:CPUIRQSEL17.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn pri_17(&mut self) -> PRI_17_W<8> {
        PRI_17_W::new(self)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Priority of interrupt 18 (See EVENT:CPUIRQSEL18.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn pri_18(&mut self) -> PRI_18_W<16> {
        PRI_18_W::new(self)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Priority of interrupt 19 (See EVENT:CPUIRQSEL19.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn pri_19(&mut self) -> PRI_19_W<24> {
        PRI_19_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Irq 16 to 19 Priority This register is used to assign a priority from 0 to 255 to each of the available interrupts. 0 is the highest priority, and 255 is the lowest. The interpretation of the Interrupt Priority Registers changes based on the setting in AIRCR.PRIGROUP.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nvic_ipr4](index.html) module"]
pub struct NVIC_IPR4_SPEC;
impl crate::RegisterSpec for NVIC_IPR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nvic_ipr4::R](R) reader structure"]
impl crate::Readable for NVIC_IPR4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nvic_ipr4::W](W) writer structure"]
impl crate::Writable for NVIC_IPR4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NVIC_IPR4 to value 0"]
impl crate::Resettable for NVIC_IPR4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
