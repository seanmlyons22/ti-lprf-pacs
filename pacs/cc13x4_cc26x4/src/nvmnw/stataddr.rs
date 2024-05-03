#[doc = "Register `STATADDR` reader"]
pub type R = crate::R<StataddrSpec>;
#[doc = "Register `STATADDR` writer"]
pub type W = crate::W<StataddrSpec>;
#[doc = "15:0\\]
Current Bank Address A bank offset address is stored in this register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Bankaddr {
    #[doc = "65535: Maximum value"]
    Maximum = 65535,
    #[doc = "0: Minimum value"]
    Minimum = 0,
}
impl From<Bankaddr> for u16 {
    #[inline(always)]
    fn from(variant: Bankaddr) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Bankaddr {
    type Ux = u16;
}
impl crate::IsEnum for Bankaddr {}
#[doc = "Field `BANKADDR` reader - 15:0\\]
Current Bank Address A bank offset address is stored in this register."]
pub type BankaddrR = crate::FieldReader<Bankaddr>;
impl BankaddrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Bankaddr> {
        match self.bits {
            65535 => Some(Bankaddr::Maximum),
            0 => Some(Bankaddr::Minimum),
            _ => None,
        }
    }
    #[doc = "Maximum value"]
    #[inline(always)]
    pub fn is_maximum(&self) -> bool {
        *self == Bankaddr::Maximum
    }
    #[doc = "Minimum value"]
    #[inline(always)]
    pub fn is_minimum(&self) -> bool {
        *self == Bankaddr::Minimum
    }
}
#[doc = "Field `BANKADDR` writer - 15:0\\]
Current Bank Address A bank offset address is stored in this register."]
pub type BankaddrW<'a, REG> = crate::FieldWriter<'a, REG, 16, Bankaddr>;
impl<'a, REG> BankaddrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "Maximum value"]
    #[inline(always)]
    pub fn maximum(self) -> &'a mut crate::W<REG> {
        self.variant(Bankaddr::Maximum)
    }
    #[doc = "Minimum value"]
    #[inline(always)]
    pub fn minimum(self) -> &'a mut crate::W<REG> {
        self.variant(Bankaddr::Minimum)
    }
}
#[doc = "20:16\\]
Current Region ID A region indicator is stored in this register which represents the current flash region on which the state machine is operating.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Regionid {
    #[doc = "8: Engr Region"]
    Engr = 8,
    #[doc = "4: Trim Region"]
    Trim = 4,
    #[doc = "2: Non-Main Region"]
    Nonmain = 2,
    #[doc = "1: Main Region"]
    Main = 1,
}
impl From<Regionid> for u8 {
    #[inline(always)]
    fn from(variant: Regionid) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Regionid {
    type Ux = u8;
}
impl crate::IsEnum for Regionid {}
#[doc = "Field `REGIONID` reader - 20:16\\]
Current Region ID A region indicator is stored in this register which represents the current flash region on which the state machine is operating."]
pub type RegionidR = crate::FieldReader<Regionid>;
impl RegionidR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Regionid> {
        match self.bits {
            8 => Some(Regionid::Engr),
            4 => Some(Regionid::Trim),
            2 => Some(Regionid::Nonmain),
            1 => Some(Regionid::Main),
            _ => None,
        }
    }
    #[doc = "Engr Region"]
    #[inline(always)]
    pub fn is_engr(&self) -> bool {
        *self == Regionid::Engr
    }
    #[doc = "Trim Region"]
    #[inline(always)]
    pub fn is_trim(&self) -> bool {
        *self == Regionid::Trim
    }
    #[doc = "Non-Main Region"]
    #[inline(always)]
    pub fn is_nonmain(&self) -> bool {
        *self == Regionid::Nonmain
    }
    #[doc = "Main Region"]
    #[inline(always)]
    pub fn is_main(&self) -> bool {
        *self == Regionid::Main
    }
}
#[doc = "Field `REGIONID` writer - 20:16\\]
Current Region ID A region indicator is stored in this register which represents the current flash region on which the state machine is operating."]
pub type RegionidW<'a, REG> = crate::FieldWriter<'a, REG, 5, Regionid>;
impl<'a, REG> RegionidW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Engr Region"]
    #[inline(always)]
    pub fn engr(self) -> &'a mut crate::W<REG> {
        self.variant(Regionid::Engr)
    }
    #[doc = "Trim Region"]
    #[inline(always)]
    pub fn trim(self) -> &'a mut crate::W<REG> {
        self.variant(Regionid::Trim)
    }
    #[doc = "Non-Main Region"]
    #[inline(always)]
    pub fn nonmain(self) -> &'a mut crate::W<REG> {
        self.variant(Regionid::Nonmain)
    }
    #[doc = "Main Region"]
    #[inline(always)]
    pub fn main(self) -> &'a mut crate::W<REG> {
        self.variant(Regionid::Main)
    }
}
#[doc = "25:21\\]
Current Bank ID A bank indicator is stored in this register which represents the current bank on which the state machine is operating. There is 1 bit per bank.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Bankid {
    #[doc = "16: Bank 4"]
    Bank4 = 16,
    #[doc = "8: Bank 3"]
    Bank3 = 8,
    #[doc = "4: Bank 2"]
    Bank2 = 4,
    #[doc = "2: Bank 1"]
    Bank1 = 2,
    #[doc = "1: Bank 0"]
    Bank0 = 1,
}
impl From<Bankid> for u8 {
    #[inline(always)]
    fn from(variant: Bankid) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Bankid {
    type Ux = u8;
}
impl crate::IsEnum for Bankid {}
#[doc = "Field `BANKID` reader - 25:21\\]
Current Bank ID A bank indicator is stored in this register which represents the current bank on which the state machine is operating. There is 1 bit per bank."]
pub type BankidR = crate::FieldReader<Bankid>;
impl BankidR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Bankid> {
        match self.bits {
            16 => Some(Bankid::Bank4),
            8 => Some(Bankid::Bank3),
            4 => Some(Bankid::Bank2),
            2 => Some(Bankid::Bank1),
            1 => Some(Bankid::Bank0),
            _ => None,
        }
    }
    #[doc = "Bank 4"]
    #[inline(always)]
    pub fn is_bank4(&self) -> bool {
        *self == Bankid::Bank4
    }
    #[doc = "Bank 3"]
    #[inline(always)]
    pub fn is_bank3(&self) -> bool {
        *self == Bankid::Bank3
    }
    #[doc = "Bank 2"]
    #[inline(always)]
    pub fn is_bank2(&self) -> bool {
        *self == Bankid::Bank2
    }
    #[doc = "Bank 1"]
    #[inline(always)]
    pub fn is_bank1(&self) -> bool {
        *self == Bankid::Bank1
    }
    #[doc = "Bank 0"]
    #[inline(always)]
    pub fn is_bank0(&self) -> bool {
        *self == Bankid::Bank0
    }
}
#[doc = "Field `BANKID` writer - 25:21\\]
Current Bank ID A bank indicator is stored in this register which represents the current bank on which the state machine is operating. There is 1 bit per bank."]
pub type BankidW<'a, REG> = crate::FieldWriter<'a, REG, 5, Bankid>;
impl<'a, REG> BankidW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Bank 4"]
    #[inline(always)]
    pub fn bank4(self) -> &'a mut crate::W<REG> {
        self.variant(Bankid::Bank4)
    }
    #[doc = "Bank 3"]
    #[inline(always)]
    pub fn bank3(self) -> &'a mut crate::W<REG> {
        self.variant(Bankid::Bank3)
    }
    #[doc = "Bank 2"]
    #[inline(always)]
    pub fn bank2(self) -> &'a mut crate::W<REG> {
        self.variant(Bankid::Bank2)
    }
    #[doc = "Bank 1"]
    #[inline(always)]
    pub fn bank1(self) -> &'a mut crate::W<REG> {
        self.variant(Bankid::Bank1)
    }
    #[doc = "Bank 0"]
    #[inline(always)]
    pub fn bank0(self) -> &'a mut crate::W<REG> {
        self.variant(Bankid::Bank0)
    }
}
#[doc = "Field `RESERVED26` reader - 31:26\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved26R = crate::FieldReader;
#[doc = "Field `RESERVED26` writer - 31:26\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved26W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Current Bank Address A bank offset address is stored in this register."]
    #[inline(always)]
    pub fn bankaddr(&self) -> BankaddrR {
        BankaddrR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Current Region ID A region indicator is stored in this register which represents the current flash region on which the state machine is operating."]
    #[inline(always)]
    pub fn regionid(&self) -> RegionidR {
        RegionidR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:25 - 25:21\\]
Current Bank ID A bank indicator is stored in this register which represents the current bank on which the state machine is operating. There is 1 bit per bank."]
    #[inline(always)]
    pub fn bankid(&self) -> BankidR {
        BankidR::new(((self.bits >> 21) & 0x1f) as u8)
    }
    #[doc = "Bits 26:31 - 31:26\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved26(&self) -> Reserved26R {
        Reserved26R::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Current Bank Address A bank offset address is stored in this register."]
    #[inline(always)]
    #[must_use]
    pub fn bankaddr(&mut self) -> BankaddrW<StataddrSpec> {
        BankaddrW::new(self, 0)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Current Region ID A region indicator is stored in this register which represents the current flash region on which the state machine is operating."]
    #[inline(always)]
    #[must_use]
    pub fn regionid(&mut self) -> RegionidW<StataddrSpec> {
        RegionidW::new(self, 16)
    }
    #[doc = "Bits 21:25 - 25:21\\]
Current Bank ID A bank indicator is stored in this register which represents the current bank on which the state machine is operating. There is 1 bit per bank."]
    #[inline(always)]
    #[must_use]
    pub fn bankid(&mut self) -> BankidW<StataddrSpec> {
        BankidW::new(self, 21)
    }
    #[doc = "Bits 26:31 - 31:26\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved26(&mut self) -> Reserved26W<StataddrSpec> {
        Reserved26W::new(self, 26)
    }
}
#[doc = "Current Address Counter Value Read only register giving read access to the state machine current address. A bank id, region id and address are stored in this register and are incremented as necessary during execution of a command.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stataddr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stataddr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StataddrSpec;
impl crate::RegisterSpec for StataddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stataddr::R`](R) reader structure"]
impl crate::Readable for StataddrSpec {}
#[doc = "`write(|w| ..)` method takes [`stataddr::W`](W) writer structure"]
impl crate::Writable for StataddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STATADDR to value 0x0020_0000"]
impl crate::Resettable for StataddrSpec {
    const RESET_VALUE: u32 = 0x0020_0000;
}
