#[doc = "Register `OPMODEACK` reader"]
pub type R = crate::R<OpmodeackSpec>;
#[doc = "Register `OPMODEACK` writer"]
pub type W = crate::W<OpmodeackSpec>;
#[doc = "1:0\\]
AUX operational mode acknowledgement.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ack {
    #[doc = "3: Powerdown operational mode with wakeup to lowpower mode is acknowledged."]
    Pdlp = 3,
    #[doc = "2: Powerdown operational mode with wakeup to active mode is acknowledged."]
    Pda = 2,
    #[doc = "1: Lowpower operational mode is acknowledged."]
    Lp = 1,
    #[doc = "0: Active operational mode is acknowledged."]
    A = 0,
}
impl From<Ack> for u8 {
    #[inline(always)]
    fn from(variant: Ack) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ack {
    type Ux = u8;
}
impl crate::IsEnum for Ack {}
#[doc = "Field `ACK` reader - 1:0\\]
AUX operational mode acknowledgement."]
pub type AckR = crate::FieldReader<Ack>;
impl AckR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ack {
        match self.bits {
            3 => Ack::Pdlp,
            2 => Ack::Pda,
            1 => Ack::Lp,
            0 => Ack::A,
            _ => unreachable!(),
        }
    }
    #[doc = "Powerdown operational mode with wakeup to lowpower mode is acknowledged."]
    #[inline(always)]
    pub fn is_pdlp(&self) -> bool {
        *self == Ack::Pdlp
    }
    #[doc = "Powerdown operational mode with wakeup to active mode is acknowledged."]
    #[inline(always)]
    pub fn is_pda(&self) -> bool {
        *self == Ack::Pda
    }
    #[doc = "Lowpower operational mode is acknowledged."]
    #[inline(always)]
    pub fn is_lp(&self) -> bool {
        *self == Ack::Lp
    }
    #[doc = "Active operational mode is acknowledged."]
    #[inline(always)]
    pub fn is_a(&self) -> bool {
        *self == Ack::A
    }
}
#[doc = "Field `RESERVED2` reader - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
AUX operational mode acknowledgement."]
    #[inline(always)]
    pub fn ack(&self) -> AckR {
        AckR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {}
#[doc = "Operational Mode Acknowledgement AUX_SCE program must assume that the current operational mode is the one acknowledged.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`opmodeack::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`opmodeack::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OpmodeackSpec;
impl crate::RegisterSpec for OpmodeackSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`opmodeack::R`](R) reader structure"]
impl crate::Readable for OpmodeackSpec {}
#[doc = "`write(|w| ..)` method takes [`opmodeack::W`](W) writer structure"]
impl crate::Writable for OpmodeackSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OPMODEACK to value 0"]
impl crate::Resettable for OpmodeackSpec {
    const RESET_VALUE: u32 = 0;
}
