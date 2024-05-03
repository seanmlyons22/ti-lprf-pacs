#[doc = "Register `CMDR` reader"]
pub type R = crate::R<CmdrSpec>;
#[doc = "Register `CMDR` writer"]
pub type W = crate::W<CmdrSpec>;
#[doc = "Field `CMD` reader - 31:0\\]
Command register. Raises an interrupt to the Command and packet engine (CPE) upon write."]
pub type CmdR = crate::FieldReader<u32>;
#[doc = "Field `CMD` writer - 31:0\\]
Command register. Raises an interrupt to the Command and packet engine (CPE) upon write."]
pub type CmdW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Command register. Raises an interrupt to the Command and packet engine (CPE) upon write."]
    #[inline(always)]
    pub fn cmd(&self) -> CmdR {
        CmdR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Command register. Raises an interrupt to the Command and packet engine (CPE) upon write."]
    #[inline(always)]
    #[must_use]
    pub fn cmd(&mut self) -> CmdW<CmdrSpec> {
        CmdW::new(self, 0)
    }
}
#[doc = "Doorbell Command Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmdr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmdr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmdrSpec;
impl crate::RegisterSpec for CmdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmdr::R`](R) reader structure"]
impl crate::Readable for CmdrSpec {}
#[doc = "`write(|w| ..)` method takes [`cmdr::W`](W) writer structure"]
impl crate::Writable for CmdrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMDR to value 0"]
impl crate::Resettable for CmdrSpec {
    const RESET_VALUE: u32 = 0;
}
