#[doc = "Register `IMR` reader"]
pub type R = crate::R<ImrSpec>;
#[doc = "Register `IMR` writer"]
pub type W = crate::W<ImrSpec>;
#[doc = "0:0\\]
Enabling this bit will make the RIS.TATORIS interrupt propagate to MIS.TATOMIS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tatoim {
    #[doc = "1: Enable Interrupt"]
    En = 1,
    #[doc = "0: Disable Interrupt"]
    Dis = 0,
}
impl From<Tatoim> for bool {
    #[inline(always)]
    fn from(variant: Tatoim) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TATOIM` reader - 0:0\\]
Enabling this bit will make the RIS.TATORIS interrupt propagate to MIS.TATOMIS"]
pub type TatoimR = crate::BitReader<Tatoim>;
impl TatoimR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tatoim {
        match self.bits {
            true => Tatoim::En,
            false => Tatoim::Dis,
        }
    }
    #[doc = "Enable Interrupt"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Tatoim::En
    }
    #[doc = "Disable Interrupt"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Tatoim::Dis
    }
}
#[doc = "Field `TATOIM` writer - 0:0\\]
Enabling this bit will make the RIS.TATORIS interrupt propagate to MIS.TATOMIS"]
pub type TatoimW<'a, REG> = crate::BitWriter<'a, REG, Tatoim>;
impl<'a, REG> TatoimW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable Interrupt"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Tatoim::En)
    }
    #[doc = "Disable Interrupt"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Tatoim::Dis)
    }
}
#[doc = "1:1\\]
Enabling this bit will make the RIS.CAMRIS interrupt propagate to MIS.CAMMIS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Camim {
    #[doc = "1: Enable Interrupt"]
    En = 1,
    #[doc = "0: Disable Interrupt"]
    Dis = 0,
}
impl From<Camim> for bool {
    #[inline(always)]
    fn from(variant: Camim) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAMIM` reader - 1:1\\]
Enabling this bit will make the RIS.CAMRIS interrupt propagate to MIS.CAMMIS"]
pub type CamimR = crate::BitReader<Camim>;
impl CamimR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Camim {
        match self.bits {
            true => Camim::En,
            false => Camim::Dis,
        }
    }
    #[doc = "Enable Interrupt"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Camim::En
    }
    #[doc = "Disable Interrupt"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Camim::Dis
    }
}
#[doc = "Field `CAMIM` writer - 1:1\\]
Enabling this bit will make the RIS.CAMRIS interrupt propagate to MIS.CAMMIS"]
pub type CamimW<'a, REG> = crate::BitWriter<'a, REG, Camim>;
impl<'a, REG> CamimW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable Interrupt"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Camim::En)
    }
    #[doc = "Disable Interrupt"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Camim::Dis)
    }
}
#[doc = "2:2\\]
Enabling this bit will make the RIS.CAERIS interrupt propagate to MIS.CAEMIS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Caeim {
    #[doc = "1: Enable Interrupt"]
    En = 1,
    #[doc = "0: Disable Interrupt"]
    Dis = 0,
}
impl From<Caeim> for bool {
    #[inline(always)]
    fn from(variant: Caeim) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAEIM` reader - 2:2\\]
Enabling this bit will make the RIS.CAERIS interrupt propagate to MIS.CAEMIS"]
pub type CaeimR = crate::BitReader<Caeim>;
impl CaeimR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Caeim {
        match self.bits {
            true => Caeim::En,
            false => Caeim::Dis,
        }
    }
    #[doc = "Enable Interrupt"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Caeim::En
    }
    #[doc = "Disable Interrupt"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Caeim::Dis
    }
}
#[doc = "Field `CAEIM` writer - 2:2\\]
Enabling this bit will make the RIS.CAERIS interrupt propagate to MIS.CAEMIS"]
pub type CaeimW<'a, REG> = crate::BitWriter<'a, REG, Caeim>;
impl<'a, REG> CaeimW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable Interrupt"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Caeim::En)
    }
    #[doc = "Disable Interrupt"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Caeim::Dis)
    }
}
#[doc = "Field `RESERVED3` reader - 3:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `RESERVED3` writer - 3:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "4:4\\]
Enabling this bit will make the RIS.TAMRIS interrupt propagate to MIS.TAMMIS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tamim {
    #[doc = "1: Enable Interrupt"]
    En = 1,
    #[doc = "0: Disable Interrupt"]
    Dis = 0,
}
impl From<Tamim> for bool {
    #[inline(always)]
    fn from(variant: Tamim) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TAMIM` reader - 4:4\\]
