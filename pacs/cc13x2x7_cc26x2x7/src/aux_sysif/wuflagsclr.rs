#[doc = "Register `WUFLAGSCLR` reader"]
pub struct R(crate::R<WUFLAGSCLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WUFLAGSCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WUFLAGSCLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WUFLAGSCLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WUFLAGSCLR` writer"]
pub struct W(crate::W<WUFLAGSCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WUFLAGSCLR_SPEC>;
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
impl From<crate::W<WUFLAGSCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WUFLAGSCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PROG_WU0` reader - 0:0\\]
Programmable wakeup flag 0. 0: No effect. 1: Clear WUFLAGS.PROG_WU0. Keep high until WUFLAGS.PROG_WU0 is 0. The wakeup flag becomes edge sensitive if you write PROG_WU0 to 0 when PROGWU0CFG.EN is 1. The wakeup flag becomes level sensitive if you write PROG_WU0 to 0 when PROGWU0CFG.EN is 0, then set PROGWU0CFG.EN."]
pub type PROG_WU0_R = crate::BitReader<bool>;
#[doc = "Field `PROG_WU0` writer - 0:0\\]
Programmable wakeup flag 0. 0: No effect. 1: Clear WUFLAGS.PROG_WU0. Keep high until WUFLAGS.PROG_WU0 is 0. The wakeup flag becomes edge sensitive if you write PROG_WU0 to 0 when PROGWU0CFG.EN is 1. The wakeup flag becomes level sensitive if you write PROG_WU0 to 0 when PROGWU0CFG.EN is 0, then set PROGWU0CFG.EN."]
pub type PROG_WU0_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUFLAGSCLR_SPEC, bool, O>;
#[doc = "Field `PROG_WU1` reader - 1:1\\]
Programmable wakeup flag 1. 0: No effect. 1: Clear WUFLAGS.PROG_WU1. Keep high until WUFLAGS.PROG_WU1 is 0. The wakeup flag becomes edge sensitive if you write PROG_WU1 to 0 when PROGWU1CFG.EN is 1. The wakeup flag becomes level sensitive if you write PROG_WU1 to 0 when PROGWU1CFG.EN is 0, then set PROGWU1CFG.EN."]
pub type PROG_WU1_R = crate::BitReader<bool>;
#[doc = "Field `PROG_WU1` writer - 1:1\\]
Programmable wakeup flag 1. 0: No effect. 1: Clear WUFLAGS.PROG_WU1. Keep high until WUFLAGS.PROG_WU1 is 0. The wakeup flag becomes edge sensitive if you write PROG_WU1 to 0 when PROGWU1CFG.EN is 1. The wakeup flag becomes level sensitive if you write PROG_WU1 to 0 when PROGWU1CFG.EN is 0, then set PROGWU1CFG.EN."]
pub type PROG_WU1_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUFLAGSCLR_SPEC, bool, O>;
#[doc = "Field `PROG_WU2` reader - 2:2\\]
Programmable wakeup flag 2. 0: No effect. 1: Clear WUFLAGS.PROG_WU2. Keep high until WUFLAGS.PROG_WU2 is 0. The wakeup flag becomes edge sensitive if you write PROG_WU2 to 0 when PROGWU2CFG.EN is 1. The wakeup flag becomes level sensitive if you write PROG_WU2 to 0 when PROGWU2CFG.EN is 0, then set PROGWU2CFG.EN."]
pub type PROG_WU2_R = crate::BitReader<bool>;
#[doc = "Field `PROG_WU2` writer - 2:2\\]
Programmable wakeup flag 2. 0: No effect. 1: Clear WUFLAGS.PROG_WU2. Keep high until WUFLAGS.PROG_WU2 is 0. The wakeup flag becomes edge sensitive if you write PROG_WU2 to 0 when PROGWU2CFG.EN is 1. The wakeup flag becomes level sensitive if you write PROG_WU2 to 0 when PROGWU2CFG.EN is 0, then set PROGWU2CFG.EN."]
pub type PROG_WU2_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUFLAGSCLR_SPEC, bool, O>;
#[doc = "Field `PROG_WU3` reader - 3:3\\]
Programmable wakeup flag 3. 0: No effect. 1: Clear WUFLAGS.PROG_WU3. Keep high until WUFLAGS.PROG_WU3 is 0. The wakeup flag becomes edge sensitive if you write PROG_WU3 to 0 when PROGWU3CFG.EN is 1. The wakeup flag becomes level sensitive if you write PROG_WU3 to 0 when PROGWU3CFG.EN is 0, then set PROGWU3CFG.EN."]
pub type PROG_WU3_R = crate::BitReader<bool>;
#[doc = "Field `PROG_WU3` writer - 3:3\\]
Programmable wakeup flag 3. 0: No effect. 1: Clear WUFLAGS.PROG_WU3. Keep high until WUFLAGS.PROG_WU3 is 0. The wakeup flag becomes edge sensitive if you write PROG_WU3 to 0 when PROGWU3CFG.EN is 1. The wakeup flag becomes level sensitive if you write PROG_WU3 to 0 when PROGWU3CFG.EN is 0, then set PROGWU3CFG.EN."]
pub type PROG_WU3_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUFLAGSCLR_SPEC, bool, O>;
#[doc = "Field `SW_WU0` reader - 4:4\\]
Clear software wakeup flag 0. 0: No effect. 1: Clear WUFLAGS.SW_WU0. Keep high until WUFLAGS.SW_WU0 is 0."]
pub type SW_WU0_R = crate::BitReader<bool>;
#[doc = "Field `SW_WU0` writer - 4:4\\]
Clear software wakeup flag 0. 0: No effect. 1: Clear WUFLAGS.SW_WU0. Keep high until WUFLAGS.SW_WU0 is 0."]
pub type SW_WU0_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUFLAGSCLR_SPEC, bool, O>;
#[doc = "Field `SW_WU1` reader - 5:5\\]
Clear software wakeup flag 1. 0: No effect. 1: Clear WUFLAGS.SW_WU1. Keep high until WUFLAGS.SW_WU1 is 0."]
pub type SW_WU1_R = crate::BitReader<bool>;
#[doc = "Field `SW_WU1` writer - 5:5\\]
Clear software wakeup flag 1. 0: No effect. 1: Clear WUFLAGS.SW_WU1. Keep high until WUFLAGS.SW_WU1 is 0."]
pub type SW_WU1_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUFLAGSCLR_SPEC, bool, O>;
#[doc = "Field `SW_WU2` reader - 6:6\\]
Clear software wakeup flag 2. 0: No effect. 1: Clear WUFLAGS.SW_WU2. Keep high until WUFLAGS.SW_WU2 is 0."]
pub type SW_WU2_R = crate::BitReader<bool>;
#[doc = "Field `SW_WU2` writer - 6:6\\]
Clear software wakeup flag 2. 0: No effect. 1: Clear WUFLAGS.SW_WU2. Keep high until WUFLAGS.SW_WU2 is 0."]
pub type SW_WU2_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUFLAGSCLR_SPEC, bool, O>;
#[doc = "Field `SW_WU3` reader - 7:7\\]
Clear software wakeup flag 3. 0: No effect. 1: Clear WUFLAGS.SW_WU3. Keep high until WUFLAGS.SW_WU3 is 0."]
pub type SW_WU3_R = crate::BitReader<bool>;
#[doc = "Field `SW_WU3` writer - 7:7\\]
Clear software wakeup flag 3. 0: No effect. 1: Clear WUFLAGS.SW_WU3. Keep high until WUFLAGS.SW_WU3 is 0."]
pub type SW_WU3_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUFLAGSCLR_SPEC, bool, O>;
#[doc = "Field `RESERVED8` reader - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED8_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED8` writer - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED8_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, WUFLAGSCLR_SPEC, u32, u32, 24, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Programmable wakeup flag 0. 0: No effect. 1: Clear WUFLAGS.PROG_WU0. Keep high until WUFLAGS.PROG_WU0 is 0. The wakeup flag becomes edge sensitive if you write PROG_WU0 to 0 when PROGWU0CFG.EN is 1. The wakeup flag becomes level sensitive if you write PROG_WU0 to 0 when PROGWU0CFG.EN is 0, then set PROGWU0CFG.EN."]
    #[inline(always)]
    pub fn prog_wu0(&self) -> PROG_WU0_R {
        PROG_WU0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Programmable wakeup flag 1. 0: No effect. 1: Clear WUFLAGS.PROG_WU1. Keep high until WUFLAGS.PROG_WU1 is 0. The wakeup flag becomes edge sensitive if you write PROG_WU1 to 0 when PROGWU1CFG.EN is 1. The wakeup flag becomes level sensitive if you write PROG_WU1 to 0 when PROGWU1CFG.EN is 0, then set PROGWU1CFG.EN."]
    #[inline(always)]
    pub fn prog_wu1(&self) -> PROG_WU1_R {
        PROG_WU1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Programmable wakeup flag 2. 0: No effect. 1: Clear WUFLAGS.PROG_WU2. Keep high until WUFLAGS.PROG_WU2 is 0. The wakeup flag becomes edge sensitive if you write PROG_WU2 to 0 when PROGWU2CFG.EN is 1. The wakeup flag becomes level sensitive if you write PROG_WU2 to 0 when PROGWU2CFG.EN is 0, then set PROGWU2CFG.EN."]
    #[inline(always)]
    pub fn prog_wu2(&self) -> PROG_WU2_R {
        PROG_WU2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Programmable wakeup flag 3. 0: No effect. 1: Clear WUFLAGS.PROG_WU3. Keep high until WUFLAGS.PROG_WU3 is 0. The wakeup flag becomes edge sensitive if you write PROG_WU3 to 0 when PROGWU3CFG.EN is 1. The wakeup flag becomes level sensitive if you write PROG_WU3 to 0 when PROGWU3CFG.EN is 0, then set PROGWU3CFG.EN."]
    #[inline(always)]
    pub fn prog_wu3(&self) -> PROG_WU3_R {
        PROG_WU3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Clear software wakeup flag 0. 0: No effect. 1: Clear WUFLAGS.SW_WU0. Keep high until WUFLAGS.SW_WU0 is 0."]
    #[inline(always)]
    pub fn sw_wu0(&self) -> SW_WU0_R {
        SW_WU0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Clear software wakeup flag 1. 0: No effect. 1: Clear WUFLAGS.SW_WU1. Keep high until WUFLAGS.SW_WU1 is 0."]
    #[inline(always)]
    pub fn sw_wu1(&self) -> SW_WU1_R {
        SW_WU1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Clear software wakeup flag 2. 0: No effect. 1: Clear WUFLAGS.SW_WU2. Keep high until WUFLAGS.SW_WU2 is 0."]
    #[inline(always)]
    pub fn sw_wu2(&self) -> SW_WU2_R {
        SW_WU2_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Clear software wakeup flag 3. 0: No effect. 1: Clear WUFLAGS.SW_WU3. Keep high until WUFLAGS.SW_WU3 is 0."]
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
Programmable wakeup flag 0. 0: No effect. 1: Clear WUFLAGS.PROG_WU0. Keep high until WUFLAGS.PROG_WU0 is 0. The wakeup flag becomes edge sensitive if you write PROG_WU0 to 0 when PROGWU0CFG.EN is 1. The wakeup flag becomes level sensitive if you write PROG_WU0 to 0 when PROGWU0CFG.EN is 0, then set PROGWU0CFG.EN."]
    #[inline(always)]
    #[must_use]
    pub fn prog_wu0(&mut self) -> PROG_WU0_W<0> {
        PROG_WU0_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
Programmable wakeup flag 1. 0: No effect. 1: Clear WUFLAGS.PROG_WU1. Keep high until WUFLAGS.PROG_WU1 is 0. The wakeup flag becomes edge sensitive if you write PROG_WU1 to 0 when PROGWU1CFG.EN is 1. The wakeup flag becomes level sensitive if you write PROG_WU1 to 0 when PROGWU1CFG.EN is 0, then set PROGWU1CFG.EN."]
    #[inline(always)]
    #[must_use]
    pub fn prog_wu1(&mut self) -> PROG_WU1_W<1> {
        PROG_WU1_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
Programmable wakeup flag 2. 0: No effect. 1: Clear WUFLAGS.PROG_WU2. Keep high until WUFLAGS.PROG_WU2 is 0. The wakeup flag becomes edge sensitive if you write PROG_WU2 to 0 when PROGWU2CFG.EN is 1. The wakeup flag becomes level sensitive if you write PROG_WU2 to 0 when PROGWU2CFG.EN is 0, then set PROGWU2CFG.EN."]
    #[inline(always)]
    #[must_use]
    pub fn prog_wu2(&mut self) -> PROG_WU2_W<2> {
        PROG_WU2_W::new(self)
    }
    #[doc = "Bit 3 - 3:3\\]
Programmable wakeup flag 3. 0: No effect. 1: Clear WUFLAGS.PROG_WU3. Keep high until WUFLAGS.PROG_WU3 is 0. The wakeup flag becomes edge sensitive if you write PROG_WU3 to 0 when PROGWU3CFG.EN is 1. The wakeup flag becomes level sensitive if you write PROG_WU3 to 0 when PROGWU3CFG.EN is 0, then set PROGWU3CFG.EN."]
    #[inline(always)]
    #[must_use]
    pub fn prog_wu3(&mut self) -> PROG_WU3_W<3> {
        PROG_WU3_W::new(self)
    }
    #[doc = "Bit 4 - 4:4\\]
Clear software wakeup flag 0. 0: No effect. 1: Clear WUFLAGS.SW_WU0. Keep high until WUFLAGS.SW_WU0 is 0."]
    #[inline(always)]
    #[must_use]
    pub fn sw_wu0(&mut self) -> SW_WU0_W<4> {
        SW_WU0_W::new(self)
    }
    #[doc = "Bit 5 - 5:5\\]
Clear software wakeup flag 1. 0: No effect. 1: Clear WUFLAGS.SW_WU1. Keep high until WUFLAGS.SW_WU1 is 0."]
    #[inline(always)]
    #[must_use]
    pub fn sw_wu1(&mut self) -> SW_WU1_W<5> {
        SW_WU1_W::new(self)
    }
    #[doc = "Bit 6 - 6:6\\]
Clear software wakeup flag 2. 0: No effect. 1: Clear WUFLAGS.SW_WU2. Keep high until WUFLAGS.SW_WU2 is 0."]
    #[inline(always)]
    #[must_use]
    pub fn sw_wu2(&mut self) -> SW_WU2_W<6> {
        SW_WU2_W::new(self)
    }
    #[doc = "Bit 7 - 7:7\\]
Clear software wakeup flag 3. 0: No effect. 1: Clear WUFLAGS.SW_WU3. Keep high until WUFLAGS.SW_WU3 is 0."]
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
#[doc = "Wakeup Flags Clear This register clears AUX wakeup flags WUFLAGS. To clear programmable wakeup flags you must disable the AUX wakeup output first. After the programmable wakeup flags are cleared you must re-enable the AUX wakeup output. Write WUGATE to disable or enable the AUX wakeup output. This procedure is not required when you want to clear a software-triggered wakeup.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wuflagsclr](index.html) module"]
pub struct WUFLAGSCLR_SPEC;
impl crate::RegisterSpec for WUFLAGSCLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wuflagsclr::R](R) reader structure"]
impl crate::Readable for WUFLAGSCLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wuflagsclr::W](W) writer structure"]
impl crate::Writable for WUFLAGSCLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WUFLAGSCLR to value 0x0f"]
impl crate::Resettable for WUFLAGSCLR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0f;
}
