#[doc = "Register `WUFLAGS` reader"]
pub struct R(crate::R<WUFLAGS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WUFLAGS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WUFLAGS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WUFLAGS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WUFLAGS` writer"]
pub struct W(crate::W<WUFLAGS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WUFLAGS_SPEC>;
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
impl From<crate::W<WUFLAGS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WUFLAGS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PROG_WU0` reader - 0:0\\]
Programmable wakeup 0. 0: Programmable wakeup 0 not triggered. 1: Programmable wakeup 0 triggered."]
pub type PROG_WU0_R = crate::BitReader<bool>;
#[doc = "Field `PROG_WU0` writer - 0:0\\]
Programmable wakeup 0. 0: Programmable wakeup 0 not triggered. 1: Programmable wakeup 0 triggered."]
pub type PROG_WU0_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUFLAGS_SPEC, bool, O>;
#[doc = "Field `PROG_WU1` reader - 1:1\\]
Programmable wakeup 1. 0: Programmable wakeup 1 not triggered. 1: Programmable wakeup 1 triggered."]
pub type PROG_WU1_R = crate::BitReader<bool>;
#[doc = "Field `PROG_WU1` writer - 1:1\\]
Programmable wakeup 1. 0: Programmable wakeup 1 not triggered. 1: Programmable wakeup 1 triggered."]
pub type PROG_WU1_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUFLAGS_SPEC, bool, O>;
#[doc = "Field `PROG_WU2` reader - 2:2\\]
Programmable wakeup 2. 0: Programmable wakeup 2 not triggered. 1: Programmable wakeup 2 triggered."]
pub type PROG_WU2_R = crate::BitReader<bool>;
#[doc = "Field `PROG_WU2` writer - 2:2\\]
Programmable wakeup 2. 0: Programmable wakeup 2 not triggered. 1: Programmable wakeup 2 triggered."]
pub type PROG_WU2_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUFLAGS_SPEC, bool, O>;
#[doc = "Field `PROG_WU3` reader - 3:3\\]
Programmable wakeup 3. 0: Programmable wakeup 3 not triggered. 1: Programmable wakeup 3 triggered."]
pub type PROG_WU3_R = crate::BitReader<bool>;
#[doc = "Field `PROG_WU3` writer - 3:3\\]
Programmable wakeup 3. 0: Programmable wakeup 3 not triggered. 1: Programmable wakeup 3 triggered."]
pub type PROG_WU3_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUFLAGS_SPEC, bool, O>;
#[doc = "Field `SW_WU0` reader - 4:4\\]
Software wakeup 0 flag. 0: Software wakeup 0 not triggered. 1: Software wakeup 0 triggered."]
pub type SW_WU0_R = crate::BitReader<bool>;
#[doc = "Field `SW_WU0` writer - 4:4\\]
Software wakeup 0 flag. 0: Software wakeup 0 not triggered. 1: Software wakeup 0 triggered."]
pub type SW_WU0_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUFLAGS_SPEC, bool, O>;
#[doc = "Field `SW_WU1` reader - 5:5\\]
Software wakeup 1 flag. 0: Software wakeup 1 not triggered. 1: Software wakeup 1 triggered."]
pub type SW_WU1_R = crate::BitReader<bool>;
#[doc = "Field `SW_WU1` writer - 5:5\\]
Software wakeup 1 flag. 0: Software wakeup 1 not triggered. 1: Software wakeup 1 triggered."]
pub type SW_WU1_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUFLAGS_SPEC, bool, O>;
#[doc = "Field `SW_WU2` reader - 6:6\\]
Software wakeup 2 flag. 0: Software wakeup 2 not triggered. 1: Software wakeup 2 triggered."]
pub type SW_WU2_R = crate::BitReader<bool>;
#[doc = "Field `SW_WU2` writer - 6:6\\]
Software wakeup 2 flag. 0: Software wakeup 2 not triggered. 1: Software wakeup 2 triggered."]
pub type SW_WU2_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUFLAGS_SPEC, bool, O>;
#[doc = "Field `SW_WU3` reader - 7:7\\]
Software wakeup 3 flag. 0: Software wakeup 3 not triggered. 1: Software wakeup 3 triggered."]
pub type SW_WU3_R = crate::BitReader<bool>;
#[doc = "Field `SW_WU3` writer - 7:7\\]
Software wakeup 3 flag. 0: Software wakeup 3 not triggered. 1: Software wakeup 3 triggered."]
pub type SW_WU3_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUFLAGS_SPEC, bool, O>;
#[doc = "Field `RESERVED8` reader - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED8_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED8` writer - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED8_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WUFLAGS_SPEC, u32, u32, 24, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Programmable wakeup 0. 0: Programmable wakeup 0 not triggered. 1: Programmable wakeup 0 triggered."]
    #[inline(always)]
    pub fn prog_wu0(&self) -> PROG_WU0_R {
        PROG_WU0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Programmable wakeup 1. 0: Programmable wakeup 1 not triggered. 1: Programmable wakeup 1 triggered."]
    #[inline(always)]
    pub fn prog_wu1(&self) -> PROG_WU1_R {
        PROG_WU1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Programmable wakeup 2. 0: Programmable wakeup 2 not triggered. 1: Programmable wakeup 2 triggered."]
    #[inline(always)]
    pub fn prog_wu2(&self) -> PROG_WU2_R {
        PROG_WU2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Programmable wakeup 3. 0: Programmable wakeup 3 not triggered. 1: Programmable wakeup 3 triggered."]
    #[inline(always)]
    pub fn prog_wu3(&self) -> PROG_WU3_R {
        PROG_WU3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Software wakeup 0 flag. 0: Software wakeup 0 not triggered. 1: Software wakeup 0 triggered."]
    #[inline(always)]
    pub fn sw_wu0(&self) -> SW_WU0_R {
        SW_WU0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Software wakeup 1 flag. 0: Software wakeup 1 not triggered. 1: Software wakeup 1 triggered."]
    #[inline(always)]
    pub fn sw_wu1(&self) -> SW_WU1_R {
        SW_WU1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Software wakeup 2 flag. 0: Software wakeup 2 not triggered. 1: Software wakeup 2 triggered."]
    #[inline(always)]
    pub fn sw_wu2(&self) -> SW_WU2_R {
        SW_WU2_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Software wakeup 3 flag. 0: Software wakeup 3 not triggered. 1: Software wakeup 3 triggered."]
    #[inline(always)]
    pub fn sw_wu3(&self) -> SW_WU3_R {
        SW_WU3_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&self) -> RESERVED8_R {
        RESERVED8_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Programmable wakeup 0. 0: Programmable wakeup 0 not triggered. 1: Programmable wakeup 0 triggered."]
    #[inline(always)]
    #[must_use]
    pub fn prog_wu0(&mut self) -> PROG_WU0_W<0> {
        PROG_WU0_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
Programmable wakeup 1. 0: Programmable wakeup 1 not triggered. 1: Programmable wakeup 1 triggered."]
    #[inline(always)]
    #[must_use]
    pub fn prog_wu1(&mut self) -> PROG_WU1_W<1> {
        PROG_WU1_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
Programmable wakeup 2. 0: Programmable wakeup 2 not triggered. 1: Programmable wakeup 2 triggered."]
    #[inline(always)]
    #[must_use]
    pub fn prog_wu2(&mut self) -> PROG_WU2_W<2> {
        PROG_WU2_W::new(self)
    }
    #[doc = "Bit 3 - 3:3\\]
Programmable wakeup 3. 0: Programmable wakeup 3 not triggered. 1: Programmable wakeup 3 triggered."]
    #[inline(always)]
    #[must_use]
    pub fn prog_wu3(&mut self) -> PROG_WU3_W<3> {
        PROG_WU3_W::new(self)
    }
    #[doc = "Bit 4 - 4:4\\]
Software wakeup 0 flag. 0: Software wakeup 0 not triggered. 1: Software wakeup 0 triggered."]
    #[inline(always)]
    #[must_use]
    pub fn sw_wu0(&mut self) -> SW_WU0_W<4> {
        SW_WU0_W::new(self)
    }
    #[doc = "Bit 5 - 5:5\\]
Software wakeup 1 flag. 0: Software wakeup 1 not triggered. 1: Software wakeup 1 triggered."]
    #[inline(always)]
    #[must_use]
    pub fn sw_wu1(&mut self) -> SW_WU1_W<5> {
        SW_WU1_W::new(self)
    }
    #[doc = "Bit 6 - 6:6\\]
Software wakeup 2 flag. 0: Software wakeup 2 not triggered. 1: Software wakeup 2 triggered."]
    #[inline(always)]
    #[must_use]
    pub fn sw_wu2(&mut self) -> SW_WU2_W<6> {
        SW_WU2_W::new(self)
    }
    #[doc = "Bit 7 - 7:7\\]
Software wakeup 3 flag. 0: Software wakeup 3 not triggered. 1: Software wakeup 3 triggered."]
    #[inline(always)]
    #[must_use]
    pub fn sw_wu3(&mut self) -> SW_WU3_W<7> {
        SW_WU3_W::new(self)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved8(&mut self) -> RESERVED8_W<8> {
        RESERVED8_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Wakeup Flags This register holds the eight AUX wakeup flags. Each flag can cause AUX operational mode to change as given in OPMODEREQ. To clear flag n you must set bit n in WUFLAGSCLR until flag n is read as 0. You must clear bit n in WUFLAGSCLR before flag n can be set again.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wuflags](index.html) module"]
pub struct WUFLAGS_SPEC;
impl crate::RegisterSpec for WUFLAGS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wuflags::R](R) reader structure"]
impl crate::Readable for WUFLAGS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wuflags::W](W) writer structure"]
impl crate::Writable for WUFLAGS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WUFLAGS to value 0"]
impl crate::Resettable for WUFLAGS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
