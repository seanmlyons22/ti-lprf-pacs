#[doc = "Register `NVIC_IPR5` reader"]
pub struct R(crate::R<NVIC_IPR5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NVIC_IPR5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NVIC_IPR5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NVIC_IPR5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NVIC_IPR5` writer"]
pub struct W(crate::W<NVIC_IPR5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NVIC_IPR5_SPEC>;
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
impl From<crate::W<NVIC_IPR5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NVIC_IPR5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRI_20` reader - 7:0\\]
Priority of interrupt 20 (See EVENT:CPUIRQSEL20.EV for details)."]
pub type PRI_20_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRI_20` writer - 7:0\\]
Priority of interrupt 20 (See EVENT:CPUIRQSEL20.EV for details)."]
pub type PRI_20_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NVIC_IPR5_SPEC, u8, u8, 8, O>;
#[doc = "Field `PRI_21` reader - 15:8\\]
Priority of interrupt 21 (See EVENT:CPUIRQSEL21.EV for details)."]
pub type PRI_21_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRI_21` writer - 15:8\\]
Priority of interrupt 21 (See EVENT:CPUIRQSEL21.EV for details)."]
pub type PRI_21_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NVIC_IPR5_SPEC, u8, u8, 8, O>;
#[doc = "Field `PRI_22` reader - 23:16\\]
Priority of interrupt 22 (See EVENT:CPUIRQSEL22.EV for details)."]
pub type PRI_22_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRI_22` writer - 23:16\\]
Priority of interrupt 22 (See EVENT:CPUIRQSEL22.EV for details)."]
pub type PRI_22_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NVIC_IPR5_SPEC, u8, u8, 8, O>;
#[doc = "Field `PRI_23` reader - 31:24\\]
Priority of interrupt 23 (See EVENT:CPUIRQSEL23.EV for details)."]
pub type PRI_23_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRI_23` writer - 31:24\\]
Priority of interrupt 23 (See EVENT:CPUIRQSEL23.EV for details)."]
pub type PRI_23_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NVIC_IPR5_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Priority of interrupt 20 (See EVENT:CPUIRQSEL20.EV for details)."]
    #[inline(always)]
    pub fn pri_20(&self) -> PRI_20_R {
        PRI_20_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Priority of interrupt 21 (See EVENT:CPUIRQSEL21.EV for details)."]
    #[inline(always)]
    pub fn pri_21(&self) -> PRI_21_R {
        PRI_21_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Priority of interrupt 22 (See EVENT:CPUIRQSEL22.EV for details)."]
    #[inline(always)]
    pub fn pri_22(&self) -> PRI_22_R {
        PRI_22_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Priority of interrupt 23 (See EVENT:CPUIRQSEL23.EV for details)."]
    #[inline(always)]
    pub fn pri_23(&self) -> PRI_23_R {
        PRI_23_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Priority of interrupt 20 (See EVENT:CPUIRQSEL20.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn pri_20(&mut self) -> PRI_20_W<0> {
        PRI_20_W::new(self)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Priority of interrupt 21 (See EVENT:CPUIRQSEL21.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn pri_21(&mut self) -> PRI_21_W<8> {
        PRI_21_W::new(self)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Priority of interrupt 22 (See EVENT:CPUIRQSEL22.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn pri_22(&mut self) -> PRI_22_W<16> {
        PRI_22_W::new(self)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Priority of interrupt 23 (See EVENT:CPUIRQSEL23.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn pri_23(&mut self) -> PRI_23_W<24> {
        PRI_23_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Irq 20 to 23 Priority This register is used to assign a priority from 0 to 255 to each of the available interrupts. 0 is the highest priority, and 255 is the lowest. The interpretation of the Interrupt Priority Registers changes based on the setting in AIRCR.PRIGROUP.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nvic_ipr5](index.html) module"]
pub struct NVIC_IPR5_SPEC;
impl crate::RegisterSpec for NVIC_IPR5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nvic_ipr5::R](R) reader structure"]
impl crate::Readable for NVIC_IPR5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nvic_ipr5::W](W) writer structure"]
impl crate::Writable for NVIC_IPR5_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NVIC_IPR5 to value 0"]
impl crate::Resettable for NVIC_IPR5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
