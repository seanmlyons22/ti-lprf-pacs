#[doc = "Register `ICTR` reader"]
pub type R = crate::R<IctrSpec>;
#[doc = "Register `ICTR` writer"]
pub type W = crate::W<IctrSpec>;
#[doc = "Field `INTLINESNUM` reader - 2:0\\]
Total number of interrupt lines in groups of 32. 0: 0...32 1: 33...64 2: 65...96 3: 97...128 4: 129...160 5: 161...192 6: 193...224 7: 225...256"]
pub type IntlinesnumR = crate::FieldReader;
#[doc = "Field `INTLINESNUM` writer - 2:0\\]
Total number of interrupt lines in groups of 32. 0: 0...32 1: 33...64 2: 65...96 3: 97...128 4: 129...160 5: 161...192 6: 193...224 7: 225...256"]
pub type IntlinesnumW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RESERVED3` reader - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved3R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED3` writer - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved3W<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Total number of interrupt lines in groups of 32. 0: 0...32 1: 33...64 2: 65...96 3: 97...128 4: 129...160 5: 161...192 6: 193...224 7: 225...256"]
    #[inline(always)]
    pub fn intlinesnum(&self) -> IntlinesnumR {
        IntlinesnumR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:31 - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Total number of interrupt lines in groups of 32. 0: 0...32 1: 33...64 2: 65...96 3: 97...128 4: 129...160 5: 161...192 6: 193...224 7: 225...256"]
    #[inline(always)]
    #[must_use]
    pub fn intlinesnum(&mut self) -> IntlinesnumW<IctrSpec> {
        IntlinesnumW::new(self, 0)
    }
    #[doc = "Bits 3:31 - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> Reserved3W<IctrSpec> {
        Reserved3W::new(self, 3)
    }
}
#[doc = "Interrupt Control Type Read this register to see the number of interrupt lines that the NVIC supports.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ictr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ictr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IctrSpec;
impl crate::RegisterSpec for IctrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ictr::R`](R) reader structure"]
impl crate::Readable for IctrSpec {}
#[doc = "`write(|w| ..)` method takes [`ictr::W`](W) writer structure"]
impl crate::Writable for IctrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ICTR to value 0x01"]
impl crate::Resettable for IctrSpec {
    const RESET_VALUE: u32 = 0x01;
}
