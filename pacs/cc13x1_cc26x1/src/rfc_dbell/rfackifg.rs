#[doc = "Register `RFACKIFG` reader"]
pub type R = crate::R<RfackifgSpec>;
#[doc = "Register `RFACKIFG` writer"]
pub type W = crate::W<RfackifgSpec>;
#[doc = "Field `ACKFLAG` reader - 0:0\\]
Interrupt flag for Command ACK"]
pub type AckflagR = crate::BitReader;
#[doc = "Field `ACKFLAG` writer - 0:0\\]
Interrupt flag for Command ACK"]
pub type AckflagW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED1` reader - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Interrupt flag for Command ACK"]
    #[inline(always)]
    pub fn ackflag(&self) -> AckflagR {
        AckflagR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Interrupt flag for Command ACK"]
    #[inline(always)]
    #[must_use]
    pub fn ackflag(&mut self) -> AckflagW<RfackifgSpec> {
        AckflagW::new(self, 0)
    }
}
#[doc = "Doorbell Command Acknowledgement Interrupt Flag\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfackifg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rfackifg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RfackifgSpec;
impl crate::RegisterSpec for RfackifgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rfackifg::R`](R) reader structure"]
impl crate::Readable for RfackifgSpec {}
#[doc = "`write(|w| ..)` method takes [`rfackifg::W`](W) writer structure"]
impl crate::Writable for RfackifgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RFACKIFG to value 0"]
impl crate::Resettable for RfackifgSpec {
    const RESET_VALUE: u32 = 0;
}
