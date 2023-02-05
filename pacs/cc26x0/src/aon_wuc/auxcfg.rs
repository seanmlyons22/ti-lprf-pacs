#[doc = "Register `AUXCFG` reader"]
pub struct R(crate::R<AUXCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AUXCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AUXCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AUXCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AUXCFG` writer"]
pub struct W(crate::W<AUXCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AUXCFG_SPEC>;
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
impl From<crate::W<AUXCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AUXCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RAM_RET_EN` reader - 0:0\\]
This bit controls retention mode for the AUX_RAM:BANK0: 0: Retention is disabled 1: Retention is enabled NB: If retention is disabled, the AUX_RAM will be powered off when it would otherwise be put in retention mode"]
pub type RAM_RET_EN_R = crate::BitReader<bool>;
#[doc = "Field `RAM_RET_EN` writer - 0:0\\]
This bit controls retention mode for the AUX_RAM:BANK0: 0: Retention is disabled 1: Retention is enabled NB: If retention is disabled, the AUX_RAM will be powered off when it would otherwise be put in retention mode"]
pub type RAM_RET_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AUXCFG_SPEC, bool, O>;
#[doc = "Field `RESERVED1` reader - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED1_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED1` writer - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AUXCFG_SPEC, u32, u32, 31, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
This bit controls retention mode for the AUX_RAM:BANK0: 0: Retention is disabled 1: Retention is enabled NB: If retention is disabled, the AUX_RAM will be powered off when it would otherwise be put in retention mode"]
    #[inline(always)]
    pub fn ram_ret_en(&self) -> RAM_RET_EN_R {
        RAM_RET_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
This bit controls retention mode for the AUX_RAM:BANK0: 0: Retention is disabled 1: Retention is enabled NB: If retention is disabled, the AUX_RAM will be powered off when it would otherwise be put in retention mode"]
    #[inline(always)]
    #[must_use]
    pub fn ram_ret_en(&mut self) -> RAM_RET_EN_W<0> {
        RAM_RET_EN_W::new(self)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> RESERVED1_W<1> {
        RESERVED1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AUX Configuration This register contains power management related signals for the AUX domain.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [auxcfg](index.html) module"]
pub struct AUXCFG_SPEC;
impl crate::RegisterSpec for AUXCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [auxcfg::R](R) reader structure"]
impl crate::Readable for AUXCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [auxcfg::W](W) writer structure"]
impl crate::Writable for AUXCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AUXCFG to value 0x01"]
impl crate::Resettable for AUXCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
