#[doc = "Register `NVIC_IPR0` reader"]
pub struct R(crate::R<NVIC_IPR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NVIC_IPR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NVIC_IPR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NVIC_IPR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NVIC_IPR0` writer"]
pub struct W(crate::W<NVIC_IPR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NVIC_IPR0_SPEC>;
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
impl From<crate::W<NVIC_IPR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NVIC_IPR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRI_0` reader - 7:0\\]
Priority of interrupt 0 (See EVENT:CPUIRQSEL0.EV for details)."]
pub type PRI_0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRI_0` writer - 7:0\\]
Priority of interrupt 0 (See EVENT:CPUIRQSEL0.EV for details)."]
pub type PRI_0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NVIC_IPR0_SPEC, u8, u8, 8, O>;
#[doc = "Field `PRI_1` reader - 15:8\\]
Priority of interrupt 1 (See EVENT:CPUIRQSEL1.EV for details)."]
pub type PRI_1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRI_1` writer - 15:8\\]
Priority of interrupt 1 (See EVENT:CPUIRQSEL1.EV for details)."]
pub type PRI_1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NVIC_IPR0_SPEC, u8, u8, 8, O>;
#[doc = "Field `PRI_2` reader - 23:16\\]
Priority of interrupt 2 (See EVENT:CPUIRQSEL2.EV for details)."]
pub type PRI_2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRI_2` writer - 23:16\\]
Priority of interrupt 2 (See EVENT:CPUIRQSEL2.EV for details)."]
pub type PRI_2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NVIC_IPR0_SPEC, u8, u8, 8, O>;
#[doc = "Field `PRI_3` reader - 31:24\\]
Priority of interrupt 3 (See EVENT:CPUIRQSEL3.EV for details)."]
pub type PRI_3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRI_3` writer - 31:24\\]
Priority of interrupt 3 (See EVENT:CPUIRQSEL3.EV for details)."]
pub type PRI_3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NVIC_IPR0_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Priority of interrupt 0 (See EVENT:CPUIRQSEL0.EV for details)."]
    #[inline(always)]
    pub fn pri_0(&self) -> PRI_0_R {
        PRI_0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Priority of interrupt 1 (See EVENT:CPUIRQSEL1.EV for details)."]
    #[inline(always)]
    pub fn pri_1(&self) -> PRI_1_R {
        PRI_1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Priority of interrupt 2 (See EVENT:CPUIRQSEL2.EV for details)."]
    #[inline(always)]
    pub fn pri_2(&self) -> PRI_2_R {
        PRI_2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Priority of interrupt 3 (See EVENT:CPUIRQSEL3.EV for details)."]
    #[inline(always)]
    pub fn pri_3(&self) -> PRI_3_R {
        PRI_3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Priority of interrupt 0 (See EVENT:CPUIRQSEL0.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn pri_0(&mut self) -> PRI_0_W<0> {
        PRI_0_W::new(self)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Priority of interrupt 1 (See EVENT:CPUIRQSEL1.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn pri_1(&mut self) -> PRI_1_W<8> {
        PRI_1_W::new(self)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Priority of interrupt 2 (See EVENT:CPUIRQSEL2.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn pri_2(&mut self) -> PRI_2_W<16> {
        PRI_2_W::new(self)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Priority of interrupt 3 (See EVENT:CPUIRQSEL3.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn pri_3(&mut self) -> PRI_3_W<24> {
        PRI_3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Irq 0 to 3 Priority This register is used to assign a priority from 0 to 255 to each of the available interrupts. 0 is the highest priority, and 255 is the lowest. The interpretation of the Interrupt Priority Registers changes based on the setting in AIRCR.PRIGROUP.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nvic_ipr0](index.html) module"]
pub struct NVIC_IPR0_SPEC;
impl crate::RegisterSpec for NVIC_IPR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nvic_ipr0::R](R) reader structure"]
impl crate::Readable for NVIC_IPR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nvic_ipr0::W](W) writer structure"]
impl crate::Writable for NVIC_IPR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NVIC_IPR0 to value 0"]
impl crate::Resettable for NVIC_IPR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
