#[doc = "Register `FWLOCK` reader"]
pub type R = crate::R<FwlockSpec>;
#[doc = "Register `FWLOCK` writer"]
pub type W = crate::W<FwlockSpec>;
#[doc = "Field `FWLOCK` reader - 2:0\\]
Internal. Only to be used through TI provided API."]
pub type FwlockR = crate::FieldReader;
#[doc = "Field `FWLOCK` writer - 2:0\\]
Internal. Only to be used through TI provided API."]
pub type FwlockW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RESERVED3` reader - 31:3\\]
Internal. Only to be used through TI provided API."]
pub type Reserved3R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn fwlock(&self) -> FwlockR {
        FwlockR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:31 - 31:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn fwlock(&mut self) -> FwlockW<FwlockSpec> {
        FwlockW::new(self, 0)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fwlock::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fwlock::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FwlockSpec;
impl crate::RegisterSpec for FwlockSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fwlock::R`](R) reader structure"]
impl crate::Readable for FwlockSpec {}
#[doc = "`write(|w| ..)` method takes [`fwlock::W`](W) writer structure"]
impl crate::Writable for FwlockSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FWLOCK to value 0"]
impl crate::Resettable for FwlockSpec {
    const RESET_VALUE: u32 = 0;
}
