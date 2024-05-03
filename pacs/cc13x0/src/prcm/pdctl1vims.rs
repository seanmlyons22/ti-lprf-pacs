#[doc = "Register `PDCTL1VIMS` reader"]
pub type R = crate::R<Pdctl1vimsSpec>;
#[doc = "Register `PDCTL1VIMS` writer"]
pub type W = crate::W<Pdctl1vimsSpec>;
#[doc = "Field `ON` reader - 0:0\\]
This is an alias for PDCTL1.VIMS_MODE"]
pub type OnR = crate::BitReader;
#[doc = "Field `ON` writer - 0:0\\]
This is an alias for PDCTL1.VIMS_MODE"]
pub type OnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED1` reader - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED1` writer - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
This is an alias for PDCTL1.VIMS_MODE"]
    #[inline(always)]
    pub fn on(&self) -> OnR {
        OnR::new((self.bits & 1) != 0)
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
This is an alias for PDCTL1.VIMS_MODE"]
    #[inline(always)]
    #[must_use]
    pub fn on(&mut self) -> OnW<Pdctl1vimsSpec> {
        OnW::new(self, 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<Pdctl1vimsSpec> {
        Reserved1W::new(self, 1)
    }
}
#[doc = "VIMS Mode Direct Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdctl1vims::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pdctl1vims::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pdctl1vimsSpec;
impl crate::RegisterSpec for Pdctl1vimsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pdctl1vims::R`](R) reader structure"]
impl crate::Readable for Pdctl1vimsSpec {}
#[doc = "`write(|w| ..)` method takes [`pdctl1vims::W`](W) writer structure"]
impl crate::Writable for Pdctl1vimsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PDCTL1VIMS to value 0x01"]
impl crate::Resettable for Pdctl1vimsSpec {
    const RESET_VALUE: u32 = 0x01;
}
