#[doc = "Register `MICR` reader"]
pub type R = crate::R<MicrSpec>;
#[doc = "Register `MICR` writer"]
pub type W = crate::W<MicrSpec>;
#[doc = "Field `IC` writer - 0:0\\]
Interrupt clear Writing 1 to this bit clears MRIS.RIS and MMIS.MIS . Reading this register returns no meaningful data."]
pub type IcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED1` writer - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl W {
    #[doc = "Bit 0 - 0:0\\]
Interrupt clear Writing 1 to this bit clears MRIS.RIS and MMIS.MIS . Reading this register returns no meaningful data."]
    #[inline(always)]
    #[must_use]
    pub fn ic(&mut self) -> IcW<MicrSpec> {
        IcW::new(self, 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<MicrSpec> {
        Reserved1W::new(self, 1)
    }
}
#[doc = "Master Interrupt Clear This register clears the raw and masked interrupt.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`micr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`micr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MicrSpec;
impl crate::RegisterSpec for MicrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`micr::R`](R) reader structure"]
impl crate::Readable for MicrSpec {}
#[doc = "`write(|w| ..)` method takes [`micr::W`](W) writer structure"]
impl crate::Writable for MicrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MICR to value 0"]
impl crate::Resettable for MicrSpec {
    const RESET_VALUE: u32 = 0;
}
