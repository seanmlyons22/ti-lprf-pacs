#[doc = "Register `FLBSTAT` reader"]
pub type R = crate::R<FlbstatSpec>;
#[doc = "Register `FLBSTAT` writer"]
pub type W = crate::W<FlbstatSpec>;
#[doc = "0:0\\]
This bit indicates if flash is ready in 1T mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum B1trdy {
    #[doc = "1: Ready"]
    Ready = 1,
    #[doc = "0: Not Ready"]
    Notready = 0,
}
impl From<B1trdy> for bool {
    #[inline(always)]
    fn from(variant: B1trdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `B1TRDY` reader - 0:0\\]
This bit indicates if flash is ready in 1T mode."]
pub type B1trdyR = crate::BitReader<B1trdy>;
impl B1trdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> B1trdy {
        match self.bits {
            true => B1trdy::Ready,
            false => B1trdy::Notready,
        }
    }
    #[doc = "Ready"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == B1trdy::Ready
    }
    #[doc = "Not Ready"]
    #[inline(always)]
    pub fn is_notready(&self) -> bool {
        *self == B1trdy::Notready
    }
}
#[doc = "Field `B1TRDY` writer - 0:0\\]
This bit indicates if flash is ready in 1T mode."]
pub type B1trdyW<'a, REG> = crate::BitWriter<'a, REG, B1trdy>;
impl<'a, REG> B1trdyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Ready"]
    #[inline(always)]
    pub fn ready(self) -> &'a mut crate::W<REG> {
        self.variant(B1trdy::Ready)
    }
    #[doc = "Not Ready"]
    #[inline(always)]
    pub fn notready(self) -> &'a mut crate::W<REG> {
        self.variant(B1trdy::Notready)
    }
}
#[doc = "1:1\\]
This bit indicates if flash is ready in 2T mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum B2trdy {
    #[doc = "1: Ready"]
    Ready = 1,
    #[doc = "0: Not Ready"]
    Notready = 0,
}
impl From<B2trdy> for bool {
    #[inline(always)]
    fn from(variant: B2trdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `B2TRDY` reader - 1:1\\]
This bit indicates if flash is ready in 2T mode."]
pub type B2trdyR = crate::BitReader<B2trdy>;
impl B2trdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> B2trdy {
        match self.bits {
            true => B2trdy::Ready,
            false => B2trdy::Notready,
        }
    }
    #[doc = "Ready"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == B2trdy::Ready
    }
    #[doc = "Not Ready"]
    #[inline(always)]
    pub fn is_notready(&self) -> bool {
        *self == B2trdy::Notready
    }
}
#[doc = "Field `B2TRDY` writer - 1:1\\]
This bit indicates if flash is ready in 2T mode."]
pub type B2trdyW<'a, REG> = crate::BitWriter<'a, REG, B2trdy>;
impl<'a, REG> B2trdyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Ready"]
    #[inline(always)]
    pub fn ready(self) -> &'a mut crate::W<REG> {
        self.variant(B2trdy::Ready)
    }
    #[doc = "Not Ready"]
    #[inline(always)]
    pub fn notready(self) -> &'a mut crate::W<REG> {
        self.variant(B2trdy::Notready)
    }
}
#[doc = "2:2\\]
This bit indicates if flash is busy.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum B0bsy {
    #[doc = "1: Busy"]
    Busy = 1,
    #[doc = "0: Idle"]
    Idle = 0,
}
impl From<B0bsy> for bool {
    #[inline(always)]
    fn from(variant: B0bsy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `B0BSY` reader - 2:2\\]
This bit indicates if flash is busy."]
pub type B0bsyR = crate::BitReader<B0bsy>;
impl B0bsyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> B0bsy {
        match self.bits {
            true => B0bsy::Busy,
            false => B0bsy::Idle,
        }
    }
    #[doc = "Busy"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == B0bsy::Busy
    }
    #[doc = "Idle"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == B0bsy::Idle
    }
}
#[doc = "Field `B0BSY` writer - 2:2\\]
This bit indicates if flash is busy."]
pub type B0bsyW<'a, REG> = crate::BitWriter<'a, REG, B0bsy>;
impl<'a, REG> B0bsyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Busy"]
    #[inline(always)]
    pub fn busy(self) -> &'a mut crate::W<REG> {
        self.variant(B0bsy::Busy)
    }
    #[doc = "Idle"]
    #[inline(always)]
    pub fn idle(self) -> &'a mut crate::W<REG> {
        self.variant(B0bsy::Idle)
    }
}
#[doc = "3:3\\]
This bit indicates parity error on write/erase $amp; read protection MMRs. This bit is sticky when set to 1 by hardware.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Parerr {
    #[doc = "1: Error"]
    Error = 1,
    #[doc = "0: No Error"]
    Noerror = 0,
}
impl From<Parerr> for bool {
    #[inline(always)]
    fn from(variant: Parerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PARERR` reader - 3:3\\]
This bit indicates parity error on write/erase $amp; read protection MMRs. This bit is sticky when set to 1 by hardware."]
pub type ParerrR = crate::BitReader<Parerr>;
impl ParerrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Parerr {
        match self.bits {
            true => Parerr::Error,
            false => Parerr::Noerror,
        }
    }
    #[doc = "Error"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == Parerr::Error
    }
    #[doc = "No Error"]
    #[inline(always)]
    pub fn is_noerror(&self) -> bool {
        *self == Parerr::Noerror
    }
}
#[doc = "Field `PARERR` writer - 3:3\\]
This bit indicates parity error on write/erase $amp; read protection MMRs. This bit is sticky when set to 1 by hardware."]
pub type ParerrW<'a, REG> = crate::BitWriter<'a, REG, Parerr>;
impl<'a, REG> ParerrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Error"]
    #[inline(always)]
    pub fn error(self) -> &'a mut crate::W<REG> {
        self.variant(Parerr::Error)
    }
    #[doc = "No Error"]
    #[inline(always)]
    pub fn noerror(self) -> &'a mut crate::W<REG> {
        self.variant(Parerr::Noerror)
    }
}
#[doc = "Field `RESERVED4` reader - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved4R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED4` writer - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved4W<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
This bit indicates if flash is ready in 1T mode."]
    #[inline(always)]
    pub fn b1trdy(&self) -> B1trdyR {
        B1trdyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
This bit indicates if flash is ready in 2T mode."]
    #[inline(always)]
    pub fn b2trdy(&self) -> B2trdyR {
        B2trdyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
This bit indicates if flash is busy."]
    #[inline(always)]
    pub fn b0bsy(&self) -> B0bsyR {
        B0bsyR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
This bit indicates parity error on write/erase $amp; read protection MMRs. This bit is sticky when set to 1 by hardware."]
    #[inline(always)]
    pub fn parerr(&self) -> ParerrR {
        ParerrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:31 - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
This bit indicates if flash is ready in 1T mode."]
    #[inline(always)]
    #[must_use]
    pub fn b1trdy(&mut self) -> B1trdyW<FlbstatSpec> {
        B1trdyW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
This bit indicates if flash is ready in 2T mode."]
    #[inline(always)]
    #[must_use]
    pub fn b2trdy(&mut self) -> B2trdyW<FlbstatSpec> {
        B2trdyW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
This bit indicates if flash is busy."]
    #[inline(always)]
    #[must_use]
    pub fn b0bsy(&mut self) -> B0bsyW<FlbstatSpec> {
        B0bsyW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
This bit indicates parity error on write/erase $amp; read protection MMRs. This bit is sticky when set to 1 by hardware."]
    #[inline(always)]
    #[must_use]
    pub fn parerr(&mut self) -> ParerrW<FlbstatSpec> {
        ParerrW::new(self, 3)
    }
    #[doc = "Bits 4:31 - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved4(&mut self) -> Reserved4W<FlbstatSpec> {
        Reserved4W::new(self, 4)
    }
}
#[doc = "This register is used to indicate status of flash.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flbstat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flbstat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlbstatSpec;
impl crate::RegisterSpec for FlbstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flbstat::R`](R) reader structure"]
impl crate::Readable for FlbstatSpec {}
#[doc = "`write(|w| ..)` method takes [`flbstat::W`](W) writer structure"]
impl crate::Writable for FlbstatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLBSTAT to value 0"]
impl crate::Resettable for FlbstatSpec {
    const RESET_VALUE: u32 = 0;
}
