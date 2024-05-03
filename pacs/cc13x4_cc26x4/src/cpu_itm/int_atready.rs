#[doc = "Register `INT_ATREADY` reader"]
pub type R = crate::R<IntAtreadySpec>;
#[doc = "Register `INT_ATREADY` writer"]
pub type W = crate::W<IntAtreadySpec>;
#[doc = "Field `ATREADY` reader - 0:0\\]
A read of this bit returns the value of ATREADY"]
pub type AtreadyR = crate::BitReader;
#[doc = "Field `ATREADY` writer - 0:0\\]
A read of this bit returns the value of ATREADY"]
pub type AtreadyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AFVALID` reader - 1:1\\]
A read of this bit returns the value of AFVALID"]
pub type AfvalidR = crate::BitReader;
#[doc = "Field `AFVALID` writer - 1:1\\]
A read of this bit returns the value of AFVALID"]
pub type AfvalidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED2` reader - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED2` writer - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
A read of this bit returns the value of ATREADY"]
    #[inline(always)]
    pub fn atready(&self) -> AtreadyR {
        AtreadyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
A read of this bit returns the value of AFVALID"]
    #[inline(always)]
    pub fn afvalid(&self) -> AfvalidR {
        AfvalidR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
A read of this bit returns the value of ATREADY"]
    #[inline(always)]
    #[must_use]
    pub fn atready(&mut self) -> AtreadyW<IntAtreadySpec> {
        AtreadyW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
A read of this bit returns the value of AFVALID"]
    #[inline(always)]
    #[must_use]
    pub fn afvalid(&mut self) -> AfvalidW<IntAtreadySpec> {
        AfvalidW::new(self, 1)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> Reserved2W<IntAtreadySpec> {
        Reserved2W::new(self, 2)
    }
}
#[doc = "Integration Mode: Read ATB Ready\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_atready::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_atready::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntAtreadySpec;
impl crate::RegisterSpec for IntAtreadySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_atready::R`](R) reader structure"]
impl crate::Readable for IntAtreadySpec {}
#[doc = "`write(|w| ..)` method takes [`int_atready::W`](W) writer structure"]
impl crate::Writable for IntAtreadySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_ATREADY to value 0"]
impl crate::Resettable for IntAtreadySpec {
    const RESET_VALUE: u32 = 0;
}
