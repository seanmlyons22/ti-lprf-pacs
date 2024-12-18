#[doc = "Register `IOSTRMED` reader"]
pub type R = crate::R<IostrmedSpec>;
#[doc = "Register `IOSTRMED` writer"]
pub type W = crate::W<IostrmedSpec>;
#[doc = "Field `GRAY_CODE` reader - 2:0\\]
Internal. Only to be used through TI provided API."]
pub type GrayCodeR = crate::FieldReader;
#[doc = "Field `GRAY_CODE` writer - 2:0\\]
Internal. Only to be used through TI provided API."]
pub type GrayCodeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RESERVED3` reader - 31:3\\]
Internal. Only to be used through TI provided API."]
pub type Reserved3R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn gray_code(&self) -> GrayCodeR {
        GrayCodeR::new((self.bits & 7) as u8)
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
    pub fn gray_code(&mut self) -> GrayCodeW<IostrmedSpec> {
        GrayCodeW::new(self, 0)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iostrmed::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iostrmed::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IostrmedSpec;
impl crate::RegisterSpec for IostrmedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iostrmed::R`](R) reader structure"]
impl crate::Readable for IostrmedSpec {}
#[doc = "`write(|w| ..)` method takes [`iostrmed::W`](W) writer structure"]
impl crate::Writable for IostrmedSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IOSTRMED to value 0x06"]
impl crate::Resettable for IostrmedSpec {
    const RESET_VALUE: u32 = 0x06;
}
