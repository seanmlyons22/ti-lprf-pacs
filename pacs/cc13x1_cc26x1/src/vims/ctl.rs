#[doc = "Register `CTL` reader"]
pub struct R(crate::R<CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTL` writer"]
pub struct W(crate::W<CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTL_SPEC>;
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
impl From<crate::W<CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MODE` reader - 1:0\\]
VIMS mode request. Write accesses to this field will be blocked while STAT.MODE_CHANGING is set to 1."]
pub type MODE_R = crate::FieldReader<u8, MODE_A>;
#[doc = "1:0\\]
VIMS mode request. Write accesses to this field will be blocked while STAT.MODE_CHANGING is set to 1.\n\nValue on reset: 0"]
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
VIMS mode request. Write accesses to this field will be blocked while STAT.MODE_CHANGING is set to 1."]
pub type MODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTL_SPEC, u8, MODE_A, 2, O>;
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
#[doc = "Field `PREF_EN` reader - 2:2\\]
Tag prefetch control 0: Disabled 1: Enabled"]
pub type PREF_EN_R = crate::BitReader<bool>;
#[doc = "Field `PREF_EN` writer - 2:2\\]
Tag prefetch control 0: Disabled 1: Enabled"]
pub type PREF_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
#[doc = "Field `ARB_CFG` reader - 3:3\\]
Icode/Dcode and sysbus arbitation scheme 0: Static arbitration (icode/docde > sysbus) 1: Round-robin arbitration"]
pub type ARB_CFG_R = crate::BitReader<bool>;
#[doc = "Field `ARB_CFG` writer - 3:3\\]
Icode/Dcode and sysbus arbitation scheme 0: Static arbitration (icode/docde > sysbus) 1: Round-robin arbitration"]
pub type ARB_CFG_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
#[doc = "Field `SYSBUS_LB_DIS` reader - 4:4\\]
Sysbus flash line buffer control 0: Enable 1: Disable"]
pub type SYSBUS_LB_DIS_R = crate::BitReader<bool>;
#[doc = "Field `SYSBUS_LB_DIS` writer - 4:4\\]
Sysbus flash line buffer control 0: Enable 1: Disable"]
pub type SYSBUS_LB_DIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
#[doc = "Field `IDCODE_LB_DIS` reader - 5:5\\]
Icode/Dcode flash line buffer control 0: Enable 1: Disable"]
pub type IDCODE_LB_DIS_R = crate::BitReader<bool>;
#[doc = "Field `IDCODE_LB_DIS` writer - 5:5\\]
Icode/Dcode flash line buffer control 0: Enable 1: Disable"]
pub type IDCODE_LB_DIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
#[doc = "Field `RESERVED6` reader - 28:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED6_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED6` writer - 28:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED6_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTL_SPEC, u32, u32, 23, O>;
#[doc = "Field `DYN_CG_EN` reader - 29:29\\]
0: The in-built clock gate functionality is bypassed. 1: The in-built clock gate functionality is enabled, automatically gating the clock when not needed."]
pub type DYN_CG_EN_R = crate::BitReader<bool>;
#[doc = "Field `DYN_CG_EN` writer - 29:29\\]
0: The in-built clock gate functionality is bypassed. 1: The in-built clock gate functionality is enabled, automatically gating the clock when not needed."]
pub type DYN_CG_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
#[doc = "Field `STATS_EN` reader - 30:30\\]
Set this bit to enable statistic counters."]
pub type STATS_EN_R = crate::BitReader<bool>;
#[doc = "Field `STATS_EN` writer - 30:30\\]
Set this bit to enable statistic counters."]
pub type STATS_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
#[doc = "Field `STATS_CLR` reader - 31:31\\]
Set this bit to clear statistic counters."]
pub type STATS_CLR_R = crate::BitReader<bool>;
#[doc = "Field `STATS_CLR` writer - 31:31\\]
Set this bit to clear statistic counters."]
pub type STATS_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
VIMS mode request. Write accesses to this field will be blocked while STAT.MODE_CHANGING is set to 1."]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - 2:2\\]
Tag prefetch control 0: Disabled 1: Enabled"]
    #[inline(always)]
    pub fn pref_en(&self) -> PREF_EN_R {
        PREF_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Icode/Dcode and sysbus arbitation scheme 0: Static arbitration (icode/docde > sysbus) 1: Round-robin arbitration"]
    #[inline(always)]
    pub fn arb_cfg(&self) -> ARB_CFG_R {
        ARB_CFG_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Sysbus flash line buffer control 0: Enable 1: Disable"]
    #[inline(always)]
    pub fn sysbus_lb_dis(&self) -> SYSBUS_LB_DIS_R {
        SYSBUS_LB_DIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Icode/Dcode flash line buffer control 0: Enable 1: Disable"]
    #[inline(always)]
    pub fn idcode_lb_dis(&self) -> IDCODE_LB_DIS_R {
        IDCODE_LB_DIS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:28 - 28:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved6(&self) -> RESERVED6_R {
        RESERVED6_R::new((self.bits >> 6) & 0x007f_ffff)
    }
    #[doc = "Bit 29 - 29:29\\]
0: The in-built clock gate functionality is bypassed. 1: The in-built clock gate functionality is enabled, automatically gating the clock when not needed."]
    #[inline(always)]
    pub fn dyn_cg_en(&self) -> DYN_CG_EN_R {
        DYN_CG_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 30:30\\]
Set this bit to enable statistic counters."]
    #[inline(always)]
    pub fn stats_en(&self) -> STATS_EN_R {
        STATS_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Set this bit to clear statistic counters."]
    #[inline(always)]
    pub fn stats_clr(&self) -> STATS_CLR_R {
        STATS_CLR_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
VIMS mode request. Write accesses to this field will be blocked while STAT.MODE_CHANGING is set to 1."]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<0> {
        MODE_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
Tag prefetch control 0: Disabled 1: Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn pref_en(&mut self) -> PREF_EN_W<2> {
        PREF_EN_W::new(self)
    }
    #[doc = "Bit 3 - 3:3\\]
Icode/Dcode and sysbus arbitation scheme 0: Static arbitration (icode/docde > sysbus) 1: Round-robin arbitration"]
    #[inline(always)]
    #[must_use]
    pub fn arb_cfg(&mut self) -> ARB_CFG_W<3> {
        ARB_CFG_W::new(self)
    }
    #[doc = "Bit 4 - 4:4\\]
Sysbus flash line buffer control 0: Enable 1: Disable"]
    #[inline(always)]
    #[must_use]
    pub fn sysbus_lb_dis(&mut self) -> SYSBUS_LB_DIS_W<4> {
        SYSBUS_LB_DIS_W::new(self)
    }
    #[doc = "Bit 5 - 5:5\\]
Icode/Dcode flash line buffer control 0: Enable 1: Disable"]
    #[inline(always)]
    #[must_use]
    pub fn idcode_lb_dis(&mut self) -> IDCODE_LB_DIS_W<5> {
        IDCODE_LB_DIS_W::new(self)
    }
    #[doc = "Bits 6:28 - 28:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved6(&mut self) -> RESERVED6_W<6> {
        RESERVED6_W::new(self)
    }
    #[doc = "Bit 29 - 29:29\\]
0: The in-built clock gate functionality is bypassed. 1: The in-built clock gate functionality is enabled, automatically gating the clock when not needed."]
    #[inline(always)]
    #[must_use]
    pub fn dyn_cg_en(&mut self) -> DYN_CG_EN_W<29> {
        DYN_CG_EN_W::new(self)
    }
    #[doc = "Bit 30 - 30:30\\]
Set this bit to enable statistic counters."]
    #[inline(always)]
    #[must_use]
    pub fn stats_en(&mut self) -> STATS_EN_W<30> {
        STATS_EN_W::new(self)
    }
    #[doc = "Bit 31 - 31:31\\]
Set this bit to clear statistic counters."]
    #[inline(always)]
    #[must_use]
    pub fn stats_clr(&mut self) -> STATS_CLR_W<31> {
        STATS_CLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control Configure VIMS mode and line buffer settings\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl](index.html) module"]
pub struct CTL_SPEC;
impl crate::RegisterSpec for CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctl::R](R) reader structure"]
impl crate::Readable for CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctl::W](W) writer structure"]
impl crate::Writable for CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTL to value 0"]
impl crate::Resettable for CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
