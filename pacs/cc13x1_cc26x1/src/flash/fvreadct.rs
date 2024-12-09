#[doc = "Register `FVREADCT` reader"]
pub type R = crate::R<FvreadctSpec>;
#[doc = "Register `FVREADCT` writer"]
pub type W = crate::W<FvreadctSpec>;
#[doc = "Field `VREADCT` reader - 3:0\\]
Internal. Only to be used through TI provided API."]
pub type VreadctR = crate::FieldReader;
#[doc = "Field `VREADCT` writer - 3:0\\]
Internal. Only to be used through TI provided API."]
pub type VreadctW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RESERVED4` reader - 31:4\\]
Internal. Only to be used through TI provided API."]
pub type Reserved4R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vreadct(&self) -> VreadctR {
        VreadctR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:31 - 31:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn vreadct(&mut self) -> VreadctW<FvreadctSpec> {
        VreadctW::new(self, 0)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fvreadct::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fvreadct::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FvreadctSpec;
impl crate::RegisterSpec for FvreadctSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fvreadct::R`](R) reader structure"]
impl crate::Readable for FvreadctSpec {}
#[doc = "`write(|w| ..)` method takes [`fvreadct::W`](W) writer structure"]
impl crate::Writable for FvreadctSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FVREADCT to value 0x08"]
impl crate::Resettable for FvreadctSpec {
    const RESET_VALUE: u32 = 0x08;
}
