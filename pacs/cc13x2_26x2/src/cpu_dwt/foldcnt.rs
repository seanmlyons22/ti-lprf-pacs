#[doc = "Register `FOLDCNT` reader"]
pub type R = crate::R<FoldcntSpec>;
#[doc = "Register `FOLDCNT` writer"]
pub type W = crate::W<FoldcntSpec>;
#[doc = "Field `FOLDCNT` reader - 7:0\\]
This counts the total number folded instructions. This counter initializes to 0 when it is enabled using CTRL.FOLDEVTENA."]
pub type FoldcntR = crate::FieldReader;
#[doc = "Field `FOLDCNT` writer - 7:0\\]
This counts the total number folded instructions. This counter initializes to 0 when it is enabled using CTRL.FOLDEVTENA."]
pub type FoldcntW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RESERVED8` reader - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED8` writer - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
This counts the total number folded instructions. This counter initializes to 0 when it is enabled using CTRL.FOLDEVTENA."]
    #[inline(always)]
    pub fn foldcnt(&self) -> FoldcntR {
        FoldcntR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&self) -> Reserved8R {
        Reserved8R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
This counts the total number folded instructions. This counter initializes to 0 when it is enabled using CTRL.FOLDEVTENA."]
    #[inline(always)]
    #[must_use]
    pub fn foldcnt(&mut self) -> FoldcntW<FoldcntSpec> {
        FoldcntW::new(self, 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved8(&mut self) -> Reserved8W<FoldcntSpec> {
        Reserved8W::new(self, 8)
    }
}
#[doc = "Fold Count This register is used to count the total number of folded instructions. The counter increments on each instruction which takes 0 cycles.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`foldcnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`foldcnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FoldcntSpec;
impl crate::RegisterSpec for FoldcntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`foldcnt::R`](R) reader structure"]
impl crate::Readable for FoldcntSpec {}
#[doc = "`write(|w| ..)` method takes [`foldcnt::W`](W) writer structure"]
impl crate::Writable for FoldcntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FOLDCNT to value 0"]
impl crate::Resettable for FoldcntSpec {
    const RESET_VALUE: u32 = 0;
}
