#[doc = "Register `MCUCFG` reader"]
pub struct R(crate::R<MCUCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCUCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCUCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCUCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCUCFG` writer"]
pub struct W(crate::W<MCUCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCUCFG_SPEC>;
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
impl From<crate::W<MCUCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCUCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SRAM_RET_EN` reader - 3:0\\]
MCU SRAM is partitioned into 4 banks . This register controls which of the banks that has retention during MCU power off"]
pub type SRAM_RET_EN_R = crate::FieldReader<u8, SRAM_RET_EN_A>;
#[doc = "3:0\\]
MCU SRAM is partitioned into 4 banks . This register controls which of the banks that has retention during MCU power off\n\nValue on reset: 15"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SRAM_RET_EN_A {
    #[doc = "15: Retention on for all banks (SRAM:BANK0, SRAM:BANK1 ,SRAM:BANK2 and SRAM:BANK3)"]
    RET_FULL = 15,
    #[doc = "7: Retention on for SRAM:BANK0, SRAM:BANK1 and SRAM:BANK2"]
    RET_LEVEL3 = 7,
    #[doc = "3: Retention on for SRAM:BANK0 and SRAM:BANK1"]
    RET_LEVEL2 = 3,
    #[doc = "1: Retention on for SRAM:BANK0"]
    RET_LEVEL1 = 1,
    #[doc = "0: Retention is disabled"]
    RET_NONE = 0,
}
impl From<SRAM_RET_EN_A> for u8 {
    #[inline(always)]
    fn from(variant: SRAM_RET_EN_A) -> Self {
        variant as _
    }
}
impl SRAM_RET_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SRAM_RET_EN_A> {
        match self.bits {
            15 => Some(SRAM_RET_EN_A::RET_FULL),
            7 => Some(SRAM_RET_EN_A::RET_LEVEL3),
            3 => Some(SRAM_RET_EN_A::RET_LEVEL2),
            1 => Some(SRAM_RET_EN_A::RET_LEVEL1),
            0 => Some(SRAM_RET_EN_A::RET_NONE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RET_FULL`"]
    #[inline(always)]
    pub fn is_ret_full(&self) -> bool {
        *self == SRAM_RET_EN_A::RET_FULL
    }
    #[doc = "Checks if the value of the field is `RET_LEVEL3`"]
    #[inline(always)]
    pub fn is_ret_level3(&self) -> bool {
        *self == SRAM_RET_EN_A::RET_LEVEL3
    }
    #[doc = "Checks if the value of the field is `RET_LEVEL2`"]
    #[inline(always)]
    pub fn is_ret_level2(&self) -> bool {
        *self == SRAM_RET_EN_A::RET_LEVEL2
    }
    #[doc = "Checks if the value of the field is `RET_LEVEL1`"]
    #[inline(always)]
    pub fn is_ret_level1(&self) -> bool {
        *self == SRAM_RET_EN_A::RET_LEVEL1
    }
    #[doc = "Checks if the value of the field is `RET_NONE`"]
    #[inline(always)]
    pub fn is_ret_none(&self) -> bool {
        *self == SRAM_RET_EN_A::RET_NONE
    }
}
#[doc = "Field `SRAM_RET_EN` writer - 3:0\\]
MCU SRAM is partitioned into 4 banks . This register controls which of the banks that has retention during MCU power off"]
pub type SRAM_RET_EN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MCUCFG_SPEC, u8, SRAM_RET_EN_A, 4, O>;
impl<'a, const O: u8> SRAM_RET_EN_W<'a, O> {
    #[doc = "Retention on for all banks (SRAM:BANK0, SRAM:BANK1 ,SRAM:BANK2 and SRAM:BANK3)"]
    #[inline(always)]
    pub fn ret_full(self) -> &'a mut W {
        self.variant(SRAM_RET_EN_A::RET_FULL)
    }
    #[doc = "Retention on for SRAM:BANK0, SRAM:BANK1 and SRAM:BANK2"]
    #[inline(always)]
    pub fn ret_level3(self) -> &'a mut W {
        self.variant(SRAM_RET_EN_A::RET_LEVEL3)
    }
    #[doc = "Retention on for SRAM:BANK0 and SRAM:BANK1"]
    #[inline(always)]
    pub fn ret_level2(self) -> &'a mut W {
        self.variant(SRAM_RET_EN_A::RET_LEVEL2)
    }
    #[doc = "Retention on for SRAM:BANK0"]
    #[inline(always)]
    pub fn ret_level1(self) -> &'a mut W {
        self.variant(SRAM_RET_EN_A::RET_LEVEL1)
    }
    #[doc = "Retention is disabled"]
    #[inline(always)]
    pub fn ret_none(self) -> &'a mut W {
        self.variant(SRAM_RET_EN_A::RET_NONE)
    }
}
#[doc = "Field `RESERVED4` reader - 15:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED4_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RESERVED4` writer - 15:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MCUCFG_SPEC, u16, u16, 12, O>;
#[doc = "Field `FIXED_WU_EN` reader - 16:16\\]
Internal. Only to be used through TI provided API."]
pub type FIXED_WU_EN_R = crate::BitReader<bool>;
#[doc = "Field `FIXED_WU_EN` writer - 16:16\\]
Internal. Only to be used through TI provided API."]
pub type FIXED_WU_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCUCFG_SPEC, bool, O>;
#[doc = "Field `VIRT_OFF` reader - 17:17\\]
Internal. Only to be used through TI provided API."]
pub type VIRT_OFF_R = crate::BitReader<bool>;
#[doc = "Field `VIRT_OFF` writer - 17:17\\]
Internal. Only to be used through TI provided API."]
pub type VIRT_OFF_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCUCFG_SPEC, bool, O>;
#[doc = "Field `RESERVED18` reader - 31:18\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED18_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RESERVED18` writer - 31:18\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED18_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MCUCFG_SPEC, u16, u16, 14, O>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
MCU SRAM is partitioned into 4 banks . This register controls which of the banks that has retention during MCU power off"]
    #[inline(always)]
    pub fn sram_ret_en(&self) -> SRAM_RET_EN_R {
        SRAM_RET_EN_R::new((self.bits & 0x0f) as u8)
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
    pub fn fixed_wu_en(&self) -> FIXED_WU_EN_R {
        FIXED_WU_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn virt_off(&self) -> VIRT_OFF_R {
        VIRT_OFF_R::new(((self.bits >> 17) & 1) != 0)
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
MCU SRAM is partitioned into 4 banks . This register controls which of the banks that has retention during MCU power off"]
    #[inline(always)]
    #[must_use]
    pub fn sram_ret_en(&mut self) -> SRAM_RET_EN_W<0> {
        SRAM_RET_EN_W::new(self)
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
    pub fn fixed_wu_en(&mut self) -> FIXED_WU_EN_W<16> {
        FIXED_WU_EN_W::new(self)
    }
    #[doc = "Bit 17 - 17:17\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn virt_off(&mut self) -> VIRT_OFF_W<17> {
        VIRT_OFF_W::new(self)
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
#[doc = "MCU Configuration This register contains power management related bitfields for the MCU domain.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcucfg](index.html) module"]
pub struct MCUCFG_SPEC;
impl crate::RegisterSpec for MCUCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mcucfg::R](R) reader structure"]
impl crate::Readable for MCUCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mcucfg::W](W) writer structure"]
impl crate::Writable for MCUCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MCUCFG to value 0x0f"]
impl crate::Resettable for MCUCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0x0f;
}
