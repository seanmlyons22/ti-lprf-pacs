#[doc = "Register `IOMODE` reader"]
pub type R = crate::R<IomodeSpec>;
#[doc = "Register `IOMODE` writer"]
pub type W = crate::W<IomodeSpec>;
#[doc = "1:0\\]
Select mode for AUXIO\\[8i+0\\].\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Io0 {
    #[doc = "3: Open-Source Mode: When IOPOE bit 0 is 0: - If GPIODOUT bit 0 is 0: AUXIO\\[8i+0\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. - If GPIODOUT bit 0 is 1: AUXIO\\[8i+0\\]
is driven high. When IOPOE bit 0 is 1: - If signal selected by IO0PSEL.SRC is 0: AUXIO\\[8i+0\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. - If signal selected by IO0PSEL.SRC is 1: AUXIO\\[8i+0\\]
is driven high."]
    OpenSource = 3,
    #[doc = "2: Open-Drain Mode: When IOPOE bit 0 is 0: - If GPIODOUT bit 0 is 0: AUXIO\\[8i+0\\]
is driven low. - If GPIODOUT bit 0 is 1: AUXIO\\[8i+0\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. When IOPOE bit 0 is 1: - If signal selected by IO0PSEL.SRC is 0: AUXIO\\[8i+0\\]
is driven low. - If signal selected by IO0PSEL.SRC is 1: AUXIO\\[8i+0\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL."]
    OpenDrain = 2,
    #[doc = "1: Input Mode: When GPIODIE bit 0 is 0: AUXIO\\[8i+0\\]
is enabled for analog signal transfer. When GPIODIE bit 0 is 1: AUXIO\\[8i+0\\]
is enabled for digital input."]
    In = 1,
    #[doc = "0: Output Mode: When IOPOE bit 0 is 0: GPIODOUT bit 0 drives AUXIO\\[8i+0\\]. When IOPOE bit 0 is 1: The signal selected by IO0PSEL.SRC drives AUXIO\\[8i+0\\]."]
    Out = 0,
}
impl From<Io0> for u8 {
    #[inline(always)]
    fn from(variant: Io0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Io0 {
    type Ux = u8;
}
impl crate::IsEnum for Io0 {}
#[doc = "Field `IO0` reader - 1:0\\]
Select mode for AUXIO\\[8i+0\\]."]
pub type Io0R = crate::FieldReader<Io0>;
impl Io0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Io0 {
        match self.bits {
            3 => Io0::OpenSource,
            2 => Io0::OpenDrain,
            1 => Io0::In,
            0 => Io0::Out,
            _ => unreachable!(),
        }
    }
    #[doc = "Open-Source Mode: When IOPOE bit 0 is 0: - If GPIODOUT bit 0 is 0: AUXIO\\[8i+0\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. - If GPIODOUT bit 0 is 1: AUXIO\\[8i+0\\]
is driven high. When IOPOE bit 0 is 1: - If signal selected by IO0PSEL.SRC is 0: AUXIO\\[8i+0\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. - If signal selected by IO0PSEL.SRC is 1: AUXIO\\[8i+0\\]
is driven high."]
    #[inline(always)]
    pub fn is_open_source(&self) -> bool {
        *self == Io0::OpenSource
    }
    #[doc = "Open-Drain Mode: When IOPOE bit 0 is 0: - If GPIODOUT bit 0 is 0: AUXIO\\[8i+0\\]
is driven low. - If GPIODOUT bit 0 is 1: AUXIO\\[8i+0\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. When IOPOE bit 0 is 1: - If signal selected by IO0PSEL.SRC is 0: AUXIO\\[8i+0\\]
is driven low. - If signal selected by IO0PSEL.SRC is 1: AUXIO\\[8i+0\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == Io0::OpenDrain
    }
    #[doc = "Input Mode: When GPIODIE bit 0 is 0: AUXIO\\[8i+0\\]
is enabled for analog signal transfer. When GPIODIE bit 0 is 1: AUXIO\\[8i+0\\]
is enabled for digital input."]
    #[inline(always)]
    pub fn is_in(&self) -> bool {
        *self == Io0::In
    }
    #[doc = "Output Mode: When IOPOE bit 0 is 0: GPIODOUT bit 0 drives AUXIO\\[8i+0\\]. When IOPOE bit 0 is 1: The signal selected by IO0PSEL.SRC drives AUXIO\\[8i+0\\]."]
    #[inline(always)]
    pub fn is_out(&self) -> bool {
        *self == Io0::Out
    }
}
#[doc = "Field `IO0` writer - 1:0\\]
Select mode for AUXIO\\[8i+0\\]."]
pub type Io0W<'a, REG> = crate::FieldWriter<'a, REG, 2, Io0, crate::Safe>;
impl<'a, REG> Io0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Open-Source Mode: When IOPOE bit 0 is 0: - If GPIODOUT bit 0 is 0: AUXIO\\[8i+0\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. - If GPIODOUT bit 0 is 1: AUXIO\\[8i+0\\]
is driven high. When IOPOE bit 0 is 1: - If signal selected by IO0PSEL.SRC is 0: AUXIO\\[8i+0\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. - If signal selected by IO0PSEL.SRC is 1: AUXIO\\[8i+0\\]
is driven high."]
    #[inline(always)]
    pub fn open_source(self) -> &'a mut crate::W<REG> {
        self.variant(Io0::OpenSource)
    }
    #[doc = "Open-Drain Mode: When IOPOE bit 0 is 0: - If GPIODOUT bit 0 is 0: AUXIO\\[8i+0\\]
is driven low. - If GPIODOUT bit 0 is 1: AUXIO\\[8i+0\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. When IOPOE bit 0 is 1: - If signal selected by IO0PSEL.SRC is 0: AUXIO\\[8i+0\\]
is driven low. - If signal selected by IO0PSEL.SRC is 1: AUXIO\\[8i+0\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(Io0::OpenDrain)
    }
    #[doc = "Input Mode: When GPIODIE bit 0 is 0: AUXIO\\[8i+0\\]
is enabled for analog signal transfer. When GPIODIE bit 0 is 1: AUXIO\\[8i+0\\]
is enabled for digital input."]
    #[inline(always)]
    pub fn in_(self) -> &'a mut crate::W<REG> {
        self.variant(Io0::In)
    }
    #[doc = "Output Mode: When IOPOE bit 0 is 0: GPIODOUT bit 0 drives AUXIO\\[8i+0\\]. When IOPOE bit 0 is 1: The signal selected by IO0PSEL.SRC drives AUXIO\\[8i+0\\]."]
    #[inline(always)]
    pub fn out(self) -> &'a mut crate::W<REG> {
        self.variant(Io0::Out)
    }
}
#[doc = "3:2\\]
Select mode for AUXIO\\[8i+1\\].\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Io1 {
    #[doc = "3: Open-Source Mode: When IOPOE bit 1 is 0: - If GPIODOUT bit 1 is 0: AUXIO\\[8i+1\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. - If GPIODOUT bit 1 is 1: AUXIO\\[8i+1\\]
is driven high. When IOPOE bit 1 is 1: - If signal selected by IO1PSEL.SRC is 0: AUXIO\\[8i+1\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. - If signal selected by IO1PSEL.SRC is 1: AUXIO\\[8i+1\\]
is driven high."]
    OpenSource = 3,
    #[doc = "2: Open-Drain Mode: When IOPOE bit 1 is 0: - If GPIODOUT bit 1 is 0: AUXIO\\[8i+1\\]
is driven low. - If GPIODOUT bit 1 is 1: AUXIO\\[8i+1\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. When IOPOE bit 1 is 1: - If signal selected by IO1PSEL.SRC is 0: AUXIO\\[8i+1\\]
is driven low. - If signal selected by IO1PSEL.SRC is 1: AUXIO\\[8i+1\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL."]
    OpenDrain = 2,
    #[doc = "1: Input Mode: When GPIODIE bit 1 is 0: AUXIO\\[8i+1\\]
is enabled for analog signal transfer. When GPIODIE bit 1 is 1: AUXIO\\[8i+1\\]
is enabled for digital input."]
    In = 1,
    #[doc = "0: Output Mode: When IOPOE bit 1 is 0: GPIODOUT bit 1 drives AUXIO\\[8i+1\\]. When IOPOE bit 1 is 1: The signal selected by IO1PSEL.SRC drives AUXIO\\[8i+1\\]."]
    Out = 0,
}
impl From<Io1> for u8 {
    #[inline(always)]
    fn from(variant: Io1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Io1 {
    type Ux = u8;
}
impl crate::IsEnum for Io1 {}
#[doc = "Field `IO1` reader - 3:2\\]
Select mode for AUXIO\\[8i+1\\]."]
pub type Io1R = crate::FieldReader<Io1>;
impl Io1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Io1 {
        match self.bits {
            3 => Io1::OpenSource,
            2 => Io1::OpenDrain,
            1 => Io1::In,
            0 => Io1::Out,
            _ => unreachable!(),
        }
    }
    #[doc = "Open-Source Mode: When IOPOE bit 1 is 0: - If GPIODOUT bit 1 is 0: AUXIO\\[8i+1\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. - If GPIODOUT bit 1 is 1: AUXIO\\[8i+1\\]
is driven high. When IOPOE bit 1 is 1: - If signal selected by IO1PSEL.SRC is 0: AUXIO\\[8i+1\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. - If signal selected by IO1PSEL.SRC is 1: AUXIO\\[8i+1\\]
is driven high."]
    #[inline(always)]
    pub fn is_open_source(&self) -> bool {
        *self == Io1::OpenSource
    }
    #[doc = "Open-Drain Mode: When IOPOE bit 1 is 0: - If GPIODOUT bit 1 is 0: AUXIO\\[8i+1\\]
is driven low. - If GPIODOUT bit 1 is 1: AUXIO\\[8i+1\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. When IOPOE bit 1 is 1: - If signal selected by IO1PSEL.SRC is 0: AUXIO\\[8i+1\\]
is driven low. - If signal selected by IO1PSEL.SRC is 1: AUXIO\\[8i+1\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == Io1::OpenDrain
    }
    #[doc = "Input Mode: When GPIODIE bit 1 is 0: AUXIO\\[8i+1\\]
is enabled for analog signal transfer. When GPIODIE bit 1 is 1: AUXIO\\[8i+1\\]
is enabled for digital input."]
    #[inline(always)]
    pub fn is_in(&self) -> bool {
        *self == Io1::In
    }
    #[doc = "Output Mode: When IOPOE bit 1 is 0: GPIODOUT bit 1 drives AUXIO\\[8i+1\\]. When IOPOE bit 1 is 1: The signal selected by IO1PSEL.SRC drives AUXIO\\[8i+1\\]."]
    #[inline(always)]
    pub fn is_out(&self) -> bool {
        *self == Io1::Out
    }
}
#[doc = "Field `IO1` writer - 3:2\\]
Select mode for AUXIO\\[8i+1\\]."]
pub type Io1W<'a, REG> = crate::FieldWriter<'a, REG, 2, Io1, crate::Safe>;
impl<'a, REG> Io1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Open-Source Mode: When IOPOE bit 1 is 0: - If GPIODOUT bit 1 is 0: AUXIO\\[8i+1\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. - If GPIODOUT bit 1 is 1: AUXIO\\[8i+1\\]
is driven high. When IOPOE bit 1 is 1: - If signal selected by IO1PSEL.SRC is 0: AUXIO\\[8i+1\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. - If signal selected by IO1PSEL.SRC is 1: AUXIO\\[8i+1\\]
is driven high."]
    #[inline(always)]
    pub fn open_source(self) -> &'a mut crate::W<REG> {
        self.variant(Io1::OpenSource)
    }
    #[doc = "Open-Drain Mode: When IOPOE bit 1 is 0: - If GPIODOUT bit 1 is 0: AUXIO\\[8i+1\\]
is driven low. - If GPIODOUT bit 1 is 1: AUXIO\\[8i+1\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. When IOPOE bit 1 is 1: - If signal selected by IO1PSEL.SRC is 0: AUXIO\\[8i+1\\]
is driven low. - If signal selected by IO1PSEL.SRC is 1: AUXIO\\[8i+1\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(Io1::OpenDrain)
    }
    #[doc = "Input Mode: When GPIODIE bit 1 is 0: AUXIO\\[8i+1\\]
is enabled for analog signal transfer. When GPIODIE bit 1 is 1: AUXIO\\[8i+1\\]
is enabled for digital input."]
    #[inline(always)]
    pub fn in_(self) -> &'a mut crate::W<REG> {
        self.variant(Io1::In)
    }
    #[doc = "Output Mode: When IOPOE bit 1 is 0: GPIODOUT bit 1 drives AUXIO\\[8i+1\\]. When IOPOE bit 1 is 1: The signal selected by IO1PSEL.SRC drives AUXIO\\[8i+1\\]."]
    #[inline(always)]
    pub fn out(self) -> &'a mut crate::W<REG> {
        self.variant(Io1::Out)
    }
}
#[doc = "5:4\\]
Select mode for AUXIO\\[8i+2\\].\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Io2 {
    #[doc = "3: Open-Source Mode: When IOPOE bit 2 is 0: - If GPIODOUT bit 2 is 0: AUXIO\\[8i+2\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. - If GPIODOUT bit 2 is 1: AUXIO\\[8i+2\\]
is driven high. When IOPOE bit 2 is 1: - If signal selected by IO2PSEL.SRC is 0: AUXIO\\[8i+2\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. - If signal selected by IO2PSEL.SRC is 1: AUXIO\\[8i+2\\]
is driven high."]
    OpenSource = 3,
    #[doc = "2: Open-Drain Mode: When IOPOE bit 2 is 0: - If GPIODOUT bit 2 is 0: AUXIO\\[8i+2\\]
is driven low. - If GPIODOUT bit 2 is 1: AUXIO\\[8i+2\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. When IOPOE bit 2 is 1: - If signal selected by IO2PSEL.SRC is 0: AUXIO\\[8i+2\\]
is driven low. - If signal selected by IO2PSEL.SRC is 1: AUXIO\\[8i+2\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL."]
    OpenDrain = 2,
    #[doc = "1: Input Mode: When GPIODIE bit 2 is 0: AUXIO\\[8i+2\\]
is enabled for analog signal transfer. When GPIODIE bit 2 is 1: AUXIO\\[8i+2\\]
is enabled for digital input."]
    In = 1,
    #[doc = "0: Output Mode: When IOPOE bit 2 is 0: GPIODOUT bit 2 drives AUXIO\\[8i+2\\]. When IOPOE bit 2 is 1: The signal selected by IO2PSEL.SRC drives AUXIO\\[8i+2\\]."]
    Out = 0,
}
impl From<Io2> for u8 {
    #[inline(always)]
    fn from(variant: Io2) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Io2 {
    type Ux = u8;
}
impl crate::IsEnum for Io2 {}
#[doc = "Field `IO2` reader - 5:4\\]
Select mode for AUXIO\\[8i+2\\]."]
pub type Io2R = crate::FieldReader<Io2>;
impl Io2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Io2 {
        match self.bits {
            3 => Io2::OpenSource,
            2 => Io2::OpenDrain,
            1 => Io2::In,
            0 => Io2::Out,
            _ => unreachable!(),
        }
    }
    #[doc = "Open-Source Mode: When IOPOE bit 2 is 0: - If GPIODOUT bit 2 is 0: AUXIO\\[8i+2\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. - If GPIODOUT bit 2 is 1: AUXIO\\[8i+2\\]
is driven high. When IOPOE bit 2 is 1: - If signal selected by IO2PSEL.SRC is 0: AUXIO\\[8i+2\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. - If signal selected by IO2PSEL.SRC is 1: AUXIO\\[8i+2\\]
is driven high."]
    #[inline(always)]
    pub fn is_open_source(&self) -> bool {
        *self == Io2::OpenSource
    }
    #[doc = "Open-Drain Mode: When IOPOE bit 2 is 0: - If GPIODOUT bit 2 is 0: AUXIO\\[8i+2\\]
is driven low. - If GPIODOUT bit 2 is 1: AUXIO\\[8i+2\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. When IOPOE bit 2 is 1: - If signal selected by IO2PSEL.SRC is 0: AUXIO\\[8i+2\\]
is driven low. - If signal selected by IO2PSEL.SRC is 1: AUXIO\\[8i+2\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == Io2::OpenDrain
    }
    #[doc = "Input Mode: When GPIODIE bit 2 is 0: AUXIO\\[8i+2\\]
is enabled for analog signal transfer. When GPIODIE bit 2 is 1: AUXIO\\[8i+2\\]
is enabled for digital input."]
    #[inline(always)]
    pub fn is_in(&self) -> bool {
        *self == Io2::In
    }
    #[doc = "Output Mode: When IOPOE bit 2 is 0: GPIODOUT bit 2 drives AUXIO\\[8i+2\\]. When IOPOE bit 2 is 1: The signal selected by IO2PSEL.SRC drives AUXIO\\[8i+2\\]."]
    #[inline(always)]
    pub fn is_out(&self) -> bool {
        *self == Io2::Out
    }
}
#[doc = "Field `IO2` writer - 5:4\\]
Select mode for AUXIO\\[8i+2\\]."]
pub type Io2W<'a, REG> = crate::FieldWriter<'a, REG, 2, Io2, crate::Safe>;
impl<'a, REG> Io2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Open-Source Mode: When IOPOE bit 2 is 0: - If GPIODOUT bit 2 is 0: AUXIO\\[8i+2\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. - If GPIODOUT bit 2 is 1: AUXIO\\[8i+2\\]
is driven high. When IOPOE bit 2 is 1: - If signal selected by IO2PSEL.SRC is 0: AUXIO\\[8i+2\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. - If signal selected by IO2PSEL.SRC is 1: AUXIO\\[8i+2\\]
is driven high."]
    #[inline(always)]
    pub fn open_source(self) -> &'a mut crate::W<REG> {
        self.variant(Io2::OpenSource)
    }
    #[doc = "Open-Drain Mode: When IOPOE bit 2 is 0: - If GPIODOUT bit 2 is 0: AUXIO\\[8i+2\\]
is driven low. - If GPIODOUT bit 2 is 1: AUXIO\\[8i+2\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. When IOPOE bit 2 is 1: - If signal selected by IO2PSEL.SRC is 0: AUXIO\\[8i+2\\]
is driven low. - If signal selected by IO2PSEL.SRC is 1: AUXIO\\[8i+2\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(Io2::OpenDrain)
    }
    #[doc = "Input Mode: When GPIODIE bit 2 is 0: AUXIO\\[8i+2\\]
is enabled for analog signal transfer. When GPIODIE bit 2 is 1: AUXIO\\[8i+2\\]
is enabled for digital input."]
    #[inline(always)]
    pub fn in_(self) -> &'a mut crate::W<REG> {
        self.variant(Io2::In)
    }
    #[doc = "Output Mode: When IOPOE bit 2 is 0: GPIODOUT bit 2 drives AUXIO\\[8i+2\\]. When IOPOE bit 2 is 1: The signal selected by IO2PSEL.SRC drives AUXIO\\[8i+2\\]."]
    #[inline(always)]
    pub fn out(self) -> &'a mut crate::W<REG> {
        self.variant(Io2::Out)
    }
}
#[doc = "7:6\\]
Selects mode for AUXIO\\[8i+3\\].\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Io3 {
    #[doc = "3: Open-Source Mode: When IOPOE bit 3 is 0: - If GPIODOUT bit 3 is 0: AUXIO\\[8i+3\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. - If GPIODOUT bit 3 is 1: AUXIO\\[8i+3\\]
is driven high. When IOPOE bit 3 is 1: - If signal selected by IO3PSEL.SRC is 0: AUXIO\\[8i+3\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. - If signal selected by IO3PSEL.SRC is 1: AUXIO\\[8i+3\\]
is driven high."]
    OpenSource = 3,
    #[doc = "2: Open-Drain Mode: When IOPOE bit 3 is 0: - If GPIODOUT bit 3 is 0: AUXIO\\[8i+3\\]
is driven low. - If GPIODOUT bit 3 is 1: AUXIO\\[8i+3\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. When IOPOE bit 3 is 1: - If signal selected by IO3PSEL.SRC is 0: AUXIO\\[8i+3\\]
is driven low. - If signal selected by IO3PSEL.SRC is 1: AUXIO\\[8i+3\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL."]
    OpenDrain = 2,
    #[doc = "1: Input Mode: When GPIODIE bit 3 is 0: AUXIO\\[8i+3\\]
is enabled for analog signal transfer. When GPIODIE bit 3 is 1: AUXIO\\[8i+3\\]
is enabled for digital input."]
    In = 1,
    #[doc = "0: Output Mode: When IOPOE bit 3 is 0: GPIODOUT bit 3 drives AUXIO\\[8i+3\\]. When IOPOE bit 3 is 1: The signal selected by IO3PSEL.SRC drives AUXIO\\[8i+3\\]."]
    Out = 0,
}
impl From<Io3> for u8 {
    #[inline(always)]
    fn from(variant: Io3) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Io3 {
    type Ux = u8;
}
impl crate::IsEnum for Io3 {}
#[doc = "Field `IO3` reader - 7:6\\]
Selects mode for AUXIO\\[8i+3\\]."]
pub type Io3R = crate::FieldReader<Io3>;
impl Io3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Io3 {
        match self.bits {
            3 => Io3::OpenSource,
            2 => Io3::OpenDrain,
            1 => Io3::In,
            0 => Io3::Out,
            _ => unreachable!(),
        }
    }
    #[doc = "Open-Source Mode: When IOPOE bit 3 is 0: - If GPIODOUT bit 3 is 0: AUXIO\\[8i+3\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. - If GPIODOUT bit 3 is 1: AUXIO\\[8i+3\\]
is driven high. When IOPOE bit 3 is 1: - If signal selected by IO3PSEL.SRC is 0: AUXIO\\[8i+3\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. - If signal selected by IO3PSEL.SRC is 1: AUXIO\\[8i+3\\]
is driven high."]
    #[inline(always)]
    pub fn is_open_source(&self) -> bool {
        *self == Io3::OpenSource
    }
    #[doc = "Open-Drain Mode: When IOPOE bit 3 is 0: - If GPIODOUT bit 3 is 0: AUXIO\\[8i+3\\]
is driven low. - If GPIODOUT bit 3 is 1: AUXIO\\[8i+3\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. When IOPOE bit 3 is 1: - If signal selected by IO3PSEL.SRC is 0: AUXIO\\[8i+3\\]
is driven low. - If signal selected by IO3PSEL.SRC is 1: AUXIO\\[8i+3\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == Io3::OpenDrain
    }
    #[doc = "Input Mode: When GPIODIE bit 3 is 0: AUXIO\\[8i+3\\]
is enabled for analog signal transfer. When GPIODIE bit 3 is 1: AUXIO\\[8i+3\\]
is enabled for digital input."]
    #[inline(always)]
    pub fn is_in(&self) -> bool {
        *self == Io3::In
    }
    #[doc = "Output Mode: When IOPOE bit 3 is 0: GPIODOUT bit 3 drives AUXIO\\[8i+3\\]. When IOPOE bit 3 is 1: The signal selected by IO3PSEL.SRC drives AUXIO\\[8i+3\\]."]
    #[inline(always)]
    pub fn is_out(&self) -> bool {
        *self == Io3::Out
    }
}
#[doc = "Field `IO3` writer - 7:6\\]
Selects mode for AUXIO\\[8i+3\\]."]
pub type Io3W<'a, REG> = crate::FieldWriter<'a, REG, 2, Io3, crate::Safe>;
impl<'a, REG> Io3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Open-Source Mode: When IOPOE bit 3 is 0: - If GPIODOUT bit 3 is 0: AUXIO\\[8i+3\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. - If GPIODOUT bit 3 is 1: AUXIO\\[8i+3\\]
is driven high. When IOPOE bit 3 is 1: - If signal selected by IO3PSEL.SRC is 0: AUXIO\\[8i+3\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. - If signal selected by IO3PSEL.SRC is 1: AUXIO\\[8i+3\\]
is driven high."]
    #[inline(always)]
    pub fn open_source(self) -> &'a mut crate::W<REG> {
        self.variant(Io3::OpenSource)
    }
    #[doc = "Open-Drain Mode: When IOPOE bit 3 is 0: - If GPIODOUT bit 3 is 0: AUXIO\\[8i+3\\]
is driven low. - If GPIODOUT bit 3 is 1: AUXIO\\[8i+3\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. When IOPOE bit 3 is 1: - If signal selected by IO3PSEL.SRC is 0: AUXIO\\[8i+3\\]
is driven low. - If signal selected by IO3PSEL.SRC is 1: AUXIO\\[8i+3\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(Io3::OpenDrain)
    }
    #[doc = "Input Mode: When GPIODIE bit 3 is 0: AUXIO\\[8i+3\\]
is enabled for analog signal transfer. When GPIODIE bit 3 is 1: AUXIO\\[8i+3\\]
is enabled for digital input."]
    #[inline(always)]
    pub fn in_(self) -> &'a mut crate::W<REG> {
        self.variant(Io3::In)
    }
    #[doc = "Output Mode: When IOPOE bit 3 is 0: GPIODOUT bit 3 drives AUXIO\\[8i+3\\]. When IOPOE bit 3 is 1: The signal selected by IO3PSEL.SRC drives AUXIO\\[8i+3\\]."]
    #[inline(always)]
    pub fn out(self) -> &'a mut crate::W<REG> {
        self.variant(Io3::Out)
    }
}
#[doc = "9:8\\]
Selects mode for AUXIO\\[8i+4\\].\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Io4 {
    #[doc = "3: Open-Source Mode: When IOPOE bit 4 is 0: - If GPIODOUT bit 4 is 0: AUXIO\\[8i+4\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. - If GPIODOUT bit 4 is 1: AUXIO\\[8i+4\\]
is driven high. When IOPOE bit 4 is 1: - If signal selected by IO4PSEL.SRC is 0: AUXIO\\[8i+4\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. - If signal selected by IO4PSEL.SRC is 1: AUXIO\\[8i+4\\]
is driven high."]
    OpenSource = 3,
    #[doc = "2: Open-Drain Mode: When IOPOE bit 4 is 0: - If GPIODOUT bit 4 is 0: AUXIO\\[8i+4\\]
is driven low. - If GPIODOUT bit 4 is 1: AUXIO\\[8i+4\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. When IOPOE bit 4 is 1: - If signal selected by IO4PSEL.SRC is 0: AUXIO\\[8i+4\\]
is driven low. - If signal selected by IO4PSEL.SRC is 1: AUXIO\\[8i+4\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL."]
    OpenDrain = 2,
    #[doc = "1: Input Mode: When GPIODIE bit 4 is 0: AUXIO\\[8i+4\\]
is enabled for analog signal transfer. When GPIODIE bit 4 is 1: AUXIO\\[8i+4\\]
is enabled for digital input."]
    In = 1,
    #[doc = "0: Output Mode: When IOPOE bit 4 is 0: GPIODOUT bit 4 drives AUXIO\\[8i+4\\]. When IOPOE bit 4 is 1: The signal selected by IO4PSEL.SRC drives AUXIO\\[8i+4\\]."]
    Out = 0,
}
impl From<Io4> for u8 {
    #[inline(always)]
    fn from(variant: Io4) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Io4 {
    type Ux = u8;
}
impl crate::IsEnum for Io4 {}
#[doc = "Field `IO4` reader - 9:8\\]
Selects mode for AUXIO\\[8i+4\\]."]
pub type Io4R = crate::FieldReader<Io4>;
impl Io4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Io4 {
        match self.bits {
            3 => Io4::OpenSource,
            2 => Io4::OpenDrain,
            1 => Io4::In,
            0 => Io4::Out,
            _ => unreachable!(),
        }
    }
    #[doc = "Open-Source Mode: When IOPOE bit 4 is 0: - If GPIODOUT bit 4 is 0: AUXIO\\[8i+4\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. - If GPIODOUT bit 4 is 1: AUXIO\\[8i+4\\]
is driven high. When IOPOE bit 4 is 1: - If signal selected by IO4PSEL.SRC is 0: AUXIO\\[8i+4\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. - If signal selected by IO4PSEL.SRC is 1: AUXIO\\[8i+4\\]
is driven high."]
    #[inline(always)]
    pub fn is_open_source(&self) -> bool {
        *self == Io4::OpenSource
    }
    #[doc = "Open-Drain Mode: When IOPOE bit 4 is 0: - If GPIODOUT bit 4 is 0: AUXIO\\[8i+4\\]
is driven low. - If GPIODOUT bit 4 is 1: AUXIO\\[8i+4\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. When IOPOE bit 4 is 1: - If signal selected by IO4PSEL.SRC is 0: AUXIO\\[8i+4\\]
is driven low. - If signal selected by IO4PSEL.SRC is 1: AUXIO\\[8i+4\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == Io4::OpenDrain
    }
    #[doc = "Input Mode: When GPIODIE bit 4 is 0: AUXIO\\[8i+4\\]
is enabled for analog signal transfer. When GPIODIE bit 4 is 1: AUXIO\\[8i+4\\]
is enabled for digital input."]
    #[inline(always)]
    pub fn is_in(&self) -> bool {
        *self == Io4::In
    }
    #[doc = "Output Mode: When IOPOE bit 4 is 0: GPIODOUT bit 4 drives AUXIO\\[8i+4\\]. When IOPOE bit 4 is 1: The signal selected by IO4PSEL.SRC drives AUXIO\\[8i+4\\]."]
    #[inline(always)]
    pub fn is_out(&self) -> bool {
        *self == Io4::Out
    }
}
#[doc = "Field `IO4` writer - 9:8\\]
Selects mode for AUXIO\\[8i+4\\]."]
pub type Io4W<'a, REG> = crate::FieldWriter<'a, REG, 2, Io4, crate::Safe>;
impl<'a, REG> Io4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Open-Source Mode: When IOPOE bit 4 is 0: - If GPIODOUT bit 4 is 0: AUXIO\\[8i+4\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. - If GPIODOUT bit 4 is 1: AUXIO\\[8i+4\\]
is driven high. When IOPOE bit 4 is 1: - If signal selected by IO4PSEL.SRC is 0: AUXIO\\[8i+4\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. - If signal selected by IO4PSEL.SRC is 1: AUXIO\\[8i+4\\]
is driven high."]
    #[inline(always)]
    pub fn open_source(self) -> &'a mut crate::W<REG> {
        self.variant(Io4::OpenSource)
    }
    #[doc = "Open-Drain Mode: When IOPOE bit 4 is 0: - If GPIODOUT bit 4 is 0: AUXIO\\[8i+4\\]
is driven low. - If GPIODOUT bit 4 is 1: AUXIO\\[8i+4\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. When IOPOE bit 4 is 1: - If signal selected by IO4PSEL.SRC is 0: AUXIO\\[8i+4\\]
is driven low. - If signal selected by IO4PSEL.SRC is 1: AUXIO\\[8i+4\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(Io4::OpenDrain)
    }
    #[doc = "Input Mode: When GPIODIE bit 4 is 0: AUXIO\\[8i+4\\]
is enabled for analog signal transfer. When GPIODIE bit 4 is 1: AUXIO\\[8i+4\\]
is enabled for digital input."]
    #[inline(always)]
    pub fn in_(self) -> &'a mut crate::W<REG> {
        self.variant(Io4::In)
    }
    #[doc = "Output Mode: When IOPOE bit 4 is 0: GPIODOUT bit 4 drives AUXIO\\[8i+4\\]. When IOPOE bit 4 is 1: The signal selected by IO4PSEL.SRC drives AUXIO\\[8i+4\\]."]
    #[inline(always)]
    pub fn out(self) -> &'a mut crate::W<REG> {
        self.variant(Io4::Out)
    }
}
#[doc = "11:10\\]
Selects mode for AUXIO\\[8i+5\\].\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Io5 {
    #[doc = "3: Open-Source Mode: When IOPOE bit 5 is 0: - If GPIODOUT bit 5 is 0: AUXIO\\[8i+5\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. - If GPIODOUT bit 5 is 1: AUXIO\\[8i+5\\]
is driven high. When IOPOE bit 5 is 1: - If signal selected by IO5PSEL.SRC is 0: AUXIO\\[8i+5\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. - If signal selected by IO5PSEL.SRC is 1: AUXIO\\[8i+5\\]
is driven high."]
    OpenSource = 3,
    #[doc = "2: Open-Drain Mode: When IOPOE bit 5 is 0: - If GPIODOUT bit 5 is 0: AUXIO\\[8i+5\\]
is driven low. - If GPIODOUT bit 5 is 1: AUXIO\\[8i+5\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. When IOPOE bit 5 is 1: - If signal selected by IO5PSEL.SRC is 0: AUXIO\\[8i+5\\]
is driven low. - If signal selected by IO5PSEL.SRC is 1: AUXIO\\[8i+5\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL."]
    OpenDrain = 2,
    #[doc = "1: Input Mode: When GPIODIE bit 5 is 0: AUXIO\\[8i+5\\]
is enabled for analog signal transfer. When GPIODIE bit 5 is 1: AUXIO\\[8i+5\\]
is enabled for digital input."]
    In = 1,
    #[doc = "0: Output Mode: When IOPOE bit 5 is 0: GPIODOUT bit 5 drives AUXIO\\[8i+5\\]. When IOPOE bit 5 is 1: The signal selected by IO5PSEL.SRC drives AUXIO\\[8i+5\\]."]
    Out = 0,
}
impl From<Io5> for u8 {
    #[inline(always)]
    fn from(variant: Io5) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Io5 {
    type Ux = u8;
}
impl crate::IsEnum for Io5 {}
#[doc = "Field `IO5` reader - 11:10\\]
Selects mode for AUXIO\\[8i+5\\]."]
pub type Io5R = crate::FieldReader<Io5>;
impl Io5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Io5 {
        match self.bits {
            3 => Io5::OpenSource,
            2 => Io5::OpenDrain,
            1 => Io5::In,
            0 => Io5::Out,
            _ => unreachable!(),
        }
    }
    #[doc = "Open-Source Mode: When IOPOE bit 5 is 0: - If GPIODOUT bit 5 is 0: AUXIO\\[8i+5\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. - If GPIODOUT bit 5 is 1: AUXIO\\[8i+5\\]
is driven high. When IOPOE bit 5 is 1: - If signal selected by IO5PSEL.SRC is 0: AUXIO\\[8i+5\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. - If signal selected by IO5PSEL.SRC is 1: AUXIO\\[8i+5\\]
is driven high."]
    #[inline(always)]
    pub fn is_open_source(&self) -> bool {
        *self == Io5::OpenSource
    }
    #[doc = "Open-Drain Mode: When IOPOE bit 5 is 0: - If GPIODOUT bit 5 is 0: AUXIO\\[8i+5\\]
is driven low. - If GPIODOUT bit 5 is 1: AUXIO\\[8i+5\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. When IOPOE bit 5 is 1: - If signal selected by IO5PSEL.SRC is 0: AUXIO\\[8i+5\\]
is driven low. - If signal selected by IO5PSEL.SRC is 1: AUXIO\\[8i+5\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == Io5::OpenDrain
    }
    #[doc = "Input Mode: When GPIODIE bit 5 is 0: AUXIO\\[8i+5\\]
is enabled for analog signal transfer. When GPIODIE bit 5 is 1: AUXIO\\[8i+5\\]
is enabled for digital input."]
    #[inline(always)]
    pub fn is_in(&self) -> bool {
        *self == Io5::In
    }
    #[doc = "Output Mode: When IOPOE bit 5 is 0: GPIODOUT bit 5 drives AUXIO\\[8i+5\\]. When IOPOE bit 5 is 1: The signal selected by IO5PSEL.SRC drives AUXIO\\[8i+5\\]."]
    #[inline(always)]
    pub fn is_out(&self) -> bool {
        *self == Io5::Out
    }
}
#[doc = "Field `IO5` writer - 11:10\\]
Selects mode for AUXIO\\[8i+5\\]."]
pub type Io5W<'a, REG> = crate::FieldWriter<'a, REG, 2, Io5, crate::Safe>;
impl<'a, REG> Io5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Open-Source Mode: When IOPOE bit 5 is 0: - If GPIODOUT bit 5 is 0: AUXIO\\[8i+5\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. - If GPIODOUT bit 5 is 1: AUXIO\\[8i+5\\]
is driven high. When IOPOE bit 5 is 1: - If signal selected by IO5PSEL.SRC is 0: AUXIO\\[8i+5\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. - If signal selected by IO5PSEL.SRC is 1: AUXIO\\[8i+5\\]
is driven high."]
    #[inline(always)]
    pub fn open_source(self) -> &'a mut crate::W<REG> {
        self.variant(Io5::OpenSource)
    }
    #[doc = "Open-Drain Mode: When IOPOE bit 5 is 0: - If GPIODOUT bit 5 is 0: AUXIO\\[8i+5\\]
is driven low. - If GPIODOUT bit 5 is 1: AUXIO\\[8i+5\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. When IOPOE bit 5 is 1: - If signal selected by IO5PSEL.SRC is 0: AUXIO\\[8i+5\\]
is driven low. - If signal selected by IO5PSEL.SRC is 1: AUXIO\\[8i+5\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(Io5::OpenDrain)
    }
    #[doc = "Input Mode: When GPIODIE bit 5 is 0: AUXIO\\[8i+5\\]
is enabled for analog signal transfer. When GPIODIE bit 5 is 1: AUXIO\\[8i+5\\]
is enabled for digital input."]
    #[inline(always)]
    pub fn in_(self) -> &'a mut crate::W<REG> {
        self.variant(Io5::In)
    }
    #[doc = "Output Mode: When IOPOE bit 5 is 0: GPIODOUT bit 5 drives AUXIO\\[8i+5\\]. When IOPOE bit 5 is 1: The signal selected by IO5PSEL.SRC drives AUXIO\\[8i+5\\]."]
    #[inline(always)]
    pub fn out(self) -> &'a mut crate::W<REG> {
        self.variant(Io5::Out)
    }
}
#[doc = "13:12\\]
Selects mode for AUXIO\\[8i+6\\].\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Io6 {
    #[doc = "3: Open-Source Mode: When IOPOE bit 6 is 0: - If GPIODOUT bit 6 is 0: AUXIO\\[8i+6\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. - If GPIODOUT bit 6 is 1: AUXIO\\[8i+6\\]
is driven high. When IOPOE bit 6 is 1: - If signal selected by IO6PSEL.SRC is 0: AUXIO\\[8i+6\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. - If signal selected by IO6PSEL.SRC is 1: AUXIO\\[8i+6\\]
is driven high."]
    OpenSource = 3,
    #[doc = "2: Open-Drain Mode: When IOPOE bit 6 is 0: - If GPIODOUT bit 6 is 0: AUXIO\\[8i+6\\]
is driven low. - If GPIODOUT bit 6 is 1: AUXIO\\[8i+6\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. When IOPOE bit 6 is 1: - If signal selected by IO6PSEL.SRC is 0: AUXIO\\[8i+6\\]
is driven low. - If signal selected by IO6PSEL.SRC is 1: AUXIO\\[8i+6\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL."]
    OpenDrain = 2,
    #[doc = "1: Input Mode: When GPIODIE bit 6 is 0: AUXIO\\[8i+6\\]
is enabled for analog signal transfer. When GPIODIE bit 6 is 1: AUXIO\\[8i+6\\]
is enabled for digital input."]
    In = 1,
    #[doc = "0: Output Mode: When IOPOE bit 6 is 0: GPIODOUT bit 6 drives AUXIO\\[8i+6\\]. When IOPOE bit 6 is 1: The signal selected by IO6PSEL.SRC drives AUXIO\\[8i+6\\]."]
    Out = 0,
}
impl From<Io6> for u8 {
    #[inline(always)]
    fn from(variant: Io6) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Io6 {
    type Ux = u8;
}
impl crate::IsEnum for Io6 {}
#[doc = "Field `IO6` reader - 13:12\\]
Selects mode for AUXIO\\[8i+6\\]."]
pub type Io6R = crate::FieldReader<Io6>;
impl Io6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Io6 {
        match self.bits {
            3 => Io6::OpenSource,
            2 => Io6::OpenDrain,
            1 => Io6::In,
            0 => Io6::Out,
            _ => unreachable!(),
        }
    }
    #[doc = "Open-Source Mode: When IOPOE bit 6 is 0: - If GPIODOUT bit 6 is 0: AUXIO\\[8i+6\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. - If GPIODOUT bit 6 is 1: AUXIO\\[8i+6\\]
is driven high. When IOPOE bit 6 is 1: - If signal selected by IO6PSEL.SRC is 0: AUXIO\\[8i+6\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. - If signal selected by IO6PSEL.SRC is 1: AUXIO\\[8i+6\\]
is driven high."]
    #[inline(always)]
    pub fn is_open_source(&self) -> bool {
        *self == Io6::OpenSource
    }
    #[doc = "Open-Drain Mode: When IOPOE bit 6 is 0: - If GPIODOUT bit 6 is 0: AUXIO\\[8i+6\\]
is driven low. - If GPIODOUT bit 6 is 1: AUXIO\\[8i+6\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. When IOPOE bit 6 is 1: - If signal selected by IO6PSEL.SRC is 0: AUXIO\\[8i+6\\]
is driven low. - If signal selected by IO6PSEL.SRC is 1: AUXIO\\[8i+6\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == Io6::OpenDrain
    }
    #[doc = "Input Mode: When GPIODIE bit 6 is 0: AUXIO\\[8i+6\\]
is enabled for analog signal transfer. When GPIODIE bit 6 is 1: AUXIO\\[8i+6\\]
is enabled for digital input."]
    #[inline(always)]
    pub fn is_in(&self) -> bool {
        *self == Io6::In
    }
    #[doc = "Output Mode: When IOPOE bit 6 is 0: GPIODOUT bit 6 drives AUXIO\\[8i+6\\]. When IOPOE bit 6 is 1: The signal selected by IO6PSEL.SRC drives AUXIO\\[8i+6\\]."]
    #[inline(always)]
    pub fn is_out(&self) -> bool {
        *self == Io6::Out
    }
}
#[doc = "Field `IO6` writer - 13:12\\]
Selects mode for AUXIO\\[8i+6\\]."]
pub type Io6W<'a, REG> = crate::FieldWriter<'a, REG, 2, Io6, crate::Safe>;
impl<'a, REG> Io6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Open-Source Mode: When IOPOE bit 6 is 0: - If GPIODOUT bit 6 is 0: AUXIO\\[8i+6\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. - If GPIODOUT bit 6 is 1: AUXIO\\[8i+6\\]
is driven high. When IOPOE bit 6 is 1: - If signal selected by IO6PSEL.SRC is 0: AUXIO\\[8i+6\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. - If signal selected by IO6PSEL.SRC is 1: AUXIO\\[8i+6\\]
is driven high."]
    #[inline(always)]
    pub fn open_source(self) -> &'a mut crate::W<REG> {
        self.variant(Io6::OpenSource)
    }
    #[doc = "Open-Drain Mode: When IOPOE bit 6 is 0: - If GPIODOUT bit 6 is 0: AUXIO\\[8i+6\\]
is driven low. - If GPIODOUT bit 6 is 1: AUXIO\\[8i+6\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. When IOPOE bit 6 is 1: - If signal selected by IO6PSEL.SRC is 0: AUXIO\\[8i+6\\]
is driven low. - If signal selected by IO6PSEL.SRC is 1: AUXIO\\[8i+6\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(Io6::OpenDrain)
    }
    #[doc = "Input Mode: When GPIODIE bit 6 is 0: AUXIO\\[8i+6\\]
is enabled for analog signal transfer. When GPIODIE bit 6 is 1: AUXIO\\[8i+6\\]
is enabled for digital input."]
    #[inline(always)]
    pub fn in_(self) -> &'a mut crate::W<REG> {
        self.variant(Io6::In)
    }
    #[doc = "Output Mode: When IOPOE bit 6 is 0: GPIODOUT bit 6 drives AUXIO\\[8i+6\\]. When IOPOE bit 6 is 1: The signal selected by IO6PSEL.SRC drives AUXIO\\[8i+6\\]."]
    #[inline(always)]
    pub fn out(self) -> &'a mut crate::W<REG> {
        self.variant(Io6::Out)
    }
}
#[doc = "15:14\\]
Selects mode for AUXIO\\[8i+7\\].\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Io7 {
    #[doc = "3: Open-Source Mode: When IOPOE bit 7 is 0: - If GPIODOUT bit 7 is 0: AUXIO\\[8i+7\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. - If GPIODOUT bit 7 is 1: AUXIO\\[8i+7\\]
is driven high. When IOPOE bit 7 is 1: - If signal selected by IO7PSEL.SRC is 0: AUXIO\\[8i+7\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. - If signal selected by IO7PSEL.SRC is 1: AUXIO\\[8i+7\\]
is driven high."]
    OpenSource = 3,
    #[doc = "2: Open-Drain Mode: When IOPOE bit 7 is 0: - If GPIODOUT bit 7 is 0: AUXIO\\[8i+7\\]
is driven low. - If GPIODOUT bit 7 is 1: AUXIO\\[8i+7\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. When IOPOE bit 7 is 1: - If signal selected by IO7PSEL.SRC is 0: AUXIO\\[8i+7\\]
is driven low. - If signal selected by IO7PSEL.SRC is 1: AUXIO\\[8i+7\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL."]
    OpenDrain = 2,
    #[doc = "1: Input Mode: When GPIODIE bit 7 is 0: AUXIO\\[8i+7\\]
is enabled for analog signal transfer. When GPIODIE bit 7 is 1: AUXIO\\[8i+7\\]
is enabled for digital input."]
    In = 1,
    #[doc = "0: Output Mode: When IOPOE bit 7 is 0: GPIODOUT bit 7 drives AUXIO\\[8i+7\\]. When IOPOE bit 7 is 1: The signal selected by IO7PSEL.SRC drives AUXIO\\[8i+7\\]."]
    Out = 0,
}
impl From<Io7> for u8 {
    #[inline(always)]
    fn from(variant: Io7) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Io7 {
    type Ux = u8;
}
impl crate::IsEnum for Io7 {}
#[doc = "Field `IO7` reader - 15:14\\]
Selects mode for AUXIO\\[8i+7\\]."]
pub type Io7R = crate::FieldReader<Io7>;
impl Io7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Io7 {
        match self.bits {
            3 => Io7::OpenSource,
            2 => Io7::OpenDrain,
            1 => Io7::In,
            0 => Io7::Out,
            _ => unreachable!(),
        }
    }
    #[doc = "Open-Source Mode: When IOPOE bit 7 is 0: - If GPIODOUT bit 7 is 0: AUXIO\\[8i+7\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. - If GPIODOUT bit 7 is 1: AUXIO\\[8i+7\\]
is driven high. When IOPOE bit 7 is 1: - If signal selected by IO7PSEL.SRC is 0: AUXIO\\[8i+7\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. - If signal selected by IO7PSEL.SRC is 1: AUXIO\\[8i+7\\]
is driven high."]
    #[inline(always)]
    pub fn is_open_source(&self) -> bool {
        *self == Io7::OpenSource
    }
    #[doc = "Open-Drain Mode: When IOPOE bit 7 is 0: - If GPIODOUT bit 7 is 0: AUXIO\\[8i+7\\]
is driven low. - If GPIODOUT bit 7 is 1: AUXIO\\[8i+7\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. When IOPOE bit 7 is 1: - If signal selected by IO7PSEL.SRC is 0: AUXIO\\[8i+7\\]
is driven low. - If signal selected by IO7PSEL.SRC is 1: AUXIO\\[8i+7\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == Io7::OpenDrain
    }
    #[doc = "Input Mode: When GPIODIE bit 7 is 0: AUXIO\\[8i+7\\]
is enabled for analog signal transfer. When GPIODIE bit 7 is 1: AUXIO\\[8i+7\\]
is enabled for digital input."]
    #[inline(always)]
    pub fn is_in(&self) -> bool {
        *self == Io7::In
    }
    #[doc = "Output Mode: When IOPOE bit 7 is 0: GPIODOUT bit 7 drives AUXIO\\[8i+7\\]. When IOPOE bit 7 is 1: The signal selected by IO7PSEL.SRC drives AUXIO\\[8i+7\\]."]
    #[inline(always)]
    pub fn is_out(&self) -> bool {
        *self == Io7::Out
    }
}
#[doc = "Field `IO7` writer - 15:14\\]
Selects mode for AUXIO\\[8i+7\\]."]
pub type Io7W<'a, REG> = crate::FieldWriter<'a, REG, 2, Io7, crate::Safe>;
impl<'a, REG> Io7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Open-Source Mode: When IOPOE bit 7 is 0: - If GPIODOUT bit 7 is 0: AUXIO\\[8i+7\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. - If GPIODOUT bit 7 is 1: AUXIO\\[8i+7\\]
is driven high. When IOPOE bit 7 is 1: - If signal selected by IO7PSEL.SRC is 0: AUXIO\\[8i+7\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. - If signal selected by IO7PSEL.SRC is 1: AUXIO\\[8i+7\\]
is driven high."]
    #[inline(always)]
    pub fn open_source(self) -> &'a mut crate::W<REG> {
        self.variant(Io7::OpenSource)
    }
    #[doc = "Open-Drain Mode: When IOPOE bit 7 is 0: - If GPIODOUT bit 7 is 0: AUXIO\\[8i+7\\]
is driven low. - If GPIODOUT bit 7 is 1: AUXIO\\[8i+7\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. When IOPOE bit 7 is 1: - If signal selected by IO7PSEL.SRC is 0: AUXIO\\[8i+7\\]
is driven low. - If signal selected by IO7PSEL.SRC is 1: AUXIO\\[8i+7\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(Io7::OpenDrain)
    }
    #[doc = "Input Mode: When GPIODIE bit 7 is 0: AUXIO\\[8i+7\\]
is enabled for analog signal transfer. When GPIODIE bit 7 is 1: AUXIO\\[8i+7\\]
is enabled for digital input."]
    #[inline(always)]
    pub fn in_(self) -> &'a mut crate::W<REG> {
        self.variant(Io7::In)
    }
    #[doc = "Output Mode: When IOPOE bit 7 is 0: GPIODOUT bit 7 drives AUXIO\\[8i+7\\]. When IOPOE bit 7 is 1: The signal selected by IO7PSEL.SRC drives AUXIO\\[8i+7\\]."]
    #[inline(always)]
    pub fn out(self) -> &'a mut crate::W<REG> {
        self.variant(Io7::Out)
    }
}
#[doc = "Field `RESERVED16` reader - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved16R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Select mode for AUXIO\\[8i+0\\]."]
    #[inline(always)]
    pub fn io0(&self) -> Io0R {
        Io0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - 3:2\\]
Select mode for AUXIO\\[8i+1\\]."]
    #[inline(always)]
    pub fn io1(&self) -> Io1R {
        Io1R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - 5:4\\]
Select mode for AUXIO\\[8i+2\\]."]
    #[inline(always)]
    pub fn io2(&self) -> Io2R {
        Io2R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Selects mode for AUXIO\\[8i+3\\]."]
    #[inline(always)]
    pub fn io3(&self) -> Io3R {
        Io3R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Selects mode for AUXIO\\[8i+4\\]."]
    #[inline(always)]
    pub fn io4(&self) -> Io4R {
        Io4R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - 11:10\\]
Selects mode for AUXIO\\[8i+5\\]."]
    #[inline(always)]
    pub fn io5(&self) -> Io5R {
        Io5R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - 13:12\\]
Selects mode for AUXIO\\[8i+6\\]."]
    #[inline(always)]
    pub fn io6(&self) -> Io6R {
        Io6R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - 15:14\\]
Selects mode for AUXIO\\[8i+7\\]."]
    #[inline(always)]
    pub fn io7(&self) -> Io7R {
        Io7R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved16(&self) -> Reserved16R {
        Reserved16R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Select mode for AUXIO\\[8i+0\\]."]
    #[inline(always)]
    #[must_use]
    pub fn io0(&mut self) -> Io0W<IomodeSpec> {
        Io0W::new(self, 0)
    }
    #[doc = "Bits 2:3 - 3:2\\]
Select mode for AUXIO\\[8i+1\\]."]
    #[inline(always)]
    #[must_use]
    pub fn io1(&mut self) -> Io1W<IomodeSpec> {
        Io1W::new(self, 2)
    }
    #[doc = "Bits 4:5 - 5:4\\]
Select mode for AUXIO\\[8i+2\\]."]
    #[inline(always)]
    #[must_use]
    pub fn io2(&mut self) -> Io2W<IomodeSpec> {
        Io2W::new(self, 4)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Selects mode for AUXIO\\[8i+3\\]."]
    #[inline(always)]
    #[must_use]
    pub fn io3(&mut self) -> Io3W<IomodeSpec> {
        Io3W::new(self, 6)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Selects mode for AUXIO\\[8i+4\\]."]
    #[inline(always)]
    #[must_use]
    pub fn io4(&mut self) -> Io4W<IomodeSpec> {
        Io4W::new(self, 8)
    }
    #[doc = "Bits 10:11 - 11:10\\]
Selects mode for AUXIO\\[8i+5\\]."]
    #[inline(always)]
    #[must_use]
    pub fn io5(&mut self) -> Io5W<IomodeSpec> {
        Io5W::new(self, 10)
    }
    #[doc = "Bits 12:13 - 13:12\\]
Selects mode for AUXIO\\[8i+6\\]."]
    #[inline(always)]
    #[must_use]
    pub fn io6(&mut self) -> Io6W<IomodeSpec> {
        Io6W::new(self, 12)
    }
    #[doc = "Bits 14:15 - 15:14\\]
Selects mode for AUXIO\\[8i+7\\]."]
    #[inline(always)]
    #[must_use]
    pub fn io7(&mut self) -> Io7W<IomodeSpec> {
        Io7W::new(self, 14)
    }
}
#[doc = "Input Output Mode This register controls pull-up, pull-down, and output mode for AUXIO that are controlled by instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iomode::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iomode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IomodeSpec;
impl crate::RegisterSpec for IomodeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iomode::R`](R) reader structure"]
impl crate::Readable for IomodeSpec {}
#[doc = "`write(|w| ..)` method takes [`iomode::W`](W) writer structure"]
impl crate::Writable for IomodeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IOMODE to value 0"]
impl crate::Resettable for IomodeSpec {
    const RESET_VALUE: u32 = 0;
}
