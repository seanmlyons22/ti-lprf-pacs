#[doc = "Register `COMPARE` reader"]
pub type R = crate::R<CompareSpec>;
#[doc = "Register `COMPARE` writer"]
pub type W = crate::W<CompareSpec>;
#[doc = "Field `A_EQUALS_B` reader - 0:0\\]
Vector_A is equal to Vector_B"]
pub type AEqualsBR = crate::BitReader;
#[doc = "Field `A_EQUALS_B` writer - 0:0\\]
Vector_A is equal to Vector_B"]
pub type AEqualsBW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `A_LESS_THAN_B` reader - 1:1\\]
Vector_A is less than Vector_B"]
pub type ALessThanBR = crate::BitReader;
#[doc = "Field `A_LESS_THAN_B` writer - 1:1\\]
Vector_A is less than Vector_B"]
pub type ALessThanBW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `A_GREATER_THAN_B` reader - 2:2\\]
Vector_A is greater than Vector_B"]
pub type AGreaterThanBR = crate::BitReader;
#[doc = "Field `A_GREATER_THAN_B` writer - 2:2\\]
Vector_A is greater than Vector_B"]
pub type AGreaterThanBW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED3` reader - 31:3\\]
Ignore on read"]
pub type Reserved3R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED3` writer - 31:3\\]
Ignore on read"]
pub type Reserved3W<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Vector_A is equal to Vector_B"]
    #[inline(always)]
    pub fn a_equals_b(&self) -> AEqualsBR {
        AEqualsBR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Vector_A is less than Vector_B"]
    #[inline(always)]
    pub fn a_less_than_b(&self) -> ALessThanBR {
        ALessThanBR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Vector_A is greater than Vector_B"]
    #[inline(always)]
    pub fn a_greater_than_b(&self) -> AGreaterThanBR {
        AGreaterThanBR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:31 - 31:3\\]
Ignore on read"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Vector_A is equal to Vector_B"]
    #[inline(always)]
    #[must_use]
    pub fn a_equals_b(&mut self) -> AEqualsBW<CompareSpec> {
        AEqualsBW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Vector_A is less than Vector_B"]
    #[inline(always)]
    #[must_use]
    pub fn a_less_than_b(&mut self) -> ALessThanBW<CompareSpec> {
        ALessThanBW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Vector_A is greater than Vector_B"]
    #[inline(always)]
    #[must_use]
    pub fn a_greater_than_b(&mut self) -> AGreaterThanBW<CompareSpec> {
        AGreaterThanBW::new(self, 2)
    }
    #[doc = "Bits 3:31 - 31:3\\]
Ignore on read"]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> Reserved3W<CompareSpec> {
        Reserved3W::new(self, 3)
    }
}
#[doc = "PKA compare result This register provides the result of a basic PKCP compare operation. It is updated when the FUNCTION.RUN bit is reset at the end of that operation. Status after a complex sequencer operation is unknown\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`compare::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`compare::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CompareSpec;
impl crate::RegisterSpec for CompareSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`compare::R`](R) reader structure"]
impl crate::Readable for CompareSpec {}
#[doc = "`write(|w| ..)` method takes [`compare::W`](W) writer structure"]
impl crate::Writable for CompareSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets COMPARE to value 0x01"]
impl crate::Resettable for CompareSpec {
    const RESET_VALUE: u32 = 0x01;
}
