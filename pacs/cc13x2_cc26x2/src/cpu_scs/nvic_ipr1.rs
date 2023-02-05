#[doc = "Register `NVIC_IPR1` reader"]
pub struct R(crate::R<NVIC_IPR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NVIC_IPR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NVIC_IPR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NVIC_IPR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NVIC_IPR1` writer"]
pub struct W(crate::W<NVIC_IPR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NVIC_IPR1_SPEC>;
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
impl From<crate::W<NVIC_IPR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NVIC_IPR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRI_4` reader - 7:0\\]
Priority of interrupt 4 (See EVENT:CPUIRQSEL4.EV for details)."]
pub type PRI_4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRI_4` writer - 7:0\\]
Priority of interrupt 4 (See EVENT:CPUIRQSEL4.EV for details)."]
pub type PRI_4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NVIC_IPR1_SPEC, u8, u8, 8, O>;
#[doc = "Field `PRI_5` reader - 15:8\\]
Priority of interrupt 5 (See EVENT:CPUIRQSEL5.EV for details)."]
pub type PRI_5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRI_5` writer - 15:8\\]
Priority of interrupt 5 (See EVENT:CPUIRQSEL5.EV for details)."]
pub type PRI_5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NVIC_IPR1_SPEC, u8, u8, 8, O>;
#[doc = "Field `PRI_6` reader - 23:16\\]
Priority of interrupt 6 (See EVENT:CPUIRQSEL6.EV for details)."]
pub type PRI_6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRI_6` writer - 23:16\\]
Priority of interrupt 6 (See EVENT:CPUIRQSEL6.EV for details)."]
pub type PRI_6_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NVIC_IPR1_SPEC, u8, u8, 8, O>;
#[doc = "Field `PRI_7` reader - 31:24\\]
Priority of interrupt 7 (See EVENT:CPUIRQSEL7.EV for details)."]
pub type PRI_7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRI_7` writer - 31:24\\]
Priority of interrupt 7 (See EVENT:CPUIRQSEL7.EV for details)."]
pub type PRI_7_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NVIC_IPR1_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Priority of interrupt 4 (See EVENT:CPUIRQSEL4.EV for details)."]
    #[inline(always)]
    pub fn pri_4(&self) -> PRI_4_R {
        PRI_4_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Priority of interrupt 5 (See EVENT:CPUIRQSEL5.EV for details)."]
    #[inline(always)]
    pub fn pri_5(&self) -> PRI_5_R {
        PRI_5_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Priority of interrupt 6 (See EVENT:CPUIRQSEL6.EV for details)."]
    #[inline(always)]
    pub fn pri_6(&self) -> PRI_6_R {
        PRI_6_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Priority of interrupt 7 (See EVENT:CPUIRQSEL7.EV for details)."]
    #[inline(always)]
    pub fn pri_7(&self) -> PRI_7_R {
        PRI_7_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Priority of interrupt 4 (See EVENT:CPUIRQSEL4.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn pri_4(&mut self) -> PRI_4_W<0> {
        PRI_4_W::new(self)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Priority of interrupt 5 (See EVENT:CPUIRQSEL5.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn pri_5(&mut self) -> PRI_5_W<8> {
        PRI_5_W::new(self)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Priority of interrupt 6 (See EVENT:CPUIRQSEL6.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn pri_6(&mut self) -> PRI_6_W<16> {
        PRI_6_W::new(self)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Priority of interrupt 7 (See EVENT:CPUIRQSEL7.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn pri_7(&mut self) -> PRI_7_W<24> {
        PRI_7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Irq 4 to 7 Priority This register is used to assign a priority from 0 to 255 to each of the available interrupts. 0 is the highest priority, and 255 is the lowest. The interpretation of the Interrupt Priority Registers changes based on the setting in AIRCR.PRIGROUP.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nvic_ipr1](index.html) module"]
pub struct NVIC_IPR1_SPEC;
impl crate::RegisterSpec for NVIC_IPR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nvic_ipr1::R](R) reader structure"]
impl crate::Readable for NVIC_IPR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nvic_ipr1::W](W) writer structure"]
impl crate::Writable for NVIC_IPR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NVIC_IPR1 to value 0"]
impl crate::Resettable for NVIC_IPR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
