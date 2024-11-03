#[doc = "Register `SETCHNLPRIORITY` reader"]
pub type R = crate::R<SetchnlprioritySpec>;
#[doc = "Register `SETCHNLPRIORITY` writer"]
pub type W = crate::W<SetchnlprioritySpec>;
#[doc = "Field `CHNLS` reader - 7:0\\]
Returns the channel priority mask status, or sets the channel priority to high. Read as: Bit \\[Ch\\]
= 0: uDMA channel Ch is using the default priority level. Bit \\[Ch\\]
= 1: uDMA channel Ch is using a high priority level. Write as: Bit \\[Ch\\]
= 0: No effect. Use the CLEARCHNLPRIORITY.CHNLS to set channel Ch to the default priority level. Bit \\[Ch\\]
= 1: Channel Ch uses the high priority level. Writing to a bit where a uDMA channel is not implemented has no effect"]
pub type ChnlsR = crate::FieldReader;
#[doc = "Field `CHNLS` writer - 7:0\\]
Returns the channel priority mask status, or sets the channel priority to high. Read as: Bit \\[Ch\\]
= 0: uDMA channel Ch is using the default priority level. Bit \\[Ch\\]
= 1: uDMA channel Ch is using a high priority level. Write as: Bit \\[Ch\\]
= 0: No effect. Use the CLEARCHNLPRIORITY.CHNLS to set channel Ch to the default priority level. Bit \\[Ch\\]
= 1: Channel Ch uses the high priority level. Writing to a bit where a uDMA channel is not implemented has no effect"]
pub type ChnlsW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RESERVED8` reader - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED8` writer - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Returns the channel priority mask status, or sets the channel priority to high. Read as: Bit \\[Ch\\]
= 0: uDMA channel Ch is using the default priority level. Bit \\[Ch\\]
= 1: uDMA channel Ch is using a high priority level. Write as: Bit \\[Ch\\]
= 0: No effect. Use the CLEARCHNLPRIORITY.CHNLS to set channel Ch to the default priority level. Bit \\[Ch\\]
= 1: Channel Ch uses the high priority level. Writing to a bit where a uDMA channel is not implemented has no effect"]
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
Returns the channel priority mask status, or sets the channel priority to high. Read as: Bit \\[Ch\\]
= 0: uDMA channel Ch is using the default priority level. Bit \\[Ch\\]
= 1: uDMA channel Ch is using a high priority level. Write as: Bit \\[Ch\\]
= 0: No effect. Use the CLEARCHNLPRIORITY.CHNLS to set channel Ch to the default priority level. Bit \\[Ch\\]
= 1: Channel Ch uses the high priority level. Writing to a bit where a uDMA channel is not implemented has no effect"]
    #[inline(always)]
    #[must_use]
    pub fn chnls(&mut self) -> ChnlsW<SetchnlprioritySpec> {
        ChnlsW::new(self, 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved8(&mut self) -> Reserved8W<SetchnlprioritySpec> {
        Reserved8W::new(self, 8)
    }
}
#[doc = "Set Channel Priority Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`setchnlpriority::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`setchnlpriority::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SetchnlprioritySpec;
impl crate::RegisterSpec for SetchnlprioritySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`setchnlpriority::R`](R) reader structure"]
impl crate::Readable for SetchnlprioritySpec {}
#[doc = "`write(|w| ..)` method takes [`setchnlpriority::W`](W) writer structure"]
impl crate::Writable for SetchnlprioritySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SETCHNLPRIORITY to value 0"]
impl crate::Resettable for SetchnlprioritySpec {
    const RESET_VALUE: u32 = 0;
}
