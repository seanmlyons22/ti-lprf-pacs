#[doc = "Register `CTL` reader"]
pub type R = crate::R<CtlSpec>;
#[doc = "Register `CTL` writer"]
pub type W = crate::W<CtlSpec>;
#[doc = "0:0\\]
GPT Timer A Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Taen {
    #[doc = "1: Timer A is enabled and begins counting or the capture logic is enabled based on the CFG register."]
    En = 1,
    #[doc = "0: Timer A is disabled."]
    Dis = 0,
}
impl From<Taen> for bool {
    #[inline(always)]
    fn from(variant: Taen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TAEN` reader - 0:0\\]
GPT Timer A Enable"]
pub type TaenR = crate::BitReader<Taen>;
impl TaenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Taen {
        match self.bits {
            true => Taen::En,
            false => Taen::Dis,
        }
    }
    #[doc = "Timer A is enabled and begins counting or the capture logic is enabled based on the CFG register."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Taen::En
    }
    #[doc = "Timer A is disabled."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Taen::Dis
    }
}
#[doc = "Field `TAEN` writer - 0:0\\]
GPT Timer A Enable"]
pub type TaenW<'a, REG> = crate::BitWriter<'a, REG, Taen>;
impl<'a, REG> TaenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Timer A is enabled and begins counting or the capture logic is enabled based on the CFG register."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Taen::En)
    }
    #[doc = "Timer A is disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Taen::Dis)
    }
}
#[doc = "1:1\\]
GPT Timer A Stall Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tastall {
    #[doc = "1: Timer A freezes counting while the processor is halted by the debugger."]
    En = 1,
    #[doc = "0: Timer A continues counting while the processor is halted by the debugger."]
    Dis = 0,
}
impl From<Tastall> for bool {
    #[inline(always)]
    fn from(variant: Tastall) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TASTALL` reader - 1:1\\]
GPT Timer A Stall Enable"]
pub type TastallR = crate::BitReader<Tastall>;
impl TastallR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tastall {
        match self.bits {
            true => Tastall::En,
            false => Tastall::Dis,
        }
    }
    #[doc = "Timer A freezes counting while the processor is halted by the debugger."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Tastall::En
    }
    #[doc = "Timer A continues counting while the processor is halted by the debugger."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Tastall::Dis
    }
}
#[doc = "Field `TASTALL` writer - 1:1\\]
GPT Timer A Stall Enable"]
pub type TastallW<'a, REG> = crate::BitWriter<'a, REG, Tastall>;
impl<'a, REG> TastallW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Timer A freezes counting while the processor is halted by the debugger."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Tastall::En)
    }
    #[doc = "Timer A continues counting while the processor is halted by the debugger."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Tastall::Dis)
    }
}
#[doc = "3:2\\]
GPT Timer A Event Mode The values in this register are defined as follows: Value Description 0x0 Positive edge 0x1 Negative edge 0x2 Reserved 0x3 Both edges Note: If PWM output inversion is enabled, edge detection interrupt behavior is reversed. Thus, if a positive-edge interrupt trigger has been set and the PWM inversion generates a postive edge, no event-trigger interrupt asserts. Instead, the interrupt is generated on the negative edge of the PWM signal.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Taevent {
    #[doc = "3: Both edges"]
    Both = 3,
    #[doc = "1: Negative edge"]
    Neg = 1,
    #[doc = "0: Positive edge"]
    Pos = 0,
}
impl From<Taevent> for u8 {
    #[inline(always)]
    fn from(variant: Taevent) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Taevent {
    type Ux = u8;
}
impl crate::IsEnum for Taevent {}
#[doc = "Field `TAEVENT` reader - 3:2\\]
GPT Timer A Event Mode The values in this register are defined as follows: Value Description 0x0 Positive edge 0x1 Negative edge 0x2 Reserved 0x3 Both edges Note: If PWM output inversion is enabled, edge detection interrupt behavior is reversed. Thus, if a positive-edge interrupt trigger has been set and the PWM inversion generates a postive edge, no event-trigger interrupt asserts. Instead, the interrupt is generated on the negative edge of the PWM signal."]
pub type TaeventR = crate::FieldReader<Taevent>;
impl TaeventR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Taevent> {
        match self.bits {
            3 => Some(Taevent::Both),
            1 => Some(Taevent::Neg),
            0 => Some(Taevent::Pos),
            _ => None,
        }
    }
    #[doc = "Both edges"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == Taevent::Both
    }
    #[doc = "Negative edge"]
    #[inline(always)]
    pub fn is_neg(&self) -> bool {
        *self == Taevent::Neg
    }
    #[doc = "Positive edge"]
    #[inline(always)]
    pub fn is_pos(&self) -> bool {
        *self == Taevent::Pos
    }
}
#[doc = "Field `TAEVENT` writer - 3:2\\]
GPT Timer A Event Mode The values in this register are defined as follows: Value Description 0x0 Positive edge 0x1 Negative edge 0x2 Reserved 0x3 Both edges Note: If PWM output inversion is enabled, edge detection interrupt behavior is reversed. Thus, if a positive-edge interrupt trigger has been set and the PWM inversion generates a postive edge, no event-trigger interrupt asserts. Instead, the interrupt is generated on the negative edge of the PWM signal."]
pub type TaeventW<'a, REG> = crate::FieldWriter<'a, REG, 2, Taevent>;
impl<'a, REG> TaeventW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Both edges"]
    #[inline(always)]
    pub fn both(self) -> &'a mut crate::W<REG> {
        self.variant(Taevent::Both)
    }
    #[doc = "Negative edge"]
    #[inline(always)]
    pub fn neg(self) -> &'a mut crate::W<REG> {
        self.variant(Taevent::Neg)
    }
    #[doc = "Positive edge"]
    #[inline(always)]
    pub fn pos(self) -> &'a mut crate::W<REG> {
        self.variant(Taevent::Pos)
    }
}
#[doc = "Field `RESERVED4` reader - 5:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved4R = crate::FieldReader;
#[doc = "Field `RESERVED4` writer - 5:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved4W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "6:6\\]
GPT Timer A PWM Output Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tapwml {
    #[doc = "1: Inverted"]
    Inverted = 1,
    #[doc = "0: Not inverted"]
    Normal = 0,
}
impl From<Tapwml> for bool {
    #[inline(always)]
    fn from(variant: Tapwml) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TAPWML` reader - 6:6\\]
GPT Timer A PWM Output Level"]
pub type TapwmlR = crate::BitReader<Tapwml>;
impl TapwmlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tapwml {
        match self.bits {
            true => Tapwml::Inverted,
            false => Tapwml::Normal,
        }
    }
    #[doc = "Inverted"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == Tapwml::Inverted
    }
    #[doc = "Not inverted"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == Tapwml::Normal
    }
}
#[doc = "Field `TAPWML` writer - 6:6\\]
GPT Timer A PWM Output Level"]
pub type TapwmlW<'a, REG> = crate::BitWriter<'a, REG, Tapwml>;
impl<'a, REG> TapwmlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Inverted"]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut crate::W<REG> {
        self.variant(Tapwml::Inverted)
    }
    #[doc = "Not inverted"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(Tapwml::Normal)
    }
}
#[doc = "Field `RESERVED7` reader - 7:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved7R = crate::BitReader;
#[doc = "Field `RESERVED7` writer - 7:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "8:8\\]
GPT Timer B Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tben {
    #[doc = "1: Timer B is enabled and begins counting or the capture logic is enabled based on CFG register."]
    En = 1,
    #[doc = "0: Timer B is disabled."]
    Dis = 0,
}
impl From<Tben> for bool {
    #[inline(always)]
    fn from(variant: Tben) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TBEN` reader - 8:8\\]
