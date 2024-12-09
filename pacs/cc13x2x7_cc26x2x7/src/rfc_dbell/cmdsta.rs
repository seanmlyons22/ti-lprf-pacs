#[doc = "Register `CMDSTA` reader"]
pub type R = crate::R<CmdstaSpec>;
#[doc = "Register `CMDSTA` writer"]
pub type W = crate::W<CmdstaSpec>;
#[doc = "Field `STAT` reader - 31:0\\]
Status of the last command used"]
pub type StatR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Status of the last command used"]
    #[inline(always)]
    pub fn stat(&self) -> StatR {
        StatR::new(self.bits)
    }
}
impl W {}
#[doc = "Doorbell Command Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmdsta::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmdsta::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmdstaSpec;
impl crate::RegisterSpec for CmdstaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmdsta::R`](R) reader structure"]
impl crate::Readable for CmdstaSpec {}
#[doc = "`write(|w| ..)` method takes [`cmdsta::W`](W) writer structure"]
impl crate::Writable for CmdstaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMDSTA to value 0"]
impl crate::Resettable for CmdstaSpec {
    const RESET_VALUE: u32 = 0;
}
