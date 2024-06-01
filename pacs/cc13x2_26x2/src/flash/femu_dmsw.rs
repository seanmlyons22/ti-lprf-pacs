#[doc = "Register `FEMU_DMSW` reader"]
pub type R = crate::R<FemuDmswSpec>;
#[doc = "Register `FEMU_DMSW` writer"]
pub type W = crate::W<FemuDmswSpec>;
#[doc = "Field `FEMU_DMSW` reader - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type FemuDmswR = crate::FieldReader<u32>;
#[doc = "Field `FEMU_DMSW` writer - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type FemuDmswW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn femu_dmsw(&self) -> FemuDmswR {
        FemuDmswR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn femu_dmsw(&mut self) -> FemuDmswW<FemuDmswSpec> {
        FemuDmswW::new(self, 0)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`femu_dmsw::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`femu_dmsw::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FemuDmswSpec;
impl crate::RegisterSpec for FemuDmswSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`femu_dmsw::R`](R) reader structure"]
impl crate::Readable for FemuDmswSpec {}
#[doc = "`write(|w| ..)` method takes [`femu_dmsw::W`](W) writer structure"]
impl crate::Writable for FemuDmswSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FEMU_DMSW to value 0"]
impl crate::Resettable for FemuDmswSpec {
    const RESET_VALUE: u32 = 0;
}
