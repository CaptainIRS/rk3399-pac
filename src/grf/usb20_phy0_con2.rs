#[doc = "Register `USB20_PHY0_CON2` reader"]
pub type R = crate::R<Usb20Phy0Con2Spec>;
#[doc = "Register `USB20_PHY0_CON2` writer"]
pub type W = crate::W<Usb20Phy0Con2Spec>;
#[doc = "utmi_sel\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UtmiSel {
    #[doc = "1: select utmi interface signals from GRF reister to usb2phy"]
    B1 = 1,
    #[doc = "0: select utmi interface signals from utmi interface of usb20 host0 controller to usb2phy"]
    B0 = 0,
}
impl From<UtmiSel> for bool {
    #[inline(always)]
    fn from(variant: UtmiSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UTMI_SEL` reader - utmi_sel"]
pub type UtmiSelR = crate::BitReader<UtmiSel>;
impl UtmiSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UtmiSel {
        match self.bits {
            true => UtmiSel::B1,
            false => UtmiSel::B0,
        }
    }
    #[doc = "select utmi interface signals from GRF reister to usb2phy"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == UtmiSel::B1
    }
    #[doc = "select utmi interface signals from utmi interface of usb20 host0 controller to usb2phy"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == UtmiSel::B0
    }
}
#[doc = "Field `UTMI_SEL` writer - utmi_sel"]
pub type UtmiSelW<'a, REG> = crate::BitWriter<'a, REG, UtmiSel>;
impl<'a, REG> UtmiSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "select utmi interface signals from GRF reister to usb2phy"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(UtmiSel::B1)
    }
    #[doc = "select utmi interface signals from utmi interface of usb20 host0 controller to usb2phy"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(UtmiSel::B0)
    }
}
#[doc = "Field `SUSPEND_N` reader - suspend_n\n\nSelect the value of this register to usb2phy\n\nwhen utmi_sel=1"]
pub type SuspendNR = crate::BitReader;
#[doc = "Field `SUSPEND_N` writer - suspend_n\n\nSelect the value of this register to usb2phy\n\nwhen utmi_sel=1"]
pub type SuspendNW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPMODE` reader - opmode\n\nSelect the value of this register to usb2phy\n\nwhen utmi_sel=1"]
pub type OpmodeR = crate::FieldReader;
#[doc = "Field `OPMODE` writer - opmode\n\nSelect the value of this register to usb2phy\n\nwhen utmi_sel=1"]
pub type OpmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `XCVRSELECT` reader - xcvrselect\n\nSelect the value of this register to usb2phy\n\nwhen utmi_sel=1"]
pub type XcvrselectR = crate::FieldReader;
#[doc = "Field `XCVRSELECT` writer - xcvrselect\n\nSelect the value of this register to usb2phy\n\nwhen utmi_sel=1"]
pub type XcvrselectW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TERMSELECT` reader - termselect\n\nSelect the value of this register to usb2phy\n\nwhen utmi_sel=1"]
pub type TermselectR = crate::BitReader;
#[doc = "Field `TERMSELECT` writer - termselect\n\nSelect the value of this register to usb2phy\n\nwhen utmi_sel=1"]
pub type TermselectW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DPPULLDOWN` reader - dppulldown\n\nUse the value of this register input to\n\ndppulldown of usb2phy"]
pub type DppulldownR = crate::BitReader;
#[doc = "Field `DPPULLDOWN` writer - dppulldown\n\nUse the value of this register input to\n\ndppulldown of usb2phy"]
pub type DppulldownW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMPULLDOWN` reader - dmpulldown\n\nUse the value of this register input to\n\ndmpulldown of usb2phy"]
pub type DmpulldownR = crate::BitReader;
#[doc = "Field `DMPULLDOWN` writer - dmpulldown\n\nUse the value of this register input to\n\ndmpulldown of usb2phy"]
pub type DmpulldownW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDPULLUP` reader - idpullup\n\nUse the value of this register input to idpullup\n\nof usb2phy"]
pub type IdpullupR = crate::BitReader;
#[doc = "Field `IDPULLUP` writer - idpullup\n\nUse the value of this register input to idpullup\n\nof usb2phy"]
pub type IdpullupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_ENABLE` reader - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - utmi_sel"]
    #[inline(always)]
    pub fn utmi_sel(&self) -> UtmiSelR {
        UtmiSelR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - suspend_n\n\nSelect the value of this register to usb2phy\n\nwhen utmi_sel=1"]
    #[inline(always)]
    pub fn suspend_n(&self) -> SuspendNR {
        SuspendNR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - opmode\n\nSelect the value of this register to usb2phy\n\nwhen utmi_sel=1"]
    #[inline(always)]
    pub fn opmode(&self) -> OpmodeR {
        OpmodeR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - xcvrselect\n\nSelect the value of this register to usb2phy\n\nwhen utmi_sel=1"]
    #[inline(always)]
    pub fn xcvrselect(&self) -> XcvrselectR {
        XcvrselectR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - termselect\n\nSelect the value of this register to usb2phy\n\nwhen utmi_sel=1"]
    #[inline(always)]
    pub fn termselect(&self) -> TermselectR {
        TermselectR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - dppulldown\n\nUse the value of this register input to\n\ndppulldown of usb2phy"]
    #[inline(always)]
    pub fn dppulldown(&self) -> DppulldownR {
        DppulldownR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - dmpulldown\n\nUse the value of this register input to\n\ndmpulldown of usb2phy"]
    #[inline(always)]
    pub fn dmpulldown(&self) -> DmpulldownR {
        DmpulldownR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - idpullup\n\nUse the value of this register input to idpullup\n\nof usb2phy"]
    #[inline(always)]
    pub fn idpullup(&self) -> IdpullupR {
        IdpullupR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    pub fn write_enable(&self) -> WriteEnableR {
        WriteEnableR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - utmi_sel"]
    #[inline(always)]
    #[must_use]
    pub fn utmi_sel(&mut self) -> UtmiSelW<Usb20Phy0Con2Spec> {
        UtmiSelW::new(self, 0)
    }
    #[doc = "Bit 1 - suspend_n\n\nSelect the value of this register to usb2phy\n\nwhen utmi_sel=1"]
    #[inline(always)]
    #[must_use]
    pub fn suspend_n(&mut self) -> SuspendNW<Usb20Phy0Con2Spec> {
        SuspendNW::new(self, 1)
    }
    #[doc = "Bits 2:3 - opmode\n\nSelect the value of this register to usb2phy\n\nwhen utmi_sel=1"]
    #[inline(always)]
    #[must_use]
    pub fn opmode(&mut self) -> OpmodeW<Usb20Phy0Con2Spec> {
        OpmodeW::new(self, 2)
    }
    #[doc = "Bits 4:5 - xcvrselect\n\nSelect the value of this register to usb2phy\n\nwhen utmi_sel=1"]
    #[inline(always)]
    #[must_use]
    pub fn xcvrselect(&mut self) -> XcvrselectW<Usb20Phy0Con2Spec> {
        XcvrselectW::new(self, 4)
    }
    #[doc = "Bit 6 - termselect\n\nSelect the value of this register to usb2phy\n\nwhen utmi_sel=1"]
    #[inline(always)]
    #[must_use]
    pub fn termselect(&mut self) -> TermselectW<Usb20Phy0Con2Spec> {
        TermselectW::new(self, 6)
    }
    #[doc = "Bit 7 - dppulldown\n\nUse the value of this register input to\n\ndppulldown of usb2phy"]
    #[inline(always)]
    #[must_use]
    pub fn dppulldown(&mut self) -> DppulldownW<Usb20Phy0Con2Spec> {
        DppulldownW::new(self, 7)
    }
    #[doc = "Bit 8 - dmpulldown\n\nUse the value of this register input to\n\ndmpulldown of usb2phy"]
    #[inline(always)]
    #[must_use]
    pub fn dmpulldown(&mut self) -> DmpulldownW<Usb20Phy0Con2Spec> {
        DmpulldownW::new(self, 8)
    }
    #[doc = "Bit 9 - idpullup\n\nUse the value of this register input to idpullup\n\nof usb2phy"]
    #[inline(always)]
    #[must_use]
    pub fn idpullup(&mut self) -> IdpullupW<Usb20Phy0Con2Spec> {
        IdpullupW::new(self, 9)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<Usb20Phy0Con2Spec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "USB20 PHY0 GRF Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb20_phy0_con2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb20_phy0_con2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Usb20Phy0Con2Spec;
impl crate::RegisterSpec for Usb20Phy0Con2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usb20_phy0_con2::R`](R) reader structure"]
impl crate::Readable for Usb20Phy0Con2Spec {}
#[doc = "`write(|w| ..)` method takes [`usb20_phy0_con2::W`](W) writer structure"]
impl crate::Writable for Usb20Phy0Con2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets USB20_PHY0_CON2 to value 0x03d2"]
impl crate::Resettable for Usb20Phy0Con2Spec {
    const RESET_VALUE: u32 = 0x03d2;
}
