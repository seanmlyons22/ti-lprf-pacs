#[doc = "Register `TPR` reader"]
pub struct R(crate::R<TPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TPR` writer"]
pub struct W(crate::W<TPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TPR_SPEC>;
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
impl From<crate::W<TPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRIVMASK` reader - 3:0\\]
Bit mask to enable unprivileged (User) access to ITM stimulus ports: Bit \\[0\\]
enables stimulus ports 0, 1, ..., and 7. Bit \\[1\\]
enables stimulus ports 8, 9, ..., and 15. Bit \\[2\\]
enables stimulus ports 16, 17, ..., and 23. Bit \\[3\\]
enables stimulus ports 24, 25, ..., and 31. 0: User access allowed to stimulus ports 1: Privileged access only to stimulus ports"]
pub type PRIVMASK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRIVMASK` writer - 3:0\\]
Bit mask to enable unprivileged (User) access to ITM stimulus ports: Bit \\[0\\]
enables stimulus ports 0, 1, ..., and 7. Bit \\[1\\]
enables stimulus ports 8, 9, ..., and 15. Bit \\[2\\]
enables stimulus ports 16, 17, ..., and 23. Bit \\[3\\]
enables stimulus ports 24, 25, ..., and 31. 0: User access allowed to stimulus ports 1: Privileged access only to stimulus ports"]
pub type PRIVMASK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TPR_SPEC, u8, u8, 4, O>;
#[doc = "Field `RESERVED4` reader - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED4_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED4` writer - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TPR_SPEC, u32, u32, 28, O>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Bit mask to enable unprivileged (User) access to ITM stimulus ports: Bit \\[0\\]
enables stimulus ports 0, 1, ..., and 7. Bit \\[1\\]
enables stimulus ports 8, 9, ..., and 15. Bit \\[2\\]
enables stimulus ports 16, 17, ..., and 23. Bit \\[3\\]
enables stimulus ports 24, 25, ..., and 31. 0: User access allowed to stimulus ports 1: Privileged access only to stimulus ports"]
    #[inline(always)]
    pub fn privmask(&self) -> PRIVMASK_R {
        PRIVMASK_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:31 - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved4(&self) -> RESERVED4_R {
        RESERVED4_R::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Bit mask to enable unprivileged (User) access to ITM stimulus ports: Bit \\[0\\]
enables stimulus ports 0, 1, ..., and 7. Bit \\[1\\]
enables stimulus ports 8, 9, ..., and 15. Bit \\[2\\]
enables stimulus ports 16, 17, ..., and 23. Bit \\[3\\]
enables stimulus ports 24, 25, ..., and 31. 0: User access allowed to stimulus ports 1: Privileged access only to stimulus ports"]
    #[inline(always)]
    #[must_use]
    pub fn privmask(&mut self) -> PRIVMASK_W<0> {
        PRIVMASK_W::new(self)
    }
    #[doc = "Bits 4:31 - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved4(&mut self) -> RESERVED4_W<4> {
        RESERVED4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Trace Privilege This register is used to enable an operating system to control which stimulus ports are accessible by user code. This register can only be used in privileged mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tpr](index.html) module"]
pub struct TPR_SPEC;
impl crate::RegisterSpec for TPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tpr::R](R) reader structure"]
impl crate::Readable for TPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tpr::W](W) writer structure"]
impl crate::Writable for TPR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TPR to value 0"]
impl crate::Resettable for TPR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
