#[doc = "Register `NVIC_IPR3` reader"]
pub struct R(crate::R<NVIC_IPR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NVIC_IPR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NVIC_IPR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NVIC_IPR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NVIC_IPR3` writer"]
pub struct W(crate::W<NVIC_IPR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NVIC_IPR3_SPEC>;
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
impl From<crate::W<NVIC_IPR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NVIC_IPR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRI_12` reader - 7:0\\]
Priority of interrupt 12 (See EVENT:CPUIRQSEL12.EV for details)."]
pub type PRI_12_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRI_12` writer - 7:0\\]
Priority of interrupt 12 (See EVENT:CPUIRQSEL12.EV for details)."]
pub type PRI_12_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NVIC_IPR3_SPEC, u8, u8, 8, O>;
#[doc = "Field `PRI_13` reader - 15:8\\]
Priority of interrupt 13 (See EVENT:CPUIRQSEL13.EV for details)."]
pub type PRI_13_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRI_13` writer - 15:8\\]
Priority of interrupt 13 (See EVENT:CPUIRQSEL13.EV for details)."]
pub type PRI_13_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NVIC_IPR3_SPEC, u8, u8, 8, O>;
#[doc = "Field `PRI_14` reader - 23:16\\]
Priority of interrupt 14 (See EVENT:CPUIRQSEL14.EV for details)."]
pub type PRI_14_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRI_14` writer - 23:16\\]
Priority of interrupt 14 (See EVENT:CPUIRQSEL14.EV for details)."]
pub type PRI_14_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NVIC_IPR3_SPEC, u8, u8, 8, O>;
#[doc = "Field `PRI_15` reader - 31:24\\]
Priority of interrupt 15 (See EVENT:CPUIRQSEL15.EV for details)."]
pub type PRI_15_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRI_15` writer - 31:24\\]
Priority of interrupt 15 (See EVENT:CPUIRQSEL15.EV for details)."]
pub type PRI_15_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NVIC_IPR3_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Priority of interrupt 12 (See EVENT:CPUIRQSEL12.EV for details)."]
    #[inline(always)]
    pub fn pri_12(&self) -> PRI_12_R {
        PRI_12_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Priority of interrupt 13 (See EVENT:CPUIRQSEL13.EV for details)."]
    #[inline(always)]
    pub fn pri_13(&self) -> PRI_13_R {
        PRI_13_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Priority of interrupt 14 (See EVENT:CPUIRQSEL14.EV for details)."]
    #[inline(always)]
    pub fn pri_14(&self) -> PRI_14_R {
        PRI_14_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Priority of interrupt 15 (See EVENT:CPUIRQSEL15.EV for details)."]
    #[inline(always)]
    pub fn pri_15(&self) -> PRI_15_R {
        PRI_15_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Priority of interrupt 12 (See EVENT:CPUIRQSEL12.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn pri_12(&mut self) -> PRI_12_W<0> {
        PRI_12_W::new(self)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Priority of interrupt 13 (See EVENT:CPUIRQSEL13.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn pri_13(&mut self) -> PRI_13_W<8> {
        PRI_13_W::new(self)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Priority of interrupt 14 (See EVENT:CPUIRQSEL14.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn pri_14(&mut self) -> PRI_14_W<16> {
        PRI_14_W::new(self)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Priority of interrupt 15 (See EVENT:CPUIRQSEL15.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn pri_15(&mut self) -> PRI_15_W<24> {
        PRI_15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Irq 12 to 15 Priority This register is used to assign a priority from 0 to 255 to each of the available interrupts. 0 is the highest priority, and 255 is the lowest. The interpretation of the Interrupt Priority Registers changes based on the setting in AIRCR.PRIGROUP.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nvic_ipr3](index.html) module"]
pub struct NVIC_IPR3_SPEC;
impl crate::RegisterSpec for NVIC_IPR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nvic_ipr3::R](R) reader structure"]
impl crate::Readable for NVIC_IPR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nvic_ipr3::W](W) writer structure"]
impl crate::Writable for NVIC_IPR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NVIC_IPR3 to value 0"]
impl crate::Resettable for NVIC_IPR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
