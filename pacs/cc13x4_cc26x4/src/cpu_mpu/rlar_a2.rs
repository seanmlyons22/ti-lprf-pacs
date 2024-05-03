#[doc = "Register `RLAR_A2` reader"]
pub type R = crate::R<RlarA2Spec>;
#[doc = "Register `RLAR_A2` writer"]
pub type W = crate::W<RlarA2Spec>;
#[doc = "Field `EN` reader - 0:0\\]
Region enable"]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - 0:0\\]
Region enable"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ATTRINDX` reader - 3:1\\]
Associates a set of attributes in the MPU_MAIR0 and MPU_MAIR1 fields"]
pub type AttrindxR = crate::FieldReader;
#[doc = "Field `ATTRINDX` writer - 3:1\\]
Associates a set of attributes in the MPU_MAIR0 and MPU_MAIR1 fields"]
pub type AttrindxW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RESERVED4` reader - 4:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `RESERVED4` writer - 4:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LIMIT` reader - 31:5\\]
Contains bits \\[31:5\\]
of the upper inclusive limit of the selected MPU memory region. This value is postfixed with 0x1F to provide the limit address to be checked against"]
pub type LimitR = crate::FieldReader<u32>;
#[doc = "Field `LIMIT` writer - 31:5\\]
Contains bits \\[31:5\\]
of the upper inclusive limit of the selected MPU memory region. This value is postfixed with 0x1F to provide the limit address to be checked against"]
pub type LimitW<'a, REG> = crate::FieldWriter<'a, REG, 27, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Region enable"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - 3:1\\]
Associates a set of attributes in the MPU_MAIR0 and MPU_MAIR1 fields"]
    #[inline(always)]
    pub fn attrindx(&self) -> AttrindxR {
        AttrindxR::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bit 4 - 4:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:31 - 31:5\\]
Contains bits \\[31:5\\]
of the upper inclusive limit of the selected MPU memory region. This value is postfixed with 0x1F to provide the limit address to be checked against"]
    #[inline(always)]
    pub fn limit(&self) -> LimitR {
        LimitR::new((self.bits >> 5) & 0x07ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Region enable"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EnW<RlarA2Spec> {
        EnW::new(self, 0)
    }
    #[doc = "Bits 1:3 - 3:1\\]
Associates a set of attributes in the MPU_MAIR0 and MPU_MAIR1 fields"]
    #[inline(always)]
    #[must_use]
    pub fn attrindx(&mut self) -> AttrindxW<RlarA2Spec> {
        AttrindxW::new(self, 1)
    }
    #[doc = "Bit 4 - 4:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved4(&mut self) -> Reserved4W<RlarA2Spec> {
        Reserved4W::new(self, 4)
    }
    #[doc = "Bits 5:31 - 31:5\\]
Contains bits \\[31:5\\]
of the upper inclusive limit of the selected MPU memory region. This value is postfixed with 0x1F to provide the limit address to be checked against"]
    #[inline(always)]
    #[must_use]
    pub fn limit(&mut self) -> LimitW<RlarA2Spec> {
        LimitW::new(self, 5)
    }
}
#[doc = "Provides indirect read and write access to the limit address of the currently selected MPU region selected by MPU_RNR\\[7:2\\]:(2\\[1:0\\])\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rlar_a2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rlar_a2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RlarA2Spec;
impl crate::RegisterSpec for RlarA2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rlar_a2::R`](R) reader structure"]
impl crate::Readable for RlarA2Spec {}
#[doc = "`write(|w| ..)` method takes [`rlar_a2::W`](W) writer structure"]
impl crate::Writable for RlarA2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RLAR_A2 to value 0"]
impl crate::Resettable for RlarA2Spec {
    const RESET_VALUE: u32 = 0;
}
