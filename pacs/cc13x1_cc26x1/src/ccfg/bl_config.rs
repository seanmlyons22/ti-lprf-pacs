#[doc = "Register `BL_CONFIG` reader"]
pub type R = crate::R<BlConfigSpec>;
#[doc = "Register `BL_CONFIG` writer"]
pub type W = crate::W<BlConfigSpec>;
#[doc = "Field `BL_ENABLE` reader - 7:0\\]
Enables the boot loader backdoor. 0xC5: Boot loader backdoor is enabled. Any other value: Boot loader backdoor is disabled. NOTE! Boot loader must be enabled (see BOOTLOADER_ENABLE) if boot loader backdoor is enabled."]
pub type BlEnableR = crate::FieldReader;
#[doc = "Field `BL_ENABLE` writer - 7:0\\]
Enables the boot loader backdoor. 0xC5: Boot loader backdoor is enabled. Any other value: Boot loader backdoor is disabled. NOTE! Boot loader must be enabled (see BOOTLOADER_ENABLE) if boot loader backdoor is enabled."]
pub type BlEnableW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BL_PIN_NUMBER` reader - 15:8\\]
DIO number that is level checked if the boot loader backdoor is enabled by the BL_ENABLE field."]
pub type BlPinNumberR = crate::FieldReader;
#[doc = "Field `BL_PIN_NUMBER` writer - 15:8\\]
DIO number that is level checked if the boot loader backdoor is enabled by the BL_ENABLE field."]
pub type BlPinNumberW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BL_LEVEL` reader - 16:16\\]
Sets the active level of the selected DIO number BL_PIN_NUMBER if boot loader backdoor is enabled by the BL_ENABLE field. 0: Active low. 1: Active high."]
pub type BlLevelR = crate::BitReader;
#[doc = "Field `BL_LEVEL` writer - 16:16\\]
Sets the active level of the selected DIO number BL_PIN_NUMBER if boot loader backdoor is enabled by the BL_ENABLE field. 0: Active low. 1: Active high."]
pub type BlLevelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOOTLOADER_ENABLE` reader - 31:24\\]
Bootloader enable. Boot loader can be accessed if IMAGE_VALID_CONF.IMAGE_VALID is non-zero or BL_ENABLE is enabled (and conditions for boot loader backdoor are met). 0xC5: Boot loader is enabled. Any other value: Boot loader is disabled."]
pub type BootloaderEnableR = crate::FieldReader;
#[doc = "Field `BOOTLOADER_ENABLE` writer - 31:24\\]
Bootloader enable. Boot loader can be accessed if IMAGE_VALID_CONF.IMAGE_VALID is non-zero or BL_ENABLE is enabled (and conditions for boot loader backdoor are met). 0xC5: Boot loader is enabled. Any other value: Boot loader is disabled."]
pub type BootloaderEnableW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Enables the boot loader backdoor. 0xC5: Boot loader backdoor is enabled. Any other value: Boot loader backdoor is disabled. NOTE! Boot loader must be enabled (see BOOTLOADER_ENABLE) if boot loader backdoor is enabled."]
    #[inline(always)]
    pub fn bl_enable(&self) -> BlEnableR {
        BlEnableR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
DIO number that is level checked if the boot loader backdoor is enabled by the BL_ENABLE field."]
    #[inline(always)]
    pub fn bl_pin_number(&self) -> BlPinNumberR {
        BlPinNumberR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
Sets the active level of the selected DIO number BL_PIN_NUMBER if boot loader backdoor is enabled by the BL_ENABLE field. 0: Active low. 1: Active high."]
    #[inline(always)]
    pub fn bl_level(&self) -> BlLevelR {
        BlLevelR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Bootloader enable. Boot loader can be accessed if IMAGE_VALID_CONF.IMAGE_VALID is non-zero or BL_ENABLE is enabled (and conditions for boot loader backdoor are met). 0xC5: Boot loader is enabled. Any other value: Boot loader is disabled."]
    #[inline(always)]
    pub fn bootloader_enable(&self) -> BootloaderEnableR {
        BootloaderEnableR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Enables the boot loader backdoor. 0xC5: Boot loader backdoor is enabled. Any other value: Boot loader backdoor is disabled. NOTE! Boot loader must be enabled (see BOOTLOADER_ENABLE) if boot loader backdoor is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn bl_enable(&mut self) -> BlEnableW<BlConfigSpec> {
        BlEnableW::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
DIO number that is level checked if the boot loader backdoor is enabled by the BL_ENABLE field."]
    #[inline(always)]
    #[must_use]
    pub fn bl_pin_number(&mut self) -> BlPinNumberW<BlConfigSpec> {
        BlPinNumberW::new(self, 8)
    }
    #[doc = "Bit 16 - 16:16\\]
Sets the active level of the selected DIO number BL_PIN_NUMBER if boot loader backdoor is enabled by the BL_ENABLE field. 0: Active low. 1: Active high."]
    #[inline(always)]
    #[must_use]
    pub fn bl_level(&mut self) -> BlLevelW<BlConfigSpec> {
        BlLevelW::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Bootloader enable. Boot loader can be accessed if IMAGE_VALID_CONF.IMAGE_VALID is non-zero or BL_ENABLE is enabled (and conditions for boot loader backdoor are met). 0xC5: Boot loader is enabled. Any other value: Boot loader is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn bootloader_enable(&mut self) -> BootloaderEnableW<BlConfigSpec> {
        BootloaderEnableW::new(self, 24)
    }
}
#[doc = "Bootloader Configuration Configures the functionality of the ROM boot loader. If both the boot loader is enabled by the BOOTLOADER_ENABLE field and the boot loader backdoor is enabled by the BL_ENABLE field it is possible to force entry of the ROM boot loader even if a valid image is present in flash.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bl_config::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bl_config::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlConfigSpec;
impl crate::RegisterSpec for BlConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bl_config::R`](R) reader structure"]
impl crate::Readable for BlConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`bl_config::W`](W) writer structure"]
impl crate::Writable for BlConfigSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BL_CONFIG to value 0xc5ff_ffff"]
impl crate::Resettable for BlConfigSpec {
    const RESET_VALUE: u32 = 0xc5ff_ffff;
}
