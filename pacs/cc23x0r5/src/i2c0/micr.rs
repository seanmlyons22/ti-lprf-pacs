#[doc = "Register `MICR` reader"]
pub type R = crate::R<MicrSpec>;
#[doc = "Register `MICR` writer"]
pub type W = crate::W<MicrSpec>;
#[doc = "Field `IC` writer - 0:0\\]
Interrupt clear 0 - Writing 0 has no effect 1 - Clear Interrupt Writing 1 to this bit clears MRIS.RIS and MMIS.MIS."]
pub type IcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED1` reader - 31:1\\]
Reads to this field return zero.Writes to this field are ignored."]
pub type Reserved1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 1:31 - 31:1\\]
Reads to this field return zero.Writes to this field are ignored."]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Interrupt clear 0 - Writing 0 has no effect 1 - Clear Interrupt Writing 1 to this bit clears MRIS.RIS and MMIS.MIS."]
    #[inline(always)]
    #[must_use]
    pub fn ic(&mut self) -> IcW<MicrSpec> {
        IcW::new(self, 0)
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
