#[doc = "Register `INT_ATVALID` reader"]
pub type R = crate::R<IntAtvalidSpec>;
#[doc = "Register `INT_ATVALID` writer"]
pub type W = crate::W<IntAtvalidSpec>;
#[doc = "Field `ATREADY` reader - 0:0\\]
A write to this bit gives the value of ATVALID"]
pub type AtreadyR = crate::BitReader;
#[doc = "Field `ATREADY` writer - 0:0\\]
A write to this bit gives the value of ATVALID"]
pub type AtreadyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AFREADY` reader - 1:1\\]
A write to this bit gives the value of AFREADY"]
pub type AfreadyR = crate::BitReader;
#[doc = "Field `AFREADY` writer - 1:1\\]
A write to this bit gives the value of AFREADY"]
pub type AfreadyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED2` reader - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED2` writer - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
A write to this bit gives the value of ATVALID"]
    #[inline(always)]
    pub fn atready(&self) -> AtreadyR {
        AtreadyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
A write to this bit gives the value of AFREADY"]
    #[inline(always)]
    pub fn afready(&self) -> AfreadyR {
        AfreadyR::new(((self.bits >> 1) & 1) != 0)
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
A write to this bit gives the value of ATVALID"]
    #[inline(always)]
    #[must_use]
    pub fn atready(&mut self) -> AtreadyW<IntAtvalidSpec> {
        AtreadyW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
A write to this bit gives the value of AFREADY"]
    #[inline(always)]
    #[must_use]
    pub fn afready(&mut self) -> AfreadyW<IntAtvalidSpec> {
        AfreadyW::new(self, 1)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> Reserved2W<IntAtvalidSpec> {
        Reserved2W::new(self, 2)
    }
}
#[doc = "Integration Mode: Write ATB Valid\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_atvalid::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_atvalid::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntAtvalidSpec;
impl crate::RegisterSpec for IntAtvalidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_atvalid::R`](R) reader structure"]
impl crate::Readable for IntAtvalidSpec {}
#[doc = "`write(|w| ..)` method takes [`int_atvalid::W`](W) writer structure"]
impl crate::Writable for IntAtvalidSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_ATVALID to value 0"]
impl crate::Resettable for IntAtvalidSpec {
    const RESET_VALUE: u32 = 0;
}
