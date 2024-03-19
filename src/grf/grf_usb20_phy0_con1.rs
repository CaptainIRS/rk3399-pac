#[doc = "Register `GRF_USB20_PHY0_CON1` reader"]
pub type R = crate::R<GrfUsb20Phy0Con1Spec>;
#[doc = "Register `GRF_USB20_PHY0_CON1` writer"]
pub type W = crate::W<GrfUsb20Phy0Con1Spec>;
#[doc = "utmi_sel\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UtmiSel {
    #[doc = "1: select utmi interface signals from GRF reister to usb2phy"]
    B1 = 1,
    #[doc = "0: select utmi interface signals from utmi interface of usb3otg0 controller to usb2phy"]
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
    #[doc = "select utmi interface signals from utmi interface of usb3otg0 controller to usb2phy"]
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
    #[doc = "select utmi interface signals from utmi interface of usb3otg0 controller to usb2phy"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(UtmiSel::B0)
    }
}
#[doc = "Field `SUSPEND_N` reader - suspend_n\n\nutmi_sel=1, select the value of this register to\n\nusb2phy\n\nutmi_sel=0 and bit11 of\n\nUSB20_PHY0_CON1=0, and bit 12 of\n\nUSB20_PHY0_CON1=0 select the value of the\n\nvalue of this bit to usb2phy\n\nutmi_sel=0 and bit11 of\n\nUSB20_PHY0_CON1=0, and bit 12 of\n\nUSB20_PHY0_CON1=1 select suspend_n\n\nsignals from usb3otg0 controller to usb2phy\n\nfor free running utmi clock\n\nutmi_sel=0 and bit11 of\n\nUSB20_PHY0_CON1=1, select\n\nsuspend_com_n signals from usb3otg0\n\ncontroller to usb2phy for not free running utmi\n\nclock"]
pub type SuspendNR = crate::BitReader;
#[doc = "Field `SUSPEND_N` writer - suspend_n\n\nutmi_sel=1, select the value of this register to\n\nusb2phy\n\nutmi_sel=0 and bit11 of\n\nUSB20_PHY0_CON1=0, and bit 12 of\n\nUSB20_PHY0_CON1=0 select the value of the\n\nvalue of this bit to usb2phy\n\nutmi_sel=0 and bit11 of\n\nUSB20_PHY0_CON1=0, and bit 12 of\n\nUSB20_PHY0_CON1=1 select suspend_n\n\nsignals from usb3otg0 controller to usb2phy\n\nfor free running utmi clock\n\nutmi_sel=0 and bit11 of\n\nUSB20_PHY0_CON1=1, select\n\nsuspend_com_n signals from usb3otg0\n\ncontroller to usb2phy for not free running utmi\n\nclock"]
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
#[doc = "Field `DPPULLDOWN` reader - dppulldown\n\nSelect the value of this register to usb2phy\n\nwhen utmi_sel=1"]
pub type DppulldownR = crate::BitReader;
#[doc = "Field `DPPULLDOWN` writer - dppulldown\n\nSelect the value of this register to usb2phy\n\nwhen utmi_sel=1"]
pub type DppulldownW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMPULLDOWN` reader - dmpulldown\n\nSelect the value of this register to usb2phy\n\nwhen utmi_sel=1"]
pub type DmpulldownR = crate::BitReader;
#[doc = "Field `DMPULLDOWN` writer - dmpulldown\n\nSelect the value of this register to usb2phy\n\nwhen utmi_sel=1"]
pub type DmpulldownW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "iddig_sel\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IddigSel {
    #[doc = "1: select the value of bit10 of USB20_PHY0_CON1 to usb3otg0 controller"]
    B1 = 1,
    #[doc = "0: select the iddig from usb2phy to usb3otg0 controller"]
    B0 = 0,
}
impl From<IddigSel> for bool {
    #[inline(always)]
    fn from(variant: IddigSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDDIG_SEL` reader - iddig_sel"]
