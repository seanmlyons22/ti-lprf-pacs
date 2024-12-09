#[doc = "Register `REQDONE` reader"]
pub type R = crate::R<ReqdoneSpec>;
#[doc = "Register `REQDONE` writer"]
pub type W = crate::W<ReqdoneSpec>;
#[doc = "Field `CHNLS` reader - 7:0\\]
Reflects the uDMA done status for the given channel, channel \\[Ch\\]. It's a sticky done bit. Unless cleared by writing a 1, it holds the value of 1. Read as: Bit \\[Ch\\]
= 0: Request has not completed for channel Ch Bit \\[Ch\\]
= 1: Request has completed for the channel Ch Writing a 1 to individual bits would clear the corresponding bit. Write as: Bit \\[Ch\\]
= 0: No effect. Bit \\[Ch\\]
= 1: The corresponding \\[Ch\\]
bit is cleared and is set to 0"]
pub type ChnlsR = crate::FieldReader;
#[doc = "Field `CHNLS` writer - 7:0\\]
Reflects the uDMA done status for the given channel, channel \\[Ch\\]. It's a sticky done bit. Unless cleared by writing a 1, it holds the value of 1. Read as: Bit \\[Ch\\]
= 0: Request has not completed for channel Ch Bit \\[Ch\\]
= 1: Request has completed for the channel Ch Writing a 1 to individual bits would clear the corresponding bit. Write as: Bit \\[Ch\\]
= 0: No effect. Bit \\[Ch\\]
= 1: The corresponding \\[Ch\\]
bit is cleared and is set to 0"]
pub type ChnlsW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RESERVED8` reader - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Reflects the uDMA done status for the given channel, channel \\[Ch\\]. It's a sticky done bit. Unless cleared by writing a 1, it holds the value of 1. Read as: Bit \\[Ch\\]
= 0: Request has not completed for channel Ch Bit \\[Ch\\]
= 1: Request has completed for the channel Ch Writing a 1 to individual bits would clear the corresponding bit. Write as: Bit \\[Ch\\]
= 0: No effect. Bit \\[Ch\\]
= 1: The corresponding \\[Ch\\]
bit is cleared and is set to 0"]
    #[inline(always)]
    pub fn chnls(&self) -> ChnlsR {
        ChnlsR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&self) -> Reserved8R {
        Reserved8R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Reflects the uDMA done status for the given channel, channel \\[Ch\\]. It's a sticky done bit. Unless cleared by writing a 1, it holds the value of 1. Read as: Bit \\[Ch\\]
= 0: Request has not completed for channel Ch Bit \\[Ch\\]
= 1: Request has completed for the channel Ch Writing a 1 to individual bits would clear the corresponding bit. Write as: Bit \\[Ch\\]
= 0: No effect. Bit \\[Ch\\]
= 1: The corresponding \\[Ch\\]
bit is cleared and is set to 0"]
    #[inline(always)]
    #[must_use]
    pub fn chnls(&mut self) -> ChnlsW<ReqdoneSpec> {
        ChnlsW::new(self, 0)
    }
}
#[doc = "Channel Request Done Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reqdone::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reqdone::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ReqdoneSpec;
impl crate::RegisterSpec for ReqdoneSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reqdone::R`](R) reader structure"]
impl crate::Readable for ReqdoneSpec {}
#[doc = "`write(|w| ..)` method takes [`reqdone::W`](W) writer structure"]
impl crate::Writable for ReqdoneSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REQDONE to value 0"]
impl crate::Resettable for ReqdoneSpec {
    const RESET_VALUE: u32 = 0;
}
