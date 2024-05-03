#[doc = "Register `CPUID` reader"]
pub type R = crate::R<CpuidSpec>;
#[doc = "Register `CPUID` writer"]
pub type W = crate::W<CpuidSpec>;
#[doc = "Field `REVISION` reader - 3:0\\]
Implementation defined revision number."]
pub type RevisionR = crate::FieldReader;
#[doc = "Field `REVISION` writer - 3:0\\]
Implementation defined revision number."]
pub type RevisionW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PARTNO` reader - 15:4\\]
Number of processor within family."]
pub type PartnoR = crate::FieldReader<u16>;
#[doc = "Field `PARTNO` writer - 15:4\\]
Number of processor within family."]
pub type PartnoW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `CONSTANT` reader - 19:16\\]
Reads as 0xF"]
pub type ConstantR = crate::FieldReader;
#[doc = "Field `CONSTANT` writer - 19:16\\]
Reads as 0xF"]
pub type ConstantW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `VARIANT` reader - 23:20\\]
Implementation defined variant number."]
pub type VariantR = crate::FieldReader;
#[doc = "Field `VARIANT` writer - 23:20\\]
Implementation defined variant number."]
pub type VariantW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `IMPLEMENTER` reader - 31:24\\]
Implementor code."]
pub type ImplementerR = crate::FieldReader;
#[doc = "Field `IMPLEMENTER` writer - 31:24\\]
Implementor code."]
pub type ImplementerW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Implementation defined revision number."]
    #[inline(always)]
    pub fn revision(&self) -> RevisionR {
        RevisionR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:15 - 15:4\\]
Number of processor within family."]
    #[inline(always)]
    pub fn partno(&self) -> PartnoR {
        PartnoR::new(((self.bits >> 4) & 0x0fff) as u16)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Reads as 0xF"]
    #[inline(always)]
    pub fn constant(&self) -> ConstantR {
        ConstantR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Implementation defined variant number."]
    #[inline(always)]
    pub fn variant(&self) -> VariantR {
        VariantR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Implementor code."]
    #[inline(always)]
    pub fn implementer(&self) -> ImplementerR {
        ImplementerR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Implementation defined revision number."]
    #[inline(always)]
    #[must_use]
    pub fn revision(&mut self) -> RevisionW<CpuidSpec> {
        RevisionW::new(self, 0)
    }
    #[doc = "Bits 4:15 - 15:4\\]
Number of processor within family."]
    #[inline(always)]
    #[must_use]
    pub fn partno(&mut self) -> PartnoW<CpuidSpec> {
        PartnoW::new(self, 4)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Reads as 0xF"]
    #[inline(always)]
    #[must_use]
    pub fn constant(&mut self) -> ConstantW<CpuidSpec> {
        ConstantW::new(self, 16)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Implementation defined variant number."]
    #[inline(always)]
    #[must_use]
    pub fn variant(&mut self) -> VariantW<CpuidSpec> {
        VariantW::new(self, 20)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Implementor code."]
    #[inline(always)]
    #[must_use]
    pub fn implementer(&mut self) -> ImplementerW<CpuidSpec> {
        ImplementerW::new(self, 24)
    }
}
#[doc = "CPUID Base This register determines the ID number of the processor core, the version number of the processor core and the implementation details of the processor core.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpuid::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpuid::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpuidSpec;
impl crate::RegisterSpec for CpuidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpuid::R`](R) reader structure"]
impl crate::Readable for CpuidSpec {}
#[doc = "`write(|w| ..)` method takes [`cpuid::W`](W) writer structure"]
impl crate::Writable for CpuidSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPUID to value 0x412f_c231"]
impl crate::Resettable for CpuidSpec {
    const RESET_VALUE: u32 = 0x412f_c231;
}
