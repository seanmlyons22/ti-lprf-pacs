#[doc = "Register `STAT` reader"]
pub struct R(crate::R<STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STAT` writer"]
pub struct W(crate::W<STAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STAT_SPEC>;
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
impl From<crate::W<STAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MODE` reader - 1:0\\]
Current VIMS mode"]
pub type MODE_R = crate::FieldReader<u8, MODE_A>;
#[doc = "1:0\\]
Current VIMS mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "3: VIMS Off mode"]
    OFF = 3,
    #[doc = "1: VIMS Cache mode"]
    CACHE = 1,
    #[doc = "0: VIMS GPRAM mode"]
    GPRAM = 0,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
impl MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MODE_A> {
        match self.bits {
            3 => Some(MODE_A::OFF),
            1 => Some(MODE_A::CACHE),
            0 => Some(MODE_A::GPRAM),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == MODE_A::OFF
    }
    #[doc = "Checks if the value of the field is `CACHE`"]
    #[inline(always)]
    pub fn is_cache(&self) -> bool {
        *self == MODE_A::CACHE
    }
    #[doc = "Checks if the value of the field is `GPRAM`"]
    #[inline(always)]
    pub fn is_gpram(&self) -> bool {
        *self == MODE_A::GPRAM
    }
}
#[doc = "Field `MODE` writer - 1:0\\]
Current VIMS mode"]
pub type MODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, STAT_SPEC, u8, MODE_A, 2, O>;
impl<'a, const O: u8> MODE_W<'a, O> {
    #[doc = "VIMS Off mode"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(MODE_A::OFF)
    }
    #[doc = "VIMS Cache mode"]
    #[inline(always)]
    pub fn cache(self) -> &'a mut W {
        self.variant(MODE_A::CACHE)
    }
    #[doc = "VIMS GPRAM mode"]
    #[inline(always)]
    pub fn gpram(self) -> &'a mut W {
        self.variant(MODE_A::GPRAM)
    }
}
#[doc = "Field `INV` reader - 2:2\\]
This bit is set when invalidation of the cache memory is active / ongoing"]
pub type INV_R = crate::BitReader<bool>;
#[doc = "Field `INV` writer - 2:2\\]
This bit is set when invalidation of the cache memory is active / ongoing"]
pub type INV_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT_SPEC, bool, O>;
#[doc = "Field `MODE_CHANGING` reader - 3:3\\]
VIMS mode change status 0: VIMS is in the mode defined by MODE 1: VIMS is in the process of changing to the mode given in CTL.MODE"]
pub type MODE_CHANGING_R = crate::BitReader<bool>;
#[doc = "Field `MODE_CHANGING` writer - 3:3\\]
VIMS mode change status 0: VIMS is in the mode defined by MODE 1: VIMS is in the process of changing to the mode given in CTL.MODE"]
pub type MODE_CHANGING_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT_SPEC, bool, O>;
#[doc = "Field `SYSBUS_LB_DIS` reader - 4:4\\]
Sysbus flash line buffer control 0: Enabled or in transition to disabled 1: Disabled and flushed"]
pub type SYSBUS_LB_DIS_R = crate::BitReader<bool>;
#[doc = "Field `SYSBUS_LB_DIS` writer - 4:4\\]
Sysbus flash line buffer control 0: Enabled or in transition to disabled 1: Disabled and flushed"]
pub type SYSBUS_LB_DIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT_SPEC, bool, O>;
#[doc = "Field `IDCODE_LB_DIS` reader - 5:5\\]
Icode/Dcode flash line buffer status 0: Enabled or in transition to disabled 1: Disabled and flushed"]
pub type IDCODE_LB_DIS_R = crate::BitReader<bool>;
#[doc = "Field `IDCODE_LB_DIS` writer - 5:5\\]
Icode/Dcode flash line buffer status 0: Enabled or in transition to disabled 1: Disabled and flushed"]
pub type IDCODE_LB_DIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT_SPEC, bool, O>;
#[doc = "Field `RESERVED6` reader - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED6_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED6` writer - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED6_W<'a, const O: u8> = crate::FieldWriter<'a, u32, STAT_SPEC, u32, u32, 26, O>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Current VIMS mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - 2:2\\]
This bit is set when invalidation of the cache memory is active / ongoing"]
    #[inline(always)]
    pub fn inv(&self) -> INV_R {
        INV_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
VIMS mode change status 0: VIMS is in the mode defined by MODE 1: VIMS is in the process of changing to the mode given in CTL.MODE"]
    #[inline(always)]
    pub fn mode_changing(&self) -> MODE_CHANGING_R {
        MODE_CHANGING_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Sysbus flash line buffer control 0: Enabled or in transition to disabled 1: Disabled and flushed"]
    #[inline(always)]
    pub fn sysbus_lb_dis(&self) -> SYSBUS_LB_DIS_R {
        SYSBUS_LB_DIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Icode/Dcode flash line buffer status 0: Enabled or in transition to disabled 1: Disabled and flushed"]
    #[inline(always)]
    pub fn idcode_lb_dis(&self) -> IDCODE_LB_DIS_R {
        IDCODE_LB_DIS_R::new(((self.bits >> 5) & 1) != 0)
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
Current VIMS mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<0> {
        MODE_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
This bit is set when invalidation of the cache memory is active / ongoing"]
    #[inline(always)]
    #[must_use]
    pub fn inv(&mut self) -> INV_W<2> {
        INV_W::new(self)
    }
    #[doc = "Bit 3 - 3:3\\]
VIMS mode change status 0: VIMS is in the mode defined by MODE 1: VIMS is in the process of changing to the mode given in CTL.MODE"]
    #[inline(always)]
    #[must_use]
    pub fn mode_changing(&mut self) -> MODE_CHANGING_W<3> {
        MODE_CHANGING_W::new(self)
    }
    #[doc = "Bit 4 - 4:4\\]
Sysbus flash line buffer control 0: Enabled or in transition to disabled 1: Disabled and flushed"]
    #[inline(always)]
    #[must_use]
    pub fn sysbus_lb_dis(&mut self) -> SYSBUS_LB_DIS_W<4> {
        SYSBUS_LB_DIS_W::new(self)
    }
    #[doc = "Bit 5 - 5:5\\]
Icode/Dcode flash line buffer status 0: Enabled or in transition to disabled 1: Disabled and flushed"]
    #[inline(always)]
    #[must_use]
    pub fn idcode_lb_dis(&mut self) -> IDCODE_LB_DIS_W<5> {
        IDCODE_LB_DIS_W::new(self)
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
#[doc = "Status Displays current VIMS mode and line buffer status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stat](index.html) module"]
pub struct STAT_SPEC;
impl crate::RegisterSpec for STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stat::R](R) reader structure"]
impl crate::Readable for STAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [stat::W](W) writer structure"]
impl crate::Writable for STAT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STAT to value 0"]
impl crate::Resettable for STAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
