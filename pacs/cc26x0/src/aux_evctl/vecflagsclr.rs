#[doc = "Register `VECFLAGSCLR` reader"]
pub type R = crate::R<VecflagsclrSpec>;
#[doc = "Register `VECFLAGSCLR` writer"]
pub type W = crate::W<VecflagsclrSpec>;
#[doc = "Field `VEC0` reader - 0:0\\]
Clear vector flag 0. 0: No effect. 1: Clear VECFLAGS.VEC0. Read value is 0."]
pub type Vec0R = crate::BitReader;
#[doc = "Field `VEC0` writer - 0:0\\]
Clear vector flag 0. 0: No effect. 1: Clear VECFLAGS.VEC0. Read value is 0."]
pub type Vec0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VEC1` reader - 1:1\\]
Clear vector flag 1. 0: No effect. 1: Clear VECFLAGS.VEC1. Read value is 0."]
pub type Vec1R = crate::BitReader;
#[doc = "Field `VEC1` writer - 1:1\\]
Clear vector flag 1. 0: No effect. 1: Clear VECFLAGS.VEC1. Read value is 0."]
pub type Vec1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VEC2` reader - 2:2\\]
Clear vector flag 2. 0: No effect. 1: Clear VECFLAGS.VEC2. Read value is 0."]
pub type Vec2R = crate::BitReader;
#[doc = "Field `VEC2` writer - 2:2\\]
Clear vector flag 2. 0: No effect. 1: Clear VECFLAGS.VEC2. Read value is 0."]
pub type Vec2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VEC3` reader - 3:3\\]
Clear vector flag 3. 0: No effect. 1: Clear VECFLAGS.VEC3. Read value is 0."]
pub type Vec3R = crate::BitReader;
#[doc = "Field `VEC3` writer - 3:3\\]
Clear vector flag 3. 0: No effect. 1: Clear VECFLAGS.VEC3. Read value is 0."]
pub type Vec3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED4` reader - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved4R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED4` writer - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved4W<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Clear vector flag 0. 0: No effect. 1: Clear VECFLAGS.VEC0. Read value is 0."]
    #[inline(always)]
    pub fn vec0(&self) -> Vec0R {
        Vec0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Clear vector flag 1. 0: No effect. 1: Clear VECFLAGS.VEC1. Read value is 0."]
    #[inline(always)]
    pub fn vec1(&self) -> Vec1R {
        Vec1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Clear vector flag 2. 0: No effect. 1: Clear VECFLAGS.VEC2. Read value is 0."]
    #[inline(always)]
    pub fn vec2(&self) -> Vec2R {
        Vec2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Clear vector flag 3. 0: No effect. 1: Clear VECFLAGS.VEC3. Read value is 0."]
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
Clear vector flag 0. 0: No effect. 1: Clear VECFLAGS.VEC0. Read value is 0."]
    #[inline(always)]
    #[must_use]
    pub fn vec0(&mut self) -> Vec0W<VecflagsclrSpec> {
        Vec0W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Clear vector flag 1. 0: No effect. 1: Clear VECFLAGS.VEC1. Read value is 0."]
    #[inline(always)]
    #[must_use]
    pub fn vec1(&mut self) -> Vec1W<VecflagsclrSpec> {
        Vec1W::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Clear vector flag 2. 0: No effect. 1: Clear VECFLAGS.VEC2. Read value is 0."]
    #[inline(always)]
    #[must_use]
    pub fn vec2(&mut self) -> Vec2W<VecflagsclrSpec> {
        Vec2W::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Clear vector flag 3. 0: No effect. 1: Clear VECFLAGS.VEC3. Read value is 0."]
    #[inline(always)]
    #[must_use]
    pub fn vec3(&mut self) -> Vec3W<VecflagsclrSpec> {
        Vec3W::new(self, 3)
    }
    #[doc = "Bits 4:31 - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved4(&mut self) -> Reserved4W<VecflagsclrSpec> {
        Reserved4W::new(self, 4)
    }
}
#[doc = "Vector Flags Clear Strobes for clearing flags in VECFLAGS.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vecflagsclr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vecflagsclr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VecflagsclrSpec;
impl crate::RegisterSpec for VecflagsclrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vecflagsclr::R`](R) reader structure"]
impl crate::Readable for VecflagsclrSpec {}
#[doc = "`write(|w| ..)` method takes [`vecflagsclr::W`](W) writer structure"]
impl crate::Writable for VecflagsclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VECFLAGSCLR to value 0"]
impl crate::Resettable for VecflagsclrSpec {
    const RESET_VALUE: u32 = 0;
}
