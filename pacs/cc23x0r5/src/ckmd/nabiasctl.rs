#[doc = "Register `NABIASCTL` reader"]
pub type R = crate::R<NabiasctlSpec>;
#[doc = "Register `NABIASCTL` writer"]
pub type W = crate::W<NabiasctlSpec>;
#[doc = "Field `EN` reader - 0:0\\]
Enable nanoamp-bias"]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - 0:0\\]
Enable nanoamp-bias"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED1` reader - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED1` writer - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Enable nanoamp-bias"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Enable nanoamp-bias"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EnW<NabiasctlSpec> {
        EnW::new(self, 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<NabiasctlSpec> {
        Reserved1W::new(self, 1)
    }
}
#[doc = "Nanoamp-bias control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nabiasctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nabiasctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NabiasctlSpec;
impl crate::RegisterSpec for NabiasctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nabiasctl::R`](R) reader structure"]
impl crate::Readable for NabiasctlSpec {}
#[doc = "`write(|w| ..)` method takes [`nabiasctl::W`](W) writer structure"]
impl crate::Writable for NabiasctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets NABIASCTL to value 0"]
impl crate::Resettable for NabiasctlSpec {
    const RESET_VALUE: u32 = 0;
}
