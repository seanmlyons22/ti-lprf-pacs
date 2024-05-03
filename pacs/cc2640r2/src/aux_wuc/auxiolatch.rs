#[doc = "Register `AUXIOLATCH` reader"]
pub type R = crate::R<AuxiolatchSpec>;
#[doc = "Register `AUXIOLATCH` writer"]
pub type W = crate::W<AuxiolatchSpec>;
#[doc = "0:0\\]
Opens (1) or closes (0) the AUX_AIODIO0/AUX_AIODIO1 signal latching. At startup, set EN = TRANSP before configuring AUX_AIODIO0/AUX_AIODIO1 and subsequently selecting AUX mode in the AON_IOC. When powering off the AUX domain (using PWROFFREQ.REQ), set EN = STATIC in advance preserve the current state (mode and output value) of the I/O pins.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum En {
    #[doc = "1: Latches are transparent ( open )"]
    Transp = 1,
    #[doc = "0: Latches are static ( closed )"]
    Static = 0,
}
impl From<En> for bool {
    #[inline(always)]
    fn from(variant: En) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN` reader - 0:0\\]
Opens (1) or closes (0) the AUX_AIODIO0/AUX_AIODIO1 signal latching. At startup, set EN = TRANSP before configuring AUX_AIODIO0/AUX_AIODIO1 and subsequently selecting AUX mode in the AON_IOC. When powering off the AUX domain (using PWROFFREQ.REQ), set EN = STATIC in advance preserve the current state (mode and output value) of the I/O pins."]
pub type EnR = crate::BitReader<En>;
impl EnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> En {
        match self.bits {
            true => En::Transp,
            false => En::Static,
        }
    }
    #[doc = "Latches are transparent ( open )"]
    #[inline(always)]
    pub fn is_transp(&self) -> bool {
        *self == En::Transp
    }
    #[doc = "Latches are static ( closed )"]
    #[inline(always)]
    pub fn is_static(&self) -> bool {
        *self == En::Static
    }
}
#[doc = "Field `EN` writer - 0:0\\]
Opens (1) or closes (0) the AUX_AIODIO0/AUX_AIODIO1 signal latching. At startup, set EN = TRANSP before configuring AUX_AIODIO0/AUX_AIODIO1 and subsequently selecting AUX mode in the AON_IOC. When powering off the AUX domain (using PWROFFREQ.REQ), set EN = STATIC in advance preserve the current state (mode and output value) of the I/O pins."]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG, En>;
impl<'a, REG> EnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Latches are transparent ( open )"]
    #[inline(always)]
    pub fn transp(self) -> &'a mut crate::W<REG> {
        self.variant(En::Transp)
    }
    #[doc = "Latches are static ( closed )"]
    #[inline(always)]
    pub fn static_(self) -> &'a mut crate::W<REG> {
        self.variant(En::Static)
    }
}
impl R {
    #[doc = "Bit 0 - 0:0\\]
Opens (1) or closes (0) the AUX_AIODIO0/AUX_AIODIO1 signal latching. At startup, set EN = TRANSP before configuring AUX_AIODIO0/AUX_AIODIO1 and subsequently selecting AUX mode in the AON_IOC. When powering off the AUX domain (using PWROFFREQ.REQ), set EN = STATIC in advance preserve the current state (mode and output value) of the I/O pins."]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Opens (1) or closes (0) the AUX_AIODIO0/AUX_AIODIO1 signal latching. At startup, set EN = TRANSP before configuring AUX_AIODIO0/AUX_AIODIO1 and subsequently selecting AUX mode in the AON_IOC. When powering off the AUX domain (using PWROFFREQ.REQ), set EN = STATIC in advance preserve the current state (mode and output value) of the I/O pins."]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EnW<AuxiolatchSpec> {
        EnW::new(self, 0)
    }
}
#[doc = "AUX Input Output Latch Controls latching of signals between AUX_AIODIO0/AUX_AIODIO1 and AON_IOC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`auxiolatch::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`auxiolatch::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AuxiolatchSpec;
impl crate::RegisterSpec for AuxiolatchSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`auxiolatch::R`](R) reader structure"]
impl crate::Readable for AuxiolatchSpec {}
#[doc = "`write(|w| ..)` method takes [`auxiolatch::W`](W) writer structure"]
impl crate::Writable for AuxiolatchSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AUXIOLATCH to value 0"]
impl crate::Resettable for AuxiolatchSpec {
    const RESET_VALUE: u32 = 0;
}
