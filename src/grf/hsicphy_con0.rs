#[doc = "Register `HSICPHY_CON0` reader"]
pub type R = crate::R<HsicphyCon0Spec>;
#[doc = "Register `HSICPHY_CON0` writer"]
pub type W = crate::W<HsicphyCon0Spec>;
#[doc = "utmi_dppulldown\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HsicphyUtmiDppulldown {
    #[doc = "1: DP pull down resistor enable"]
    B1 = 1,
    #[doc = "0: DP pull down resistor disable"]
    B0 = 0,
}
impl From<HsicphyUtmiDppulldown> for bool {
    #[inline(always)]
    fn from(variant: HsicphyUtmiDppulldown) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSICPHY_UTMI_DPPULLDOWN` reader - utmi_dppulldown"]
pub type HsicphyUtmiDppulldownR = crate::BitReader<HsicphyUtmiDppulldown>;
impl HsicphyUtmiDppulldownR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HsicphyUtmiDppulldown {
        match self.bits {
            true => HsicphyUtmiDppulldown::B1,
            false => HsicphyUtmiDppulldown::B0,
        }
    }
    #[doc = "DP pull down resistor enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == HsicphyUtmiDppulldown::B1
    }
    #[doc = "DP pull down resistor disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == HsicphyUtmiDppulldown::B0
    }
}
#[doc = "Field `HSICPHY_UTMI_DPPULLDOWN` writer - utmi_dppulldown"]
pub type HsicphyUtmiDppulldownW<'a, REG> = crate::BitWriter<'a, REG, HsicphyUtmiDppulldown>;
impl<'a, REG> HsicphyUtmiDppulldownW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DP pull down resistor enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(HsicphyUtmiDppulldown::B1)
    }
    #[doc = "DP pull down resistor disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(HsicphyUtmiDppulldown::B0)
    }
}
#[doc = "utmi_dmpulldown\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HsicphyUtmiDmpulldown {
    #[doc = "1: DM pull down resistor enable"]
    B1 = 1,
    #[doc = "0: DM pull down resistor disable"]
    B0 = 0,
}
impl From<HsicphyUtmiDmpulldown> for bool {
    #[inline(always)]
    fn from(variant: HsicphyUtmiDmpulldown) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSICPHY_UTMI_DMPULLDOWN` reader - utmi_dmpulldown"]
pub type HsicphyUtmiDmpulldownR = crate::BitReader<HsicphyUtmiDmpulldown>;
impl HsicphyUtmiDmpulldownR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HsicphyUtmiDmpulldown {
        match self.bits {
            true => HsicphyUtmiDmpulldown::B1,
            false => HsicphyUtmiDmpulldown::B0,
        }
    }
    #[doc = "DM pull down resistor enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == HsicphyUtmiDmpulldown::B1
    }
    #[doc = "DM pull down resistor disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == HsicphyUtmiDmpulldown::B0
    }
}
#[doc = "Field `HSICPHY_UTMI_DMPULLDOWN` writer - utmi_dmpulldown"]
pub type HsicphyUtmiDmpulldownW<'a, REG> = crate::BitWriter<'a, REG, HsicphyUtmiDmpulldown>;
impl<'a, REG> HsicphyUtmiDmpulldownW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DM pull down resistor enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(HsicphyUtmiDmpulldown::B1)
    }
    #[doc = "DM pull down resistor disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(HsicphyUtmiDmpulldown::B0)
    }
}
#[doc = "Field `I_HSIC_UTMI_SUSPEND_N` reader - utmi_suspend_n\n\nselect the value of this register to ususpend_n\n\nport of HSIC PHY when soft_con_sel=1"]
pub type IHsicUtmiSuspendNR = crate::BitReader;
#[doc = "Field `I_HSIC_UTMI_SUSPEND_N` writer - utmi_suspend_n\n\nselect the value of this register to ususpend_n\n\nport of HSIC PHY when soft_con_sel=1"]
pub type IHsicUtmiSuspendNW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I_HSIC_UTMI_TERMSELECT` reader - utmi_termselect\n\nselect the value of this register to termselect\n\nport of HSIC PHY when soft_con_sel=1"]
pub type IHsicUtmiTermselectR = crate::BitReader;
#[doc = "Field `I_HSIC_UTMI_TERMSELECT` writer - utmi_termselect\n\nselect the value of this register to termselect\n\nport of HSIC PHY when soft_con_sel=1"]
pub type IHsicUtmiTermselectW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I_HSIC_UTMI_OPMODE` reader - utmi_opmode\n\nselect the value of this register to opmode\n\nport of HSIC PHY when soft_con_sel=1"]
pub type IHsicUtmiOpmodeR = crate::FieldReader;
#[doc = "Field `I_HSIC_UTMI_OPMODE` writer - utmi_opmode\n\nselect the value of this register to opmode\n\nport of HSIC PHY when soft_con_sel=1"]
pub type IHsicUtmiOpmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `I_HSIC_UTMI_XCVRSELECT` reader - utmi_xcvrselect\n\nselect the value of this register to xcvrselect\n\nport of HSIC PHY when soft_con_sel=1."]
pub type IHsicUtmiXcvrselectR = crate::FieldReader;
#[doc = "Field `I_HSIC_UTMI_XCVRSELECT` writer - utmi_xcvrselect\n\nselect the value of this register to xcvrselect\n\nport of HSIC PHY when soft_con_sel=1."]
pub type IHsicUtmiXcvrselectW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "soft_con_sel\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HsicphySoftConSel {
    #[doc = "1: soft control select utmi signals from GRF to HSIC PHY"]
    B1 = 1,
    #[doc = "0: soft control select utmi signals from HSIC controller to HSIC PHY"]
    B0 = 0,
}
impl From<HsicphySoftConSel> for bool {
    #[inline(always)]
    fn from(variant: HsicphySoftConSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSICPHY_SOFT_CON_SEL` reader - soft_con_sel"]
pub type HsicphySoftConSelR = crate::BitReader<HsicphySoftConSel>;
impl HsicphySoftConSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HsicphySoftConSel {
        match self.bits {
            true => HsicphySoftConSel::B1,
            false => HsicphySoftConSel::B0,
        }
    }
    #[doc = "soft control select utmi signals from GRF to HSIC PHY"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == HsicphySoftConSel::B1
    }
    #[doc = "soft control select utmi signals from HSIC controller to HSIC PHY"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == HsicphySoftConSel::B0
    }
}
#[doc = "Field `HSICPHY_SOFT_CON_SEL` writer - soft_con_sel"]
pub type HsicphySoftConSelW<'a, REG> = crate::BitWriter<'a, REG, HsicphySoftConSel>;
impl<'a, REG> HsicphySoftConSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "soft control select utmi signals from GRF to HSIC PHY"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(HsicphySoftConSel::B1)
    }
    #[doc = "soft control select utmi signals from HSIC controller to HSIC PHY"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(HsicphySoftConSel::B0)
    }
}
#[doc = "Field `WRITE_ENABLE` reader - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - utmi_dppulldown"]
    #[inline(always)]
    pub fn hsicphy_utmi_dppulldown(&self) -> HsicphyUtmiDppulldownR {
        HsicphyUtmiDppulldownR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - utmi_dmpulldown"]
    #[inline(always)]
    pub fn hsicphy_utmi_dmpulldown(&self) -> HsicphyUtmiDmpulldownR {
        HsicphyUtmiDmpulldownR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - utmi_suspend_n\n\nselect the value of this register to ususpend_n\n\nport of HSIC PHY when soft_con_sel=1"]
    #[inline(always)]
    pub fn i_hsic_utmi_suspend_n(&self) -> IHsicUtmiSuspendNR {
        IHsicUtmiSuspendNR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - utmi_termselect\n\nselect the value of this register to termselect\n\nport of HSIC PHY when soft_con_sel=1"]
    #[inline(always)]
    pub fn i_hsic_utmi_termselect(&self) -> IHsicUtmiTermselectR {
        IHsicUtmiTermselectR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - utmi_opmode\n\nselect the value of this register to opmode\n\nport of HSIC PHY when soft_con_sel=1"]
    #[inline(always)]
    pub fn i_hsic_utmi_opmode(&self) -> IHsicUtmiOpmodeR {
        IHsicUtmiOpmodeR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - utmi_xcvrselect\n\nselect the value of this register to xcvrselect\n\nport of HSIC PHY when soft_con_sel=1."]
    #[inline(always)]
    pub fn i_hsic_utmi_xcvrselect(&self) -> IHsicUtmiXcvrselectR {
        IHsicUtmiXcvrselectR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - soft_con_sel"]
    #[inline(always)]
    pub fn hsicphy_soft_con_sel(&self) -> HsicphySoftConSelR {
        HsicphySoftConSelR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    pub fn write_enable(&self) -> WriteEnableR {
        WriteEnableR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - utmi_dppulldown"]
    #[inline(always)]
    #[must_use]
    pub fn hsicphy_utmi_dppulldown(&mut self) -> HsicphyUtmiDppulldownW<HsicphyCon0Spec> {
        HsicphyUtmiDppulldownW::new(self, 0)
    }
    #[doc = "Bit 1 - utmi_dmpulldown"]
    #[inline(always)]
    #[must_use]
    pub fn hsicphy_utmi_dmpulldown(&mut self) -> HsicphyUtmiDmpulldownW<HsicphyCon0Spec> {
        HsicphyUtmiDmpulldownW::new(self, 1)
    }
    #[doc = "Bit 2 - utmi_suspend_n\n\nselect the value of this register to ususpend_n\n\nport of HSIC PHY when soft_con_sel=1"]
    #[inline(always)]
    #[must_use]
    pub fn i_hsic_utmi_suspend_n(&mut self) -> IHsicUtmiSuspendNW<HsicphyCon0Spec> {
        IHsicUtmiSuspendNW::new(self, 2)
    }
    #[doc = "Bit 3 - utmi_termselect\n\nselect the value of this register to termselect\n\nport of HSIC PHY when soft_con_sel=1"]
    #[inline(always)]
    #[must_use]
    pub fn i_hsic_utmi_termselect(&mut self) -> IHsicUtmiTermselectW<HsicphyCon0Spec> {
        IHsicUtmiTermselectW::new(self, 3)
    }
    #[doc = "Bits 4:5 - utmi_opmode\n\nselect the value of this register to opmode\n\nport of HSIC PHY when soft_con_sel=1"]
    #[inline(always)]
    #[must_use]
    pub fn i_hsic_utmi_opmode(&mut self) -> IHsicUtmiOpmodeW<HsicphyCon0Spec> {
        IHsicUtmiOpmodeW::new(self, 4)
    }
    #[doc = "Bits 6:7 - utmi_xcvrselect\n\nselect the value of this register to xcvrselect\n\nport of HSIC PHY when soft_con_sel=1."]
    #[inline(always)]
    #[must_use]
    pub fn i_hsic_utmi_xcvrselect(&mut self) -> IHsicUtmiXcvrselectW<HsicphyCon0Spec> {
        IHsicUtmiXcvrselectW::new(self, 6)
    }
    #[doc = "Bit 8 - soft_con_sel"]
    #[inline(always)]
    #[must_use]
    pub fn hsicphy_soft_con_sel(&mut self) -> HsicphySoftConSelW<HsicphyCon0Spec> {
        HsicphySoftConSelW::new(self, 8)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<HsicphyCon0Spec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "HSICPHY GRF control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hsicphy_con0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hsicphy_con0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HsicphyCon0Spec;
impl crate::RegisterSpec for HsicphyCon0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hsicphy_con0::R`](R) reader structure"]
impl crate::Readable for HsicphyCon0Spec {}
#[doc = "`write(|w| ..)` method takes [`hsicphy_con0::W`](W) writer structure"]
impl crate::Writable for HsicphyCon0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HSICPHY_CON0 to value 0x4f"]
impl crate::Resettable for HsicphyCon0Spec {
    const RESET_VALUE: u32 = 0x4f;
}