Enabling this bit will make the RIS.TAMRIS interrupt propagate to MIS.TAMMIS"]
pub type TamimR = crate::BitReader<Tamim>;
impl TamimR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tamim {
        match self.bits {
            true => Tamim::En,
            false => Tamim::Dis,
        }
    }
    #[doc = "Enable Interrupt"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Tamim::En
    }
    #[doc = "Disable Interrupt"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Tamim::Dis
    }
}
#[doc = "Field `TAMIM` writer - 4:4\\]
Enabling this bit will make the RIS.TAMRIS interrupt propagate to MIS.TAMMIS"]
pub type TamimW<'a, REG> = crate::BitWriter<'a, REG, Tamim>;
impl<'a, REG> TamimW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable Interrupt"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Tamim::En)
    }
    #[doc = "Disable Interrupt"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Tamim::Dis)
    }
}
#[doc = "5:5\\]
Enabling this bit will make the RIS.DMAARIS interrupt propagate to MIS.DMAAMIS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmaaim {
    #[doc = "1: Enable Interrupt"]
    En = 1,
    #[doc = "0: Disable Interrupt"]
    Dis = 0,
}
impl From<Dmaaim> for bool {
    #[inline(always)]
    fn from(variant: Dmaaim) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAAIM` reader - 5:5\\]
Enabling this bit will make the RIS.DMAARIS interrupt propagate to MIS.DMAAMIS"]
pub type DmaaimR = crate::BitReader<Dmaaim>;
impl DmaaimR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmaaim {
        match self.bits {
            true => Dmaaim::En,
            false => Dmaaim::Dis,
        }
    }
    #[doc = "Enable Interrupt"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Dmaaim::En
    }
    #[doc = "Disable Interrupt"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Dmaaim::Dis
    }
}
#[doc = "Field `DMAAIM` writer - 5:5\\]
Enabling this bit will make the RIS.DMAARIS interrupt propagate to MIS.DMAAMIS"]
pub type DmaaimW<'a, REG> = crate::BitWriter<'a, REG, Dmaaim>;
impl<'a, REG> DmaaimW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable Interrupt"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaaim::En)
    }
    #[doc = "Disable Interrupt"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaaim::Dis)
    }
}
#[doc = "Field `RESERVED6` reader - 7:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved6R = crate::FieldReader;
#[doc = "Field `RESERVED6` writer - 7:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved6W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "8:8\\]
Enabling this bit will make the RIS.TBTORIS interrupt propagate to MIS.TBTOMIS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tbtoim {
    #[doc = "1: Enable Interrupt"]
    En = 1,
    #[doc = "0: Disable Interrupt"]
    Dis = 0,
}
impl From<Tbtoim> for bool {
    #[inline(always)]
    fn from(variant: Tbtoim) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TBTOIM` reader - 8:8\\]
