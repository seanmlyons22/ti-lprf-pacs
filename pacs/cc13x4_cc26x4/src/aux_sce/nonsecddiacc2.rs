#[doc = "Register `NONSECDDIACC2` reader"]
pub type R = crate::R<Nonsecddiacc2Spec>;
#[doc = "Register `NONSECDDIACC2` writer"]
pub type W = crate::W<Nonsecddiacc2Spec>;
#[doc = "Field `WR_MASK` reader - 15:0\\]
Mask AUX_SCE is allowed to update bits in half-word given by ADDR according to this bit mask."]
pub type WrMaskR = crate::FieldReader<u16>;
#[doc = "Field `WR_MASK` writer - 15:0\\]
Mask AUX_SCE is allowed to update bits in half-word given by ADDR according to this bit mask."]
pub type WrMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `ADDR` reader - 21:16\\]
Address AUX_SCE is allowed to update this DDI half-word using SET or CLR access."]
pub type AddrR = crate::FieldReader;
#[doc = "Field `ADDR` writer - 21:16\\]
Address AUX_SCE is allowed to update this DDI half-word using SET or CLR access."]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `RD_EN` reader - 22:22\\]
Read Enable 0: AUX_SCE is not allowed to read DDI half-word given by ADDR. 1: AUX_SCE is allowed to read DDI half-word given by ADDR."]
pub type RdEnR = crate::BitReader;
#[doc = "Field `RD_EN` writer - 22:22\\]
Read Enable 0: AUX_SCE is not allowed to read DDI half-word given by ADDR. 1: AUX_SCE is allowed to read DDI half-word given by ADDR."]
pub type RdEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED23` reader - 31:23\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved23R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Mask AUX_SCE is allowed to update bits in half-word given by ADDR according to this bit mask."]
    #[inline(always)]
    pub fn wr_mask(&self) -> WrMaskR {
        WrMaskR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:21 - 21:16\\]
Address AUX_SCE is allowed to update this DDI half-word using SET or CLR access."]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 22 - 22:22\\]
Read Enable 0: AUX_SCE is not allowed to read DDI half-word given by ADDR. 1: AUX_SCE is allowed to read DDI half-word given by ADDR."]
    #[inline(always)]
    pub fn rd_en(&self) -> RdEnR {
        RdEnR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 23:31 - 31:23\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved23(&self) -> Reserved23R {
        Reserved23R::new(((self.bits >> 23) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Mask AUX_SCE is allowed to update bits in half-word given by ADDR according to this bit mask."]
    #[inline(always)]
    #[must_use]
    pub fn wr_mask(&mut self) -> WrMaskW<Nonsecddiacc2Spec> {
        WrMaskW::new(self, 0)
    }
    #[doc = "Bits 16:21 - 21:16\\]
Address AUX_SCE is allowed to update this DDI half-word using SET or CLR access."]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> AddrW<Nonsecddiacc2Spec> {
        AddrW::new(self, 16)
    }
    #[doc = "Bit 22 - 22:22\\]
Read Enable 0: AUX_SCE is not allowed to read DDI half-word given by ADDR. 1: AUX_SCE is allowed to read DDI half-word given by ADDR."]
    #[inline(always)]
    #[must_use]
    pub fn rd_en(&mut self) -> RdEnW<Nonsecddiacc2Spec> {
        RdEnW::new(self, 22)
    }
}
#[doc = "Non-Secure DDI Access 2 When system is in secure state, AUX_SCE is allowed to update a predefined DDI half-word using SET or CLR access. Configuration will determine if AUX_SCE can read the same half-word. An access to a non-allowed register will suspend the AUX_SCE when system state is secure. If ADDR field in two or more NONSECDDIACC registers are equal, the MASK and RD_EN from the highest numbered register will be used. Examples: Half-word with address of 0 corresponds to DDI_0_OSC:CTL0 bit range \\[15:0\\]. Half-word with address of 1 corresponds to DDI_0_OSC:CTL0 bit range \\[31:16\\]. … Half-word with address of 34 corresponds to DDI_0_OSC:STAT2 bit range \\[15:0\\]. Half-word with address of 35 corresponds to DDI_0_OSC:STAT2 bit range \\[31:16\\].\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nonsecddiacc2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nonsecddiacc2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Nonsecddiacc2Spec;
impl crate::RegisterSpec for Nonsecddiacc2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nonsecddiacc2::R`](R) reader structure"]
impl crate::Readable for Nonsecddiacc2Spec {}
#[doc = "`write(|w| ..)` method takes [`nonsecddiacc2::W`](W) writer structure"]
impl crate::Writable for Nonsecddiacc2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets NONSECDDIACC2 to value 0"]
impl crate::Resettable for Nonsecddiacc2Spec {
    const RESET_VALUE: u32 = 0;
}
