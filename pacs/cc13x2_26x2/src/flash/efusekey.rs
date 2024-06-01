#[doc = "Register `EFUSEKEY` reader"]
pub type R = crate::R<EfusekeySpec>;
#[doc = "Register `EFUSEKEY` writer"]
pub type W = crate::W<EfusekeySpec>;
#[doc = "Field `CODE` reader - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type CodeR = crate::FieldReader<u32>;
#[doc = "Field `CODE` writer - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type CodeW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn code(&self) -> CodeR {
        CodeR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn code(&mut self) -> CodeW<EfusekeySpec> {
        CodeW::new(self, 0)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`efusekey::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`efusekey::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EfusekeySpec;
impl crate::RegisterSpec for EfusekeySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`efusekey::R`](R) reader structure"]
impl crate::Readable for EfusekeySpec {}
#[doc = "`write(|w| ..)` method takes [`efusekey::W`](W) writer structure"]
impl crate::Writable for EfusekeySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EFUSEKEY to value 0"]
impl crate::Resettable for EfusekeySpec {
    const RESET_VALUE: u32 = 0;
}
