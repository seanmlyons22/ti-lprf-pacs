#[doc = "Register `MPU_CTRL` reader"]
pub struct R(crate::R<MPU_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MPU_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MPU_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MPU_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MPU_CTRL` writer"]
pub struct W(crate::W<MPU_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MPU_CTRL_SPEC>;
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
impl From<crate::W<MPU_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MPU_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLE` reader - 0:0\\]
Enable MPU 0: MPU disabled 1: MPU enabled"]
pub type ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE` writer - 0:0\\]
Enable MPU 0: MPU disabled 1: MPU enabled"]
pub type ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPU_CTRL_SPEC, bool, O>;
#[doc = "Field `HFNMIENA` reader - 1:1\\]
This bit enables the MPU when in Hard Fault, NMI, and FAULTMASK escalated handlers. If this bit and ENABLE are set, the MPU is enabled when in these handlers. If this bit is not set, the MPU is disabled when in these handlers, regardless of the value of ENABLE bit. If this bit is set and ENABLE is not set, behavior is unpredictable."]
pub type HFNMIENA_R = crate::BitReader<bool>;
#[doc = "Field `HFNMIENA` writer - 1:1\\]
This bit enables the MPU when in Hard Fault, NMI, and FAULTMASK escalated handlers. If this bit and ENABLE are set, the MPU is enabled when in these handlers. If this bit is not set, the MPU is disabled when in these handlers, regardless of the value of ENABLE bit. If this bit is set and ENABLE is not set, behavior is unpredictable."]
pub type HFNMIENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPU_CTRL_SPEC, bool, O>;
#[doc = "Field `PRIVDEFENA` reader - 2:2\\]
This bit enables the default memory map for privileged access, as a background region, when the MPU is enabled. The background region acts as if it was region number 1 before any settable regions. Any region that is set up overlays this default map, and overrides it. If this bit is not set, the default memory map is disabled, and memory not covered by a region faults. This applies to memory type, Execute Never (XN), cache and shareable rules. However, this only applies to privileged mode (fetch and data access). User mode code faults unless a region has been set up for its code and data. When the MPU is disabled, the default map acts on both privileged and user mode code. XN and SO rules always apply to the system partition whether this enable is set or not. If the MPU is disabled, this bit is ignored."]
pub type PRIVDEFENA_R = crate::BitReader<bool>;
#[doc = "Field `PRIVDEFENA` writer - 2:2\\]
This bit enables the default memory map for privileged access, as a background region, when the MPU is enabled. The background region acts as if it was region number 1 before any settable regions. Any region that is set up overlays this default map, and overrides it. If this bit is not set, the default memory map is disabled, and memory not covered by a region faults. This applies to memory type, Execute Never (XN), cache and shareable rules. However, this only applies to privileged mode (fetch and data access). User mode code faults unless a region has been set up for its code and data. When the MPU is disabled, the default map acts on both privileged and user mode code. XN and SO rules always apply to the system partition whether this enable is set or not. If the MPU is disabled, this bit is ignored."]
pub type PRIVDEFENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPU_CTRL_SPEC, bool, O>;
#[doc = "Field `RESERVED3` reader - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED3_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED3` writer - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MPU_CTRL_SPEC, u32, u32, 29, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Enable MPU 0: MPU disabled 1: MPU enabled"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
This bit enables the MPU when in Hard Fault, NMI, and FAULTMASK escalated handlers. If this bit and ENABLE are set, the MPU is enabled when in these handlers. If this bit is not set, the MPU is disabled when in these handlers, regardless of the value of ENABLE bit. If this bit is set and ENABLE is not set, behavior is unpredictable."]
    #[inline(always)]
    pub fn hfnmiena(&self) -> HFNMIENA_R {
        HFNMIENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
This bit enables the default memory map for privileged access, as a background region, when the MPU is enabled. The background region acts as if it was region number 1 before any settable regions. Any region that is set up overlays this default map, and overrides it. If this bit is not set, the default memory map is disabled, and memory not covered by a region faults. This applies to memory type, Execute Never (XN), cache and shareable rules. However, this only applies to privileged mode (fetch and data access). User mode code faults unless a region has been set up for its code and data. When the MPU is disabled, the default map acts on both privileged and user mode code. XN and SO rules always apply to the system partition whether this enable is set or not. If the MPU is disabled, this bit is ignored."]
    #[inline(always)]
    pub fn privdefena(&self) -> PRIVDEFENA_R {
        PRIVDEFENA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:31 - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> RESERVED3_R {
        RESERVED3_R::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Enable MPU 0: MPU disabled 1: MPU enabled"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<0> {
        ENABLE_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
This bit enables the MPU when in Hard Fault, NMI, and FAULTMASK escalated handlers. If this bit and ENABLE are set, the MPU is enabled when in these handlers. If this bit is not set, the MPU is disabled when in these handlers, regardless of the value of ENABLE bit. If this bit is set and ENABLE is not set, behavior is unpredictable."]
    #[inline(always)]
    #[must_use]
    pub fn hfnmiena(&mut self) -> HFNMIENA_W<1> {
        HFNMIENA_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
This bit enables the default memory map for privileged access, as a background region, when the MPU is enabled. The background region acts as if it was region number 1 before any settable regions. Any region that is set up overlays this default map, and overrides it. If this bit is not set, the default memory map is disabled, and memory not covered by a region faults. This applies to memory type, Execute Never (XN), cache and shareable rules. However, this only applies to privileged mode (fetch and data access). User mode code faults unless a region has been set up for its code and data. When the MPU is disabled, the default map acts on both privileged and user mode code. XN and SO rules always apply to the system partition whether this enable is set or not. If the MPU is disabled, this bit is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn privdefena(&mut self) -> PRIVDEFENA_W<2> {
        PRIVDEFENA_W::new(self)
    }
    #[doc = "Bits 3:31 - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> RESERVED3_W<3> {
        RESERVED3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MPU Control This register is used to enable the MPU, enable the default memory map (background region), and enable the MPU when in Hard Fault, Non-maskable Interrupt (NMI), and FAULTMASK escalated handlers. When the MPU is enabled, at least one region of the memory map must be enabled for the MPU to function unless the PRIVDEFENA bit is set. If the PRIVDEFENA bit is set and no regions are enabled, then only privileged code can operate. When the MPU is disabled, the default address map is used, as if no MPU is present. When the MPU is enabled, only the system partition and vector table loads are always accessible. Other areas are accessible based on regions and whether PRIVDEFENA is enabled. Unless HFNMIENA is set, the MPU is not enabled when the exception priority is -1 or -2. These priorities are only possible when in Hard fault, NMI, or when FAULTMASK is enabled. The HFNMIENA bit enables the MPU when operating with these two priorities.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpu_ctrl](index.html) module"]
pub struct MPU_CTRL_SPEC;
impl crate::RegisterSpec for MPU_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mpu_ctrl::R](R) reader structure"]
impl crate::Readable for MPU_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mpu_ctrl::W](W) writer structure"]
impl crate::Writable for MPU_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MPU_CTRL to value 0"]
impl crate::Resettable for MPU_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