GPT Timer B Enable"]
pub type TbenR = crate::BitReader<Tben>;
impl TbenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tben {
        match self.bits {
            true => Tben::En,
            false => Tben::Dis,
        }
    }
    #[doc = "Timer B is enabled and begins counting or the capture logic is enabled based on CFG register."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Tben::En
    }
    #[doc = "Timer B is disabled."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Tben::Dis
    }
}
#[doc = "Field `TBEN` writer - 8:8\\]
GPT Timer B Enable"]
pub type TbenW<'a, REG> = crate::BitWriter<'a, REG, Tben>;
impl<'a, REG> TbenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Timer B is enabled and begins counting or the capture logic is enabled based on CFG register."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Tben::En)
    }
    #[doc = "Timer B is disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Tben::Dis)
    }
}
#[doc = "9:9\\]
GPT Timer B Stall Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tbstall {
    #[doc = "1: Timer B freezes counting while the processor is halted by the debugger."]
    En = 1,
    #[doc = "0: Timer B continues counting while the processor is halted by the debugger."]
    Dis = 0,
}
impl From<Tbstall> for bool {
    #[inline(always)]
    fn from(variant: Tbstall) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TBSTALL` reader - 9:9\\]
GPT Timer B Stall Enable"]
pub type TbstallR = crate::BitReader<Tbstall>;
impl TbstallR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tbstall {
        match self.bits {
            true => Tbstall::En,
            false => Tbstall::Dis,
        }
    }
    #[doc = "Timer B freezes counting while the processor is halted by the debugger."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Tbstall::En
    }
    #[doc = "Timer B continues counting while the processor is halted by the debugger."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Tbstall::Dis
    }
}
#[doc = "Field `TBSTALL` writer - 9:9\\]
GPT Timer B Stall Enable"]
pub type TbstallW<'a, REG> = crate::BitWriter<'a, REG, Tbstall>;
impl<'a, REG> TbstallW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Timer B freezes counting while the processor is halted by the debugger."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Tbstall::En)
    }
    #[doc = "Timer B continues counting while the processor is halted by the debugger."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Tbstall::Dis)
    }
}
#[doc = "11:10\\]
GPT Timer B Event Mode The values in this register are defined as follows: Value Description 0x0 Positive edge 0x1 Negative edge 0x2 Reserved 0x3 Both edges Note: If PWM output inversion is enabled, edge detection interrupt behavior is reversed. Thus, if a positive-edge interrupt trigger has been set and the PWM inversion generates a postive edge, no event-trigger interrupt asserts. Instead, the interrupt is generated on the negative edge of the PWM signal.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tbevent {
    #[doc = "3: Both edges"]
    Both = 3,
    #[doc = "1: Negative edge"]
    Neg = 1,
    #[doc = "0: Positive edge"]
    Pos = 0,
}
impl From<Tbevent> for u8 {
    #[inline(always)]
    fn from(variant: Tbevent) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tbevent {
    type Ux = u8;
}
impl crate::IsEnum for Tbevent {}
#[doc = "Field `TBEVENT` reader - 11:10\\]
GPT Timer B Event Mode The values in this register are defined as follows: Value Description 0x0 Positive edge 0x1 Negative edge 0x2 Reserved 0x3 Both edges Note: If PWM output inversion is enabled, edge detection interrupt behavior is reversed. Thus, if a positive-edge interrupt trigger has been set and the PWM inversion generates a postive edge, no event-trigger interrupt asserts. Instead, the interrupt is generated on the negative edge of the PWM signal."]
pub type TbeventR = crate::FieldReader<Tbevent>;
impl TbeventR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Tbevent> {
        match self.bits {
            3 => Some(Tbevent::Both),
            1 => Some(Tbevent::Neg),
            0 => Some(Tbevent::Pos),
            _ => None,
        }
    }
    #[doc = "Both edges"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == Tbevent::Both
    }
    #[doc = "Negative edge"]
    #[inline(always)]
    pub fn is_neg(&self) -> bool {
        *self == Tbevent::Neg
    }
    #[doc = "Positive edge"]
    #[inline(always)]
    pub fn is_pos(&self) -> bool {
        *self == Tbevent::Pos
    }
}
#[doc = "Field `TBEVENT` writer - 11:10\\]
GPT Timer B Event Mode The values in this register are defined as follows: Value Description 0x0 Positive edge 0x1 Negative edge 0x2 Reserved 0x3 Both edges Note: If PWM output inversion is enabled, edge detection interrupt behavior is reversed. Thus, if a positive-edge interrupt trigger has been set and the PWM inversion generates a postive edge, no event-trigger interrupt asserts. Instead, the interrupt is generated on the negative edge of the PWM signal."]
pub type TbeventW<'a, REG> = crate::FieldWriter<'a, REG, 2, Tbevent>;
impl<'a, REG> TbeventW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Both edges"]
    #[inline(always)]
    pub fn both(self) -> &'a mut crate::W<REG> {
        self.variant(Tbevent::Both)
    }
    #[doc = "Negative edge"]
    #[inline(always)]
    pub fn neg(self) -> &'a mut crate::W<REG> {
        self.variant(Tbevent::Neg)
    }
    #[doc = "Positive edge"]
    #[inline(always)]
    pub fn pos(self) -> &'a mut crate::W<REG> {
        self.variant(Tbevent::Pos)
    }
}
#[doc = "Field `RESERVED12` reader - 13:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved12R = crate::FieldReader;
#[doc = "Field `RESERVED12` writer - 13:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved12W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "14:14\\]
GPT Timer B PWM Output Level 0: Output is unaffected. 1: Output is inverted.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tbpwml {
    #[doc = "1: Inverted"]
    Inverted = 1,
    #[doc = "0: Not inverted"]
    Normal = 0,
}
impl From<Tbpwml> for bool {
    #[inline(always)]
    fn from(variant: Tbpwml) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TBPWML` reader - 14:14\\]
