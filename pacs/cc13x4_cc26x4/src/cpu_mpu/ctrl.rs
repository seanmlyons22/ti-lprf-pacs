#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `ENABLE` reader - 0:0\\]
Enables the MPU"]
pub type EnableR = crate::BitReader;
#[doc = "Field `ENABLE` writer - 0:0\\]
Enables the MPU"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HFNMIENA` reader - 1:1\\]
Controls whether handlers executing with priority less than 0 access memory with the MPU enabled or disabled. This applies to HardFaults, NMIs, and exception handlers when FAULTMASK is set to 1"]
pub type HfnmienaR = crate::BitReader;
#[doc = "Field `HFNMIENA` writer - 1:1\\]
Controls whether handlers executing with priority less than 0 access memory with the MPU enabled or disabled. This applies to HardFaults, NMIs, and exception handlers when FAULTMASK is set to 1"]
pub type HfnmienaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIVDEFENA` reader - 2:2\\]
Controls whether the default memory map is enabled for privileged software"]
pub type PrivdefenaR = crate::BitReader;
#[doc = "Field `PRIVDEFENA` writer - 2:2\\]
Controls whether the default memory map is enabled for privileged software"]
pub type PrivdefenaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED3` reader - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved3R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Enables the MPU"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Controls whether handlers executing with priority less than 0 access memory with the MPU enabled or disabled. This applies to HardFaults, NMIs, and exception handlers when FAULTMASK is set to 1"]
    #[inline(always)]
    pub fn hfnmiena(&self) -> HfnmienaR {
        HfnmienaR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Controls whether the default memory map is enabled for privileged software"]
    #[inline(always)]
    pub fn privdefena(&self) -> PrivdefenaR {
        PrivdefenaR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:31 - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Enables the MPU"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> EnableW<CtrlSpec> {
        EnableW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Controls whether handlers executing with priority less than 0 access memory with the MPU enabled or disabled. This applies to HardFaults, NMIs, and exception handlers when FAULTMASK is set to 1"]
    #[inline(always)]
    #[must_use]
    pub fn hfnmiena(&mut self) -> HfnmienaW<CtrlSpec> {
        HfnmienaW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Controls whether the default memory map is enabled for privileged software"]
    #[inline(always)]
    #[must_use]
    pub fn privdefena(&mut self) -> PrivdefenaW<CtrlSpec> {
        PrivdefenaW::new(self, 2)
    }
}
#[doc = "Enables the MPU and, when the MPU is enabled, controls whether the default memory map is enabled as a background region for privileged accesses, and whether the MPU is enabled for HardFaults, NMIs, and exception handlers when FAULTMASK is set to 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CtrlSpec {
    const RESET_VALUE: u32 = 0;
}