Enabling this bit will make the RIS.TBTORIS interrupt propagate to MIS.TBTOMIS"]
pub type TbtoimR = crate::BitReader<Tbtoim>;
impl TbtoimR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tbtoim {
        match self.bits {
            true => Tbtoim::En,
            false => Tbtoim::Dis,
        }
    }
    #[doc = "Enable Interrupt"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Tbtoim::En
    }
    #[doc = "Disable Interrupt"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Tbtoim::Dis
    }
}
#[doc = "Field `TBTOIM` writer - 8:8\\]
Enabling this bit will make the RIS.TBTORIS interrupt propagate to MIS.TBTOMIS"]
pub type TbtoimW<'a, REG> = crate::BitWriter<'a, REG, Tbtoim>;
impl<'a, REG> TbtoimW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable Interrupt"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Tbtoim::En)
    }
    #[doc = "Disable Interrupt"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Tbtoim::Dis)
    }
}
#[doc = "9:9\\]
Enabling this bit will make the RIS.CBMRIS interrupt propagate to MIS.CBMMIS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cbmim {
    #[doc = "1: Enable Interrupt"]
    En = 1,
    #[doc = "0: Disable Interrupt"]
    Dis = 0,
}
impl From<Cbmim> for bool {
    #[inline(always)]
    fn from(variant: Cbmim) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CBMIM` reader - 9:9\\]
Enabling this bit will make the RIS.CBMRIS interrupt propagate to MIS.CBMMIS"]
pub type CbmimR = crate::BitReader<Cbmim>;
impl CbmimR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cbmim {
        match self.bits {
            true => Cbmim::En,
            false => Cbmim::Dis,
        }
    }
    #[doc = "Enable Interrupt"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Cbmim::En
    }
    #[doc = "Disable Interrupt"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Cbmim::Dis
    }
}
#[doc = "Field `CBMIM` writer - 9:9\\]
Enabling this bit will make the RIS.CBMRIS interrupt propagate to MIS.CBMMIS"]
pub type CbmimW<'a, REG> = crate::BitWriter<'a, REG, Cbmim>;
impl<'a, REG> CbmimW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable Interrupt"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Cbmim::En)
    }
    #[doc = "Disable Interrupt"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Cbmim::Dis)
    }
}
#[doc = "10:10\\]
Enabling this bit will make the RIS.CBERIS interrupt propagate to MIS.CBEMIS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cbeim {
    #[doc = "1: Enable Interrupt"]
    En = 1,
    #[doc = "0: Disable Interrupt"]
    Dis = 0,
}
impl From<Cbeim> for bool {
    #[inline(always)]
    fn from(variant: Cbeim) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CBEIM` reader - 10:10\\]
Enabling this bit will make the RIS.CBERIS interrupt propagate to MIS.CBEMIS"]
pub type CbeimR = crate::BitReader<Cbeim>;
impl CbeimR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cbeim {
        match self.bits {
            true => Cbeim::En,
            false => Cbeim::Dis,
        }
    }
    #[doc = "Enable Interrupt"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Cbeim::En
    }
    #[doc = "Disable Interrupt"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Cbeim::Dis
    }
}
#[doc = "Field `CBEIM` writer - 10:10\\]
Enabling this bit will make the RIS.CBERIS interrupt propagate to MIS.CBEMIS"]
pub type CbeimW<'a, REG> = crate::BitWriter<'a, REG, Cbeim>;
impl<'a, REG> CbeimW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable Interrupt"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Cbeim::En)
    }
    #[doc = "Disable Interrupt"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Cbeim::Dis)
    }
}
#[doc = "11:11\\]
Enabling this bit will make the RIS.TBMRIS interrupt propagate to MIS.TBMMIS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tbmim {
    #[doc = "1: Enable Interrupt"]
    En = 1,
    #[doc = "0: Disable Interrupt"]
    Dis = 0,
}
impl From<Tbmim> for bool {
    #[inline(always)]
    fn from(variant: Tbmim) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TBMIM` reader - 11:11\\]
Enabling this bit will make the RIS.TBMRIS interrupt propagate to MIS.TBMMIS"]
pub type TbmimR = crate::BitReader<Tbmim>;
impl TbmimR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tbmim {
        match self.bits {
            true => Tbmim::En,
            false => Tbmim::Dis,
        }
    }
    #[doc = "Enable Interrupt"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Tbmim::En
    }
    #[doc = "Disable Interrupt"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Tbmim::Dis
    }
}
#[doc = "Field `TBMIM` writer - 11:11\\]
Enabling this bit will make the RIS.TBMRIS interrupt propagate to MIS.TBMMIS"]
pub type TbmimW<'a, REG> = crate::BitWriter<'a, REG, Tbmim>;
impl<'a, REG> TbmimW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable Interrupt"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Tbmim::En)
    }
    #[doc = "Disable Interrupt"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Tbmim::Dis)
    }
}
#[doc = "Field `RESERVED12` reader - 12:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved12R = crate::BitReader;
#[doc = "Field `RESERVED12` writer - 12:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "13:13\\]
Enabling this bit will make the RIS.DMABRIS interrupt propagate to MIS.DMABMIS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmabim {
    #[doc = "1: Enable Interrupt"]
    En = 1,
    #[doc = "0: Disable Interrupt"]
    Dis = 0,
}
impl From<Dmabim> for bool {
    #[inline(always)]
    fn from(variant: Dmabim) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMABIM` reader - 13:13\\]
