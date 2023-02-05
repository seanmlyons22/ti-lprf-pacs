#[doc = "Register `VECFLAGS` reader"]
pub struct R(crate::R<VECFLAGS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VECFLAGS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VECFLAGS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VECFLAGS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VECFLAGS` writer"]
pub struct W(crate::W<VECFLAGS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VECFLAGS_SPEC>;
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
impl From<crate::W<VECFLAGS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VECFLAGS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VEC0` reader - 0:0\\]
Vector flag 0. The vector flag is set if the edge selected VECCFG0.VEC0_POL occurs on the event selected in VECCFG0.VEC0_EV. The flag is cleared by writing a 0 to this bit, or (preferably) a 1 to VECFLAGSCLR.VEC0."]
pub type VEC0_R = crate::BitReader<bool>;
#[doc = "Field `VEC0` writer - 0:0\\]
Vector flag 0. The vector flag is set if the edge selected VECCFG0.VEC0_POL occurs on the event selected in VECCFG0.VEC0_EV. The flag is cleared by writing a 0 to this bit, or (preferably) a 1 to VECFLAGSCLR.VEC0."]
pub type VEC0_W<'a, const O: u8> = crate::BitWriter<'a, u32, VECFLAGS_SPEC, bool, O>;
#[doc = "Field `VEC1` reader - 1:1\\]
Vector flag 1. The vector flag is set if the edge selected VECCFG0.VEC1_POL occurs on the event selected in VECCFG0.VEC1_EV. The flag is cleared by writing a 0 to this bit, or (preferably) a 1 to VECFLAGSCLR.VEC1."]
pub type VEC1_R = crate::BitReader<bool>;
#[doc = "Field `VEC1` writer - 1:1\\]
Vector flag 1. The vector flag is set if the edge selected VECCFG0.VEC1_POL occurs on the event selected in VECCFG0.VEC1_EV. The flag is cleared by writing a 0 to this bit, or (preferably) a 1 to VECFLAGSCLR.VEC1."]
pub type VEC1_W<'a, const O: u8> = crate::BitWriter<'a, u32, VECFLAGS_SPEC, bool, O>;
#[doc = "Field `VEC2` reader - 2:2\\]
Vector flag 2. The vector flag is set if the edge selected VECCFG1.VEC2_POL occurs on the event selected in VECCFG1.VEC2_EV. The flag is cleared by writing a 0 to this bit, or (preferably) a 1 to VECFLAGSCLR.VEC2."]
pub type VEC2_R = crate::BitReader<bool>;
#[doc = "Field `VEC2` writer - 2:2\\]
Vector flag 2. The vector flag is set if the edge selected VECCFG1.VEC2_POL occurs on the event selected in VECCFG1.VEC2_EV. The flag is cleared by writing a 0 to this bit, or (preferably) a 1 to VECFLAGSCLR.VEC2."]
pub type VEC2_W<'a, const O: u8> = crate::BitWriter<'a, u32, VECFLAGS_SPEC, bool, O>;
#[doc = "Field `VEC3` reader - 3:3\\]
Vector flag 3. The vector flag is set if the edge selected VECCFG1.VEC3_POL occurs on the event selected in VECCFG1.VEC3_EV. The flag is cleared by writing a 0 to this bit, or (preferably) a 1 to VECFLAGSCLR.VEC3."]
pub type VEC3_R = crate::BitReader<bool>;
#[doc = "Field `VEC3` writer - 3:3\\]
Vector flag 3. The vector flag is set if the edge selected VECCFG1.VEC3_POL occurs on the event selected in VECCFG1.VEC3_EV. The flag is cleared by writing a 0 to this bit, or (preferably) a 1 to VECFLAGSCLR.VEC3."]
pub type VEC3_W<'a, const O: u8> = crate::BitWriter<'a, u32, VECFLAGS_SPEC, bool, O>;
#[doc = "Field `RESERVED4` reader - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED4_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED4` writer - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, VECFLAGS_SPEC, u32, u32, 28, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Vector flag 0. The vector flag is set if the edge selected VECCFG0.VEC0_POL occurs on the event selected in VECCFG0.VEC0_EV. The flag is cleared by writing a 0 to this bit, or (preferably) a 1 to VECFLAGSCLR.VEC0."]
    #[inline(always)]
    pub fn vec0(&self) -> VEC0_R {
        VEC0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Vector flag 1. The vector flag is set if the edge selected VECCFG0.VEC1_POL occurs on the event selected in VECCFG0.VEC1_EV. The flag is cleared by writing a 0 to this bit, or (preferably) a 1 to VECFLAGSCLR.VEC1."]
    #[inline(always)]
    pub fn vec1(&self) -> VEC1_R {
        VEC1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Vector flag 2. The vector flag is set if the edge selected VECCFG1.VEC2_POL occurs on the event selected in VECCFG1.VEC2_EV. The flag is cleared by writing a 0 to this bit, or (preferably) a 1 to VECFLAGSCLR.VEC2."]
    #[inline(always)]
    pub fn vec2(&self) -> VEC2_R {
        VEC2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Vector flag 3. The vector flag is set if the edge selected VECCFG1.VEC3_POL occurs on the event selected in VECCFG1.VEC3_EV. The flag is cleared by writing a 0 to this bit, or (preferably) a 1 to VECFLAGSCLR.VEC3."]
    #[inline(always)]
    pub fn vec3(&self) -> VEC3_R {
        VEC3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:31 - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved4(&self) -> RESERVED4_R {
        RESERVED4_R::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Vector flag 0. The vector flag is set if the edge selected VECCFG0.VEC0_POL occurs on the event selected in VECCFG0.VEC0_EV. The flag is cleared by writing a 0 to this bit, or (preferably) a 1 to VECFLAGSCLR.VEC0."]
    #[inline(always)]
    #[must_use]
    pub fn vec0(&mut self) -> VEC0_W<0> {
        VEC0_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
Vector flag 1. The vector flag is set if the edge selected VECCFG0.VEC1_POL occurs on the event selected in VECCFG0.VEC1_EV. The flag is cleared by writing a 0 to this bit, or (preferably) a 1 to VECFLAGSCLR.VEC1."]
    #[inline(always)]
    #[must_use]
    pub fn vec1(&mut self) -> VEC1_W<1> {
        VEC1_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
Vector flag 2. The vector flag is set if the edge selected VECCFG1.VEC2_POL occurs on the event selected in VECCFG1.VEC2_EV. The flag is cleared by writing a 0 to this bit, or (preferably) a 1 to VECFLAGSCLR.VEC2."]
    #[inline(always)]
    #[must_use]
    pub fn vec2(&mut self) -> VEC2_W<2> {
        VEC2_W::new(self)
    }
    #[doc = "Bit 3 - 3:3\\]
Vector flag 3. The vector flag is set if the edge selected VECCFG1.VEC3_POL occurs on the event selected in VECCFG1.VEC3_EV. The flag is cleared by writing a 0 to this bit, or (preferably) a 1 to VECFLAGSCLR.VEC3."]
    #[inline(always)]
    #[must_use]
    pub fn vec3(&mut self) -> VEC3_W<3> {
        VEC3_W::new(self)
    }
    #[doc = "Bits 4:31 - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved4(&mut self) -> RESERVED4_W<4> {
        RESERVED4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Vector Flags If a vector flag becomes 1 and AUX_SCE sleeps, AUX_SCE will wake up and execute the corresponding vector. The vector with the lowest index will execute first if multiple vectors flags are set. AUX_SCE must return to sleep to execute the next vector. During execution of a vector, AUX_SCE must clear the vector flag that triggered execution. Write 1 to bit index n in VECFLAGSCLR to clear vector flag n.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vecflags](index.html) module"]
pub struct VECFLAGS_SPEC;
impl crate::RegisterSpec for VECFLAGS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vecflags::R](R) reader structure"]
impl crate::Readable for VECFLAGS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vecflags::W](W) writer structure"]
impl crate::Writable for VECFLAGS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets VECFLAGS to value 0"]
impl crate::Resettable for VECFLAGS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
