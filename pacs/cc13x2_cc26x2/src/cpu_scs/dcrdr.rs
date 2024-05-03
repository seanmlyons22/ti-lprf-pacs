#[doc = "Register `DCRDR` reader"]
pub type R = crate::R<DcrdrSpec>;
#[doc = "Register `DCRDR` writer"]
pub type W = crate::W<DcrdrSpec>;
#[doc = "Field `DCRDR` reader - 31:0\\]
This register holds data for reading and writing registers to and from the processor. This is the data value written to the register selected by DCRSR. When the processor receives a request from DCRSR, this register is read or written by the processor using a normal load-store unit operation. If core register transfers are not being performed, software-based debug monitors can use this register for communication in non-halting debug. This enables flags and bits to acknowledge state and indicate if commands have been accepted to, replied to, or accepted and replied to."]
pub type DcrdrR = crate::FieldReader<u32>;
#[doc = "Field `DCRDR` writer - 31:0\\]
This register holds data for reading and writing registers to and from the processor. This is the data value written to the register selected by DCRSR. When the processor receives a request from DCRSR, this register is read or written by the processor using a normal load-store unit operation. If core register transfers are not being performed, software-based debug monitors can use this register for communication in non-halting debug. This enables flags and bits to acknowledge state and indicate if commands have been accepted to, replied to, or accepted and replied to."]
pub type DcrdrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
This register holds data for reading and writing registers to and from the processor. This is the data value written to the register selected by DCRSR. When the processor receives a request from DCRSR, this register is read or written by the processor using a normal load-store unit operation. If core register transfers are not being performed, software-based debug monitors can use this register for communication in non-halting debug. This enables flags and bits to acknowledge state and indicate if commands have been accepted to, replied to, or accepted and replied to."]
    #[inline(always)]
    pub fn dcrdr(&self) -> DcrdrR {
        DcrdrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
This register holds data for reading and writing registers to and from the processor. This is the data value written to the register selected by DCRSR. When the processor receives a request from DCRSR, this register is read or written by the processor using a normal load-store unit operation. If core register transfers are not being performed, software-based debug monitors can use this register for communication in non-halting debug. This enables flags and bits to acknowledge state and indicate if commands have been accepted to, replied to, or accepted and replied to."]
    #[inline(always)]
    #[must_use]
    pub fn dcrdr(&mut self) -> DcrdrW<DcrdrSpec> {
        DcrdrW::new(self, 0)
    }
}
#[doc = "Debug Core Register Data\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcrdr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcrdr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcrdrSpec;
impl crate::RegisterSpec for DcrdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcrdr::R`](R) reader structure"]
impl crate::Readable for DcrdrSpec {}
#[doc = "`write(|w| ..)` method takes [`dcrdr::W`](W) writer structure"]
impl crate::Writable for DcrdrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCRDR to value 0"]
impl crate::Resettable for DcrdrSpec {
    const RESET_VALUE: u32 = 0;
}
