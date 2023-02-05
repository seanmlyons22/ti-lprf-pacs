#[doc = "Register `RECHARGECFG` reader"]
pub struct R(crate::R<RECHARGECFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RECHARGECFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RECHARGECFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RECHARGECFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RECHARGECFG` writer"]
pub struct W(crate::W<RECHARGECFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RECHARGECFG_SPEC>;
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
impl From<crate::W<RECHARGECFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RECHARGECFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PER_E` reader - 2:0\\]
Internal. Only to be used through TI provided API."]
pub type PER_E_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PER_E` writer - 2:0\\]
Internal. Only to be used through TI provided API."]
pub type PER_E_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RECHARGECFG_SPEC, u8, u8, 3, O>;
#[doc = "Field `PER_M` reader - 7:3\\]
Internal. Only to be used through TI provided API."]
pub type PER_M_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PER_M` writer - 7:3\\]
Internal. Only to be used through TI provided API."]
pub type PER_M_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RECHARGECFG_SPEC, u8, u8, 5, O>;
#[doc = "Field `MAX_PER_E` reader - 10:8\\]
Internal. Only to be used through TI provided API."]
pub type MAX_PER_E_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MAX_PER_E` writer - 10:8\\]
Internal. Only to be used through TI provided API."]
pub type MAX_PER_E_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RECHARGECFG_SPEC, u8, u8, 3, O>;
#[doc = "Field `MAX_PER_M` reader - 15:11\\]
Internal. Only to be used through TI provided API."]
pub type MAX_PER_M_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MAX_PER_M` writer - 15:11\\]
Internal. Only to be used through TI provided API."]
pub type MAX_PER_M_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RECHARGECFG_SPEC, u8, u8, 5, O>;
#[doc = "Field `C1` reader - 19:16\\]
Internal. Only to be used through TI provided API."]
pub type C1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `C1` writer - 19:16\\]
Internal. Only to be used through TI provided API."]
pub type C1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RECHARGECFG_SPEC, u8, u8, 4, O>;
#[doc = "Field `C2` reader - 23:20\\]
Internal. Only to be used through TI provided API."]
pub type C2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `C2` writer - 23:20\\]
Internal. Only to be used through TI provided API."]
pub type C2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RECHARGECFG_SPEC, u8, u8, 4, O>;
#[doc = "Field `RESERVED24` reader - 29:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED24_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED24` writer - 29:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED24_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RECHARGECFG_SPEC, u8, u8, 6, O>;
#[doc = "Field `MODE` reader - 31:30\\]
Selects recharge algorithm for VDDR when the system is running on the uLDO"]
pub type MODE_R = crate::FieldReader<u8, MODE_A>;
#[doc = "31:30\\]
Selects recharge algorithm for VDDR when the system is running on the uLDO\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "3: External recharge comparator. Note that the clock to the recharge comparator must be enabled, \\[ANATOP_MMAP:ADI_3_REFSYS:CTL_RECHARGE_CMP0:COMP_CLK_DISABLE\\], before selecting this recharge algorithm."]
    COMPARATOR = 3,
    #[doc = "2: Adaptive timer"]
    ADAPTIVE = 2,
    #[doc = "1: Static timer"]
    STATIC = 1,
    #[doc = "0: Recharge disabled"]
    OFF = 0,
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
    pub fn variant(&self) -> MODE_A {
        match self.bits {
            3 => MODE_A::COMPARATOR,
            2 => MODE_A::ADAPTIVE,
            1 => MODE_A::STATIC,
            0 => MODE_A::OFF,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `COMPARATOR`"]
    #[inline(always)]
    pub fn is_comparator(&self) -> bool {
        *self == MODE_A::COMPARATOR
    }
    #[doc = "Checks if the value of the field is `ADAPTIVE`"]
    #[inline(always)]
    pub fn is_adaptive(&self) -> bool {
        *self == MODE_A::ADAPTIVE
    }
    #[doc = "Checks if the value of the field is `STATIC`"]
    #[inline(always)]
    pub fn is_static(&self) -> bool {
        *self == MODE_A::STATIC
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == MODE_A::OFF
    }
}
#[doc = "Field `MODE` writer - 31:30\\]
Selects recharge algorithm for VDDR when the system is running on the uLDO"]
pub type MODE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, RECHARGECFG_SPEC, u8, MODE_A, 2, O>;
impl<'a, const O: u8> MODE_W<'a, O> {
    #[doc = "External recharge comparator. Note that the clock to the recharge comparator must be enabled, \\[ANATOP_MMAP:ADI_3_REFSYS:CTL_RECHARGE_CMP0:COMP_CLK_DISABLE\\], before selecting this recharge algorithm."]
    #[inline(always)]
    pub fn comparator(self) -> &'a mut W {
        self.variant(MODE_A::COMPARATOR)
    }
    #[doc = "Adaptive timer"]
    #[inline(always)]
    pub fn adaptive(self) -> &'a mut W {
        self.variant(MODE_A::ADAPTIVE)
    }
    #[doc = "Static timer"]
    #[inline(always)]
    pub fn static_(self) -> &'a mut W {
        self.variant(MODE_A::STATIC)
    }
    #[doc = "Recharge disabled"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(MODE_A::OFF)
    }
}
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn per_e(&self) -> PER_E_R {
        PER_E_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:7 - 7:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn per_m(&self) -> PER_M_R {
        PER_M_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn max_per_e(&self) -> MAX_PER_E_R {
        MAX_PER_E_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:15 - 15:11\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn max_per_m(&self) -> MAX_PER_M_R {
        MAX_PER_M_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn c1(&self) -> C1_R {
        C1_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn c2(&self) -> C2_R {
        C2_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:29 - 29:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved24(&self) -> RESERVED24_R {
        RESERVED24_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bits 30:31 - 31:30\\]
Selects recharge algorithm for VDDR when the system is running on the uLDO"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn per_e(&mut self) -> PER_E_W<0> {
        PER_E_W::new(self)
    }
    #[doc = "Bits 3:7 - 7:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn per_m(&mut self) -> PER_M_W<3> {
        PER_M_W::new(self)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn max_per_e(&mut self) -> MAX_PER_E_W<8> {
        MAX_PER_E_W::new(self)
    }
    #[doc = "Bits 11:15 - 15:11\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn max_per_m(&mut self) -> MAX_PER_M_W<11> {
        MAX_PER_M_W::new(self)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn c1(&mut self) -> C1_W<16> {
        C1_W::new(self)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn c2(&mut self) -> C2_W<20> {
        C2_W::new(self)
    }
    #[doc = "Bits 24:29 - 29:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved24(&mut self) -> RESERVED24_W<24> {
        RESERVED24_W::new(self)
    }
    #[doc = "Bits 30:31 - 31:30\\]
Selects recharge algorithm for VDDR when the system is running on the uLDO"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<30> {
        MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Recharge Controller Configuration This register sets all relevant parameters for controlling the recharge algorithm.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rechargecfg](index.html) module"]
pub struct RECHARGECFG_SPEC;
impl crate::RegisterSpec for RECHARGECFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rechargecfg::R](R) reader structure"]
impl crate::Readable for RECHARGECFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rechargecfg::W](W) writer structure"]
impl crate::Writable for RECHARGECFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RECHARGECFG to value 0xc000_0000"]
impl crate::Resettable for RECHARGECFG_SPEC {
    const RESET_VALUE: Self::Ux = 0xc000_0000;
}
