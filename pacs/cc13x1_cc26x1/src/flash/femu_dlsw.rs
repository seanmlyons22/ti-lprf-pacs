#[doc = "Register `FEMU_DLSW` reader"]
pub type R = crate::R<FemuDlswSpec>;
#[doc = "Register `FEMU_DLSW` writer"]
pub type W = crate::W<FemuDlswSpec>;
#[doc = "Field `FEMU_DLSW` reader - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type FemuDlswR = crate::FieldReader<u32>;
#[doc = "Field `FEMU_DLSW` writer - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type FemuDlswW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn femu_dlsw(&self) -> FemuDlswR {
        FemuDlswR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn femu_dlsw(&mut self) -> FemuDlswW<FemuDlswSpec> {
        FemuDlswW::new(self, 0)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`femu_dlsw::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`femu_dlsw::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FemuDlswSpec;
impl crate::RegisterSpec for FemuDlswSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`femu_dlsw::R`](R) reader structure"]
impl crate::Readable for FemuDlswSpec {}
#[doc = "`write(|w| ..)` method takes [`femu_dlsw::W`](W) writer structure"]
impl crate::Writable for FemuDlswSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FEMU_DLSW to value 0"]
impl crate::Resettable for FemuDlswSpec {
    const RESET_VALUE: u32 = 0;
}
