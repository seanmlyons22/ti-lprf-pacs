#[doc = "Register `EntryEnd` reader"]
pub type R = crate::R<EntryEndSpec>;
#[doc = "Register `EntryEnd` writer"]
pub type W = crate::W<EntryEndSpec>;
#[doc = "Field `END` reader - 31:0\\]
Blank ROM Table entry indicating the end of the ROM Table content."]
pub type EndR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Blank ROM Table entry indicating the end of the ROM Table content."]
    #[inline(always)]
    pub fn end(&self) -> EndR {
        EndR::new(self.bits)
    }
}
impl W {}
#[doc = "Marks of end of table.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`entry_end::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`entry_end::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EntryEndSpec;
impl crate::RegisterSpec for EntryEndSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`entry_end::R`](R) reader structure"]
impl crate::Readable for EntryEndSpec {}
#[doc = "`write(|w| ..)` method takes [`entry_end::W`](W) writer structure"]
impl crate::Writable for EntryEndSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EntryEnd to value 0"]
impl crate::Resettable for EntryEndSpec {
    const RESET_VALUE: u32 = 0;
}
