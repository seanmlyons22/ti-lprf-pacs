#[doc = "Register `AIFDIRCFG` reader"]
pub type R = crate::R<AifdircfgSpec>;
#[doc = "Register `AIFDIRCFG` writer"]
pub type W = crate::W<AifdircfgSpec>;
#[doc = "1:0\\]
Configures the AD0 audio data pin usage: 0x3: Reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ad0 {
    #[doc = "2: Output mode"]
    Out = 2,
    #[doc = "1: Input mode"]
    In = 1,
    #[doc = "0: Not in use (disabled)"]
    Dis = 0,
}
impl From<Ad0> for u8 {
    #[inline(always)]
    fn from(variant: Ad0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ad0 {
    type Ux = u8;
}
impl crate::IsEnum for Ad0 {}
#[doc = "Field `AD0` reader - 1:0\\]
Configures the AD0 audio data pin usage: 0x3: Reserved"]
pub type Ad0R = crate::FieldReader<Ad0>;
impl Ad0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ad0> {
        match self.bits {
            2 => Some(Ad0::Out),
            1 => Some(Ad0::In),
            0 => Some(Ad0::Dis),
            _ => None,
        }
    }
    #[doc = "Output mode"]
    #[inline(always)]
    pub fn is_out(&self) -> bool {
        *self == Ad0::Out
    }
    #[doc = "Input mode"]
    #[inline(always)]
    pub fn is_in(&self) -> bool {
        *self == Ad0::In
    }
    #[doc = "Not in use (disabled)"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Ad0::Dis
    }
}
#[doc = "Field `AD0` writer - 1:0\\]
Configures the AD0 audio data pin usage: 0x3: Reserved"]
pub type Ad0W<'a, REG> = crate::FieldWriter<'a, REG, 2, Ad0>;
impl<'a, REG> Ad0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Output mode"]
    #[inline(always)]
    pub fn out(self) -> &'a mut crate::W<REG> {
        self.variant(Ad0::Out)
    }
    #[doc = "Input mode"]
    #[inline(always)]
    pub fn in_(self) -> &'a mut crate::W<REG> {
        self.variant(Ad0::In)
    }
    #[doc = "Not in use (disabled)"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Ad0::Dis)
    }
}
#[doc = "Field `RESERVED2` reader - 3:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2R = crate::FieldReader;
#[doc = "5:4\\]
Configures the AD1 audio data pin usage: 0x3: Reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ad1 {
    #[doc = "2: Output mode"]
    Out = 2,
    #[doc = "1: Input mode"]
    In = 1,
    #[doc = "0: Not in use (disabled)"]
    Dis = 0,
}
impl From<Ad1> for u8 {
    #[inline(always)]
    fn from(variant: Ad1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ad1 {
    type Ux = u8;
}
impl crate::IsEnum for Ad1 {}
#[doc = "Field `AD1` reader - 5:4\\]
Configures the AD1 audio data pin usage: 0x3: Reserved"]
pub type Ad1R = crate::FieldReader<Ad1>;
impl Ad1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ad1> {
        match self.bits {
            2 => Some(Ad1::Out),
            1 => Some(Ad1::In),
            0 => Some(Ad1::Dis),
            _ => None,
        }
    }
    #[doc = "Output mode"]
    #[inline(always)]
    pub fn is_out(&self) -> bool {
        *self == Ad1::Out
    }
    #[doc = "Input mode"]
    #[inline(always)]
    pub fn is_in(&self) -> bool {
        *self == Ad1::In
    }
    #[doc = "Not in use (disabled)"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Ad1::Dis
    }
}
#[doc = "Field `AD1` writer - 5:4\\]
Configures the AD1 audio data pin usage: 0x3: Reserved"]
pub type Ad1W<'a, REG> = crate::FieldWriter<'a, REG, 2, Ad1>;
impl<'a, REG> Ad1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Output mode"]
    #[inline(always)]
    pub fn out(self) -> &'a mut crate::W<REG> {
        self.variant(Ad1::Out)
    }
    #[doc = "Input mode"]
    #[inline(always)]
    pub fn in_(self) -> &'a mut crate::W<REG> {
        self.variant(Ad1::In)
    }
    #[doc = "Not in use (disabled)"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Ad1::Dis)
    }
}
#[doc = "Field `RESERVED6` reader - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved6R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Configures the AD0 audio data pin usage: 0x3: Reserved"]
    #[inline(always)]
    pub fn ad0(&self) -> Ad0R {
        Ad0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - 3:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - 5:4\\]
Configures the AD1 audio data pin usage: 0x3: Reserved"]
    #[inline(always)]
    pub fn ad1(&self) -> Ad1R {
        Ad1R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:31 - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new((self.bits >> 6) & 0x03ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Configures the AD0 audio data pin usage: 0x3: Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn ad0(&mut self) -> Ad0W<AifdircfgSpec> {
        Ad0W::new(self, 0)
    }
    #[doc = "Bits 4:5 - 5:4\\]
Configures the AD1 audio data pin usage: 0x3: Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn ad1(&mut self) -> Ad1W<AifdircfgSpec> {
        Ad1W::new(self, 4)
    }
}
#[doc = "Pin Direction\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aifdircfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aifdircfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AifdircfgSpec;
impl crate::RegisterSpec for AifdircfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aifdircfg::R`](R) reader structure"]
impl crate::Readable for AifdircfgSpec {}
#[doc = "`write(|w| ..)` method takes [`aifdircfg::W`](W) writer structure"]
impl crate::Writable for AifdircfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AIFDIRCFG to value 0"]
impl crate::Resettable for AifdircfgSpec {
    const RESET_VALUE: u32 = 0;
}
