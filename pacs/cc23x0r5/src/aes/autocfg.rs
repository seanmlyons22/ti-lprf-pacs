#[doc = "Register `AUTOCFG` reader"]
pub type R = crate::R<AutocfgSpec>;
#[doc = "Register `AUTOCFG` writer"]
pub type W = crate::W<AutocfgSpec>;
#[doc = "3:0\\]
Trigger Electronic Codebook This field specifies one or more actions that indirectly trigger ECB operation It is allowed to configure this field with an OR-combination of supported enums.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Trgecb {
    #[doc = "8: Write to BUF3 shall schedule to trigger single action once STA.STATE is or becomes IDLE. Subsequent writes do not trigger any action unless this setting is written again to this field."]
    Wrbuf3s = 8,
    #[doc = "4: All writes to BUF3 shall be scheduled to trigger action once STA.STATE is or becomes IDLE, only when AUTOCFG.CTRSIZE = DIS."]
    Wrbuf3 = 4,
    #[doc = "2: All reads of TXT3 or TXTXBUF3 trigger action, only when STA.STATE = IDLE."]
    Rdtxt3 = 2,
    #[doc = "1: All writes to TXT3 or TXTX3 trigger action, only when STA.STATE = IDLE."]
    Wrtxt3 = 1,
    #[doc = "0: No user action indirectly triggers ECB."]
    Dis = 0,
}
impl From<Trgecb> for u8 {
    #[inline(always)]
    fn from(variant: Trgecb) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Trgecb {
    type Ux = u8;
}
impl crate::IsEnum for Trgecb {}
#[doc = "Field `TRGECB` reader - 3:0\\]
Trigger Electronic Codebook This field specifies one or more actions that indirectly trigger ECB operation It is allowed to configure this field with an OR-combination of supported enums."]
pub type TrgecbR = crate::FieldReader<Trgecb>;
impl TrgecbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Trgecb> {
        match self.bits {
            8 => Some(Trgecb::Wrbuf3s),
            4 => Some(Trgecb::Wrbuf3),
            2 => Some(Trgecb::Rdtxt3),
            1 => Some(Trgecb::Wrtxt3),
            0 => Some(Trgecb::Dis),
            _ => None,
        }
    }
    #[doc = "Write to BUF3 shall schedule to trigger single action once STA.STATE is or becomes IDLE. Subsequent writes do not trigger any action unless this setting is written again to this field."]
    #[inline(always)]
    pub fn is_wrbuf3s(&self) -> bool {
        *self == Trgecb::Wrbuf3s
    }
    #[doc = "All writes to BUF3 shall be scheduled to trigger action once STA.STATE is or becomes IDLE, only when AUTOCFG.CTRSIZE = DIS."]
    #[inline(always)]
    pub fn is_wrbuf3(&self) -> bool {
        *self == Trgecb::Wrbuf3
    }
    #[doc = "All reads of TXT3 or TXTXBUF3 trigger action, only when STA.STATE = IDLE."]
    #[inline(always)]
    pub fn is_rdtxt3(&self) -> bool {
        *self == Trgecb::Rdtxt3
    }
    #[doc = "All writes to TXT3 or TXTX3 trigger action, only when STA.STATE = IDLE."]
    #[inline(always)]
    pub fn is_wrtxt3(&self) -> bool {
        *self == Trgecb::Wrtxt3
    }
    #[doc = "No user action indirectly triggers ECB."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Trgecb::Dis
    }
}
#[doc = "Field `TRGECB` writer - 3:0\\]
Trigger Electronic Codebook This field specifies one or more actions that indirectly trigger ECB operation It is allowed to configure this field with an OR-combination of supported enums."]
pub type TrgecbW<'a, REG> = crate::FieldWriter<'a, REG, 4, Trgecb>;
impl<'a, REG> TrgecbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Write to BUF3 shall schedule to trigger single action once STA.STATE is or becomes IDLE. Subsequent writes do not trigger any action unless this setting is written again to this field."]
    #[inline(always)]
    pub fn wrbuf3s(self) -> &'a mut crate::W<REG> {
        self.variant(Trgecb::Wrbuf3s)
    }
    #[doc = "All writes to BUF3 shall be scheduled to trigger action once STA.STATE is or becomes IDLE, only when AUTOCFG.CTRSIZE = DIS."]
    #[inline(always)]
    pub fn wrbuf3(self) -> &'a mut crate::W<REG> {
        self.variant(Trgecb::Wrbuf3)
    }
    #[doc = "All reads of TXT3 or TXTXBUF3 trigger action, only when STA.STATE = IDLE."]
    #[inline(always)]
    pub fn rdtxt3(self) -> &'a mut crate::W<REG> {
        self.variant(Trgecb::Rdtxt3)
    }
    #[doc = "All writes to TXT3 or TXTX3 trigger action, only when STA.STATE = IDLE."]
    #[inline(always)]
    pub fn wrtxt3(self) -> &'a mut crate::W<REG> {
        self.variant(Trgecb::Wrtxt3)
    }
    #[doc = "No user action indirectly triggers ECB."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Trgecb::Dis)
    }
}
#[doc = "5:4\\]
Electronic Codebook Source This field specifies the data source to hardware-triggered ECB encryptions. Non-enumerated values are not supported and ignored.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ecbsrc {
    #[doc = "3: TXT = ECB(KEY, TXT XOR BUF)"]
    Txtxbuf = 3,
    #[doc = "2: TXT = ECB(KEY,BUF)"]
    Buf = 2,
    #[doc = "1: TXT = ECB(KEY,TXT)"]
    Txt = 1,
}
impl From<Ecbsrc> for u8 {
    #[inline(always)]
    fn from(variant: Ecbsrc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ecbsrc {
    type Ux = u8;
}
impl crate::IsEnum for Ecbsrc {}
#[doc = "Field `ECBSRC` reader - 5:4\\]
Electronic Codebook Source This field specifies the data source to hardware-triggered ECB encryptions. Non-enumerated values are not supported and ignored."]
pub type EcbsrcR = crate::FieldReader<Ecbsrc>;
impl EcbsrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ecbsrc> {
        match self.bits {
            3 => Some(Ecbsrc::Txtxbuf),
            2 => Some(Ecbsrc::Buf),
            1 => Some(Ecbsrc::Txt),
            _ => None,
        }
    }
    #[doc = "TXT = ECB(KEY, TXT XOR BUF)"]
    #[inline(always)]
    pub fn is_txtxbuf(&self) -> bool {
        *self == Ecbsrc::Txtxbuf
    }
    #[doc = "TXT = ECB(KEY,BUF)"]
    #[inline(always)]
    pub fn is_buf(&self) -> bool {
        *self == Ecbsrc::Buf
    }
    #[doc = "TXT = ECB(KEY,TXT)"]
    #[inline(always)]
    pub fn is_txt(&self) -> bool {
        *self == Ecbsrc::Txt
    }
}
#[doc = "Field `ECBSRC` writer - 5:4\\]
Electronic Codebook Source This field specifies the data source to hardware-triggered ECB encryptions. Non-enumerated values are not supported and ignored."]
pub type EcbsrcW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ecbsrc>;
impl<'a, REG> EcbsrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "TXT = ECB(KEY, TXT XOR BUF)"]
    #[inline(always)]
    pub fn txtxbuf(self) -> &'a mut crate::W<REG> {
        self.variant(Ecbsrc::Txtxbuf)
    }
    #[doc = "TXT = ECB(KEY,BUF)"]
    #[inline(always)]
    pub fn buf(self) -> &'a mut crate::W<REG> {
        self.variant(Ecbsrc::Buf)
    }
    #[doc = "TXT = ECB(KEY,TXT)"]
    #[inline(always)]
    pub fn txt(self) -> &'a mut crate::W<REG> {
        self.variant(Ecbsrc::Txt)
    }
}
#[doc = "Field `RESERVED6` reader - 7:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved6R = crate::FieldReader;
#[doc = "Field `RESERVED6` writer - 7:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved6W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "9:8\\]
Trigger for TXT This field determines if and when hardware automatically XORs BUF into TXT. Non-enumerated values are not supported and ignored. It is allowed to configure this field with an OR-combination of supported enums.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Trgtxt {
    #[doc = "2: Hardware XORs content of BUF into TXT upon read of TXTXBUF3."]
    Rdtxtxbuf3 = 2,
    #[doc = "1: Hardware XORs content of BUF into TXT upon read of TXT3."]
    Rdtxt3 = 1,
    #[doc = "0: No hardware update of TXT"]
    Dis = 0,
}
impl From<Trgtxt> for u8 {
    #[inline(always)]
    fn from(variant: Trgtxt) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Trgtxt {
    type Ux = u8;
}
impl crate::IsEnum for Trgtxt {}
#[doc = "Field `TRGTXT` reader - 9:8\\]
Trigger for TXT This field determines if and when hardware automatically XORs BUF into TXT. Non-enumerated values are not supported and ignored. It is allowed to configure this field with an OR-combination of supported enums."]
pub type TrgtxtR = crate::FieldReader<Trgtxt>;
impl TrgtxtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Trgtxt> {
        match self.bits {
            2 => Some(Trgtxt::Rdtxtxbuf3),
            1 => Some(Trgtxt::Rdtxt3),
            0 => Some(Trgtxt::Dis),
            _ => None,
        }
    }
    #[doc = "Hardware XORs content of BUF into TXT upon read of TXTXBUF3."]
    #[inline(always)]
    pub fn is_rdtxtxbuf3(&self) -> bool {
        *self == Trgtxt::Rdtxtxbuf3
    }
    #[doc = "Hardware XORs content of BUF into TXT upon read of TXT3."]
    #[inline(always)]
    pub fn is_rdtxt3(&self) -> bool {
        *self == Trgtxt::Rdtxt3
    }
    #[doc = "No hardware update of TXT"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Trgtxt::Dis
    }
}
#[doc = "Field `TRGTXT` writer - 9:8\\]
Trigger for TXT This field determines if and when hardware automatically XORs BUF into TXT. Non-enumerated values are not supported and ignored. It is allowed to configure this field with an OR-combination of supported enums."]
pub type TrgtxtW<'a, REG> = crate::FieldWriter<'a, REG, 2, Trgtxt>;
impl<'a, REG> TrgtxtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Hardware XORs content of BUF into TXT upon read of TXTXBUF3."]
    #[inline(always)]
    pub fn rdtxtxbuf3(self) -> &'a mut crate::W<REG> {
        self.variant(Trgtxt::Rdtxtxbuf3)
    }
    #[doc = "Hardware XORs content of BUF into TXT upon read of TXT3."]
    #[inline(always)]
    pub fn rdtxt3(self) -> &'a mut crate::W<REG> {
        self.variant(Trgtxt::Rdtxt3)
    }
    #[doc = "No hardware update of TXT"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Trgtxt::Dis)
    }
}
#[doc = "Field `RESERVED11` reader - 16:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved11R = crate::FieldReader;
#[doc = "Field `RESERVED11` writer - 16:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved11W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "17:17\\]
Counter Endianness Specifies Endianness of counter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctrendian {
    #[doc = "1: Specifies Big Endian Counter Carry will flow from octet 'n' to octet 'n-1'"]
    Bigendian = 1,
    #[doc = "0: Specifies Little Endian Counter. Carry will flow from octet 'n' to octet 'n+1'"]
    Littleendian = 0,
}
impl From<Ctrendian> for bool {
    #[inline(always)]
    fn from(variant: Ctrendian) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTRENDIAN` reader - 17:17\\]
Counter Endianness Specifies Endianness of counter"]
pub type CtrendianR = crate::BitReader<Ctrendian>;
impl CtrendianR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctrendian {
        match self.bits {
            true => Ctrendian::Bigendian,
            false => Ctrendian::Littleendian,
        }
    }
    #[doc = "Specifies Big Endian Counter Carry will flow from octet 'n' to octet 'n-1'"]
    #[inline(always)]
    pub fn is_bigendian(&self) -> bool {
        *self == Ctrendian::Bigendian
    }
    #[doc = "Specifies Little Endian Counter. Carry will flow from octet 'n' to octet 'n+1'"]
    #[inline(always)]
    pub fn is_littleendian(&self) -> bool {
        *self == Ctrendian::Littleendian
    }
}
#[doc = "Field `CTRENDIAN` writer - 17:17\\]
Counter Endianness Specifies Endianness of counter"]
pub type CtrendianW<'a, REG> = crate::BitWriter<'a, REG, Ctrendian>;
impl<'a, REG> CtrendianW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Specifies Big Endian Counter Carry will flow from octet 'n' to octet 'n-1'"]
    #[inline(always)]
    pub fn bigendian(self) -> &'a mut crate::W<REG> {
        self.variant(Ctrendian::Bigendian)
    }
    #[doc = "Specifies Little Endian Counter. Carry will flow from octet 'n' to octet 'n+1'"]
    #[inline(always)]
    pub fn littleendian(self) -> &'a mut crate::W<REG> {
        self.variant(Ctrendian::Littleendian)
    }
}
#[doc = "18:18\\]
Counter Alignment Specifies alignment of counter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctralign {
    #[doc = "1: Indicates right aligned counter Not applicable when counter size is 128bit For 128 bit counter, all octets will be considered If right aligned, octet 8-15 will be considered based on endianness and counter size"]
    Rightalign = 1,
    #[doc = "0: Indicates Left Aligned Counter Not applicable for 128 bit counter size. For 128 bit counter, all octets will be considered When left aligned,,octet 0-7 will be considered , based on counter size and endianness"]
    Leftalign = 0,
}
impl From<Ctralign> for bool {
    #[inline(always)]
    fn from(variant: Ctralign) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTRALIGN` reader - 18:18\\]
