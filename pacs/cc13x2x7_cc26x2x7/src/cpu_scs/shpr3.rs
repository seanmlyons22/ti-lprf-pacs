#[doc = "Register `SHPR3` reader"]
pub struct R(crate::R<SHPR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SHPR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SHPR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SHPR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SHPR3` writer"]
pub struct W(crate::W<SHPR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SHPR3_SPEC>;
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
impl From<crate::W<SHPR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SHPR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRI_12` reader - 7:0\\]
Priority of system handler 12. Debug Monitor"]
pub type PRI_12_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRI_12` writer - 7:0\\]
Priority of system handler 12. Debug Monitor"]
pub type PRI_12_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SHPR3_SPEC, u8, u8, 8, O>;
#[doc = "Field `RESERVED8` reader - 15:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED8_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED8` writer - 15:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED8_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SHPR3_SPEC, u8, u8, 8, O>;
#[doc = "Field `PRI_14` reader - 23:16\\]
Priority of system handler 14. Pend SV"]
pub type PRI_14_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRI_14` writer - 23:16\\]
Priority of system handler 14. Pend SV"]
pub type PRI_14_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SHPR3_SPEC, u8, u8, 8, O>;
#[doc = "Field `PRI_15` reader - 31:24\\]
Priority of system handler 15. SysTick exception"]
pub type PRI_15_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRI_15` writer - 31:24\\]
Priority of system handler 15. SysTick exception"]
pub type PRI_15_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SHPR3_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Priority of system handler 12. Debug Monitor"]
    #[inline(always)]
    pub fn pri_12(&self) -> PRI_12_R {
        PRI_12_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&self) -> RESERVED8_R {
        RESERVED8_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Priority of system handler 14. Pend SV"]
    #[inline(always)]
    pub fn pri_14(&self) -> PRI_14_R {
        PRI_14_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Priority of system handler 15. SysTick exception"]
    #[inline(always)]
    pub fn pri_15(&self) -> PRI_15_R {
        PRI_15_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Priority of system handler 12. Debug Monitor"]
    #[inline(always)]
    #[must_use]
    pub fn pri_12(&mut self) -> PRI_12_W<0> {
        PRI_12_W::new(self)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved8(&mut self) -> RESERVED8_W<8> {
        RESERVED8_W::new(self)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Priority of system handler 14. Pend SV"]
    #[inline(always)]
    #[must_use]
    pub fn pri_14(&mut self) -> PRI_14_W<16> {
        PRI_14_W::new(self)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Priority of system handler 15. SysTick exception"]
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
#[doc = "System Handlers 12-15 Priority This register is used to prioritize the following system handlers: SysTick, PendSV and Debug Monitor. System Handlers are a special class of exception handler that can have their priority set to any of the priority levels. Most can be masked on (enabled) or off (disabled). When disabled, the fault is always treated as a Hard Fault.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shpr3](index.html) module"]
pub struct SHPR3_SPEC;
impl crate::RegisterSpec for SHPR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [shpr3::R](R) reader structure"]
impl crate::Readable for SHPR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [shpr3::W](W) writer structure"]
impl crate::Writable for SHPR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SHPR3 to value 0"]
impl crate::Resettable for SHPR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
