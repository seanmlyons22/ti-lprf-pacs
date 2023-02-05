#[doc = "Register `AIFDIRCFG` reader"]
pub struct R(crate::R<AIFDIRCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AIFDIRCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AIFDIRCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AIFDIRCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AIFDIRCFG` writer"]
pub struct W(crate::W<AIFDIRCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AIFDIRCFG_SPEC>;
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
impl From<crate::W<AIFDIRCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AIFDIRCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AD0` reader - 1:0\\]
Configures the AD0 audio data pin usage: 0x3: Reserved"]
pub type AD0_R = crate::FieldReader<u8, AD0_A>;
#[doc = "1:0\\]
Configures the AD0 audio data pin usage: 0x3: Reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AD0_A {
    #[doc = "2: Output mode"]
    OUT = 2,
    #[doc = "1: Input mode"]
    IN = 1,
    #[doc = "0: Not in use (disabled)"]
    DIS = 0,
}
impl From<AD0_A> for u8 {
    #[inline(always)]
    fn from(variant: AD0_A) -> Self {
        variant as _
    }
}
impl AD0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<AD0_A> {
        match self.bits {
            2 => Some(AD0_A::OUT),
            1 => Some(AD0_A::IN),
            0 => Some(AD0_A::DIS),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `OUT`"]
    #[inline(always)]
    pub fn is_out(&self) -> bool {
        *self == AD0_A::OUT
    }
    #[doc = "Checks if the value of the field is `IN`"]
    #[inline(always)]
    pub fn is_in(&self) -> bool {
        *self == AD0_A::IN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == AD0_A::DIS
    }
}
#[doc = "Field `AD0` writer - 1:0\\]
Configures the AD0 audio data pin usage: 0x3: Reserved"]
pub type AD0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AIFDIRCFG_SPEC, u8, AD0_A, 2, O>;
impl<'a, const O: u8> AD0_W<'a, O> {
    #[doc = "Output mode"]
    #[inline(always)]
    pub fn out(self) -> &'a mut W {
        self.variant(AD0_A::OUT)
    }
    #[doc = "Input mode"]
    #[inline(always)]
    pub fn in_(self) -> &'a mut W {
        self.variant(AD0_A::IN)
    }
    #[doc = "Not in use (disabled)"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(AD0_A::DIS)
    }
}
#[doc = "Field `RESERVED2` reader - 3:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED2` writer - 3:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AIFDIRCFG_SPEC, u8, u8, 2, O>;
#[doc = "Field `AD1` reader - 5:4\\]
Configures the AD1 audio data pin usage: 0x3: Reserved"]
pub type AD1_R = crate::FieldReader<u8, AD1_A>;
#[doc = "5:4\\]
Configures the AD1 audio data pin usage: 0x3: Reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AD1_A {
    #[doc = "2: Output mode"]
    OUT = 2,
    #[doc = "1: Input mode"]
    IN = 1,
    #[doc = "0: Not in use (disabled)"]
    DIS = 0,
}
impl From<AD1_A> for u8 {
    #[inline(always)]
    fn from(variant: AD1_A) -> Self {
        variant as _
    }
}
impl AD1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<AD1_A> {
        match self.bits {
            2 => Some(AD1_A::OUT),
            1 => Some(AD1_A::IN),
            0 => Some(AD1_A::DIS),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `OUT`"]
    #[inline(always)]
    pub fn is_out(&self) -> bool {
        *self == AD1_A::OUT
    }
    #[doc = "Checks if the value of the field is `IN`"]
    #[inline(always)]
    pub fn is_in(&self) -> bool {
        *self == AD1_A::IN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == AD1_A::DIS
    }
}
#[doc = "Field `AD1` writer - 5:4\\]
Configures the AD1 audio data pin usage: 0x3: Reserved"]
pub type AD1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AIFDIRCFG_SPEC, u8, AD1_A, 2, O>;
impl<'a, const O: u8> AD1_W<'a, O> {
    #[doc = "Output mode"]
    #[inline(always)]
    pub fn out(self) -> &'a mut W {
        self.variant(AD1_A::OUT)
    }
    #[doc = "Input mode"]
    #[inline(always)]
    pub fn in_(self) -> &'a mut W {
        self.variant(AD1_A::IN)
    }
    #[doc = "Not in use (disabled)"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(AD1_A::DIS)
    }
}
#[doc = "Field `RESERVED6` reader - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED6_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED6` writer - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED6_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, AIFDIRCFG_SPEC, u32, u32, 26, O>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Configures the AD0 audio data pin usage: 0x3: Reserved"]
    #[inline(always)]
    pub fn ad0(&self) -> AD0_R {
        AD0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - 3:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> RESERVED2_R {
        RESERVED2_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - 5:4\\]
Configures the AD1 audio data pin usage: 0x3: Reserved"]
    #[inline(always)]
    pub fn ad1(&self) -> AD1_R {
        AD1_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:31 - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved6(&self) -> RESERVED6_R {
        RESERVED6_R::new((self.bits >> 6) & 0x03ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Configures the AD0 audio data pin usage: 0x3: Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn ad0(&mut self) -> AD0_W<0> {
        AD0_W::new(self)
    }
    #[doc = "Bits 2:3 - 3:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> RESERVED2_W<2> {
        RESERVED2_W::new(self)
    }
    #[doc = "Bits 4:5 - 5:4\\]
Configures the AD1 audio data pin usage: 0x3: Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn ad1(&mut self) -> AD1_W<4> {
        AD1_W::new(self)
    }
    #[doc = "Bits 6:31 - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved6(&mut self) -> RESERVED6_W<6> {
        RESERVED6_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pin Direction\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aifdircfg](index.html) module"]
pub struct AIFDIRCFG_SPEC;
impl crate::RegisterSpec for AIFDIRCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [aifdircfg::R](R) reader structure"]
impl crate::Readable for AIFDIRCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [aifdircfg::W](W) writer structure"]
impl crate::Writable for AIFDIRCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AIFDIRCFG to value 0"]
impl crate::Resettable for AIFDIRCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
