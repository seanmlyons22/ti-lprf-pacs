#[doc = "Register `BL_CONFIG` reader"]
pub struct R(crate::R<BL_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BL_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BL_CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BL_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BL_CONFIG` writer"]
pub struct W(crate::W<BL_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BL_CONFIG_SPEC>;
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
impl From<crate::W<BL_CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BL_CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BL_ENABLE` reader - 7:0\\]
Enables the boot loader backdoor. 0xC5: Boot loader backdoor is enabled. Any other value: Boot loader backdoor is disabled. NOTE! Boot loader must be enabled (see BOOTLOADER_ENABLE) if boot loader backdoor is enabled."]
pub type BL_ENABLE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BL_ENABLE` writer - 7:0\\]
Enables the boot loader backdoor. 0xC5: Boot loader backdoor is enabled. Any other value: Boot loader backdoor is disabled. NOTE! Boot loader must be enabled (see BOOTLOADER_ENABLE) if boot loader backdoor is enabled."]
pub type BL_ENABLE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BL_CONFIG_SPEC, u8, u8, 8, O>;
#[doc = "Field `BL_PIN_NUMBER` reader - 15:8\\]
DIO number that is level checked if the boot loader backdoor is enabled by the BL_ENABLE field."]
pub type BL_PIN_NUMBER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BL_PIN_NUMBER` writer - 15:8\\]
DIO number that is level checked if the boot loader backdoor is enabled by the BL_ENABLE field."]
pub type BL_PIN_NUMBER_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, BL_CONFIG_SPEC, u8, u8, 8, O>;
#[doc = "Field `BL_LEVEL` reader - 16:16\\]
Sets the active level of the selected DIO number BL_PIN_NUMBER if boot loader backdoor is enabled by the BL_ENABLE field. 0: Active low. 1: Active high."]
pub type BL_LEVEL_R = crate::BitReader<bool>;
#[doc = "Field `BL_LEVEL` writer - 16:16\\]
Sets the active level of the selected DIO number BL_PIN_NUMBER if boot loader backdoor is enabled by the BL_ENABLE field. 0: Active low. 1: Active high."]
pub type BL_LEVEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, BL_CONFIG_SPEC, bool, O>;
#[doc = "Field `BOOTLOADER_ENABLE` reader - 31:24\\]
Bootloader enable. Boot loader can be accessed if IMAGE_VALID_CONF.IMAGE_VALID is an invalid vector table address or BL_ENABLE is enabled (and conditions for boot loader backdoor are met). 0xC5: Boot loader is enabled. Any other value: Boot loader is disabled."]
pub type BOOTLOADER_ENABLE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BOOTLOADER_ENABLE` writer - 31:24\\]
Bootloader enable. Boot loader can be accessed if IMAGE_VALID_CONF.IMAGE_VALID is an invalid vector table address or BL_ENABLE is enabled (and conditions for boot loader backdoor are met). 0xC5: Boot loader is enabled. Any other value: Boot loader is disabled."]
pub type BOOTLOADER_ENABLE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, BL_CONFIG_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Enables the boot loader backdoor. 0xC5: Boot loader backdoor is enabled. Any other value: Boot loader backdoor is disabled. NOTE! Boot loader must be enabled (see BOOTLOADER_ENABLE) if boot loader backdoor is enabled."]
    #[inline(always)]
    pub fn bl_enable(&self) -> BL_ENABLE_R {
        BL_ENABLE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
DIO number that is level checked if the boot loader backdoor is enabled by the BL_ENABLE field."]
    #[inline(always)]
    pub fn bl_pin_number(&self) -> BL_PIN_NUMBER_R {
        BL_PIN_NUMBER_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
Sets the active level of the selected DIO number BL_PIN_NUMBER if boot loader backdoor is enabled by the BL_ENABLE field. 0: Active low. 1: Active high."]
    #[inline(always)]
    pub fn bl_level(&self) -> BL_LEVEL_R {
        BL_LEVEL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Bootloader enable. Boot loader can be accessed if IMAGE_VALID_CONF.IMAGE_VALID is an invalid vector table address or BL_ENABLE is enabled (and conditions for boot loader backdoor are met). 0xC5: Boot loader is enabled. Any other value: Boot loader is disabled."]
    #[inline(always)]
    pub fn bootloader_enable(&self) -> BOOTLOADER_ENABLE_R {
        BOOTLOADER_ENABLE_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Enables the boot loader backdoor. 0xC5: Boot loader backdoor is enabled. Any other value: Boot loader backdoor is disabled. NOTE! Boot loader must be enabled (see BOOTLOADER_ENABLE) if boot loader backdoor is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn bl_enable(&mut self) -> BL_ENABLE_W<0> {
        BL_ENABLE_W::new(self)
    }
    #[doc = "Bits 8:15 - 15:8\\]
DIO number that is level checked if the boot loader backdoor is enabled by the BL_ENABLE field."]
    #[inline(always)]
    #[must_use]
    pub fn bl_pin_number(&mut self) -> BL_PIN_NUMBER_W<8> {
        BL_PIN_NUMBER_W::new(self)
    }
    #[doc = "Bit 16 - 16:16\\]
Sets the active level of the selected DIO number BL_PIN_NUMBER if boot loader backdoor is enabled by the BL_ENABLE field. 0: Active low. 1: Active high."]
    #[inline(always)]
    #[must_use]
    pub fn bl_level(&mut self) -> BL_LEVEL_W<16> {
        BL_LEVEL_W::new(self)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Bootloader enable. Boot loader can be accessed if IMAGE_VALID_CONF.IMAGE_VALID is an invalid vector table address or BL_ENABLE is enabled (and conditions for boot loader backdoor are met). 0xC5: Boot loader is enabled. Any other value: Boot loader is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn bootloader_enable(&mut self) -> BOOTLOADER_ENABLE_W<24> {
        BOOTLOADER_ENABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Bootloader Configuration Configures the functionality of the ROM boot loader. If both the boot loader is enabled by the BOOTLOADER_ENABLE field and the boot loader backdoor is enabled by the BL_ENABLE field it is possible to force entry of the ROM boot loader even if a valid image is present in flash.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bl_config](index.html) module"]
pub struct BL_CONFIG_SPEC;
impl crate::RegisterSpec for BL_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bl_config::R](R) reader structure"]
impl crate::Readable for BL_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bl_config::W](W) writer structure"]
impl crate::Writable for BL_CONFIG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BL_CONFIG to value 0xc5ff_ffff"]
impl crate::Resettable for BL_CONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0xc5ff_ffff;
}
