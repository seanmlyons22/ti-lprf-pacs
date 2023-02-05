#[doc = "Register `RAMCFG` reader"]
pub struct R(crate::R<RAMCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RAMCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RAMCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RAMCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RAMCFG` writer"]
pub struct W(crate::W<RAMCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RAMCFG_SPEC>;
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
impl From<crate::W<RAMCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RAMCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BUS_SRAM_RET_EN` reader - 3:0\\]
MCU SRAM is partitioned into 1 bank . This register controls whether the bank that has retention during MCU Bus domain power off."]
pub type BUS_SRAM_RET_EN_R = crate::FieldReader<u8, BUS_SRAM_RET_EN_A>;
#[doc = "3:0\\]
MCU SRAM is partitioned into 1 bank . This register controls whether the bank that has retention during MCU Bus domain power off.\n\nValue on reset: 15"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BUS_SRAM_RET_EN_A {
    #[doc = "15: Retention is enabled"]
    RET_FULL = 15,
    #[doc = "0: Retention is disabled"]
    RET_NONE = 0,
}
impl From<BUS_SRAM_RET_EN_A> for u8 {
    #[inline(always)]
    fn from(variant: BUS_SRAM_RET_EN_A) -> Self {
        variant as _
    }
}
impl BUS_SRAM_RET_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BUS_SRAM_RET_EN_A> {
        match self.bits {
            15 => Some(BUS_SRAM_RET_EN_A::RET_FULL),
            0 => Some(BUS_SRAM_RET_EN_A::RET_NONE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RET_FULL`"]
    #[inline(always)]
    pub fn is_ret_full(&self) -> bool {
        *self == BUS_SRAM_RET_EN_A::RET_FULL
    }
    #[doc = "Checks if the value of the field is `RET_NONE`"]
    #[inline(always)]
    pub fn is_ret_none(&self) -> bool {
        *self == BUS_SRAM_RET_EN_A::RET_NONE
    }
}
#[doc = "Field `BUS_SRAM_RET_EN` writer - 3:0\\]
MCU SRAM is partitioned into 1 bank . This register controls whether the bank that has retention during MCU Bus domain power off."]
pub type BUS_SRAM_RET_EN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RAMCFG_SPEC, u8, BUS_SRAM_RET_EN_A, 4, O>;
impl<'a, const O: u8> BUS_SRAM_RET_EN_W<'a, O> {
    #[doc = "Retention is enabled"]
    #[inline(always)]
    pub fn ret_full(self) -> &'a mut W {
        self.variant(BUS_SRAM_RET_EN_A::RET_FULL)
    }
    #[doc = "Retention is disabled"]
    #[inline(always)]
    pub fn ret_none(self) -> &'a mut W {
        self.variant(BUS_SRAM_RET_EN_A::RET_NONE)
    }
}
#[doc = "Field `RESERVED4` reader - 15:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED4_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RESERVED4` writer - 15:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RAMCFG_SPEC, u16, u16, 12, O>;
#[doc = "Field `AUX_SRAM_RET_EN` reader - 16:16\\]
Internal. Only to be used through TI provided API."]
pub type AUX_SRAM_RET_EN_R = crate::BitReader<bool>;
#[doc = "Field `AUX_SRAM_RET_EN` writer - 16:16\\]
Internal. Only to be used through TI provided API."]
pub type AUX_SRAM_RET_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAMCFG_SPEC, bool, O>;
#[doc = "Field `AUX_SRAM_PWR_OFF` reader - 17:17\\]
Internal. Only to be used through TI provided API."]
pub type AUX_SRAM_PWR_OFF_R = crate::BitReader<bool>;
#[doc = "Field `AUX_SRAM_PWR_OFF` writer - 17:17\\]
Internal. Only to be used through TI provided API."]
pub type AUX_SRAM_PWR_OFF_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAMCFG_SPEC, bool, O>;
#[doc = "Field `RESERVED18` reader - 31:18\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED18_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RESERVED18` writer - 31:18\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED18_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RAMCFG_SPEC, u16, u16, 14, O>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
MCU SRAM is partitioned into 1 bank . This register controls whether the bank that has retention during MCU Bus domain power off."]
    #[inline(always)]
    pub fn bus_sram_ret_en(&self) -> BUS_SRAM_RET_EN_R {
        BUS_SRAM_RET_EN_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:15 - 15:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved4(&self) -> RESERVED4_R {
        RESERVED4_R::new(((self.bits >> 4) & 0x0fff) as u16)
    }
    #[doc = "Bit 16 - 16:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn aux_sram_ret_en(&self) -> AUX_SRAM_RET_EN_R {
        AUX_SRAM_RET_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn aux_sram_pwr_off(&self) -> AUX_SRAM_PWR_OFF_R {
        AUX_SRAM_PWR_OFF_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:31 - 31:18\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved18(&self) -> RESERVED18_R {
        RESERVED18_R::new(((self.bits >> 18) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
MCU SRAM is partitioned into 1 bank . This register controls whether the bank that has retention during MCU Bus domain power off."]
    #[inline(always)]
    #[must_use]
    pub fn bus_sram_ret_en(&mut self) -> BUS_SRAM_RET_EN_W<0> {
        BUS_SRAM_RET_EN_W::new(self)
    }
    #[doc = "Bits 4:15 - 15:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved4(&mut self) -> RESERVED4_W<4> {
        RESERVED4_W::new(self)
    }
    #[doc = "Bit 16 - 16:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn aux_sram_ret_en(&mut self) -> AUX_SRAM_RET_EN_W<16> {
        AUX_SRAM_RET_EN_W::new(self)
    }
    #[doc = "Bit 17 - 17:17\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn aux_sram_pwr_off(&mut self) -> AUX_SRAM_PWR_OFF_W<17> {
        AUX_SRAM_PWR_OFF_W::new(self)
    }
    #[doc = "Bits 18:31 - 31:18\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved18(&mut self) -> RESERVED18_W<18> {
        RESERVED18_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RAM Configuration This register contains power management related configuration for the SRAM in the MCU and AUX domain.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ramcfg](index.html) module"]
pub struct RAMCFG_SPEC;
impl crate::RegisterSpec for RAMCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ramcfg::R](R) reader structure"]
impl crate::Readable for RAMCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ramcfg::W](W) writer structure"]
impl crate::Writable for RAMCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RAMCFG to value 0x0001_000f"]
impl crate::Resettable for RAMCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0x0001_000f;
}
