#[doc = "Register `NVIC_IPR2` reader"]
pub struct R(crate::R<NVIC_IPR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NVIC_IPR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NVIC_IPR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NVIC_IPR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NVIC_IPR2` writer"]
pub struct W(crate::W<NVIC_IPR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NVIC_IPR2_SPEC>;
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
impl From<crate::W<NVIC_IPR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NVIC_IPR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRI_8` reader - 7:0\\]
Priority of interrupt 8 (See EVENT:CPUIRQSEL8.EV for details)."]
pub type PRI_8_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRI_8` writer - 7:0\\]
Priority of interrupt 8 (See EVENT:CPUIRQSEL8.EV for details)."]
pub type PRI_8_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NVIC_IPR2_SPEC, u8, u8, 8, O>;
#[doc = "Field `PRI_9` reader - 15:8\\]
Priority of interrupt 9 (See EVENT:CPUIRQSEL9.EV for details)."]
pub type PRI_9_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRI_9` writer - 15:8\\]
Priority of interrupt 9 (See EVENT:CPUIRQSEL9.EV for details)."]
pub type PRI_9_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NVIC_IPR2_SPEC, u8, u8, 8, O>;
#[doc = "Field `PRI_10` reader - 23:16\\]
Priority of interrupt 10 (See EVENT:CPUIRQSEL10.EV for details)."]
pub type PRI_10_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRI_10` writer - 23:16\\]
Priority of interrupt 10 (See EVENT:CPUIRQSEL10.EV for details)."]
pub type PRI_10_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NVIC_IPR2_SPEC, u8, u8, 8, O>;
#[doc = "Field `PRI_11` reader - 31:24\\]
Priority of interrupt 11 (See EVENT:CPUIRQSEL11.EV for details)."]
pub type PRI_11_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRI_11` writer - 31:24\\]
Priority of interrupt 11 (See EVENT:CPUIRQSEL11.EV for details)."]
pub type PRI_11_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NVIC_IPR2_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Priority of interrupt 8 (See EVENT:CPUIRQSEL8.EV for details)."]
    #[inline(always)]
    pub fn pri_8(&self) -> PRI_8_R {
        PRI_8_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Priority of interrupt 9 (See EVENT:CPUIRQSEL9.EV for details)."]
    #[inline(always)]
    pub fn pri_9(&self) -> PRI_9_R {
        PRI_9_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Priority of interrupt 10 (See EVENT:CPUIRQSEL10.EV for details)."]
    #[inline(always)]
    pub fn pri_10(&self) -> PRI_10_R {
        PRI_10_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Priority of interrupt 11 (See EVENT:CPUIRQSEL11.EV for details)."]
    #[inline(always)]
    pub fn pri_11(&self) -> PRI_11_R {
        PRI_11_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Priority of interrupt 8 (See EVENT:CPUIRQSEL8.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn pri_8(&mut self) -> PRI_8_W<0> {
        PRI_8_W::new(self)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Priority of interrupt 9 (See EVENT:CPUIRQSEL9.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn pri_9(&mut self) -> PRI_9_W<8> {
        PRI_9_W::new(self)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Priority of interrupt 10 (See EVENT:CPUIRQSEL10.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn pri_10(&mut self) -> PRI_10_W<16> {
        PRI_10_W::new(self)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Priority of interrupt 11 (See EVENT:CPUIRQSEL11.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn pri_11(&mut self) -> PRI_11_W<24> {
        PRI_11_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Irq 8 to 11 Priority This register is used to assign a priority from 0 to 255 to each of the available interrupts. 0 is the highest priority, and 255 is the lowest. The interpretation of the Interrupt Priority Registers changes based on the setting in AIRCR.PRIGROUP.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nvic_ipr2](index.html) module"]
pub struct NVIC_IPR2_SPEC;
impl crate::RegisterSpec for NVIC_IPR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nvic_ipr2::R](R) reader structure"]
impl crate::Readable for NVIC_IPR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nvic_ipr2::W](W) writer structure"]
impl crate::Writable for NVIC_IPR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NVIC_IPR2 to value 0"]
impl crate::Resettable for NVIC_IPR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