Enabling this bit will make the RIS.DMABRIS interrupt propagate to MIS.DMABMIS"]
pub type DmabimR = crate::BitReader<Dmabim>;
impl DmabimR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmabim {
        match self.bits {
            true => Dmabim::En,
            false => Dmabim::Dis,
        }
    }
    #[doc = "Enable Interrupt"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Dmabim::En
    }
    #[doc = "Disable Interrupt"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Dmabim::Dis
    }
}
#[doc = "Field `DMABIM` writer - 13:13\\]
Enabling this bit will make the RIS.DMABRIS interrupt propagate to MIS.DMABMIS"]
pub type DmabimW<'a, REG> = crate::BitWriter<'a, REG, Dmabim>;
impl<'a, REG> DmabimW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable Interrupt"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Dmabim::En)
    }
    #[doc = "Disable Interrupt"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Dmabim::Dis)
    }
}
#[doc = "Field `RESERVED14` reader - 31:14\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved14R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED14` writer - 31:14\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved14W<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Enabling this bit will make the RIS.TATORIS interrupt propagate to MIS.TATOMIS"]
    #[inline(always)]
    pub fn tatoim(&self) -> TatoimR {
        TatoimR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Enabling this bit will make the RIS.CAMRIS interrupt propagate to MIS.CAMMIS"]
    #[inline(always)]
    pub fn camim(&self) -> CamimR {
        CamimR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Enabling this bit will make the RIS.CAERIS interrupt propagate to MIS.CAEMIS"]
    #[inline(always)]
    pub fn caeim(&self) -> CaeimR {
        CaeimR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Enabling this bit will make the RIS.TAMRIS interrupt propagate to MIS.TAMMIS"]
    #[inline(always)]
    pub fn tamim(&self) -> TamimR {
        TamimR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Enabling this bit will make the RIS.DMAARIS interrupt propagate to MIS.DMAAMIS"]
    #[inline(always)]
    pub fn dmaaim(&self) -> DmaaimR {
        DmaaimR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Enabling this bit will make the RIS.TBTORIS interrupt propagate to MIS.TBTOMIS"]
    #[inline(always)]
    pub fn tbtoim(&self) -> TbtoimR {
        TbtoimR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Enabling this bit will make the RIS.CBMRIS interrupt propagate to MIS.CBMMIS"]
    #[inline(always)]
    pub fn cbmim(&self) -> CbmimR {
        CbmimR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Enabling this bit will make the RIS.CBERIS interrupt propagate to MIS.CBEMIS"]
    #[inline(always)]
    pub fn cbeim(&self) -> CbeimR {
        CbeimR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Enabling this bit will make the RIS.TBMRIS interrupt propagate to MIS.TBMMIS"]
    #[inline(always)]
    pub fn tbmim(&self) -> TbmimR {
        TbmimR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved12(&self) -> Reserved12R {
        Reserved12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Enabling this bit will make the RIS.DMABRIS interrupt propagate to MIS.DMABMIS"]
    #[inline(always)]
    pub fn dmabim(&self) -> DmabimR {
        DmabimR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:31 - 31:14\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved14(&self) -> Reserved14R {
        Reserved14R::new((self.bits >> 14) & 0x0003_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Enabling this bit will make the RIS.TATORIS interrupt propagate to MIS.TATOMIS"]
    #[inline(always)]
    #[must_use]
    pub fn tatoim(&mut self) -> TatoimW<ImrSpec> {
        TatoimW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Enabling this bit will make the RIS.CAMRIS interrupt propagate to MIS.CAMMIS"]
    #[inline(always)]
    #[must_use]
    pub fn camim(&mut self) -> CamimW<ImrSpec> {
        CamimW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Enabling this bit will make the RIS.CAERIS interrupt propagate to MIS.CAEMIS"]
    #[inline(always)]
    #[must_use]
    pub fn caeim(&mut self) -> CaeimW<ImrSpec> {
        CaeimW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> Reserved3W<ImrSpec> {
        Reserved3W::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Enabling this bit will make the RIS.TAMRIS interrupt propagate to MIS.TAMMIS"]
    #[inline(always)]
    #[must_use]
    pub fn tamim(&mut self) -> TamimW<ImrSpec> {
        TamimW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Enabling this bit will make the RIS.DMAARIS interrupt propagate to MIS.DMAAMIS"]
    #[inline(always)]
    #[must_use]
    pub fn dmaaim(&mut self) -> DmaaimW<ImrSpec> {
        DmaaimW::new(self, 5)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved6(&mut self) -> Reserved6W<ImrSpec> {
        Reserved6W::new(self, 6)
    }
    #[doc = "Bit 8 - 8:8\\]
Enabling this bit will make the RIS.TBTORIS interrupt propagate to MIS.TBTOMIS"]
    #[inline(always)]
    #[must_use]
    pub fn tbtoim(&mut self) -> TbtoimW<ImrSpec> {
        TbtoimW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Enabling this bit will make the RIS.CBMRIS interrupt propagate to MIS.CBMMIS"]
    #[inline(always)]
    #[must_use]
    pub fn cbmim(&mut self) -> CbmimW<ImrSpec> {
        CbmimW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
Enabling this bit will make the RIS.CBERIS interrupt propagate to MIS.CBEMIS"]
    #[inline(always)]
    #[must_use]
    pub fn cbeim(&mut self) -> CbeimW<ImrSpec> {
        CbeimW::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
Enabling this bit will make the RIS.TBMRIS interrupt propagate to MIS.TBMMIS"]
    #[inline(always)]
    #[must_use]
    pub fn tbmim(&mut self) -> TbmimW<ImrSpec> {
        TbmimW::new(self, 11)
    }
    #[doc = "Bit 12 - 12:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved12(&mut self) -> Reserved12W<ImrSpec> {
        Reserved12W::new(self, 12)
    }
    #[doc = "Bit 13 - 13:13\\]
Enabling this bit will make the RIS.DMABRIS interrupt propagate to MIS.DMABMIS"]
    #[inline(always)]
    #[must_use]
    pub fn dmabim(&mut self) -> DmabimW<ImrSpec> {
        DmabimW::new(self, 13)
    }
    #[doc = "Bits 14:31 - 31:14\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved14(&mut self) -> Reserved14W<ImrSpec> {
        Reserved14W::new(self, 14)
    }
}
#[doc = "Interrupt Mask This register is used to enable the interrupts. Associated registers: RIS, MIS, ICLR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`imr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ImrSpec;
impl crate::RegisterSpec for ImrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imr::R`](R) reader structure"]
impl crate::Readable for ImrSpec {}
#[doc = "`write(|w| ..)` method takes [`imr::W`](W) writer structure"]
impl crate::Writable for ImrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IMR to value 0"]
impl crate::Resettable for ImrSpec {
    const RESET_VALUE: u32 = 0;
}
