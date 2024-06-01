#[doc = "Register `MPU_CTRL` reader"]
pub type R = crate::R<MpuCtrlSpec>;
#[doc = "Register `MPU_CTRL` writer"]
pub type W = crate::W<MpuCtrlSpec>;
#[doc = "Field `ENABLE` reader - 0:0\\]
Enable MPU 0: MPU disabled 1: MPU enabled"]
pub type EnableR = crate::BitReader;
#[doc = "Field `ENABLE` writer - 0:0\\]
Enable MPU 0: MPU disabled 1: MPU enabled"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HFNMIENA` reader - 1:1\\]
This bit enables the MPU when in Hard Fault, NMI, and FAULTMASK escalated handlers. If this bit and ENABLE are set, the MPU is enabled when in these handlers. If this bit is not set, the MPU is disabled when in these handlers, regardless of the value of ENABLE bit. If this bit is set and ENABLE is not set, behavior is unpredictable."]
pub type HfnmienaR = crate::BitReader;
#[doc = "Field `HFNMIENA` writer - 1:1\\]
This bit enables the MPU when in Hard Fault, NMI, and FAULTMASK escalated handlers. If this bit and ENABLE are set, the MPU is enabled when in these handlers. If this bit is not set, the MPU is disabled when in these handlers, regardless of the value of ENABLE bit. If this bit is set and ENABLE is not set, behavior is unpredictable."]
pub type HfnmienaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIVDEFENA` reader - 2:2\\]
This bit enables the default memory map for privileged access, as a background region, when the MPU is enabled. The background region acts as if it was region number 1 before any settable regions. Any region that is set up overlays this default map, and overrides it. If this bit is not set, the default memory map is disabled, and memory not covered by a region faults. This applies to memory type, Execute Never (XN), cache and shareable rules. However, this only applies to privileged mode (fetch and data access). User mode code faults unless a region has been set up for its code and data. When the MPU is disabled, the default map acts on both privileged and user mode code. XN and SO rules always apply to the system partition whether this enable is set or not. If the MPU is disabled, this bit is ignored."]
pub type PrivdefenaR = crate::BitReader;
#[doc = "Field `PRIVDEFENA` writer - 2:2\\]
This bit enables the default memory map for privileged access, as a background region, when the MPU is enabled. The background region acts as if it was region number 1 before any settable regions. Any region that is set up overlays this default map, and overrides it. If this bit is not set, the default memory map is disabled, and memory not covered by a region faults. This applies to memory type, Execute Never (XN), cache and shareable rules. However, this only applies to privileged mode (fetch and data access). User mode code faults unless a region has been set up for its code and data. When the MPU is disabled, the default map acts on both privileged and user mode code. XN and SO rules always apply to the system partition whether this enable is set or not. If the MPU is disabled, this bit is ignored."]
pub type PrivdefenaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED3` reader - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved3R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED3` writer - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved3W<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Enable MPU 0: MPU disabled 1: MPU enabled"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
This bit enables the MPU when in Hard Fault, NMI, and FAULTMASK escalated handlers. If this bit and ENABLE are set, the MPU is enabled when in these handlers. If this bit is not set, the MPU is disabled when in these handlers, regardless of the value of ENABLE bit. If this bit is set and ENABLE is not set, behavior is unpredictable."]
    #[inline(always)]
    pub fn hfnmiena(&self) -> HfnmienaR {
        HfnmienaR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
This bit enables the default memory map for privileged access, as a background region, when the MPU is enabled. The background region acts as if it was region number 1 before any settable regions. Any region that is set up overlays this default map, and overrides it. If this bit is not set, the default memory map is disabled, and memory not covered by a region faults. This applies to memory type, Execute Never (XN), cache and shareable rules. However, this only applies to privileged mode (fetch and data access). User mode code faults unless a region has been set up for its code and data. When the MPU is disabled, the default map acts on both privileged and user mode code. XN and SO rules always apply to the system partition whether this enable is set or not. If the MPU is disabled, this bit is ignored."]
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
Enable MPU 0: MPU disabled 1: MPU enabled"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> EnableW<MpuCtrlSpec> {
        EnableW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
This bit enables the MPU when in Hard Fault, NMI, and FAULTMASK escalated handlers. If this bit and ENABLE are set, the MPU is enabled when in these handlers. If this bit is not set, the MPU is disabled when in these handlers, regardless of the value of ENABLE bit. If this bit is set and ENABLE is not set, behavior is unpredictable."]
    #[inline(always)]
    #[must_use]
    pub fn hfnmiena(&mut self) -> HfnmienaW<MpuCtrlSpec> {
        HfnmienaW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
This bit enables the default memory map for privileged access, as a background region, when the MPU is enabled. The background region acts as if it was region number 1 before any settable regions. Any region that is set up overlays this default map, and overrides it. If this bit is not set, the default memory map is disabled, and memory not covered by a region faults. This applies to memory type, Execute Never (XN), cache and shareable rules. However, this only applies to privileged mode (fetch and data access). User mode code faults unless a region has been set up for its code and data. When the MPU is disabled, the default map acts on both privileged and user mode code. XN and SO rules always apply to the system partition whether this enable is set or not. If the MPU is disabled, this bit is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn privdefena(&mut self) -> PrivdefenaW<MpuCtrlSpec> {
        PrivdefenaW::new(self, 2)
    }
    #[doc = "Bits 3:31 - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> Reserved3W<MpuCtrlSpec> {
        Reserved3W::new(self, 3)
    }
}
#[doc = "MPU Control This register is used to enable the MPU, enable the default memory map (background region), and enable the MPU when in Hard Fault, Non-maskable Interrupt (NMI), and FAULTMASK escalated handlers. When the MPU is enabled, at least one region of the memory map must be enabled for the MPU to function unless the PRIVDEFENA bit is set. If the PRIVDEFENA bit is set and no regions are enabled, then only privileged code can operate. When the MPU is disabled, the default address map is used, as if no MPU is present. When the MPU is enabled, only the system partition and vector table loads are always accessible. Other areas are accessible based on regions and whether PRIVDEFENA is enabled. Unless HFNMIENA is set, the MPU is not enabled when the exception priority is -1 or -2. These priorities are only possible when in Hard fault, NMI, or when FAULTMASK is enabled. The HFNMIENA bit enables the MPU when operating with these two priorities.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpu_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpu_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MpuCtrlSpec;
impl crate::RegisterSpec for MpuCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mpu_ctrl::R`](R) reader structure"]
impl crate::Readable for MpuCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`mpu_ctrl::W`](W) writer structure"]
impl crate::Writable for MpuCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MPU_CTRL to value 0"]
impl crate::Resettable for MpuCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
