#[doc = "Register `RFHWIFG` reader"]
pub struct R(crate::R<RFHWIFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RFHWIFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RFHWIFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RFHWIFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RFHWIFG` writer"]
pub struct W(crate::W<RFHWIFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RFHWIFG_SPEC>;
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
impl From<crate::W<RFHWIFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RFHWIFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED0` reader - 0:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED0_R = crate::BitReader<bool>;
#[doc = "Field `RESERVED0` writer - 0:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED0_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFHWIFG_SPEC, bool, O>;
#[doc = "Field `FSCA` reader - 1:1\\]
Frequency synthesizer calibration accelerator interrupt flag. Write zero to clear flag. Write to one has no effect."]
pub type FSCA_R = crate::BitReader<bool>;
#[doc = "Field `FSCA` writer - 1:1\\]
Frequency synthesizer calibration accelerator interrupt flag. Write zero to clear flag. Write to one has no effect."]
pub type FSCA_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFHWIFG_SPEC, bool, O>;
#[doc = "Field `MDMDONE` reader - 2:2\\]
Modem command done interrupt flag. Write zero to clear flag. Write to one has no effect."]
pub type MDMDONE_R = crate::BitReader<bool>;
#[doc = "Field `MDMDONE` writer - 2:2\\]
Modem command done interrupt flag. Write zero to clear flag. Write to one has no effect."]
pub type MDMDONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFHWIFG_SPEC, bool, O>;
#[doc = "Field `MDMIN` reader - 3:3\\]
Modem FIFO input interrupt flag. Write zero to clear flag. Write to one has no effect."]
pub type MDMIN_R = crate::BitReader<bool>;
#[doc = "Field `MDMIN` writer - 3:3\\]
Modem FIFO input interrupt flag. Write zero to clear flag. Write to one has no effect."]
pub type MDMIN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFHWIFG_SPEC, bool, O>;
#[doc = "Field `MDMOUT` reader - 4:4\\]
Modem FIFO output interrupt flag. Write zero to clear flag. Write to one has no effect."]
pub type MDMOUT_R = crate::BitReader<bool>;
#[doc = "Field `MDMOUT` writer - 4:4\\]
Modem FIFO output interrupt flag. Write zero to clear flag. Write to one has no effect."]
pub type MDMOUT_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFHWIFG_SPEC, bool, O>;
#[doc = "Field `MDMSOFT` reader - 5:5\\]
Modem synchronization word detection interrupt flag. This interrupt will be raised by modem when the synchronization word is received. The CPE may decide to reject the packet based on its header (protocol specific). Write zero to clear flag. Write to one has no effect."]
pub type MDMSOFT_R = crate::BitReader<bool>;
#[doc = "Field `MDMSOFT` writer - 5:5\\]
Modem synchronization word detection interrupt flag. This interrupt will be raised by modem when the synchronization word is received. The CPE may decide to reject the packet based on its header (protocol specific). Write zero to clear flag. Write to one has no effect."]
pub type MDMSOFT_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFHWIFG_SPEC, bool, O>;
#[doc = "Field `TRCTK` reader - 6:6\\]
Debug tracer system tick interrupt flag. Write zero to clear flag. Write to one has no effect."]
pub type TRCTK_R = crate::BitReader<bool>;
#[doc = "Field `TRCTK` writer - 6:6\\]
Debug tracer system tick interrupt flag. Write zero to clear flag. Write to one has no effect."]
pub type TRCTK_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFHWIFG_SPEC, bool, O>;
#[doc = "Field `RESERVED7` reader - 7:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED7_R = crate::BitReader<bool>;
#[doc = "Field `RESERVED7` writer - 7:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED7_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFHWIFG_SPEC, bool, O>;
#[doc = "Field `RFEDONE` reader - 8:8\\]
RF engine command done interrupt flag. Write zero to clear flag. Write to one has no effect."]
pub type RFEDONE_R = crate::BitReader<bool>;
#[doc = "Field `RFEDONE` writer - 8:8\\]
RF engine command done interrupt flag. Write zero to clear flag. Write to one has no effect."]
pub type RFEDONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFHWIFG_SPEC, bool, O>;
#[doc = "Field `RFESOFT0` reader - 9:9\\]
RF engine software defined interrupt 0 flag. Write zero to clear flag. Write to one has no effect."]
pub type RFESOFT0_R = crate::BitReader<bool>;
#[doc = "Field `RFESOFT0` writer - 9:9\\]
RF engine software defined interrupt 0 flag. Write zero to clear flag. Write to one has no effect."]
pub type RFESOFT0_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFHWIFG_SPEC, bool, O>;
#[doc = "Field `RFESOFT1` reader - 10:10\\]
RF engine software defined interrupt 1 flag. Write zero to clear flag. Write to one has no effect."]
pub type RFESOFT1_R = crate::BitReader<bool>;
#[doc = "Field `RFESOFT1` writer - 10:10\\]
RF engine software defined interrupt 1 flag. Write zero to clear flag. Write to one has no effect."]
pub type RFESOFT1_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFHWIFG_SPEC, bool, O>;
#[doc = "Field `RFESOFT2` reader - 11:11\\]
RF engine software defined interrupt 2 flag. Write zero to clear flag. Write to one has no effect."]
pub type RFESOFT2_R = crate::BitReader<bool>;
#[doc = "Field `RFESOFT2` writer - 11:11\\]
RF engine software defined interrupt 2 flag. Write zero to clear flag. Write to one has no effect."]
pub type RFESOFT2_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFHWIFG_SPEC, bool, O>;
#[doc = "Field `RATCH0` reader - 12:12\\]
Radio timer channel 0 interrupt flag. Write zero to clear flag. Write to one has no effect."]
pub type RATCH0_R = crate::BitReader<bool>;
#[doc = "Field `RATCH0` writer - 12:12\\]
Radio timer channel 0 interrupt flag. Write zero to clear flag. Write to one has no effect."]
pub type RATCH0_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFHWIFG_SPEC, bool, O>;
#[doc = "Field `RATCH1` reader - 13:13\\]
Radio timer channel 1 interrupt flag. Write zero to clear flag. Write to one has no effect."]
pub type RATCH1_R = crate::BitReader<bool>;
#[doc = "Field `RATCH1` writer - 13:13\\]
Radio timer channel 1 interrupt flag. Write zero to clear flag. Write to one has no effect."]
pub type RATCH1_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFHWIFG_SPEC, bool, O>;
#[doc = "Field `RATCH2` reader - 14:14\\]
Radio timer channel 2 interrupt flag. Write zero to clear flag. Write to one has no effect."]
pub type RATCH2_R = crate::BitReader<bool>;
#[doc = "Field `RATCH2` writer - 14:14\\]
Radio timer channel 2 interrupt flag. Write zero to clear flag. Write to one has no effect."]
pub type RATCH2_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFHWIFG_SPEC, bool, O>;
#[doc = "Field `RATCH3` reader - 15:15\\]
Radio timer channel 3 interrupt flag. Write zero to clear flag. Write to one has no effect."]
pub type RATCH3_R = crate::BitReader<bool>;
#[doc = "Field `RATCH3` writer - 15:15\\]
Radio timer channel 3 interrupt flag. Write zero to clear flag. Write to one has no effect."]
pub type RATCH3_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFHWIFG_SPEC, bool, O>;
#[doc = "Field `RATCH4` reader - 16:16\\]
Radio timer channel 4 interrupt flag. Write zero to clear flag. Write to one has no effect."]
pub type RATCH4_R = crate::BitReader<bool>;
#[doc = "Field `RATCH4` writer - 16:16\\]
Radio timer channel 4 interrupt flag. Write zero to clear flag. Write to one has no effect."]
pub type RATCH4_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFHWIFG_SPEC, bool, O>;
#[doc = "Field `RATCH5` reader - 17:17\\]
Radio timer channel 5 interrupt flag. Write zero to clear flag. Write to one has no effect."]
pub type RATCH5_R = crate::BitReader<bool>;
#[doc = "Field `RATCH5` writer - 17:17\\]
Radio timer channel 5 interrupt flag. Write zero to clear flag. Write to one has no effect."]
pub type RATCH5_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFHWIFG_SPEC, bool, O>;
#[doc = "Field `RATCH6` reader - 18:18\\]
Radio timer channel 6 interrupt flag. Write zero to clear flag. Write to one has no effect."]
pub type RATCH6_R = crate::BitReader<bool>;
#[doc = "Field `RATCH6` writer - 18:18\\]
Radio timer channel 6 interrupt flag. Write zero to clear flag. Write to one has no effect."]
pub type RATCH6_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFHWIFG_SPEC, bool, O>;
#[doc = "Field `RATCH7` reader - 19:19\\]
Radio timer channel 7 interrupt flag. Write zero to clear flag. Write to one has no effect."]
pub type RATCH7_R = crate::BitReader<bool>;
#[doc = "Field `RATCH7` writer - 19:19\\]
Radio timer channel 7 interrupt flag. Write zero to clear flag. Write to one has no effect."]
pub type RATCH7_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFHWIFG_SPEC, bool, O>;
#[doc = "Field `RESERVED20` reader - 31:20\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED20_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RESERVED20` writer - 31:20\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED20_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RFHWIFG_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&self) -> RESERVED0_R {
        RESERVED0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Frequency synthesizer calibration accelerator interrupt flag. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn fsca(&self) -> FSCA_R {
        FSCA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Modem command done interrupt flag. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn mdmdone(&self) -> MDMDONE_R {
        MDMDONE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Modem FIFO input interrupt flag. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn mdmin(&self) -> MDMIN_R {
        MDMIN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Modem FIFO output interrupt flag. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn mdmout(&self) -> MDMOUT_R {
        MDMOUT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Modem synchronization word detection interrupt flag. This interrupt will be raised by modem when the synchronization word is received. The CPE may decide to reject the packet based on its header (protocol specific). Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn mdmsoft(&self) -> MDMSOFT_R {
        MDMSOFT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Debug tracer system tick interrupt flag. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn trctk(&self) -> TRCTK_R {
        TRCTK_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved7(&self) -> RESERVED7_R {
        RESERVED7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
RF engine command done interrupt flag. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn rfedone(&self) -> RFEDONE_R {
        RFEDONE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
RF engine software defined interrupt 0 flag. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn rfesoft0(&self) -> RFESOFT0_R {
        RFESOFT0_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
RF engine software defined interrupt 1 flag. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn rfesoft1(&self) -> RFESOFT1_R {
        RFESOFT1_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
RF engine software defined interrupt 2 flag. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn rfesoft2(&self) -> RFESOFT2_R {
        RFESOFT2_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Radio timer channel 0 interrupt flag. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn ratch0(&self) -> RATCH0_R {
        RATCH0_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Radio timer channel 1 interrupt flag. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn ratch1(&self) -> RATCH1_R {
        RATCH1_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
Radio timer channel 2 interrupt flag. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn ratch2(&self) -> RATCH2_R {
        RATCH2_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Radio timer channel 3 interrupt flag. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn ratch3(&self) -> RATCH3_R {
        RATCH3_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Radio timer channel 4 interrupt flag. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn ratch4(&self) -> RATCH4_R {
        RATCH4_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Radio timer channel 5 interrupt flag. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn ratch5(&self) -> RATCH5_R {
        RATCH5_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Radio timer channel 6 interrupt flag. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn ratch6(&self) -> RATCH6_R {
        RATCH6_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
Radio timer channel 7 interrupt flag. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn ratch7(&self) -> RATCH7_R {
        RATCH7_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:31 - 31:20\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved20(&self) -> RESERVED20_R {
        RESERVED20_R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved0(&mut self) -> RESERVED0_W<0> {
        RESERVED0_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
Frequency synthesizer calibration accelerator interrupt flag. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn fsca(&mut self) -> FSCA_W<1> {
        FSCA_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
Modem command done interrupt flag. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn mdmdone(&mut self) -> MDMDONE_W<2> {
        MDMDONE_W::new(self)
    }
    #[doc = "Bit 3 - 3:3\\]
Modem FIFO input interrupt flag. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn mdmin(&mut self) -> MDMIN_W<3> {
        MDMIN_W::new(self)
    }
    #[doc = "Bit 4 - 4:4\\]
Modem FIFO output interrupt flag. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn mdmout(&mut self) -> MDMOUT_W<4> {
        MDMOUT_W::new(self)
    }
    #[doc = "Bit 5 - 5:5\\]
Modem synchronization word detection interrupt flag. This interrupt will be raised by modem when the synchronization word is received. The CPE may decide to reject the packet based on its header (protocol specific). Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn mdmsoft(&mut self) -> MDMSOFT_W<5> {
        MDMSOFT_W::new(self)
    }
    #[doc = "Bit 6 - 6:6\\]
Debug tracer system tick interrupt flag. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn trctk(&mut self) -> TRCTK_W<6> {
        TRCTK_W::new(self)
    }
    #[doc = "Bit 7 - 7:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved7(&mut self) -> RESERVED7_W<7> {
        RESERVED7_W::new(self)
    }
    #[doc = "Bit 8 - 8:8\\]
RF engine command done interrupt flag. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn rfedone(&mut self) -> RFEDONE_W<8> {
        RFEDONE_W::new(self)
    }
    #[doc = "Bit 9 - 9:9\\]
RF engine software defined interrupt 0 flag. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn rfesoft0(&mut self) -> RFESOFT0_W<9> {
        RFESOFT0_W::new(self)
    }
    #[doc = "Bit 10 - 10:10\\]
RF engine software defined interrupt 1 flag. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn rfesoft1(&mut self) -> RFESOFT1_W<10> {
        RFESOFT1_W::new(self)
    }
    #[doc = "Bit 11 - 11:11\\]
RF engine software defined interrupt 2 flag. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn rfesoft2(&mut self) -> RFESOFT2_W<11> {
        RFESOFT2_W::new(self)
    }
    #[doc = "Bit 12 - 12:12\\]
Radio timer channel 0 interrupt flag. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn ratch0(&mut self) -> RATCH0_W<12> {
        RATCH0_W::new(self)
    }
    #[doc = "Bit 13 - 13:13\\]
Radio timer channel 1 interrupt flag. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn ratch1(&mut self) -> RATCH1_W<13> {
        RATCH1_W::new(self)
    }
    #[doc = "Bit 14 - 14:14\\]
Radio timer channel 2 interrupt flag. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn ratch2(&mut self) -> RATCH2_W<14> {
        RATCH2_W::new(self)
    }
    #[doc = "Bit 15 - 15:15\\]
Radio timer channel 3 interrupt flag. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn ratch3(&mut self) -> RATCH3_W<15> {
        RATCH3_W::new(self)
    }
    #[doc = "Bit 16 - 16:16\\]
Radio timer channel 4 interrupt flag. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn ratch4(&mut self) -> RATCH4_W<16> {
        RATCH4_W::new(self)
    }
    #[doc = "Bit 17 - 17:17\\]
Radio timer channel 5 interrupt flag. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn ratch5(&mut self) -> RATCH5_W<17> {
        RATCH5_W::new(self)
    }
    #[doc = "Bit 18 - 18:18\\]
Radio timer channel 6 interrupt flag. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn ratch6(&mut self) -> RATCH6_W<18> {
        RATCH6_W::new(self)
    }
    #[doc = "Bit 19 - 19:19\\]
Radio timer channel 7 interrupt flag. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn ratch7(&mut self) -> RATCH7_W<19> {
        RATCH7_W::new(self)
    }
    #[doc = "Bits 20:31 - 31:20\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved20(&mut self) -> RESERVED20_W<20> {
        RESERVED20_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Flags From RF Hardware Modules\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rfhwifg](index.html) module"]
pub struct RFHWIFG_SPEC;
impl crate::RegisterSpec for RFHWIFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rfhwifg::R](R) reader structure"]
impl crate::Readable for RFHWIFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rfhwifg::W](W) writer structure"]
impl crate::Writable for RFHWIFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RFHWIFG to value 0"]
impl crate::Resettable for RFHWIFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
