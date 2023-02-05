#[doc = "Register `CFG` reader"]
pub struct R(crate::R<CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG` writer"]
pub struct W(crate::W<CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG_SPEC>;
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
impl From<crate::W<CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CFG` reader - 2:0\\]
GPT Configuration 0x2- 0x3 - Reserved 0x5- 0x7 - Reserved"]
pub type CFG_R = crate::FieldReader<u8, CFG_A>;
#[doc = "2:0\\]
GPT Configuration 0x2- 0x3 - Reserved 0x5- 0x7 - Reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CFG_A {
    #[doc = "4: 16-bit timer configuration. Configure for two 16-bit timers. Also see TAMR.TAMR and TBMR.TBMR."]
    _16BIT_TIMER = 4,
    #[doc = "0: 32-bit timer configuration"]
    _32BIT_TIMER = 0,
}
impl From<CFG_A> for u8 {
    #[inline(always)]
    fn from(variant: CFG_A) -> Self {
        variant as _
    }
}
impl CFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CFG_A> {
        match self.bits {
            4 => Some(CFG_A::_16BIT_TIMER),
            0 => Some(CFG_A::_32BIT_TIMER),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_16BIT_TIMER`"]
    #[inline(always)]
    pub fn is_16bit_timer(&self) -> bool {
        *self == CFG_A::_16BIT_TIMER
    }
    #[doc = "Checks if the value of the field is `_32BIT_TIMER`"]
    #[inline(always)]
    pub fn is_32bit_timer(&self) -> bool {
        *self == CFG_A::_32BIT_TIMER
    }
}
#[doc = "Field `CFG` writer - 2:0\\]
GPT Configuration 0x2- 0x3 - Reserved 0x5- 0x7 - Reserved"]
pub type CFG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG_SPEC, u8, CFG_A, 3, O>;
impl<'a, const O: u8> CFG_W<'a, O> {
    #[doc = "16-bit timer configuration. Configure for two 16-bit timers. Also see TAMR.TAMR and TBMR.TBMR."]
    #[inline(always)]
    pub fn _16bit_timer(self) -> &'a mut W {
        self.variant(CFG_A::_16BIT_TIMER)
    }
    #[doc = "32-bit timer configuration"]
    #[inline(always)]
    pub fn _32bit_timer(self) -> &'a mut W {
        self.variant(CFG_A::_32BIT_TIMER)
    }
}
#[doc = "Field `RESERVED3` reader - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED3_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED3` writer - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG_SPEC, u32, u32, 29, O>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
GPT Configuration 0x2- 0x3 - Reserved 0x5- 0x7 - Reserved"]
    #[inline(always)]
    pub fn cfg(&self) -> CFG_R {
        CFG_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:31 - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> RESERVED3_R {
        RESERVED3_R::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
GPT Configuration 0x2- 0x3 - Reserved 0x5- 0x7 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn cfg(&mut self) -> CFG_W<0> {
        CFG_W::new(self)
    }
    #[doc = "Bits 3:31 - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> RESERVED3_W<3> {
        RESERVED3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG_SPEC;
impl crate::RegisterSpec for CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg::R](R) reader structure"]
impl crate::Readable for CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg::W](W) writer structure"]
impl crate::Writable for CFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFG to value 0"]
impl crate::Resettable for CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
