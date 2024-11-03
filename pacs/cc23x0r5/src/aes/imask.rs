#[doc = "Register `IMASK` reader"]
pub type R = crate::R<ImaskSpec>;
#[doc = "Register `IMASK` writer"]
pub type W = crate::W<ImaskSpec>;
#[doc = "0:0\\]
ECB Done interrupt mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ecbdone {
    #[doc = "1: Enable interrupt mask"]
    En = 1,
    #[doc = "0: Disable interrupt mask"]
    Dis = 0,
}
impl From<Ecbdone> for bool {
    #[inline(always)]
    fn from(variant: Ecbdone) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ECBDONE` reader - 0:0\\]
ECB Done interrupt mask"]
pub type EcbdoneR = crate::BitReader<Ecbdone>;
impl EcbdoneR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ecbdone {
        match self.bits {
            true => Ecbdone::En,
            false => Ecbdone::Dis,
        }
    }
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Ecbdone::En
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Ecbdone::Dis
    }
}
#[doc = "Field `ECBDONE` writer - 0:0\\]
ECB Done interrupt mask"]
pub type EcbdoneW<'a, REG> = crate::BitWriter<'a, REG, Ecbdone>;
impl<'a, REG> EcbdoneW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Ecbdone::En)
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Ecbdone::Dis)
    }
}
#[doc = "1:1\\]
ECB Start interrupt mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ecbstart {
    #[doc = "1: Enable interrupt mask"]
    En = 1,
    #[doc = "0: Disable interrupt mask"]
    Dis = 0,
}
impl From<Ecbstart> for bool {
    #[inline(always)]
    fn from(variant: Ecbstart) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ECBSTART` reader - 1:1\\]
ECB Start interrupt mask"]
pub type EcbstartR = crate::BitReader<Ecbstart>;
impl EcbstartR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ecbstart {
        match self.bits {
            true => Ecbstart::En,
            false => Ecbstart::Dis,
        }
    }
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Ecbstart::En
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Ecbstart::Dis
    }
}
#[doc = "Field `ECBSTART` writer - 1:1\\]
ECB Start interrupt mask"]
pub type EcbstartW<'a, REG> = crate::BitWriter<'a, REG, Ecbstart>;
impl<'a, REG> EcbstartW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Ecbstart::En)
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Ecbstart::Dis)
    }
}
#[doc = "2:2\\]
DMA Channel A Done interrupt mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chadone {
    #[doc = "1: Enable interrupt mask"]
    En = 1,
    #[doc = "0: Disable interrupt mask"]
    Dis = 0,
}
impl From<Chadone> for bool {
    #[inline(always)]
    fn from(variant: Chadone) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHADONE` reader - 2:2\\]
DMA Channel A Done interrupt mask"]
pub type ChadoneR = crate::BitReader<Chadone>;
impl ChadoneR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chadone {
        match self.bits {
            true => Chadone::En,
            false => Chadone::Dis,
        }
    }
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Chadone::En
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Chadone::Dis
    }
}
#[doc = "Field `CHADONE` writer - 2:2\\]
DMA Channel A Done interrupt mask"]
pub type ChadoneW<'a, REG> = crate::BitWriter<'a, REG, Chadone>;
impl<'a, REG> ChadoneW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Chadone::En)
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Chadone::Dis)
    }
}
#[doc = "3:3\\]
DMA Channel B Done interrupt mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chbdone {
    #[doc = "1: Enable interrupt mask"]
    En = 1,
    #[doc = "0: Disable interrupt mask"]
    Dis = 0,
}
impl From<Chbdone> for bool {
    #[inline(always)]
    fn from(variant: Chbdone) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHBDONE` reader - 3:3\\]
DMA Channel B Done interrupt mask"]
pub type ChbdoneR = crate::BitReader<Chbdone>;
impl ChbdoneR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chbdone {
        match self.bits {
            true => Chbdone::En,
            false => Chbdone::Dis,
        }
    }
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Chbdone::En
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Chbdone::Dis
    }
}
#[doc = "Field `CHBDONE` writer - 3:3\\]
DMA Channel B Done interrupt mask"]
pub type ChbdoneW<'a, REG> = crate::BitWriter<'a, REG, Chbdone>;
impl<'a, REG> ChbdoneW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Chbdone::En)
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Chbdone::Dis)
    }
}
#[doc = "Field `RESERVED2` reader - 31:4\\]
Reads to this field return zero, writes to this field are ignored."]
pub type Reserved2R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED2` writer - 31:4\\]
Reads to this field return zero, writes to this field are ignored."]
pub type Reserved2W<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
ECB Done interrupt mask"]
    #[inline(always)]
    pub fn ecbdone(&self) -> EcbdoneR {
        EcbdoneR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
ECB Start interrupt mask"]
    #[inline(always)]
    pub fn ecbstart(&self) -> EcbstartR {
        EcbstartR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
DMA Channel A Done interrupt mask"]
    #[inline(always)]
    pub fn chadone(&self) -> ChadoneR {
        ChadoneR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
DMA Channel B Done interrupt mask"]
    #[inline(always)]
    pub fn chbdone(&self) -> ChbdoneR {
        ChbdoneR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:31 - 31:4\\]
Reads to this field return zero, writes to this field are ignored."]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
ECB Done interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn ecbdone(&mut self) -> EcbdoneW<ImaskSpec> {
        EcbdoneW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
ECB Start interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn ecbstart(&mut self) -> EcbstartW<ImaskSpec> {
        EcbstartW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
DMA Channel A Done interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn chadone(&mut self) -> ChadoneW<ImaskSpec> {
        ChadoneW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
DMA Channel B Done interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn chbdone(&mut self) -> ChbdoneW<ImaskSpec> {
        ChbdoneW::new(self, 3)
    }
    #[doc = "Bits 4:31 - 31:4\\]
Reads to this field return zero, writes to this field are ignored."]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> Reserved2W<ImaskSpec> {
        Reserved2W::new(self, 4)
    }
}
#[doc = "Interrupt mask. This register selects interrupt sources which are allowed to pass from RIS to MIS when the corresponding bit-fields are set to 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imask::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`imask::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ImaskSpec;
impl crate::RegisterSpec for ImaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imask::R`](R) reader structure"]
impl crate::Readable for ImaskSpec {}
#[doc = "`write(|w| ..)` method takes [`imask::W`](W) writer structure"]
impl crate::Writable for ImaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IMASK to value 0"]
impl crate::Resettable for ImaskSpec {
    const RESET_VALUE: u32 = 0;
}
