#[doc = "Register `NVIC_IPR7` reader"]
pub struct R(crate::R<NVIC_IPR7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NVIC_IPR7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NVIC_IPR7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NVIC_IPR7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NVIC_IPR7` writer"]
pub struct W(crate::W<NVIC_IPR7_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NVIC_IPR7_SPEC>;
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
impl From<crate::W<NVIC_IPR7_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NVIC_IPR7_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRI_28` reader - 7:0\\]
Priority of interrupt 28 (See EVENT:CPUIRQSEL28.EV for details)."]
pub type PRI_28_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRI_28` writer - 7:0\\]
Priority of interrupt 28 (See EVENT:CPUIRQSEL28.EV for details)."]
pub type PRI_28_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NVIC_IPR7_SPEC, u8, u8, 8, O>;
#[doc = "Field `PRI_29` reader - 15:8\\]
Priority of interrupt 29 (See EVENT:CPUIRQSEL29.EV for details)."]
pub type PRI_29_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRI_29` writer - 15:8\\]
Priority of interrupt 29 (See EVENT:CPUIRQSEL29.EV for details)."]
pub type PRI_29_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NVIC_IPR7_SPEC, u8, u8, 8, O>;
#[doc = "Field `PRI_30` reader - 23:16\\]
Priority of interrupt 30 (See EVENT:CPUIRQSEL30.EV for details)."]
pub type PRI_30_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRI_30` writer - 23:16\\]
Priority of interrupt 30 (See EVENT:CPUIRQSEL30.EV for details)."]
pub type PRI_30_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NVIC_IPR7_SPEC, u8, u8, 8, O>;
#[doc = "Field `PRI_31` reader - 31:24\\]
Priority of interrupt 31 (See EVENT:CPUIRQSEL31.EV for details)."]
pub type PRI_31_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRI_31` writer - 31:24\\]
Priority of interrupt 31 (See EVENT:CPUIRQSEL31.EV for details)."]
pub type PRI_31_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NVIC_IPR7_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Priority of interrupt 28 (See EVENT:CPUIRQSEL28.EV for details)."]
    #[inline(always)]
    pub fn pri_28(&self) -> PRI_28_R {
        PRI_28_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Priority of interrupt 29 (See EVENT:CPUIRQSEL29.EV for details)."]
    #[inline(always)]
    pub fn pri_29(&self) -> PRI_29_R {
        PRI_29_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Priority of interrupt 30 (See EVENT:CPUIRQSEL30.EV for details)."]
    #[inline(always)]
    pub fn pri_30(&self) -> PRI_30_R {
        PRI_30_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Priority of interrupt 31 (See EVENT:CPUIRQSEL31.EV for details)."]
    #[inline(always)]
    pub fn pri_31(&self) -> PRI_31_R {
        PRI_31_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Priority of interrupt 28 (See EVENT:CPUIRQSEL28.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn pri_28(&mut self) -> PRI_28_W<0> {
        PRI_28_W::new(self)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Priority of interrupt 29 (See EVENT:CPUIRQSEL29.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn pri_29(&mut self) -> PRI_29_W<8> {
        PRI_29_W::new(self)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Priority of interrupt 30 (See EVENT:CPUIRQSEL30.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn pri_30(&mut self) -> PRI_30_W<16> {
        PRI_30_W::new(self)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Priority of interrupt 31 (See EVENT:CPUIRQSEL31.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn pri_31(&mut self) -> PRI_31_W<24> {
        PRI_31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Irq 28 to 31 Priority This register is used to assign a priority from 0 to 255 to each of the available interrupts. 0 is the highest priority, and 255 is the lowest. The interpretation of the Interrupt Priority Registers changes based on the setting in AIRCR.PRIGROUP.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nvic_ipr7](index.html) module"]
pub struct NVIC_IPR7_SPEC;
impl crate::RegisterSpec for NVIC_IPR7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nvic_ipr7::R](R) reader structure"]
impl crate::Readable for NVIC_IPR7_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nvic_ipr7::W](W) writer structure"]
impl crate::Writable for NVIC_IPR7_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NVIC_IPR7 to value 0"]
impl crate::Resettable for NVIC_IPR7_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
