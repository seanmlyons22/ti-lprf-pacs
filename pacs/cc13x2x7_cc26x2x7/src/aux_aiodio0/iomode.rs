#[doc = "Register `IOMODE` reader"]
pub struct R(crate::R<IOMODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IOMODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IOMODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IOMODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IOMODE` writer"]
pub struct W(crate::W<IOMODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IOMODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<IOMODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IOMODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IO0` reader - 1:0\\]
Select mode for AUXIO\\[8i+0\\]."]
pub type IO0_R = crate::FieldReader<u8, IO0_A>;
#[doc = "1:0\\]
Select mode for AUXIO\\[8i+0\\].\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IO0_A {
    #[doc = "3: Open-Source Mode: When IOPOE bit 0 is 0: - If GPIODOUT bit 0 is 0: AUXIO\\[8i+0\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. - If GPIODOUT bit 0 is 1: AUXIO\\[8i+0\\]
is driven high. When IOPOE bit 0 is 1: - If signal selected by IO0PSEL.SRC is 0: AUXIO\\[8i+0\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. - If signal selected by IO0PSEL.SRC is 1: AUXIO\\[8i+0\\]
is driven high."]
    OPEN_SOURCE = 3,
    #[doc = "2: Open-Drain Mode: When IOPOE bit 0 is 0: - If GPIODOUT bit 0 is 0: AUXIO\\[8i+0\\]
is driven low. - If GPIODOUT bit 0 is 1: AUXIO\\[8i+0\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. When IOPOE bit 0 is 1: - If signal selected by IO0PSEL.SRC is 0: AUXIO\\[8i+0\\]
is driven low. - If signal selected by IO0PSEL.SRC is 1: AUXIO\\[8i+0\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL."]
    OPEN_DRAIN = 2,
    #[doc = "1: Input Mode: When GPIODIE bit 0 is 0: AUXIO\\[8i+0\\]
is enabled for analog signal transfer. When GPIODIE bit 0 is 1: AUXIO\\[8i+0\\]
is enabled for digital input."]
    IN = 1,
    #[doc = "0: Output Mode: When IOPOE bit 0 is 0: GPIODOUT bit 0 drives AUXIO\\[8i+0\\]. When IOPOE bit 0 is 1: The signal selected by IO0PSEL.SRC drives AUXIO\\[8i+0\\]."]
    OUT = 0,
}
impl From<IO0_A> for u8 {
    #[inline(always)]
    fn from(variant: IO0_A) -> Self {
        variant as _
    }
}
impl IO0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IO0_A {
        match self.bits {
            3 => IO0_A::OPEN_SOURCE,
            2 => IO0_A::OPEN_DRAIN,
            1 => IO0_A::IN,
            0 => IO0_A::OUT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OPEN_SOURCE`"]
    #[inline(always)]
    pub fn is_open_source(&self) -> bool {
        *self == IO0_A::OPEN_SOURCE
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == IO0_A::OPEN_DRAIN
    }
    #[doc = "Checks if the value of the field is `IN`"]
    #[inline(always)]
    pub fn is_in(&self) -> bool {
        *self == IO0_A::IN
    }
    #[doc = "Checks if the value of the field is `OUT`"]
    #[inline(always)]
    pub fn is_out(&self) -> bool {
        *self == IO0_A::OUT
    }
}
#[doc = "Field `IO0` writer - 1:0\\]
Select mode for AUXIO\\[8i+0\\]."]
pub type IO0_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, IOMODE_SPEC, u8, IO0_A, 2, O>;
impl<'a, const O: u8> IO0_W<'a, O> {
    #[doc = "Open-Source Mode: When IOPOE bit 0 is 0: - If GPIODOUT bit 0 is 0: AUXIO\\[8i+0\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. - If GPIODOUT bit 0 is 1: AUXIO\\[8i+0\\]
is driven high. When IOPOE bit 0 is 1: - If signal selected by IO0PSEL.SRC is 0: AUXIO\\[8i+0\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. - If signal selected by IO0PSEL.SRC is 1: AUXIO\\[8i+0\\]
is driven high."]
    #[inline(always)]
    pub fn open_source(self) -> &'a mut W {
        self.variant(IO0_A::OPEN_SOURCE)
    }
    #[doc = "Open-Drain Mode: When IOPOE bit 0 is 0: - If GPIODOUT bit 0 is 0: AUXIO\\[8i+0\\]
is driven low. - If GPIODOUT bit 0 is 1: AUXIO\\[8i+0\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. When IOPOE bit 0 is 1: - If signal selected by IO0PSEL.SRC is 0: AUXIO\\[8i+0\\]
is driven low. - If signal selected by IO0PSEL.SRC is 1: AUXIO\\[8i+0\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(IO0_A::OPEN_DRAIN)
    }
    #[doc = "Input Mode: When GPIODIE bit 0 is 0: AUXIO\\[8i+0\\]
is enabled for analog signal transfer. When GPIODIE bit 0 is 1: AUXIO\\[8i+0\\]
is enabled for digital input."]
    #[inline(always)]
    pub fn in_(self) -> &'a mut W {
        self.variant(IO0_A::IN)
    }
    #[doc = "Output Mode: When IOPOE bit 0 is 0: GPIODOUT bit 0 drives AUXIO\\[8i+0\\]. When IOPOE bit 0 is 1: The signal selected by IO0PSEL.SRC drives AUXIO\\[8i+0\\]."]
    #[inline(always)]
    pub fn out(self) -> &'a mut W {
        self.variant(IO0_A::OUT)
    }
}
#[doc = "Field `IO1` reader - 3:2\\]
Select mode for AUXIO\\[8i+1\\]."]
pub type IO1_R = crate::FieldReader<u8, IO1_A>;
#[doc = "3:2\\]
Select mode for AUXIO\\[8i+1\\].\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IO1_A {
    #[doc = "3: Open-Source Mode: When IOPOE bit 1 is 0: - If GPIODOUT bit 1 is 0: AUXIO\\[8i+1\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. - If GPIODOUT bit 1 is 1: AUXIO\\[8i+1\\]
is driven high. When IOPOE bit 1 is 1: - If signal selected by IO1PSEL.SRC is 0: AUXIO\\[8i+1\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. - If signal selected by IO1PSEL.SRC is 1: AUXIO\\[8i+1\\]
is driven high."]
    OPEN_SOURCE = 3,
    #[doc = "2: Open-Drain Mode: When IOPOE bit 1 is 0: - If GPIODOUT bit 1 is 0: AUXIO\\[8i+1\\]
is driven low. - If GPIODOUT bit 1 is 1: AUXIO\\[8i+1\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. When IOPOE bit 1 is 1: - If signal selected by IO1PSEL.SRC is 0: AUXIO\\[8i+1\\]
is driven low. - If signal selected by IO1PSEL.SRC is 1: AUXIO\\[8i+1\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL."]
    OPEN_DRAIN = 2,
    #[doc = "1: Input Mode: When GPIODIE bit 1 is 0: AUXIO\\[8i+1\\]
is enabled for analog signal transfer. When GPIODIE bit 1 is 1: AUXIO\\[8i+1\\]
is enabled for digital input."]
    IN = 1,
    #[doc = "0: Output Mode: When IOPOE bit 1 is 0: GPIODOUT bit 1 drives AUXIO\\[8i+1\\]. When IOPOE bit 1 is 1: The signal selected by IO1PSEL.SRC drives AUXIO\\[8i+1\\]."]
    OUT = 0,
}
impl From<IO1_A> for u8 {
    #[inline(always)]
    fn from(variant: IO1_A) -> Self {
        variant as _
    }
}
impl IO1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IO1_A {
        match self.bits {
            3 => IO1_A::OPEN_SOURCE,
            2 => IO1_A::OPEN_DRAIN,
            1 => IO1_A::IN,
            0 => IO1_A::OUT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OPEN_SOURCE`"]
    #[inline(always)]
    pub fn is_open_source(&self) -> bool {
        *self == IO1_A::OPEN_SOURCE
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == IO1_A::OPEN_DRAIN
    }
    #[doc = "Checks if the value of the field is `IN`"]
    #[inline(always)]
    pub fn is_in(&self) -> bool {
        *self == IO1_A::IN
    }
    #[doc = "Checks if the value of the field is `OUT`"]
    #[inline(always)]
    pub fn is_out(&self) -> bool {
        *self == IO1_A::OUT
    }
}
#[doc = "Field `IO1` writer - 3:2\\]
Select mode for AUXIO\\[8i+1\\]."]
pub type IO1_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, IOMODE_SPEC, u8, IO1_A, 2, O>;
impl<'a, const O: u8> IO1_W<'a, O> {
    #[doc = "Open-Source Mode: When IOPOE bit 1 is 0: - If GPIODOUT bit 1 is 0: AUXIO\\[8i+1\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. - If GPIODOUT bit 1 is 1: AUXIO\\[8i+1\\]
is driven high. When IOPOE bit 1 is 1: - If signal selected by IO1PSEL.SRC is 0: AUXIO\\[8i+1\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. - If signal selected by IO1PSEL.SRC is 1: AUXIO\\[8i+1\\]
is driven high."]
    #[inline(always)]
    pub fn open_source(self) -> &'a mut W {
        self.variant(IO1_A::OPEN_SOURCE)
    }
    #[doc = "Open-Drain Mode: When IOPOE bit 1 is 0: - If GPIODOUT bit 1 is 0: AUXIO\\[8i+1\\]
is driven low. - If GPIODOUT bit 1 is 1: AUXIO\\[8i+1\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. When IOPOE bit 1 is 1: - If signal selected by IO1PSEL.SRC is 0: AUXIO\\[8i+1\\]
is driven low. - If signal selected by IO1PSEL.SRC is 1: AUXIO\\[8i+1\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(IO1_A::OPEN_DRAIN)
    }
    #[doc = "Input Mode: When GPIODIE bit 1 is 0: AUXIO\\[8i+1\\]
is enabled for analog signal transfer. When GPIODIE bit 1 is 1: AUXIO\\[8i+1\\]
is enabled for digital input."]
    #[inline(always)]
    pub fn in_(self) -> &'a mut W {
        self.variant(IO1_A::IN)
    }
    #[doc = "Output Mode: When IOPOE bit 1 is 0: GPIODOUT bit 1 drives AUXIO\\[8i+1\\]. When IOPOE bit 1 is 1: The signal selected by IO1PSEL.SRC drives AUXIO\\[8i+1\\]."]
    #[inline(always)]
    pub fn out(self) -> &'a mut W {
        self.variant(IO1_A::OUT)
    }
}
#[doc = "Field `IO2` reader - 5:4\\]
Select mode for AUXIO\\[8i+2\\]."]
pub type IO2_R = crate::FieldReader<u8, IO2_A>;
#[doc = "5:4\\]
Select mode for AUXIO\\[8i+2\\].\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IO2_A {
    #[doc = "3: Open-Source Mode: When IOPOE bit 2 is 0: - If GPIODOUT bit 2 is 0: AUXIO\\[8i+2\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. - If GPIODOUT bit 2 is 1: AUXIO\\[8i+2\\]
is driven high. When IOPOE bit 2 is 1: - If signal selected by IO2PSEL.SRC is 0: AUXIO\\[8i+2\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. - If signal selected by IO2PSEL.SRC is 1: AUXIO\\[8i+2\\]
is driven high."]
    OPEN_SOURCE = 3,
    #[doc = "2: Open-Drain Mode: When IOPOE bit 2 is 0: - If GPIODOUT bit 2 is 0: AUXIO\\[8i+2\\]
is driven low. - If GPIODOUT bit 2 is 1: AUXIO\\[8i+2\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. When IOPOE bit 2 is 1: - If signal selected by IO2PSEL.SRC is 0: AUXIO\\[8i+2\\]
is driven low. - If signal selected by IO2PSEL.SRC is 1: AUXIO\\[8i+2\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL."]
    OPEN_DRAIN = 2,
    #[doc = "1: Input Mode: When GPIODIE bit 2 is 0: AUXIO\\[8i+2\\]
is enabled for analog signal transfer. When GPIODIE bit 2 is 1: AUXIO\\[8i+2\\]
is enabled for digital input."]
    IN = 1,
    #[doc = "0: Output Mode: When IOPOE bit 2 is 0: GPIODOUT bit 2 drives AUXIO\\[8i+2\\]. When IOPOE bit 2 is 1: The signal selected by IO2PSEL.SRC drives AUXIO\\[8i+2\\]."]
    OUT = 0,
}
impl From<IO2_A> for u8 {
    #[inline(always)]
    fn from(variant: IO2_A) -> Self {
        variant as _
    }
}
impl IO2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IO2_A {
        match self.bits {
            3 => IO2_A::OPEN_SOURCE,
            2 => IO2_A::OPEN_DRAIN,
            1 => IO2_A::IN,
            0 => IO2_A::OUT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OPEN_SOURCE`"]
    #[inline(always)]
    pub fn is_open_source(&self) -> bool {
        *self == IO2_A::OPEN_SOURCE
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == IO2_A::OPEN_DRAIN
    }
    #[doc = "Checks if the value of the field is `IN`"]
    #[inline(always)]
    pub fn is_in(&self) -> bool {
        *self == IO2_A::IN
    }
    #[doc = "Checks if the value of the field is `OUT`"]
    #[inline(always)]
    pub fn is_out(&self) -> bool {
        *self == IO2_A::OUT
    }
}
#[doc = "Field `IO2` writer - 5:4\\]
Select mode for AUXIO\\[8i+2\\]."]
pub type IO2_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, IOMODE_SPEC, u8, IO2_A, 2, O>;
impl<'a, const O: u8> IO2_W<'a, O> {
    #[doc = "Open-Source Mode: When IOPOE bit 2 is 0: - If GPIODOUT bit 2 is 0: AUXIO\\[8i+2\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. - If GPIODOUT bit 2 is 1: AUXIO\\[8i+2\\]
is driven high. When IOPOE bit 2 is 1: - If signal selected by IO2PSEL.SRC is 0: AUXIO\\[8i+2\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. - If signal selected by IO2PSEL.SRC is 1: AUXIO\\[8i+2\\]
is driven high."]
    #[inline(always)]
    pub fn open_source(self) -> &'a mut W {
        self.variant(IO2_A::OPEN_SOURCE)
    }
    #[doc = "Open-Drain Mode: When IOPOE bit 2 is 0: - If GPIODOUT bit 2 is 0: AUXIO\\[8i+2\\]
is driven low. - If GPIODOUT bit 2 is 1: AUXIO\\[8i+2\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. When IOPOE bit 2 is 1: - If signal selected by IO2PSEL.SRC is 0: AUXIO\\[8i+2\\]
is driven low. - If signal selected by IO2PSEL.SRC is 1: AUXIO\\[8i+2\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(IO2_A::OPEN_DRAIN)
    }
    #[doc = "Input Mode: When GPIODIE bit 2 is 0: AUXIO\\[8i+2\\]
is enabled for analog signal transfer. When GPIODIE bit 2 is 1: AUXIO\\[8i+2\\]
is enabled for digital input."]
    #[inline(always)]
    pub fn in_(self) -> &'a mut W {
        self.variant(IO2_A::IN)
    }
    #[doc = "Output Mode: When IOPOE bit 2 is 0: GPIODOUT bit 2 drives AUXIO\\[8i+2\\]. When IOPOE bit 2 is 1: The signal selected by IO2PSEL.SRC drives AUXIO\\[8i+2\\]."]
    #[inline(always)]
    pub fn out(self) -> &'a mut W {
        self.variant(IO2_A::OUT)
    }
}
#[doc = "Field `IO3` reader - 7:6\\]
Selects mode for AUXIO\\[8i+3\\]."]
pub type IO3_R = crate::FieldReader<u8, IO3_A>;
#[doc = "7:6\\]
Selects mode for AUXIO\\[8i+3\\].\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IO3_A {
    #[doc = "3: Open-Source Mode: When IOPOE bit 3 is 0: - If GPIODOUT bit 3 is 0: AUXIO\\[8i+3\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. - If GPIODOUT bit 3 is 1: AUXIO\\[8i+3\\]
is driven high. When IOPOE bit 3 is 1: - If signal selected by IO3PSEL.SRC is 0: AUXIO\\[8i+3\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. - If signal selected by IO3PSEL.SRC is 1: AUXIO\\[8i+3\\]
is driven high."]
    OPEN_SOURCE = 3,
    #[doc = "2: Open-Drain Mode: When IOPOE bit 3 is 0: - If GPIODOUT bit 3 is 0: AUXIO\\[8i+3\\]
is driven low. - If GPIODOUT bit 3 is 1: AUXIO\\[8i+3\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. When IOPOE bit 3 is 1: - If signal selected by IO3PSEL.SRC is 0: AUXIO\\[8i+3\\]
is driven low. - If signal selected by IO3PSEL.SRC is 1: AUXIO\\[8i+3\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL."]
    OPEN_DRAIN = 2,
    #[doc = "1: Input Mode: When GPIODIE bit 3 is 0: AUXIO\\[8i+3\\]
is enabled for analog signal transfer. When GPIODIE bit 3 is 1: AUXIO\\[8i+3\\]
is enabled for digital input."]
    IN = 1,
    #[doc = "0: Output Mode: When IOPOE bit 3 is 0: GPIODOUT bit 3 drives AUXIO\\[8i+3\\]. When IOPOE bit 3 is 1: The signal selected by IO3PSEL.SRC drives AUXIO\\[8i+3\\]."]
    OUT = 0,
}
impl From<IO3_A> for u8 {
    #[inline(always)]
    fn from(variant: IO3_A) -> Self {
        variant as _
    }
}
impl IO3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IO3_A {
        match self.bits {
            3 => IO3_A::OPEN_SOURCE,
            2 => IO3_A::OPEN_DRAIN,
            1 => IO3_A::IN,
            0 => IO3_A::OUT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OPEN_SOURCE`"]
    #[inline(always)]
    pub fn is_open_source(&self) -> bool {
        *self == IO3_A::OPEN_SOURCE
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == IO3_A::OPEN_DRAIN
    }
    #[doc = "Checks if the value of the field is `IN`"]
    #[inline(always)]
    pub fn is_in(&self) -> bool {
        *self == IO3_A::IN
    }
    #[doc = "Checks if the value of the field is `OUT`"]
    #[inline(always)]
    pub fn is_out(&self) -> bool {
        *self == IO3_A::OUT
    }
}
#[doc = "Field `IO3` writer - 7:6\\]
Selects mode for AUXIO\\[8i+3\\]."]
pub type IO3_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, IOMODE_SPEC, u8, IO3_A, 2, O>;
impl<'a, const O: u8> IO3_W<'a, O> {
    #[doc = "Open-Source Mode: When IOPOE bit 3 is 0: - If GPIODOUT bit 3 is 0: AUXIO\\[8i+3\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. - If GPIODOUT bit 3 is 1: AUXIO\\[8i+3\\]
is driven high. When IOPOE bit 3 is 1: - If signal selected by IO3PSEL.SRC is 0: AUXIO\\[8i+3\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. - If signal selected by IO3PSEL.SRC is 1: AUXIO\\[8i+3\\]
is driven high."]
    #[inline(always)]
    pub fn open_source(self) -> &'a mut W {
        self.variant(IO3_A::OPEN_SOURCE)
    }
    #[doc = "Open-Drain Mode: When IOPOE bit 3 is 0: - If GPIODOUT bit 3 is 0: AUXIO\\[8i+3\\]
is driven low. - If GPIODOUT bit 3 is 1: AUXIO\\[8i+3\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. When IOPOE bit 3 is 1: - If signal selected by IO3PSEL.SRC is 0: AUXIO\\[8i+3\\]
is driven low. - If signal selected by IO3PSEL.SRC is 1: AUXIO\\[8i+3\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(IO3_A::OPEN_DRAIN)
    }
    #[doc = "Input Mode: When GPIODIE bit 3 is 0: AUXIO\\[8i+3\\]
is enabled for analog signal transfer. When GPIODIE bit 3 is 1: AUXIO\\[8i+3\\]
is enabled for digital input."]
    #[inline(always)]
    pub fn in_(self) -> &'a mut W {
        self.variant(IO3_A::IN)
    }
    #[doc = "Output Mode: When IOPOE bit 3 is 0: GPIODOUT bit 3 drives AUXIO\\[8i+3\\]. When IOPOE bit 3 is 1: The signal selected by IO3PSEL.SRC drives AUXIO\\[8i+3\\]."]
    #[inline(always)]
    pub fn out(self) -> &'a mut W {
        self.variant(IO3_A::OUT)
    }
}
#[doc = "Field `IO4` reader - 9:8\\]
Selects mode for AUXIO\\[8i+4\\]."]
pub type IO4_R = crate::FieldReader<u8, IO4_A>;
#[doc = "9:8\\]
Selects mode for AUXIO\\[8i+4\\].\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IO4_A {
    #[doc = "3: Open-Source Mode: When IOPOE bit 4 is 0: - If GPIODOUT bit 4 is 0: AUXIO\\[8i+4\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. - If GPIODOUT bit 4 is 1: AUXIO\\[8i+4\\]
is driven high. When IOPOE bit 4 is 1: - If signal selected by IO4PSEL.SRC is 0: AUXIO\\[8i+4\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. - If signal selected by IO4PSEL.SRC is 1: AUXIO\\[8i+4\\]
is driven high."]
    OPEN_SOURCE = 3,
    #[doc = "2: Open-Drain Mode: When IOPOE bit 4 is 0: - If GPIODOUT bit 4 is 0: AUXIO\\[8i+4\\]
is driven low. - If GPIODOUT bit 4 is 1: AUXIO\\[8i+4\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. When IOPOE bit 4 is 1: - If signal selected by IO4PSEL.SRC is 0: AUXIO\\[8i+4\\]
is driven low. - If signal selected by IO4PSEL.SRC is 1: AUXIO\\[8i+4\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL."]
    OPEN_DRAIN = 2,
    #[doc = "1: Input Mode: When GPIODIE bit 4 is 0: AUXIO\\[8i+4\\]
is enabled for analog signal transfer. When GPIODIE bit 4 is 1: AUXIO\\[8i+4\\]
is enabled for digital input."]
    IN = 1,
    #[doc = "0: Output Mode: When IOPOE bit 4 is 0: GPIODOUT bit 4 drives AUXIO\\[8i+4\\]. When IOPOE bit 4 is 1: The signal selected by IO4PSEL.SRC drives AUXIO\\[8i+4\\]."]
    OUT = 0,
}
impl From<IO4_A> for u8 {
    #[inline(always)]
    fn from(variant: IO4_A) -> Self {
        variant as _
    }
}
impl IO4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IO4_A {
        match self.bits {
            3 => IO4_A::OPEN_SOURCE,
            2 => IO4_A::OPEN_DRAIN,
            1 => IO4_A::IN,
            0 => IO4_A::OUT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OPEN_SOURCE`"]
    #[inline(always)]
    pub fn is_open_source(&self) -> bool {
        *self == IO4_A::OPEN_SOURCE
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == IO4_A::OPEN_DRAIN
    }
    #[doc = "Checks if the value of the field is `IN`"]
    #[inline(always)]
    pub fn is_in(&self) -> bool {
        *self == IO4_A::IN
    }
    #[doc = "Checks if the value of the field is `OUT`"]
    #[inline(always)]
    pub fn is_out(&self) -> bool {
        *self == IO4_A::OUT
    }
}
#[doc = "Field `IO4` writer - 9:8\\]
Selects mode for AUXIO\\[8i+4\\]."]
pub type IO4_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, IOMODE_SPEC, u8, IO4_A, 2, O>;
impl<'a, const O: u8> IO4_W<'a, O> {
    #[doc = "Open-Source Mode: When IOPOE bit 4 is 0: - If GPIODOUT bit 4 is 0: AUXIO\\[8i+4\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. - If GPIODOUT bit 4 is 1: AUXIO\\[8i+4\\]
is driven high. When IOPOE bit 4 is 1: - If signal selected by IO4PSEL.SRC is 0: AUXIO\\[8i+4\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. - If signal selected by IO4PSEL.SRC is 1: AUXIO\\[8i+4\\]
is driven high."]
    #[inline(always)]
    pub fn open_source(self) -> &'a mut W {
        self.variant(IO4_A::OPEN_SOURCE)
    }
    #[doc = "Open-Drain Mode: When IOPOE bit 4 is 0: - If GPIODOUT bit 4 is 0: AUXIO\\[8i+4\\]
is driven low. - If GPIODOUT bit 4 is 1: AUXIO\\[8i+4\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. When IOPOE bit 4 is 1: - If signal selected by IO4PSEL.SRC is 0: AUXIO\\[8i+4\\]
is driven low. - If signal selected by IO4PSEL.SRC is 1: AUXIO\\[8i+4\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(IO4_A::OPEN_DRAIN)
    }
    #[doc = "Input Mode: When GPIODIE bit 4 is 0: AUXIO\\[8i+4\\]
is enabled for analog signal transfer. When GPIODIE bit 4 is 1: AUXIO\\[8i+4\\]
is enabled for digital input."]
    #[inline(always)]
    pub fn in_(self) -> &'a mut W {
        self.variant(IO4_A::IN)
    }
    #[doc = "Output Mode: When IOPOE bit 4 is 0: GPIODOUT bit 4 drives AUXIO\\[8i+4\\]. When IOPOE bit 4 is 1: The signal selected by IO4PSEL.SRC drives AUXIO\\[8i+4\\]."]
    #[inline(always)]
    pub fn out(self) -> &'a mut W {
        self.variant(IO4_A::OUT)
    }
}
#[doc = "Field `IO5` reader - 11:10\\]
Selects mode for AUXIO\\[8i+5\\]."]
pub type IO5_R = crate::FieldReader<u8, IO5_A>;
#[doc = "11:10\\]
Selects mode for AUXIO\\[8i+5\\].\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IO5_A {
    #[doc = "3: Open-Source Mode: When IOPOE bit 5 is 0: - If GPIODOUT bit 5 is 0: AUXIO\\[8i+5\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. - If GPIODOUT bit 5 is 1: AUXIO\\[8i+5\\]
is driven high. When IOPOE bit 5 is 1: - If signal selected by IO5PSEL.SRC is 0: AUXIO\\[8i+5\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. - If signal selected by IO5PSEL.SRC is 1: AUXIO\\[8i+5\\]
is driven high."]
    OPEN_SOURCE = 3,
    #[doc = "2: Open-Drain Mode: When IOPOE bit 5 is 0: - If GPIODOUT bit 5 is 0: AUXIO\\[8i+5\\]
is driven low. - If GPIODOUT bit 5 is 1: AUXIO\\[8i+5\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. When IOPOE bit 5 is 1: - If signal selected by IO5PSEL.SRC is 0: AUXIO\\[8i+5\\]
is driven low. - If signal selected by IO5PSEL.SRC is 1: AUXIO\\[8i+5\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL."]
    OPEN_DRAIN = 2,
    #[doc = "1: Input Mode: When GPIODIE bit 5 is 0: AUXIO\\[8i+5\\]
is enabled for analog signal transfer. When GPIODIE bit 5 is 1: AUXIO\\[8i+5\\]
is enabled for digital input."]
    IN = 1,
    #[doc = "0: Output Mode: When IOPOE bit 5 is 0: GPIODOUT bit 5 drives AUXIO\\[8i+5\\]. When IOPOE bit 5 is 1: The signal selected by IO5PSEL.SRC drives AUXIO\\[8i+5\\]."]
    OUT = 0,
}
impl From<IO5_A> for u8 {
    #[inline(always)]
    fn from(variant: IO5_A) -> Self {
        variant as _
    }
}
impl IO5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IO5_A {
        match self.bits {
            3 => IO5_A::OPEN_SOURCE,
            2 => IO5_A::OPEN_DRAIN,
            1 => IO5_A::IN,
            0 => IO5_A::OUT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OPEN_SOURCE`"]
    #[inline(always)]
    pub fn is_open_source(&self) -> bool {
        *self == IO5_A::OPEN_SOURCE
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == IO5_A::OPEN_DRAIN
    }
    #[doc = "Checks if the value of the field is `IN`"]
    #[inline(always)]
    pub fn is_in(&self) -> bool {
        *self == IO5_A::IN
    }
    #[doc = "Checks if the value of the field is `OUT`"]
    #[inline(always)]
    pub fn is_out(&self) -> bool {
        *self == IO5_A::OUT
    }
}
#[doc = "Field `IO5` writer - 11:10\\]
Selects mode for AUXIO\\[8i+5\\]."]
pub type IO5_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, IOMODE_SPEC, u8, IO5_A, 2, O>;
impl<'a, const O: u8> IO5_W<'a, O> {
    #[doc = "Open-Source Mode: When IOPOE bit 5 is 0: - If GPIODOUT bit 5 is 0: AUXIO\\[8i+5\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. - If GPIODOUT bit 5 is 1: AUXIO\\[8i+5\\]
is driven high. When IOPOE bit 5 is 1: - If signal selected by IO5PSEL.SRC is 0: AUXIO\\[8i+5\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. - If signal selected by IO5PSEL.SRC is 1: AUXIO\\[8i+5\\]
is driven high."]
    #[inline(always)]
    pub fn open_source(self) -> &'a mut W {
        self.variant(IO5_A::OPEN_SOURCE)
    }
    #[doc = "Open-Drain Mode: When IOPOE bit 5 is 0: - If GPIODOUT bit 5 is 0: AUXIO\\[8i+5\\]
is driven low. - If GPIODOUT bit 5 is 1: AUXIO\\[8i+5\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. When IOPOE bit 5 is 1: - If signal selected by IO5PSEL.SRC is 0: AUXIO\\[8i+5\\]
is driven low. - If signal selected by IO5PSEL.SRC is 1: AUXIO\\[8i+5\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(IO5_A::OPEN_DRAIN)
    }
    #[doc = "Input Mode: When GPIODIE bit 5 is 0: AUXIO\\[8i+5\\]
is enabled for analog signal transfer. When GPIODIE bit 5 is 1: AUXIO\\[8i+5\\]
is enabled for digital input."]
    #[inline(always)]
    pub fn in_(self) -> &'a mut W {
        self.variant(IO5_A::IN)
    }
    #[doc = "Output Mode: When IOPOE bit 5 is 0: GPIODOUT bit 5 drives AUXIO\\[8i+5\\]. When IOPOE bit 5 is 1: The signal selected by IO5PSEL.SRC drives AUXIO\\[8i+5\\]."]
    #[inline(always)]
    pub fn out(self) -> &'a mut W {
        self.variant(IO5_A::OUT)
    }
}
#[doc = "Field `IO6` reader - 13:12\\]
Selects mode for AUXIO\\[8i+6\\]."]
pub type IO6_R = crate::FieldReader<u8, IO6_A>;
#[doc = "13:12\\]
Selects mode for AUXIO\\[8i+6\\].\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IO6_A {
    #[doc = "3: Open-Source Mode: When IOPOE bit 6 is 0: - If GPIODOUT bit 6 is 0: AUXIO\\[8i+6\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. - If GPIODOUT bit 6 is 1: AUXIO\\[8i+6\\]
is driven high. When IOPOE bit 6 is 1: - If signal selected by IO6PSEL.SRC is 0: AUXIO\\[8i+6\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. - If signal selected by IO6PSEL.SRC is 1: AUXIO\\[8i+6\\]
is driven high."]
    OPEN_SOURCE = 3,
    #[doc = "2: Open-Drain Mode: When IOPOE bit 6 is 0: - If GPIODOUT bit 6 is 0: AUXIO\\[8i+6\\]
is driven low. - If GPIODOUT bit 6 is 1: AUXIO\\[8i+6\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. When IOPOE bit 6 is 1: - If signal selected by IO6PSEL.SRC is 0: AUXIO\\[8i+6\\]
is driven low. - If signal selected by IO6PSEL.SRC is 1: AUXIO\\[8i+6\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL."]
    OPEN_DRAIN = 2,
    #[doc = "1: Input Mode: When GPIODIE bit 6 is 0: AUXIO\\[8i+6\\]
is enabled for analog signal transfer. When GPIODIE bit 6 is 1: AUXIO\\[8i+6\\]
is enabled for digital input."]
    IN = 1,
    #[doc = "0: Output Mode: When IOPOE bit 6 is 0: GPIODOUT bit 6 drives AUXIO\\[8i+6\\]. When IOPOE bit 6 is 1: The signal selected by IO6PSEL.SRC drives AUXIO\\[8i+6\\]."]
    OUT = 0,
}
impl From<IO6_A> for u8 {
    #[inline(always)]
    fn from(variant: IO6_A) -> Self {
        variant as _
    }
}
impl IO6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IO6_A {
        match self.bits {
            3 => IO6_A::OPEN_SOURCE,
            2 => IO6_A::OPEN_DRAIN,
            1 => IO6_A::IN,
            0 => IO6_A::OUT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OPEN_SOURCE`"]
    #[inline(always)]
    pub fn is_open_source(&self) -> bool {
        *self == IO6_A::OPEN_SOURCE
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == IO6_A::OPEN_DRAIN
    }
    #[doc = "Checks if the value of the field is `IN`"]
    #[inline(always)]
    pub fn is_in(&self) -> bool {
        *self == IO6_A::IN
    }
    #[doc = "Checks if the value of the field is `OUT`"]
    #[inline(always)]
    pub fn is_out(&self) -> bool {
        *self == IO6_A::OUT
    }
}
#[doc = "Field `IO6` writer - 13:12\\]
Selects mode for AUXIO\\[8i+6\\]."]
pub type IO6_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, IOMODE_SPEC, u8, IO6_A, 2, O>;
impl<'a, const O: u8> IO6_W<'a, O> {
    #[doc = "Open-Source Mode: When IOPOE bit 6 is 0: - If GPIODOUT bit 6 is 0: AUXIO\\[8i+6\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. - If GPIODOUT bit 6 is 1: AUXIO\\[8i+6\\]
is driven high. When IOPOE bit 6 is 1: - If signal selected by IO6PSEL.SRC is 0: AUXIO\\[8i+6\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. - If signal selected by IO6PSEL.SRC is 1: AUXIO\\[8i+6\\]
is driven high."]
    #[inline(always)]
    pub fn open_source(self) -> &'a mut W {
        self.variant(IO6_A::OPEN_SOURCE)
    }
    #[doc = "Open-Drain Mode: When IOPOE bit 6 is 0: - If GPIODOUT bit 6 is 0: AUXIO\\[8i+6\\]
is driven low. - If GPIODOUT bit 6 is 1: AUXIO\\[8i+6\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. When IOPOE bit 6 is 1: - If signal selected by IO6PSEL.SRC is 0: AUXIO\\[8i+6\\]
is driven low. - If signal selected by IO6PSEL.SRC is 1: AUXIO\\[8i+6\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(IO6_A::OPEN_DRAIN)
    }
    #[doc = "Input Mode: When GPIODIE bit 6 is 0: AUXIO\\[8i+6\\]
is enabled for analog signal transfer. When GPIODIE bit 6 is 1: AUXIO\\[8i+6\\]
is enabled for digital input."]
    #[inline(always)]
    pub fn in_(self) -> &'a mut W {
        self.variant(IO6_A::IN)
    }
    #[doc = "Output Mode: When IOPOE bit 6 is 0: GPIODOUT bit 6 drives AUXIO\\[8i+6\\]. When IOPOE bit 6 is 1: The signal selected by IO6PSEL.SRC drives AUXIO\\[8i+6\\]."]
    #[inline(always)]
    pub fn out(self) -> &'a mut W {
        self.variant(IO6_A::OUT)
    }
}
#[doc = "Field `IO7` reader - 15:14\\]
Selects mode for AUXIO\\[8i+7\\]."]
pub type IO7_R = crate::FieldReader<u8, IO7_A>;
#[doc = "15:14\\]
Selects mode for AUXIO\\[8i+7\\].\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IO7_A {
    #[doc = "3: Open-Source Mode: When IOPOE bit 7 is 0: - If GPIODOUT bit 7 is 0: AUXIO\\[8i+7\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. - If GPIODOUT bit 7 is 1: AUXIO\\[8i+7\\]
is driven high. When IOPOE bit 7 is 1: - If signal selected by IO7PSEL.SRC is 0: AUXIO\\[8i+7\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. - If signal selected by IO7PSEL.SRC is 1: AUXIO\\[8i+7\\]
is driven high."]
    OPEN_SOURCE = 3,
    #[doc = "2: Open-Drain Mode: When IOPOE bit 7 is 0: - If GPIODOUT bit 7 is 0: AUXIO\\[8i+7\\]
is driven low. - If GPIODOUT bit 7 is 1: AUXIO\\[8i+7\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. When IOPOE bit 7 is 1: - If signal selected by IO7PSEL.SRC is 0: AUXIO\\[8i+7\\]
is driven low. - If signal selected by IO7PSEL.SRC is 1: AUXIO\\[8i+7\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL."]
    OPEN_DRAIN = 2,
    #[doc = "1: Input Mode: When GPIODIE bit 7 is 0: AUXIO\\[8i+7\\]
is enabled for analog signal transfer. When GPIODIE bit 7 is 1: AUXIO\\[8i+7\\]
is enabled for digital input."]
    IN = 1,
    #[doc = "0: Output Mode: When IOPOE bit 7 is 0: GPIODOUT bit 7 drives AUXIO\\[8i+7\\]. When IOPOE bit 7 is 1: The signal selected by IO7PSEL.SRC drives AUXIO\\[8i+7\\]."]
    OUT = 0,
}
impl From<IO7_A> for u8 {
    #[inline(always)]
    fn from(variant: IO7_A) -> Self {
        variant as _
    }
}
impl IO7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IO7_A {
        match self.bits {
            3 => IO7_A::OPEN_SOURCE,
            2 => IO7_A::OPEN_DRAIN,
            1 => IO7_A::IN,
            0 => IO7_A::OUT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OPEN_SOURCE`"]
    #[inline(always)]
    pub fn is_open_source(&self) -> bool {
        *self == IO7_A::OPEN_SOURCE
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == IO7_A::OPEN_DRAIN
    }
    #[doc = "Checks if the value of the field is `IN`"]
    #[inline(always)]
    pub fn is_in(&self) -> bool {
        *self == IO7_A::IN
    }
    #[doc = "Checks if the value of the field is `OUT`"]
    #[inline(always)]
    pub fn is_out(&self) -> bool {
        *self == IO7_A::OUT
    }
}
#[doc = "Field `IO7` writer - 15:14\\]
Selects mode for AUXIO\\[8i+7\\]."]
pub type IO7_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, IOMODE_SPEC, u8, IO7_A, 2, O>;
impl<'a, const O: u8> IO7_W<'a, O> {
    #[doc = "Open-Source Mode: When IOPOE bit 7 is 0: - If GPIODOUT bit 7 is 0: AUXIO\\[8i+7\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. - If GPIODOUT bit 7 is 1: AUXIO\\[8i+7\\]
is driven high. When IOPOE bit 7 is 1: - If signal selected by IO7PSEL.SRC is 0: AUXIO\\[8i+7\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. - If signal selected by IO7PSEL.SRC is 1: AUXIO\\[8i+7\\]
is driven high."]
    #[inline(always)]
    pub fn open_source(self) -> &'a mut W {
        self.variant(IO7_A::OPEN_SOURCE)
    }
    #[doc = "Open-Drain Mode: When IOPOE bit 7 is 0: - If GPIODOUT bit 7 is 0: AUXIO\\[8i+7\\]
is driven low. - If GPIODOUT bit 7 is 1: AUXIO\\[8i+7\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. When IOPOE bit 7 is 1: - If signal selected by IO7PSEL.SRC is 0: AUXIO\\[8i+7\\]
is driven low. - If signal selected by IO7PSEL.SRC is 1: AUXIO\\[8i+7\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(IO7_A::OPEN_DRAIN)
    }
    #[doc = "Input Mode: When GPIODIE bit 7 is 0: AUXIO\\[8i+7\\]
is enabled for analog signal transfer. When GPIODIE bit 7 is 1: AUXIO\\[8i+7\\]
is enabled for digital input."]
    #[inline(always)]
    pub fn in_(self) -> &'a mut W {
        self.variant(IO7_A::IN)
    }
    #[doc = "Output Mode: When IOPOE bit 7 is 0: GPIODOUT bit 7 drives AUXIO\\[8i+7\\]. When IOPOE bit 7 is 1: The signal selected by IO7PSEL.SRC drives AUXIO\\[8i+7\\]."]
    #[inline(always)]
    pub fn out(self) -> &'a mut W {
        self.variant(IO7_A::OUT)
    }
}
#[doc = "Field `RESERVED16` reader - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED16_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RESERVED16` writer - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED16_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IOMODE_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Select mode for AUXIO\\[8i+0\\]."]
    #[inline(always)]
    pub fn io0(&self) -> IO0_R {
        IO0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - 3:2\\]
Select mode for AUXIO\\[8i+1\\]."]
    #[inline(always)]
    pub fn io1(&self) -> IO1_R {
        IO1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - 5:4\\]
Select mode for AUXIO\\[8i+2\\]."]
    #[inline(always)]
    pub fn io2(&self) -> IO2_R {
        IO2_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Selects mode for AUXIO\\[8i+3\\]."]
    #[inline(always)]
    pub fn io3(&self) -> IO3_R {
        IO3_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Selects mode for AUXIO\\[8i+4\\]."]
    #[inline(always)]
    pub fn io4(&self) -> IO4_R {
        IO4_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - 11:10\\]
Selects mode for AUXIO\\[8i+5\\]."]
    #[inline(always)]
    pub fn io5(&self) -> IO5_R {
        IO5_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - 13:12\\]
Selects mode for AUXIO\\[8i+6\\]."]
    #[inline(always)]
    pub fn io6(&self) -> IO6_R {
        IO6_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - 15:14\\]
Selects mode for AUXIO\\[8i+7\\]."]
    #[inline(always)]
    pub fn io7(&self) -> IO7_R {
        IO7_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved16(&self) -> RESERVED16_R {
        RESERVED16_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Select mode for AUXIO\\[8i+0\\]."]
    #[inline(always)]
    #[must_use]
    pub fn io0(&mut self) -> IO0_W<0> {
        IO0_W::new(self)
    }
    #[doc = "Bits 2:3 - 3:2\\]
Select mode for AUXIO\\[8i+1\\]."]
    #[inline(always)]
    #[must_use]
    pub fn io1(&mut self) -> IO1_W<2> {
        IO1_W::new(self)
    }
    #[doc = "Bits 4:5 - 5:4\\]
Select mode for AUXIO\\[8i+2\\]."]
    #[inline(always)]
    #[must_use]
    pub fn io2(&mut self) -> IO2_W<4> {
        IO2_W::new(self)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Selects mode for AUXIO\\[8i+3\\]."]
    #[inline(always)]
    #[must_use]
    pub fn io3(&mut self) -> IO3_W<6> {
        IO3_W::new(self)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Selects mode for AUXIO\\[8i+4\\]."]
    #[inline(always)]
    #[must_use]
    pub fn io4(&mut self) -> IO4_W<8> {
        IO4_W::new(self)
    }
    #[doc = "Bits 10:11 - 11:10\\]
Selects mode for AUXIO\\[8i+5\\]."]
    #[inline(always)]
    #[must_use]
    pub fn io5(&mut self) -> IO5_W<10> {
        IO5_W::new(self)
    }
    #[doc = "Bits 12:13 - 13:12\\]
Selects mode for AUXIO\\[8i+6\\]."]
    #[inline(always)]
    #[must_use]
    pub fn io6(&mut self) -> IO6_W<12> {
        IO6_W::new(self)
    }
    #[doc = "Bits 14:15 - 15:14\\]
Selects mode for AUXIO\\[8i+7\\]."]
    #[inline(always)]
    #[must_use]
    pub fn io7(&mut self) -> IO7_W<14> {
        IO7_W::new(self)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved16(&mut self) -> RESERVED16_W<16> {
        RESERVED16_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Input Output Mode This register controls pull-up, pull-down, and output mode for AUXIO that are controlled by instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iomode](index.html) module"]
pub struct IOMODE_SPEC;
impl crate::RegisterSpec for IOMODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iomode::R](R) reader structure"]
impl crate::Readable for IOMODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iomode::W](W) writer structure"]
impl crate::Writable for IOMODE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IOMODE to value 0"]
impl crate::Resettable for IOMODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
