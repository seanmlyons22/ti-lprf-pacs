#[doc = "Register `TXCRC` reader"]
pub type R = crate::R<TxcrcSpec>;
#[doc = "Register `TXCRC` writer"]
pub type W = crate::W<TxcrcSpec>;
#[doc = "Field `DATA` reader - 15:0\\]
CRC value"]
pub type DataR = crate::FieldReader<u16>;
#[doc = "Field `DATA` writer - 15:0\\]
CRC value"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `RESERVED16` reader - 30:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved16R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED16` writer - 30:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved16W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "31:31\\]
Status to indicate if Auto CRC has been inserted into TXFIFO. This is valid only if CTL0.AUTO_CRC enable bit is set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AutoCrcInsStat {
    #[doc = "1: Auto CRC inserted"]
    Ins = 1,
    #[doc = "0: Auto CRC not yet inserted"]
    Notins = 0,
}
impl From<AutoCrcInsStat> for bool {
    #[inline(always)]
    fn from(variant: AutoCrcInsStat) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AUTO_CRC_INS_STAT` reader - 31:31\\]
Status to indicate if Auto CRC has been inserted into TXFIFO. This is valid only if CTL0.AUTO_CRC enable bit is set"]
pub type AutoCrcInsStatR = crate::BitReader<AutoCrcInsStat>;
impl AutoCrcInsStatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AutoCrcInsStat {
        match self.bits {
            true => AutoCrcInsStat::Ins,
            false => AutoCrcInsStat::Notins,
        }
    }
    #[doc = "Auto CRC inserted"]
    #[inline(always)]
    pub fn is_ins(&self) -> bool {
        *self == AutoCrcInsStat::Ins
    }
    #[doc = "Auto CRC not yet inserted"]
    #[inline(always)]
    pub fn is_notins(&self) -> bool {
        *self == AutoCrcInsStat::Notins
    }
}
#[doc = "Field `AUTO_CRC_INS_STAT` writer - 31:31\\]
Status to indicate if Auto CRC has been inserted into TXFIFO. This is valid only if CTL0.AUTO_CRC enable bit is set"]
pub type AutoCrcInsStatW<'a, REG> = crate::BitWriter<'a, REG, AutoCrcInsStat>;
impl<'a, REG> AutoCrcInsStatW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Auto CRC inserted"]
    #[inline(always)]
    pub fn ins(self) -> &'a mut crate::W<REG> {
        self.variant(AutoCrcInsStat::Ins)
    }
    #[doc = "Auto CRC not yet inserted"]
    #[inline(always)]
    pub fn notins(self) -> &'a mut crate::W<REG> {
        self.variant(AutoCrcInsStat::Notins)
    }
}
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
CRC value"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:30 - 30:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved16(&self) -> Reserved16R {
        Reserved16R::new(((self.bits >> 16) & 0x7fff) as u16)
    }
    #[doc = "Bit 31 - 31:31\\]
Status to indicate if Auto CRC has been inserted into TXFIFO. This is valid only if CTL0.AUTO_CRC enable bit is set"]
    #[inline(always)]
    pub fn auto_crc_ins_stat(&self) -> AutoCrcInsStatR {
        AutoCrcInsStatR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
CRC value"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DataW<TxcrcSpec> {
        DataW::new(self, 0)
    }
    #[doc = "Bits 16:30 - 30:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved16(&mut self) -> Reserved16W<TxcrcSpec> {
        Reserved16W::new(self, 16)
    }
    #[doc = "Bit 31 - 31:31\\]
Status to indicate if Auto CRC has been inserted into TXFIFO. This is valid only if CTL0.AUTO_CRC enable bit is set"]
    #[inline(always)]
    #[must_use]
    pub fn auto_crc_ins_stat(&mut self) -> AutoCrcInsStatW<TxcrcSpec> {
        AutoCrcInsStatW::new(self, 31)
    }
}
#[doc = "Transmit CRC register. Reading this register provides the computed CRC value from the transmit side CRC unit. Reading this register or writing to this register with any value auto initializes the seed. The seed value is 0xFF when CTL0.CRCPOLY = 0 and 0xFFFF when CTL0.CRCPOLY = 1 for CCITT CRC polynomials. Bits\\[15:8\\]
are a don't care when CTL0.CRCPOLY = 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txcrc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txcrc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxcrcSpec;
impl crate::RegisterSpec for TxcrcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txcrc::R`](R) reader structure"]
impl crate::Readable for TxcrcSpec {}
#[doc = "`write(|w| ..)` method takes [`txcrc::W`](W) writer structure"]
impl crate::Writable for TxcrcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TXCRC to value 0"]
impl crate::Resettable for TxcrcSpec {
    const RESET_VALUE: u32 = 0;
}
