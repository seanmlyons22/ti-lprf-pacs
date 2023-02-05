#[doc = "Register `RFCMODESEL` reader"]
pub struct R(crate::R<RFCMODESEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RFCMODESEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RFCMODESEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RFCMODESEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RFCMODESEL` writer"]
pub struct W(crate::W<RFCMODESEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RFCMODESEL_SPEC>;
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
impl From<crate::W<RFCMODESEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RFCMODESEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CURR` reader - 2:0\\]
Selects the set of commands that the RFC will accept. Only modes permitted by RFCMODEHWOPT.AVAIL are writeable. See the technical reference manual for details."]
pub type CURR_R = crate::FieldReader<u8, CURR_A>;
#[doc = "2:0\\]
Selects the set of commands that the RFC will accept. Only modes permitted by RFCMODEHWOPT.AVAIL are writeable. See the technical reference manual for details.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CURR_A {
    #[doc = "7: Select Mode 7"]
    MODE7 = 7,
    #[doc = "6: Select Mode 6"]
    MODE6 = 6,
    #[doc = "5: Select Mode 5"]
    MODE5 = 5,
    #[doc = "4: Select Mode 4"]
    MODE4 = 4,
    #[doc = "3: Select Mode 3"]
    MODE3 = 3,
    #[doc = "2: Select Mode 2"]
    MODE2 = 2,
    #[doc = "1: Select Mode 1"]
    MODE1 = 1,
    #[doc = "0: Select Mode 0"]
    MODE0 = 0,
}
impl From<CURR_A> for u8 {
    #[inline(always)]
    fn from(variant: CURR_A) -> Self {
        variant as _
    }
}
impl CURR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CURR_A {
        match self.bits {
            7 => CURR_A::MODE7,
            6 => CURR_A::MODE6,
            5 => CURR_A::MODE5,
            4 => CURR_A::MODE4,
            3 => CURR_A::MODE3,
            2 => CURR_A::MODE2,
            1 => CURR_A::MODE1,
            0 => CURR_A::MODE0,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `MODE7`"]
    #[inline(always)]
    pub fn is_mode7(&self) -> bool {
        *self == CURR_A::MODE7
    }
    #[doc = "Checks if the value of the field is `MODE6`"]
    #[inline(always)]
    pub fn is_mode6(&self) -> bool {
        *self == CURR_A::MODE6
    }
    #[doc = "Checks if the value of the field is `MODE5`"]
    #[inline(always)]
    pub fn is_mode5(&self) -> bool {
        *self == CURR_A::MODE5
    }
    #[doc = "Checks if the value of the field is `MODE4`"]
    #[inline(always)]
    pub fn is_mode4(&self) -> bool {
        *self == CURR_A::MODE4
    }
    #[doc = "Checks if the value of the field is `MODE3`"]
    #[inline(always)]
    pub fn is_mode3(&self) -> bool {
        *self == CURR_A::MODE3
    }
    #[doc = "Checks if the value of the field is `MODE2`"]
    #[inline(always)]
    pub fn is_mode2(&self) -> bool {
        *self == CURR_A::MODE2
    }
    #[doc = "Checks if the value of the field is `MODE1`"]
    #[inline(always)]
    pub fn is_mode1(&self) -> bool {
        *self == CURR_A::MODE1
    }
    #[doc = "Checks if the value of the field is `MODE0`"]
    #[inline(always)]
    pub fn is_mode0(&self) -> bool {
        *self == CURR_A::MODE0
    }
}
#[doc = "Field `CURR` writer - 2:0\\]
Selects the set of commands that the RFC will accept. Only modes permitted by RFCMODEHWOPT.AVAIL are writeable. See the technical reference manual for details."]
pub type CURR_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, RFCMODESEL_SPEC, u8, CURR_A, 3, O>;
impl<'a, const O: u8> CURR_W<'a, O> {
    #[doc = "Select Mode 7"]
    #[inline(always)]
    pub fn mode7(self) -> &'a mut W {
        self.variant(CURR_A::MODE7)
    }
    #[doc = "Select Mode 6"]
    #[inline(always)]
    pub fn mode6(self) -> &'a mut W {
        self.variant(CURR_A::MODE6)
    }
    #[doc = "Select Mode 5"]
    #[inline(always)]
    pub fn mode5(self) -> &'a mut W {
        self.variant(CURR_A::MODE5)
    }
    #[doc = "Select Mode 4"]
    #[inline(always)]
    pub fn mode4(self) -> &'a mut W {
        self.variant(CURR_A::MODE4)
    }
    #[doc = "Select Mode 3"]
    #[inline(always)]
    pub fn mode3(self) -> &'a mut W {
        self.variant(CURR_A::MODE3)
    }
    #[doc = "Select Mode 2"]
    #[inline(always)]
    pub fn mode2(self) -> &'a mut W {
        self.variant(CURR_A::MODE2)
    }
    #[doc = "Select Mode 1"]
    #[inline(always)]
    pub fn mode1(self) -> &'a mut W {
        self.variant(CURR_A::MODE1)
    }
    #[doc = "Select Mode 0"]
    #[inline(always)]
    pub fn mode0(self) -> &'a mut W {
        self.variant(CURR_A::MODE0)
    }
}
#[doc = "Field `RESERVED3` reader - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED3_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED3` writer - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED3_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RFCMODESEL_SPEC, u32, u32, 29, O>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Selects the set of commands that the RFC will accept. Only modes permitted by RFCMODEHWOPT.AVAIL are writeable. See the technical reference manual for details."]
    #[inline(always)]
    pub fn curr(&self) -> CURR_R {
        CURR_R::new((self.bits & 7) as u8)
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
Selects the set of commands that the RFC will accept. Only modes permitted by RFCMODEHWOPT.AVAIL are writeable. See the technical reference manual for details."]
    #[inline(always)]
    #[must_use]
    pub fn curr(&mut self) -> CURR_W<0> {
        CURR_W::new(self)
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
#[doc = "Selected RFC Mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rfcmodesel](index.html) module"]
pub struct RFCMODESEL_SPEC;
impl crate::RegisterSpec for RFCMODESEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rfcmodesel::R](R) reader structure"]
impl crate::Readable for RFCMODESEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rfcmodesel::W](W) writer structure"]
impl crate::Writable for RFCMODESEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RFCMODESEL to value 0"]
impl crate::Resettable for RFCMODESEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