GPT Timer B PWM Output Level 0: Output is unaffected. 1: Output is inverted."]
pub type TbpwmlR = crate::BitReader<Tbpwml>;
impl TbpwmlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tbpwml {
        match self.bits {
            true => Tbpwml::Inverted,
            false => Tbpwml::Normal,
        }
    }
    #[doc = "Inverted"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == Tbpwml::Inverted
    }
    #[doc = "Not inverted"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == Tbpwml::Normal
    }
}
#[doc = "Field `TBPWML` writer - 14:14\\]
GPT Timer B PWM Output Level 0: Output is unaffected. 1: Output is inverted."]
pub type TbpwmlW<'a, REG> = crate::BitWriter<'a, REG, Tbpwml>;
impl<'a, REG> TbpwmlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Inverted"]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut crate::W<REG> {
        self.variant(Tbpwml::Inverted)
    }
    #[doc = "Not inverted"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(Tbpwml::Normal)
    }
}
#[doc = "Field `RESERVED15` reader - 31:15\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved15R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED15` writer - 31:15\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved15W<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
GPT Timer A Enable"]
    #[inline(always)]
    pub fn taen(&self) -> TaenR {
        TaenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
GPT Timer A Stall Enable"]
    #[inline(always)]
    pub fn tastall(&self) -> TastallR {
        TastallR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - 3:2\\]
GPT Timer A Event Mode The values in this register are defined as follows: Value Description 0x0 Positive edge 0x1 Negative edge 0x2 Reserved 0x3 Both edges Note: If PWM output inversion is enabled, edge detection interrupt behavior is reversed. Thus, if a positive-edge interrupt trigger has been set and the PWM inversion generates a postive edge, no event-trigger interrupt asserts. Instead, the interrupt is generated on the negative edge of the PWM signal."]
    #[inline(always)]
    pub fn taevent(&self) -> TaeventR {
        TaeventR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - 5:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - 6:6\\]
GPT Timer A PWM Output Level"]
    #[inline(always)]
    pub fn tapwml(&self) -> TapwmlR {
        TapwmlR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved7(&self) -> Reserved7R {
        Reserved7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
GPT Timer B Enable"]
    #[inline(always)]
    pub fn tben(&self) -> TbenR {
        TbenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
GPT Timer B Stall Enable"]
    #[inline(always)]
    pub fn tbstall(&self) -> TbstallR {
        TbstallR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - 11:10\\]
GPT Timer B Event Mode The values in this register are defined as follows: Value Description 0x0 Positive edge 0x1 Negative edge 0x2 Reserved 0x3 Both edges Note: If PWM output inversion is enabled, edge detection interrupt behavior is reversed. Thus, if a positive-edge interrupt trigger has been set and the PWM inversion generates a postive edge, no event-trigger interrupt asserts. Instead, the interrupt is generated on the negative edge of the PWM signal."]
    #[inline(always)]
    pub fn tbevent(&self) -> TbeventR {
        TbeventR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - 13:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved12(&self) -> Reserved12R {
        Reserved12R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - 14:14\\]
GPT Timer B PWM Output Level 0: Output is unaffected. 1: Output is inverted."]
    #[inline(always)]
    pub fn tbpwml(&self) -> TbpwmlR {
        TbpwmlR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 15:31 - 31:15\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved15(&self) -> Reserved15R {
        Reserved15R::new((self.bits >> 15) & 0x0001_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
GPT Timer A Enable"]
    #[inline(always)]
    #[must_use]
    pub fn taen(&mut self) -> TaenW<CtlSpec> {
        TaenW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
GPT Timer A Stall Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tastall(&mut self) -> TastallW<CtlSpec> {
        TastallW::new(self, 1)
    }
    #[doc = "Bits 2:3 - 3:2\\]
GPT Timer A Event Mode The values in this register are defined as follows: Value Description 0x0 Positive edge 0x1 Negative edge 0x2 Reserved 0x3 Both edges Note: If PWM output inversion is enabled, edge detection interrupt behavior is reversed. Thus, if a positive-edge interrupt trigger has been set and the PWM inversion generates a postive edge, no event-trigger interrupt asserts. Instead, the interrupt is generated on the negative edge of the PWM signal."]
    #[inline(always)]
    #[must_use]
    pub fn taevent(&mut self) -> TaeventW<CtlSpec> {
        TaeventW::new(self, 2)
    }
    #[doc = "Bits 4:5 - 5:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved4(&mut self) -> Reserved4W<CtlSpec> {
        Reserved4W::new(self, 4)
    }
    #[doc = "Bit 6 - 6:6\\]
GPT Timer A PWM Output Level"]
    #[inline(always)]
    #[must_use]
    pub fn tapwml(&mut self) -> TapwmlW<CtlSpec> {
        TapwmlW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved7(&mut self) -> Reserved7W<CtlSpec> {
        Reserved7W::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
GPT Timer B Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tben(&mut self) -> TbenW<CtlSpec> {
        TbenW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
GPT Timer B Stall Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tbstall(&mut self) -> TbstallW<CtlSpec> {
        TbstallW::new(self, 9)
    }
    #[doc = "Bits 10:11 - 11:10\\]
GPT Timer B Event Mode The values in this register are defined as follows: Value Description 0x0 Positive edge 0x1 Negative edge 0x2 Reserved 0x3 Both edges Note: If PWM output inversion is enabled, edge detection interrupt behavior is reversed. Thus, if a positive-edge interrupt trigger has been set and the PWM inversion generates a postive edge, no event-trigger interrupt asserts. Instead, the interrupt is generated on the negative edge of the PWM signal."]
    #[inline(always)]
    #[must_use]
    pub fn tbevent(&mut self) -> TbeventW<CtlSpec> {
        TbeventW::new(self, 10)
    }
    #[doc = "Bits 12:13 - 13:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved12(&mut self) -> Reserved12W<CtlSpec> {
        Reserved12W::new(self, 12)
    }
    #[doc = "Bit 14 - 14:14\\]
GPT Timer B PWM Output Level 0: Output is unaffected. 1: Output is inverted."]
    #[inline(always)]
    #[must_use]
    pub fn tbpwml(&mut self) -> TbpwmlW<CtlSpec> {
        TbpwmlW::new(self, 14)
    }
    #[doc = "Bits 15:31 - 31:15\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved15(&mut self) -> Reserved15W<CtlSpec> {
        Reserved15W::new(self, 15)
    }
}
#[doc = "Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlSpec;
impl crate::RegisterSpec for CtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl::R`](R) reader structure"]
impl crate::Readable for CtlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctl::W`](W) writer structure"]
impl crate::Writable for CtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTL to value 0"]
impl crate::Resettable for CtlSpec {
    const RESET_VALUE: u32 = 0;
}