Counter Alignment Specifies alignment of counter"]
pub type CtralignR = crate::BitReader<Ctralign>;
impl CtralignR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctralign {
        match self.bits {
            true => Ctralign::Rightalign,
            false => Ctralign::Leftalign,
        }
    }
    #[doc = "Indicates right aligned counter Not applicable when counter size is 128bit For 128 bit counter, all octets will be considered If right aligned, octet 8-15 will be considered based on endianness and counter size"]
    #[inline(always)]
    pub fn is_rightalign(&self) -> bool {
        *self == Ctralign::Rightalign
    }
    #[doc = "Indicates Left Aligned Counter Not applicable for 128 bit counter size. For 128 bit counter, all octets will be considered When left aligned,,octet 0-7 will be considered , based on counter size and endianness"]
    #[inline(always)]
    pub fn is_leftalign(&self) -> bool {
        *self == Ctralign::Leftalign
    }
}
#[doc = "Field `CTRALIGN` writer - 18:18\\]
Counter Alignment Specifies alignment of counter"]
pub type CtralignW<'a, REG> = crate::BitWriter<'a, REG, Ctralign>;
impl<'a, REG> CtralignW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Indicates right aligned counter Not applicable when counter size is 128bit For 128 bit counter, all octets will be considered If right aligned, octet 8-15 will be considered based on endianness and counter size"]
    #[inline(always)]
    pub fn rightalign(self) -> &'a mut crate::W<REG> {
        self.variant(Ctralign::Rightalign)
    }
    #[doc = "Indicates Left Aligned Counter Not applicable for 128 bit counter size. For 128 bit counter, all octets will be considered When left aligned,,octet 0-7 will be considered , based on counter size and endianness"]
    #[inline(always)]
    pub fn leftalign(self) -> &'a mut crate::W<REG> {
        self.variant(Ctralign::Leftalign)
    }
}
#[doc = "21:19\\]
Counter Size Configures size of counter as either 8,16,32,64 or 128 Non-enumerated values are not supported and ignored\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ctrsize {
    #[doc = "5: Configures counter size as 128 bit"]
    Ctr128 = 5,
    #[doc = "4: Configures counter size as 64 bit"]
    Ctr64 = 4,
    #[doc = "3: Configures counter size as 32 bit"]
    Ctr32 = 3,
    #[doc = "2: Configures counter size as 16 bit"]
    Ctr16 = 2,
    #[doc = "1: Configures counter size as 8 bit"]
    Ctr8 = 1,
    #[doc = "0: Disable CTR operation"]
    Dis = 0,
}
impl From<Ctrsize> for u8 {
    #[inline(always)]
    fn from(variant: Ctrsize) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ctrsize {
    type Ux = u8;
}
impl crate::IsEnum for Ctrsize {}
#[doc = "Field `CTRSIZE` reader - 21:19\\]
Counter Size Configures size of counter as either 8,16,32,64 or 128 Non-enumerated values are not supported and ignored"]
pub type CtrsizeR = crate::FieldReader<Ctrsize>;
impl CtrsizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ctrsize> {
        match self.bits {
            5 => Some(Ctrsize::Ctr128),
            4 => Some(Ctrsize::Ctr64),
            3 => Some(Ctrsize::Ctr32),
            2 => Some(Ctrsize::Ctr16),
            1 => Some(Ctrsize::Ctr8),
            0 => Some(Ctrsize::Dis),
            _ => None,
        }
    }
    #[doc = "Configures counter size as 128 bit"]
    #[inline(always)]
    pub fn is_ctr128(&self) -> bool {
        *self == Ctrsize::Ctr128
    }
    #[doc = "Configures counter size as 64 bit"]
    #[inline(always)]
    pub fn is_ctr64(&self) -> bool {
        *self == Ctrsize::Ctr64
    }
    #[doc = "Configures counter size as 32 bit"]
    #[inline(always)]
    pub fn is_ctr32(&self) -> bool {
        *self == Ctrsize::Ctr32
    }
    #[doc = "Configures counter size as 16 bit"]
    #[inline(always)]
    pub fn is_ctr16(&self) -> bool {
        *self == Ctrsize::Ctr16
    }
    #[doc = "Configures counter size as 8 bit"]
    #[inline(always)]
    pub fn is_ctr8(&self) -> bool {
        *self == Ctrsize::Ctr8
    }
    #[doc = "Disable CTR operation"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Ctrsize::Dis
    }
}
#[doc = "Field `CTRSIZE` writer - 21:19\\]
Counter Size Configures size of counter as either 8,16,32,64 or 128 Non-enumerated values are not supported and ignored"]
pub type CtrsizeW<'a, REG> = crate::FieldWriter<'a, REG, 3, Ctrsize>;
impl<'a, REG> CtrsizeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Configures counter size as 128 bit"]
    #[inline(always)]
    pub fn ctr128(self) -> &'a mut crate::W<REG> {
        self.variant(Ctrsize::Ctr128)
    }
    #[doc = "Configures counter size as 64 bit"]
    #[inline(always)]
    pub fn ctr64(self) -> &'a mut crate::W<REG> {
        self.variant(Ctrsize::Ctr64)
    }
    #[doc = "Configures counter size as 32 bit"]
    #[inline(always)]
    pub fn ctr32(self) -> &'a mut crate::W<REG> {
        self.variant(Ctrsize::Ctr32)
    }
    #[doc = "Configures counter size as 16 bit"]
    #[inline(always)]
    pub fn ctr16(self) -> &'a mut crate::W<REG> {
        self.variant(Ctrsize::Ctr16)
    }
    #[doc = "Configures counter size as 8 bit"]
    #[inline(always)]
    pub fn ctr8(self) -> &'a mut crate::W<REG> {
        self.variant(Ctrsize::Ctr8)
    }
    #[doc = "Disable CTR operation"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Ctrsize::Dis)
    }
}
#[doc = "Field `RESERVED22` reader - 23:22\\]
Reserved for future use. Any writes to this field are ignored"]
pub type Reserved22R = crate::FieldReader;
#[doc = "Field `RESERVED22` writer - 23:22\\]
Reserved for future use. Any writes to this field are ignored"]
pub type Reserved22W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "24:24\\]
Bus Halt This field decides if bus halts on access to KEY, TXT, BUF, TXTX and TXTXBUF when STA.STATE = BUSY.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bushalt {
    #[doc = "1: Enable bus halt When STA.STATE = BUSY, access to KEY, TXT, TXTX, TXTXBUF halt the bus until STA.STATE = IDLE. When STA.STATE = BUSY and if either STA.BUFSTA = FULL or AUTOCFG.CTRSIZE != DISABLE, access to BUF halts the bus until STA.STATE = IDLE."]
    En = 1,
    #[doc = "0: Disable bus halt When STA.STATE = BUSY, writes to KEY, TXT, TXTX are ignored, reads from TXT, TXTXBUF return zero. When STA.STATE = BUSY and if either STA.BUFSTA = FULL or AUTOCFG.CTRSIZE != DISABLE, writes to BUF are ignored, reads return zero."]
    Dis = 0,
}
impl From<Bushalt> for bool {
    #[inline(always)]
    fn from(variant: Bushalt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUSHALT` reader - 24:24\\]
Bus Halt This field decides if bus halts on access to KEY, TXT, BUF, TXTX and TXTXBUF when STA.STATE = BUSY."]
pub type BushaltR = crate::BitReader<Bushalt>;
impl BushaltR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bushalt {
        match self.bits {
            true => Bushalt::En,
            false => Bushalt::Dis,
        }
    }
    #[doc = "Enable bus halt When STA.STATE = BUSY, access to KEY, TXT, TXTX, TXTXBUF halt the bus until STA.STATE = IDLE. When STA.STATE = BUSY and if either STA.BUFSTA = FULL or AUTOCFG.CTRSIZE != DISABLE, access to BUF halts the bus until STA.STATE = IDLE."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Bushalt::En
    }
    #[doc = "Disable bus halt When STA.STATE = BUSY, writes to KEY, TXT, TXTX are ignored, reads from TXT, TXTXBUF return zero. When STA.STATE = BUSY and if either STA.BUFSTA = FULL or AUTOCFG.CTRSIZE != DISABLE, writes to BUF are ignored, reads return zero."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Bushalt::Dis
    }
}
#[doc = "Field `BUSHALT` writer - 24:24\\]
Bus Halt This field decides if bus halts on access to KEY, TXT, BUF, TXTX and TXTXBUF when STA.STATE = BUSY."]
pub type BushaltW<'a, REG> = crate::BitWriter<'a, REG, Bushalt>;
impl<'a, REG> BushaltW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable bus halt When STA.STATE = BUSY, access to KEY, TXT, TXTX, TXTXBUF halt the bus until STA.STATE = IDLE. When STA.STATE = BUSY and if either STA.BUFSTA = FULL or AUTOCFG.CTRSIZE != DISABLE, access to BUF halts the bus until STA.STATE = IDLE."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Bushalt::En)
    }
    #[doc = "Disable bus halt When STA.STATE = BUSY, writes to KEY, TXT, TXTX are ignored, reads from TXT, TXTXBUF return zero. When STA.STATE = BUSY and if either STA.BUFSTA = FULL or AUTOCFG.CTRSIZE != DISABLE, writes to BUF are ignored, reads return zero."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Bushalt::Dis)
    }
}
#[doc = "25:25\\]
This field enables auto-clear of RIS.ECBDONE interrupt on read/write of TXT3/BUF3/TXTX3/TXTXBUF3 .\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ecbdoneclr {
    #[doc = "1: Enable auto-clear of RIS.ECBDONE interrupt"]
    En = 1,
    #[doc = "0: Disable auto-clear of RIS.ECBDONE interrupt"]
    Dis = 0,
}
impl From<Ecbdoneclr> for bool {
    #[inline(always)]
    fn from(variant: Ecbdoneclr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ECBDONECLR` reader - 25:25\\]
This field enables auto-clear of RIS.ECBDONE interrupt on read/write of TXT3/BUF3/TXTX3/TXTXBUF3 ."]
pub type EcbdoneclrR = crate::BitReader<Ecbdoneclr>;
impl EcbdoneclrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ecbdoneclr {
        match self.bits {
            true => Ecbdoneclr::En,
            false => Ecbdoneclr::Dis,
        }
    }
    #[doc = "Enable auto-clear of RIS.ECBDONE interrupt"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Ecbdoneclr::En
    }
    #[doc = "Disable auto-clear of RIS.ECBDONE interrupt"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Ecbdoneclr::Dis
    }
}
#[doc = "Field `ECBDONECLR` writer - 25:25\\]
This field enables auto-clear of RIS.ECBDONE interrupt on read/write of TXT3/BUF3/TXTX3/TXTXBUF3 ."]
pub type EcbdoneclrW<'a, REG> = crate::BitWriter<'a, REG, Ecbdoneclr>;
impl<'a, REG> EcbdoneclrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable auto-clear of RIS.ECBDONE interrupt"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Ecbdoneclr::En)
    }
    #[doc = "Disable auto-clear of RIS.ECBDONE interrupt"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Ecbdoneclr::Dis)
    }
}
#[doc = "26:26\\]
This field enables auto-clear of RIS.ECBSTART interrupt on read/write of TXT3/BUF3/TXTX3/TXTXBUF3 .\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ecbstartclr {
    #[doc = "1: Enable auto-clear of RIS.ECBSTART interrupt"]
    En = 1,
    #[doc = "0: Disable auto-clear of RIS.ECBSTART interrupt"]
    Dis = 0,
}
impl From<Ecbstartclr> for bool {
    #[inline(always)]
    fn from(variant: Ecbstartclr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ECBSTARTCLR` reader - 26:26\\]
This field enables auto-clear of RIS.ECBSTART interrupt on read/write of TXT3/BUF3/TXTX3/TXTXBUF3 ."]
pub type EcbstartclrR = crate::BitReader<Ecbstartclr>;
impl EcbstartclrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ecbstartclr {
        match self.bits {
            true => Ecbstartclr::En,
            false => Ecbstartclr::Dis,
        }
    }
    #[doc = "Enable auto-clear of RIS.ECBSTART interrupt"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Ecbstartclr::En
    }
    #[doc = "Disable auto-clear of RIS.ECBSTART interrupt"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Ecbstartclr::Dis
    }
}
#[doc = "Field `ECBSTARTCLR` writer - 26:26\\]
This field enables auto-clear of RIS.ECBSTART interrupt on read/write of TXT3/BUF3/TXTX3/TXTXBUF3 ."]
pub type EcbstartclrW<'a, REG> = crate::BitWriter<'a, REG, Ecbstartclr>;
impl<'a, REG> EcbstartclrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable auto-clear of RIS.ECBSTART interrupt"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Ecbstartclr::En)
    }
    #[doc = "Disable auto-clear of RIS.ECBSTART interrupt"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Ecbstartclr::Dis)
    }
}
#[doc = "27:27\\]
This field enables auto-clear of RIS.CHADONE interrupt on read/write of TXT3/BUF3/TXTX3/TXTXBUF3 .\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chadoneclr {
    #[doc = "1: Enable auto-clear of RIS.CHADONE interrupt"]
    En = 1,
    #[doc = "0: Disable auto-clear of RIS.CHADONE interrupt"]
    Dis = 0,
}
impl From<Chadoneclr> for bool {
    #[inline(always)]
    fn from(variant: Chadoneclr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHADONECLR` reader - 27:27\\]
This field enables auto-clear of RIS.CHADONE interrupt on read/write of TXT3/BUF3/TXTX3/TXTXBUF3 ."]
pub type ChadoneclrR = crate::BitReader<Chadoneclr>;
impl ChadoneclrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chadoneclr {
        match self.bits {
            true => Chadoneclr::En,
            false => Chadoneclr::Dis,
        }
    }
    #[doc = "Enable auto-clear of RIS.CHADONE interrupt"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Chadoneclr::En
    }
    #[doc = "Disable auto-clear of RIS.CHADONE interrupt"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Chadoneclr::Dis
    }
}
#[doc = "Field `CHADONECLR` writer - 27:27\\]
This field enables auto-clear of RIS.CHADONE interrupt on read/write of TXT3/BUF3/TXTX3/TXTXBUF3 ."]
pub type ChadoneclrW<'a, REG> = crate::BitWriter<'a, REG, Chadoneclr>;
impl<'a, REG> ChadoneclrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable auto-clear of RIS.CHADONE interrupt"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Chadoneclr::En)
    }
    #[doc = "Disable auto-clear of RIS.CHADONE interrupt"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Chadoneclr::Dis)
    }
}
#[doc = "28:28\\]
This field enable auto-clear of RIS.CHBDONE interrupt on read/write of TXT3/BUF3/TXTX3/TXTXBUF3 .\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chbdoneclr {
    #[doc = "1: Enable auto-clear of RIS.CHBDONE interrupt"]
    En = 1,
    #[doc = "0: Disable auto-clear of RIS.CHBDONE interrupt"]
    Dis = 0,
}
impl From<Chbdoneclr> for bool {
    #[inline(always)]
    fn from(variant: Chbdoneclr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHBDONECLR` reader - 28:28\\]
This field enable auto-clear of RIS.CHBDONE interrupt on read/write of TXT3/BUF3/TXTX3/TXTXBUF3 ."]
pub type ChbdoneclrR = crate::BitReader<Chbdoneclr>;
impl ChbdoneclrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chbdoneclr {
        match self.bits {
            true => Chbdoneclr::En,
            false => Chbdoneclr::Dis,
        }
    }
    #[doc = "Enable auto-clear of RIS.CHBDONE interrupt"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Chbdoneclr::En
    }
    #[doc = "Disable auto-clear of RIS.CHBDONE interrupt"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Chbdoneclr::Dis
    }
}
#[doc = "Field `CHBDONECLR` writer - 28:28\\]
This field enable auto-clear of RIS.CHBDONE interrupt on read/write of TXT3/BUF3/TXTX3/TXTXBUF3 ."]
pub type ChbdoneclrW<'a, REG> = crate::BitWriter<'a, REG, Chbdoneclr>;
impl<'a, REG> ChbdoneclrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable auto-clear of RIS.CHBDONE interrupt"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Chbdoneclr::En)
    }
    #[doc = "Disable auto-clear of RIS.CHBDONE interrupt"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Chbdoneclr::Dis)
    }
}
#[doc = "Field `RESERVED29` reader - 31:29\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved29R = crate::FieldReader;
#[doc = "Field `RESERVED29` writer - 31:29\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved29W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Trigger Electronic Codebook This field specifies one or more actions that indirectly trigger ECB operation It is allowed to configure this field with an OR-combination of supported enums."]
    #[inline(always)]
    pub fn trgecb(&self) -> TrgecbR {
        TrgecbR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - 5:4\\]
Electronic Codebook Source This field specifies the data source to hardware-triggered ECB encryptions. Non-enumerated values are not supported and ignored."]
    #[inline(always)]
    pub fn ecbsrc(&self) -> EcbsrcR {
        EcbsrcR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Trigger for TXT This field determines if and when hardware automatically XORs BUF into TXT. Non-enumerated values are not supported and ignored. It is allowed to configure this field with an OR-combination of supported enums."]
    #[inline(always)]
    pub fn trgtxt(&self) -> TrgtxtR {
        TrgtxtR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:16 - 16:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved11(&self) -> Reserved11R {
        Reserved11R::new(((self.bits >> 10) & 0x7f) as u8)
    }
    #[doc = "Bit 17 - 17:17\\]
Counter Endianness Specifies Endianness of counter"]
    #[inline(always)]
    pub fn ctrendian(&self) -> CtrendianR {
        CtrendianR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Counter Alignment Specifies alignment of counter"]
    #[inline(always)]
    pub fn ctralign(&self) -> CtralignR {
        CtralignR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:21 - 21:19\\]
Counter Size Configures size of counter as either 8,16,32,64 or 128 Non-enumerated values are not supported and ignored"]
    #[inline(always)]
    pub fn ctrsize(&self) -> CtrsizeR {
        CtrsizeR::new(((self.bits >> 19) & 7) as u8)
    }
    #[doc = "Bits 22:23 - 23:22\\]
Reserved for future use. Any writes to this field are ignored"]
    #[inline(always)]
    pub fn reserved22(&self) -> Reserved22R {
        Reserved22R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bit 24 - 24:24\\]
Bus Halt This field decides if bus halts on access to KEY, TXT, BUF, TXTX and TXTXBUF when STA.STATE = BUSY."]
    #[inline(always)]
    pub fn bushalt(&self) -> BushaltR {
        BushaltR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
This field enables auto-clear of RIS.ECBDONE interrupt on read/write of TXT3/BUF3/TXTX3/TXTXBUF3 ."]
    #[inline(always)]
    pub fn ecbdoneclr(&self) -> EcbdoneclrR {
        EcbdoneclrR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 26:26\\]
This field enables auto-clear of RIS.ECBSTART interrupt on read/write of TXT3/BUF3/TXTX3/TXTXBUF3 ."]
    #[inline(always)]
    pub fn ecbstartclr(&self) -> EcbstartclrR {
        EcbstartclrR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - 27:27\\]
This field enables auto-clear of RIS.CHADONE interrupt on read/write of TXT3/BUF3/TXTX3/TXTXBUF3 ."]
    #[inline(always)]
    pub fn chadoneclr(&self) -> ChadoneclrR {
        ChadoneclrR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - 28:28\\]
This field enable auto-clear of RIS.CHBDONE interrupt on read/write of TXT3/BUF3/TXTX3/TXTXBUF3 ."]
    #[inline(always)]
    pub fn chbdoneclr(&self) -> ChbdoneclrR {
        ChbdoneclrR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bits 29:31 - 31:29\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved29(&self) -> Reserved29R {
        Reserved29R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Trigger Electronic Codebook This field specifies one or more actions that indirectly trigger ECB operation It is allowed to configure this field with an OR-combination of supported enums."]
    #[inline(always)]
    #[must_use]
    pub fn trgecb(&mut self) -> TrgecbW<AutocfgSpec> {
        TrgecbW::new(self, 0)
    }
    #[doc = "Bits 4:5 - 5:4\\]
Electronic Codebook Source This field specifies the data source to hardware-triggered ECB encryptions. Non-enumerated values are not supported and ignored."]
    #[inline(always)]
    #[must_use]
    pub fn ecbsrc(&mut self) -> EcbsrcW<AutocfgSpec> {
        EcbsrcW::new(self, 4)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved6(&mut self) -> Reserved6W<AutocfgSpec> {
        Reserved6W::new(self, 6)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Trigger for TXT This field determines if and when hardware automatically XORs BUF into TXT. Non-enumerated values are not supported and ignored. It is allowed to configure this field with an OR-combination of supported enums."]
    #[inline(always)]
    #[must_use]
    pub fn trgtxt(&mut self) -> TrgtxtW<AutocfgSpec> {
        TrgtxtW::new(self, 8)
    }
    #[doc = "Bits 10:16 - 16:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved11(&mut self) -> Reserved11W<AutocfgSpec> {
        Reserved11W::new(self, 10)
    }
    #[doc = "Bit 17 - 17:17\\]
Counter Endianness Specifies Endianness of counter"]
    #[inline(always)]
    #[must_use]
    pub fn ctrendian(&mut self) -> CtrendianW<AutocfgSpec> {
        CtrendianW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
Counter Alignment Specifies alignment of counter"]
    #[inline(always)]
    #[must_use]
    pub fn ctralign(&mut self) -> CtralignW<AutocfgSpec> {
        CtralignW::new(self, 18)
    }
    #[doc = "Bits 19:21 - 21:19\\]
Counter Size Configures size of counter as either 8,16,32,64 or 128 Non-enumerated values are not supported and ignored"]
    #[inline(always)]
    #[must_use]
    pub fn ctrsize(&mut self) -> CtrsizeW<AutocfgSpec> {
        CtrsizeW::new(self, 19)
    }
    #[doc = "Bits 22:23 - 23:22\\]
Reserved for future use. Any writes to this field are ignored"]
    #[inline(always)]
    #[must_use]
    pub fn reserved22(&mut self) -> Reserved22W<AutocfgSpec> {
        Reserved22W::new(self, 22)
    }
    #[doc = "Bit 24 - 24:24\\]
Bus Halt This field decides if bus halts on access to KEY, TXT, BUF, TXTX and TXTXBUF when STA.STATE = BUSY."]
    #[inline(always)]
    #[must_use]
    pub fn bushalt(&mut self) -> BushaltW<AutocfgSpec> {
        BushaltW::new(self, 24)
    }
    #[doc = "Bit 25 - 25:25\\]
This field enables auto-clear of RIS.ECBDONE interrupt on read/write of TXT3/BUF3/TXTX3/TXTXBUF3 ."]
    #[inline(always)]
    #[must_use]
    pub fn ecbdoneclr(&mut self) -> EcbdoneclrW<AutocfgSpec> {
        EcbdoneclrW::new(self, 25)
    }
    #[doc = "Bit 26 - 26:26\\]
This field enables auto-clear of RIS.ECBSTART interrupt on read/write of TXT3/BUF3/TXTX3/TXTXBUF3 ."]
    #[inline(always)]
    #[must_use]
    pub fn ecbstartclr(&mut self) -> EcbstartclrW<AutocfgSpec> {
        EcbstartclrW::new(self, 26)
    }
    #[doc = "Bit 27 - 27:27\\]
This field enables auto-clear of RIS.CHADONE interrupt on read/write of TXT3/BUF3/TXTX3/TXTXBUF3 ."]
    #[inline(always)]
    #[must_use]
    pub fn chadoneclr(&mut self) -> ChadoneclrW<AutocfgSpec> {
        ChadoneclrW::new(self, 27)
    }
    #[doc = "Bit 28 - 28:28\\]
This field enable auto-clear of RIS.CHBDONE interrupt on read/write of TXT3/BUF3/TXTX3/TXTXBUF3 ."]
    #[inline(always)]
    #[must_use]
    pub fn chbdoneclr(&mut self) -> ChbdoneclrW<AutocfgSpec> {
        ChbdoneclrW::new(self, 28)
    }
    #[doc = "Bits 29:31 - 31:29\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved29(&mut self) -> Reserved29W<AutocfgSpec> {
        Reserved29W::new(self, 29)
    }
}
#[doc = "Automatic Configuration This register configures automatic hardware updates to TXT and BUF. Configure this register to reduce software overhead during cipher modes.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`autocfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`autocfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AutocfgSpec;
impl crate::RegisterSpec for AutocfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`autocfg::R`](R) reader structure"]
impl crate::Readable for AutocfgSpec {}
#[doc = "`write(|w| ..)` method takes [`autocfg::W`](W) writer structure"]
impl crate::Writable for AutocfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AUTOCFG to value 0"]
impl crate::Resettable for AutocfgSpec {
    const RESET_VALUE: u32 = 0;
}