pub type IddigSelR = crate::BitReader<IddigSel>;
impl IddigSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IddigSel {
        match self.bits {
            true => IddigSel::B1,
            false => IddigSel::B0,
        }
    }
    #[doc = "select the value of bit10 of USB20_PHY0_CON1 to usb3otg0 controller"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IddigSel::B1
    }
    #[doc = "select the iddig from usb2phy to usb3otg0 controller"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IddigSel::B0
    }
}
#[doc = "Field `IDDIG_SEL` writer - iddig_sel"]
pub type IddigSelW<'a, REG> = crate::BitWriter<'a, REG, IddigSel>;
impl<'a, REG> IddigSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "select the value of bit10 of USB20_PHY0_CON1 to usb3otg0 controller"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(IddigSel::B1)
    }
    #[doc = "select the iddig from usb2phy to usb3otg0 controller"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(IddigSel::B0)
    }
}
#[doc = "Field `IDDIG` reader - iddig\n\nSelect the value of this register to usb3otg0\n\nregister"]
pub type IddigR = crate::BitReader;
#[doc = "Field `IDDIG` writer - iddig\n\nSelect the value of this register to usb3otg0\n\nregister"]
pub type IddigW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SUSPEND_N_SEL` reader - suspend_n_sel\n\nPls see suspend_n."]
pub type SuspendNSelR = crate::BitReader;
#[doc = "Field `SUSPEND_N_SEL` writer - suspend_n_sel\n\nPls see suspend_n."]
pub type SuspendNSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SUSPEND_N_SEL1` reader - suspend_n_sel1\n\nPls see suspend_n."]
pub type SuspendNSel1R = crate::BitReader;
#[doc = "Field `SUSPEND_N_SEL1` writer - suspend_n_sel1\n\nPls see suspend_n."]
pub type SuspendNSel1W<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 1 - suspend_n\n\nutmi_sel=1, select the value of this register to\n\nusb2phy\n\nutmi_sel=0 and bit11 of\n\nUSB20_PHY0_CON1=0, and bit 12 of\n\nUSB20_PHY0_CON1=0 select the value of the\n\nvalue of this bit to usb2phy\n\nutmi_sel=0 and bit11 of\n\nUSB20_PHY0_CON1=0, and bit 12 of\n\nUSB20_PHY0_CON1=1 select suspend_n\n\nsignals from usb3otg0 controller to usb2phy\n\nfor free running utmi clock\n\nutmi_sel=0 and bit11 of\n\nUSB20_PHY0_CON1=1, select\n\nsuspend_com_n signals from usb3otg0\n\ncontroller to usb2phy for not free running utmi\n\nclock"]
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
    #[doc = "Bit 7 - dppulldown\n\nSelect the value of this register to usb2phy\n\nwhen utmi_sel=1"]
    #[inline(always)]
    pub fn dppulldown(&self) -> DppulldownR {
        DppulldownR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - dmpulldown\n\nSelect the value of this register to usb2phy\n\nwhen utmi_sel=1"]
    #[inline(always)]
    pub fn dmpulldown(&self) -> DmpulldownR {
        DmpulldownR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - iddig_sel"]
    #[inline(always)]
    pub fn iddig_sel(&self) -> IddigSelR {
        IddigSelR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - iddig\n\nSelect the value of this register to usb3otg0\n\nregister"]
    #[inline(always)]
    pub fn iddig(&self) -> IddigR {
        IddigR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - suspend_n_sel\n\nPls see suspend_n."]
    #[inline(always)]
    pub fn suspend_n_sel(&self) -> SuspendNSelR {
        SuspendNSelR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - suspend_n_sel1\n\nPls see suspend_n."]
    #[inline(always)]
    pub fn suspend_n_sel1(&self) -> SuspendNSel1R {
        SuspendNSel1R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:28 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    pub fn write_enable(&self) -> WriteEnableR {
        WriteEnableR::new(((self.bits >> 13) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - utmi_sel"]
    #[inline(always)]
    #[must_use]
    pub fn utmi_sel(&mut self) -> UtmiSelW<GrfUsb20Phy0Con1Spec> {
        UtmiSelW::new(self, 0)
    }
    #[doc = "Bit 1 - suspend_n\n\nutmi_sel=1, select the value of this register to\n\nusb2phy\n\nutmi_sel=0 and bit11 of\n\nUSB20_PHY0_CON1=0, and bit 12 of\n\nUSB20_PHY0_CON1=0 select the value of the\n\nvalue of this bit to usb2phy\n\nutmi_sel=0 and bit11 of\n\nUSB20_PHY0_CON1=0, and bit 12 of\n\nUSB20_PHY0_CON1=1 select suspend_n\n\nsignals from usb3otg0 controller to usb2phy\n\nfor free running utmi clock\n\nutmi_sel=0 and bit11 of\n\nUSB20_PHY0_CON1=1, select\n\nsuspend_com_n signals from usb3otg0\n\ncontroller to usb2phy for not free running utmi\n\nclock"]
    #[inline(always)]
    #[must_use]
    pub fn suspend_n(&mut self) -> SuspendNW<GrfUsb20Phy0Con1Spec> {
        SuspendNW::new(self, 1)
    }
    #[doc = "Bits 2:3 - opmode\n\nSelect the value of this register to usb2phy\n\nwhen utmi_sel=1"]
    #[inline(always)]
    #[must_use]
    pub fn opmode(&mut self) -> OpmodeW<GrfUsb20Phy0Con1Spec> {
        OpmodeW::new(self, 2)
    }
    #[doc = "Bits 4:5 - xcvrselect\n\nSelect the value of this register to usb2phy\n\nwhen utmi_sel=1"]
    #[inline(always)]
    #[must_use]
    pub fn xcvrselect(&mut self) -> XcvrselectW<GrfUsb20Phy0Con1Spec> {
        XcvrselectW::new(self, 4)
    }
    #[doc = "Bit 6 - termselect\n\nSelect the value of this register to usb2phy\n\nwhen utmi_sel=1"]
    #[inline(always)]
    #[must_use]
    pub fn termselect(&mut self) -> TermselectW<GrfUsb20Phy0Con1Spec> {
        TermselectW::new(self, 6)
    }
    #[doc = "Bit 7 - dppulldown\n\nSelect the value of this register to usb2phy\n\nwhen utmi_sel=1"]
    #[inline(always)]
    #[must_use]
    pub fn dppulldown(&mut self) -> DppulldownW<GrfUsb20Phy0Con1Spec> {
        DppulldownW::new(self, 7)
    }
    #[doc = "Bit 8 - dmpulldown\n\nSelect the value of this register to usb2phy\n\nwhen utmi_sel=1"]
    #[inline(always)]
    #[must_use]
    pub fn dmpulldown(&mut self) -> DmpulldownW<GrfUsb20Phy0Con1Spec> {
        DmpulldownW::new(self, 8)
    }
    #[doc = "Bit 9 - iddig_sel"]
    #[inline(always)]
    #[must_use]
    pub fn iddig_sel(&mut self) -> IddigSelW<GrfUsb20Phy0Con1Spec> {
        IddigSelW::new(self, 9)
    }
    #[doc = "Bit 10 - iddig\n\nSelect the value of this register to usb3otg0\n\nregister"]
    #[inline(always)]
    #[must_use]
    pub fn iddig(&mut self) -> IddigW<GrfUsb20Phy0Con1Spec> {
        IddigW::new(self, 10)
    }
    #[doc = "Bit 11 - suspend_n_sel\n\nPls see suspend_n."]
    #[inline(always)]
    #[must_use]
    pub fn suspend_n_sel(&mut self) -> SuspendNSelW<GrfUsb20Phy0Con1Spec> {
        SuspendNSelW::new(self, 11)
    }
    #[doc = "Bit 12 - suspend_n_sel1\n\nPls see suspend_n."]
    #[inline(always)]
    #[must_use]
    pub fn suspend_n_sel1(&mut self) -> SuspendNSel1W<GrfUsb20Phy0Con1Spec> {
        SuspendNSel1W::new(self, 12)
    }
    #[doc = "Bits 13:28 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<GrfUsb20Phy0Con1Spec> {
        WriteEnableW::new(self, 13)
    }
}
#[doc = "USB20 PHY0 GRF Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_usb20_phy0_con1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_usb20_phy0_con1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrfUsb20Phy0Con1Spec;
impl crate::RegisterSpec for GrfUsb20Phy0Con1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grf_usb20_phy0_con1::R`](R) reader structure"]
impl crate::Readable for GrfUsb20Phy0Con1Spec {}
#[doc = "`write(|w| ..)` method takes [`grf_usb20_phy0_con1::W`](W) writer structure"]
impl crate::Writable for GrfUsb20Phy0Con1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GRF_USB20_PHY0_CON1 to value 0x1452"]
impl crate::Resettable for GrfUsb20Phy0Con1Spec {
    const RESET_VALUE: u32 = 0x1452;
}
