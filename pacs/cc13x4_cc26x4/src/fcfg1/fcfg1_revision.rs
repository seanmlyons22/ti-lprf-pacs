#[doc = "Register `FCFG1_REVISION` reader"]
pub type R = crate::R<Fcfg1RevisionSpec>;
#[doc = "Register `FCFG1_REVISION` writer"]
pub type W = crate::W<Fcfg1RevisionSpec>;
#[doc = "Field `REV` reader - 31:0\\]
The revision number of the FCFG1 layout. This value will be read by application SW in order to determine which FCFG1 parameters that have valid values. This revision number must be incremented by 1 before any devices are to be produced if the FCFG1 layout has changed since the previous production of devices. Value migth change without warning."]
pub type RevR = crate::FieldReader<u32>;
#[doc = "Field `REV` writer - 31:0\\]
The revision number of the FCFG1 layout. This value will be read by application SW in order to determine which FCFG1 parameters that have valid values. This revision number must be incremented by 1 before any devices are to be produced if the FCFG1 layout has changed since the previous production of devices. Value migth change without warning."]
pub type RevW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
The revision number of the FCFG1 layout. This value will be read by application SW in order to determine which FCFG1 parameters that have valid values. This revision number must be incremented by 1 before any devices are to be produced if the FCFG1 layout has changed since the previous production of devices. Value migth change without warning."]
    #[inline(always)]
    pub fn rev(&self) -> RevR {
        RevR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
The revision number of the FCFG1 layout. This value will be read by application SW in order to determine which FCFG1 parameters that have valid values. This revision number must be incremented by 1 before any devices are to be produced if the FCFG1 layout has changed since the previous production of devices. Value migth change without warning."]
    #[inline(always)]
    #[must_use]
    pub fn rev(&mut self) -> RevW<Fcfg1RevisionSpec> {
        RevW::new(self, 0)
    }
}
#[doc = "Factory Configuration (FCFG1) Revision\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcfg1_revision::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcfg1_revision::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fcfg1RevisionSpec;
impl crate::RegisterSpec for Fcfg1RevisionSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fcfg1_revision::R`](R) reader structure"]
impl crate::Readable for Fcfg1RevisionSpec {}
#[doc = "`write(|w| ..)` method takes [`fcfg1_revision::W`](W) writer structure"]
impl crate::Writable for Fcfg1RevisionSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FCFG1_REVISION to value 0x2a"]
impl crate::Resettable for Fcfg1RevisionSpec {
    const RESET_VALUE: u32 = 0x2a;
}
