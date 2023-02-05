#[doc = "Register `NVIC_IPR6` reader"]
pub struct R(crate::R<NVIC_IPR6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NVIC_IPR6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NVIC_IPR6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NVIC_IPR6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NVIC_IPR6` writer"]
pub struct W(crate::W<NVIC_IPR6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NVIC_IPR6_SPEC>;
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
impl From<crate::W<NVIC_IPR6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NVIC_IPR6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRI_24` reader - 7:0\\]
Priority of interrupt 24 (See EVENT:CPUIRQSEL24.EV for details)."]
pub type PRI_24_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRI_24` writer - 7:0\\]
Priority of interrupt 24 (See EVENT:CPUIRQSEL24.EV for details)."]
pub type PRI_24_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NVIC_IPR6_SPEC, u8, u8, 8, O>;
#[doc = "Field `PRI_25` reader - 15:8\\]
Priority of interrupt 25 (See EVENT:CPUIRQSEL25.EV for details)."]
pub type PRI_25_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRI_25` writer - 15:8\\]
Priority of interrupt 25 (See EVENT:CPUIRQSEL25.EV for details)."]
pub type PRI_25_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NVIC_IPR6_SPEC, u8, u8, 8, O>;
#[doc = "Field `PRI_26` reader - 23:16\\]
Priority of interrupt 26 (See EVENT:CPUIRQSEL26.EV for details)."]
pub type PRI_26_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRI_26` writer - 23:16\\]
Priority of interrupt 26 (See EVENT:CPUIRQSEL26.EV for details)."]
pub type PRI_26_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NVIC_IPR6_SPEC, u8, u8, 8, O>;
#[doc = "Field `PRI_27` reader - 31:24\\]
Priority of interrupt 27 (See EVENT:CPUIRQSEL27.EV for details)."]
pub type PRI_27_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRI_27` writer - 31:24\\]
Priority of interrupt 27 (See EVENT:CPUIRQSEL27.EV for details)."]
pub type PRI_27_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NVIC_IPR6_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Priority of interrupt 24 (See EVENT:CPUIRQSEL24.EV for details)."]
    #[inline(always)]
    pub fn pri_24(&self) -> PRI_24_R {
        PRI_24_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Priority of interrupt 25 (See EVENT:CPUIRQSEL25.EV for details)."]
    #[inline(always)]
    pub fn pri_25(&self) -> PRI_25_R {
        PRI_25_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Priority of interrupt 26 (See EVENT:CPUIRQSEL26.EV for details)."]
    #[inline(always)]
    pub fn pri_26(&self) -> PRI_26_R {
        PRI_26_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Priority of interrupt 27 (See EVENT:CPUIRQSEL27.EV for details)."]
    #[inline(always)]
    pub fn pri_27(&self) -> PRI_27_R {
        PRI_27_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Priority of interrupt 24 (See EVENT:CPUIRQSEL24.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn pri_24(&mut self) -> PRI_24_W<0> {
        PRI_24_W::new(self)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Priority of interrupt 25 (See EVENT:CPUIRQSEL25.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn pri_25(&mut self) -> PRI_25_W<8> {
        PRI_25_W::new(self)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Priority of interrupt 26 (See EVENT:CPUIRQSEL26.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn pri_26(&mut self) -> PRI_26_W<16> {
        PRI_26_W::new(self)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Priority of interrupt 27 (See EVENT:CPUIRQSEL27.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn pri_27(&mut self) -> PRI_27_W<24> {
        PRI_27_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Irq 24 to 27 Priority This register is used to assign a priority from 0 to 255 to each of the available interrupts. 0 is the highest priority, and 255 is the lowest. The interpretation of the Interrupt Priority Registers changes based on the setting in AIRCR.PRIGROUP.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nvic_ipr6](index.html) module"]
pub struct NVIC_IPR6_SPEC;
impl crate::RegisterSpec for NVIC_IPR6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nvic_ipr6::R](R) reader structure"]
impl crate::Readable for NVIC_IPR6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nvic_ipr6::W](W) writer structure"]
impl crate::Writable for NVIC_IPR6_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NVIC_IPR6 to value 0"]
impl crate::Resettable for NVIC_IPR6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
