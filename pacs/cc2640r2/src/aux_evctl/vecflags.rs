#[doc = "Register `VECFLAGS` reader"]
pub type R = crate::R<VecflagsSpec>;
#[doc = "Register `VECFLAGS` writer"]
pub type W = crate::W<VecflagsSpec>;
#[doc = "Field `VEC0` reader - 0:0\\]
Vector flag 0. The vector flag is set if the edge selected VECCFG0.VEC0_POL occurs on the event selected in VECCFG0.VEC0_EV. The flag is cleared by writing a 0 to this bit, or (preferably) a 1 to VECFLAGSCLR.VEC0."]
pub type Vec0R = crate::BitReader;
#[doc = "Field `VEC0` writer - 0:0\\]
Vector flag 0. The vector flag is set if the edge selected VECCFG0.VEC0_POL occurs on the event selected in VECCFG0.VEC0_EV. The flag is cleared by writing a 0 to this bit, or (preferably) a 1 to VECFLAGSCLR.VEC0."]
pub type Vec0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VEC1` reader - 1:1\\]
Vector flag 1. The vector flag is set if the edge selected VECCFG0.VEC1_POL occurs on the event selected in VECCFG0.VEC1_EV. The flag is cleared by writing a 0 to this bit, or (preferably) a 1 to VECFLAGSCLR.VEC1."]
pub type Vec1R = crate::BitReader;
#[doc = "Field `VEC1` writer - 1:1\\]
Vector flag 1. The vector flag is set if the edge selected VECCFG0.VEC1_POL occurs on the event selected in VECCFG0.VEC1_EV. The flag is cleared by writing a 0 to this bit, or (preferably) a 1 to VECFLAGSCLR.VEC1."]
pub type Vec1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VEC2` reader - 2:2\\]
Vector flag 2. The vector flag is set if the edge selected VECCFG1.VEC2_POL occurs on the event selected in VECCFG1.VEC2_EV. The flag is cleared by writing a 0 to this bit, or (preferably) a 1 to VECFLAGSCLR.VEC2."]
pub type Vec2R = crate::BitReader;
#[doc = "Field `VEC2` writer - 2:2\\]
Vector flag 2. The vector flag is set if the edge selected VECCFG1.VEC2_POL occurs on the event selected in VECCFG1.VEC2_EV. The flag is cleared by writing a 0 to this bit, or (preferably) a 1 to VECFLAGSCLR.VEC2."]
pub type Vec2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VEC3` reader - 3:3\\]
Vector flag 3. The vector flag is set if the edge selected VECCFG1.VEC3_POL occurs on the event selected in VECCFG1.VEC3_EV. The flag is cleared by writing a 0 to this bit, or (preferably) a 1 to VECFLAGSCLR.VEC3."]
pub type Vec3R = crate::BitReader;
#[doc = "Field `VEC3` writer - 3:3\\]
Vector flag 3. The vector flag is set if the edge selected VECCFG1.VEC3_POL occurs on the event selected in VECCFG1.VEC3_EV. The flag is cleared by writing a 0 to this bit, or (preferably) a 1 to VECFLAGSCLR.VEC3."]
pub type Vec3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED4` reader - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved4R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED4` writer - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved4W<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Vector flag 0. The vector flag is set if the edge selected VECCFG0.VEC0_POL occurs on the event selected in VECCFG0.VEC0_EV. The flag is cleared by writing a 0 to this bit, or (preferably) a 1 to VECFLAGSCLR.VEC0."]
    #[inline(always)]
    pub fn vec0(&self) -> Vec0R {
        Vec0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Vector flag 1. The vector flag is set if the edge selected VECCFG0.VEC1_POL occurs on the event selected in VECCFG0.VEC1_EV. The flag is cleared by writing a 0 to this bit, or (preferably) a 1 to VECFLAGSCLR.VEC1."]
    #[inline(always)]
    pub fn vec1(&self) -> Vec1R {
        Vec1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Vector flag 2. The vector flag is set if the edge selected VECCFG1.VEC2_POL occurs on the event selected in VECCFG1.VEC2_EV. The flag is cleared by writing a 0 to this bit, or (preferably) a 1 to VECFLAGSCLR.VEC2."]
    #[inline(always)]
    pub fn vec2(&self) -> Vec2R {
        Vec2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Vector flag 3. The vector flag is set if the edge selected VECCFG1.VEC3_POL occurs on the event selected in VECCFG1.VEC3_EV. The flag is cleared by writing a 0 to this bit, or (preferably) a 1 to VECFLAGSCLR.VEC3."]
    #[inline(always)]
    pub fn vec3(&self) -> Vec3R {
        Vec3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:31 - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Vector flag 0. The vector flag is set if the edge selected VECCFG0.VEC0_POL occurs on the event selected in VECCFG0.VEC0_EV. The flag is cleared by writing a 0 to this bit, or (preferably) a 1 to VECFLAGSCLR.VEC0."]
    #[inline(always)]
    #[must_use]
    pub fn vec0(&mut self) -> Vec0W<VecflagsSpec> {
        Vec0W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Vector flag 1. The vector flag is set if the edge selected VECCFG0.VEC1_POL occurs on the event selected in VECCFG0.VEC1_EV. The flag is cleared by writing a 0 to this bit, or (preferably) a 1 to VECFLAGSCLR.VEC1."]
    #[inline(always)]
    #[must_use]
    pub fn vec1(&mut self) -> Vec1W<VecflagsSpec> {
        Vec1W::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Vector flag 2. The vector flag is set if the edge selected VECCFG1.VEC2_POL occurs on the event selected in VECCFG1.VEC2_EV. The flag is cleared by writing a 0 to this bit, or (preferably) a 1 to VECFLAGSCLR.VEC2."]
    #[inline(always)]
    #[must_use]
    pub fn vec2(&mut self) -> Vec2W<VecflagsSpec> {
        Vec2W::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Vector flag 3. The vector flag is set if the edge selected VECCFG1.VEC3_POL occurs on the event selected in VECCFG1.VEC3_EV. The flag is cleared by writing a 0 to this bit, or (preferably) a 1 to VECFLAGSCLR.VEC3."]
    #[inline(always)]
    #[must_use]
    pub fn vec3(&mut self) -> Vec3W<VecflagsSpec> {
        Vec3W::new(self, 3)
    }
    #[doc = "Bits 4:31 - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved4(&mut self) -> Reserved4W<VecflagsSpec> {
        Reserved4W::new(self, 4)
    }
}
#[doc = "Vector Flags If a vector flag becomes 1 and AUX_SCE sleeps, AUX_SCE will wake up and execute the corresponding vector. The vector with the lowest index will execute first if multiple vectors flags are set. AUX_SCE must return to sleep to execute the next vector. During execution of a vector, AUX_SCE must clear the vector flag that triggered execution. Write 1 to bit index n in VECFLAGSCLR to clear vector flag n.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vecflags::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vecflags::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VecflagsSpec;
impl crate::RegisterSpec for VecflagsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vecflags::R`](R) reader structure"]
impl crate::Readable for VecflagsSpec {}
#[doc = "`write(|w| ..)` method takes [`vecflags::W`](W) writer structure"]
impl crate::Writable for VecflagsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VECFLAGS to value 0"]
impl crate::Resettable for VecflagsSpec {
    const RESET_VALUE: u32 = 0;
}
