#[doc = "Register `TX_COMMON` reader"]
pub type R = crate::R<TxCommonSpec>;
#[doc = "Register `TX_COMMON` writer"]
pub type W = crate::W<TxCommonSpec>;
#[doc = "Field `RESISTOR_CTRL` reader - TX terminal resistor control \n\nwhen tx_common&lt;3>=0 \n\n000: 58.54 \n\n. . . . . . \n\n011:54.6 \n\n. . . . . . \n\n111:50 \n\nwhen tx_common&lt;6>=1 \n\n000: 49 \n\n. . . . . . \n\n011:46 \n\n. . . . . . \n\n111:42.6"]
pub type ResistorCtrlR = crate::FieldReader;
#[doc = "Field `RESISTOR_CTRL` writer - TX terminal resistor control \n\nwhen tx_common&lt;3>=0 \n\n000: 58.54 \n\n. . . . . . \n\n011:54.6 \n\n. . . . . . \n\n111:50 \n\nwhen tx_common&lt;6>=1 \n\n000: 49 \n\n. . . . . . \n\n011:46 \n\n. . . . . . \n\n111:42.6"]
pub type ResistorCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RESISTOR_MSB_CTRL` reader - TX terminal resistor MSB control"]
pub type ResistorMsbCtrlR = crate::BitReader;
#[doc = "Field `RESISTOR_MSB_CTRL` writer - TX terminal resistor MSB control"]
pub type ResistorMsbCtrlW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Low power mode control for clock regulator\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LpModeClkRegulator {
    #[doc = "0: low power mode"]
    B0 = 0,
    #[doc = "1: high power mode"]
    B1 = 1,
}
impl From<LpModeClkRegulator> for bool {
    #[inline(always)]
    fn from(variant: LpModeClkRegulator) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LP_MODE_CLK_REGULATOR` reader - Low power mode control for clock regulator"]
pub type LpModeClkRegulatorR = crate::BitReader<LpModeClkRegulator>;
impl LpModeClkRegulatorR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LpModeClkRegulator {
        match self.bits {
            false => LpModeClkRegulator::B0,
            true => LpModeClkRegulator::B1,
        }
    }
    #[doc = "low power mode"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == LpModeClkRegulator::B0
    }
    #[doc = "high power mode"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == LpModeClkRegulator::B1
    }
}
#[doc = "Field `LP_MODE_CLK_REGULATOR` writer - Low power mode control for clock regulator"]
pub type LpModeClkRegulatorW<'a, REG> = crate::BitWriter1C<'a, REG, LpModeClkRegulator>;
impl<'a, REG> LpModeClkRegulatorW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "low power mode"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(LpModeClkRegulator::B0)
    }
    #[doc = "high power mode"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(LpModeClkRegulator::B1)
    }
}
#[doc = "Pre-driver extra power control\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PreDriverPwCtrl1 {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<PreDriverPwCtrl1> for u8 {
    #[inline(always)]
    fn from(variant: PreDriverPwCtrl1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PreDriverPwCtrl1 {
    type Ux = u8;
}
#[doc = "Field `PRE_DRIVER_PW_CTRL1` reader - Pre-driver extra power control"]
pub type PreDriverPwCtrl1R = crate::FieldReader<PreDriverPwCtrl1>;
impl PreDriverPwCtrl1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PreDriverPwCtrl1> {
        match self.bits {
            0 => Some(PreDriverPwCtrl1::B0),
            1 => Some(PreDriverPwCtrl1::B1),
            _ => None,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PreDriverPwCtrl1::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PreDriverPwCtrl1::B1
    }
}
#[doc = "Field `PRE_DRIVER_PW_CTRL1` writer - Pre-driver extra power control"]
pub type PreDriverPwCtrl1W<'a, REG> = crate::FieldWriter<'a, REG, 2, PreDriverPwCtrl1>;
impl<'a, REG> PreDriverPwCtrl1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PreDriverPwCtrl1::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PreDriverPwCtrl1::B1)
    }
}
#[doc = "TX swing and pre emphasis control mode \n\nselection\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TxSwingPreEmpModeSel {
    #[doc = "1: TX swing and pre emphasis control by register dp_reserv2&lt;7:0>"]
    B1 = 1,
    #[doc = "0: TX swing and pre emphasis control by register chx_reg_swing&lt;7:0> and chx_reg_pre&lt;7:0>"]
    B0 = 0,
}
impl From<TxSwingPreEmpModeSel> for bool {
    #[inline(always)]
    fn from(variant: TxSwingPreEmpModeSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TX_SWING_PRE_EMP_MODE_SEL` reader - TX swing and pre emphasis control mode \n\nselection"]
pub type TxSwingPreEmpModeSelR = crate::BitReader<TxSwingPreEmpModeSel>;
impl TxSwingPreEmpModeSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TxSwingPreEmpModeSel {
        match self.bits {
            true => TxSwingPreEmpModeSel::B1,
            false => TxSwingPreEmpModeSel::B0,
        }
    }
    #[doc = "TX swing and pre emphasis control by register dp_reserv2&lt;7:0>"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == TxSwingPreEmpModeSel::B1
    }
    #[doc = "TX swing and pre emphasis control by register chx_reg_swing&lt;7:0> and chx_reg_pre&lt;7:0>"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == TxSwingPreEmpModeSel::B0
    }
}
#[doc = "Field `TX_SWING_PRE_EMP_MODE_SEL` writer - TX swing and pre emphasis control mode \n\nselection"]
pub type TxSwingPreEmpModeSelW<'a, REG> = crate::BitWriter<'a, REG, TxSwingPreEmpModeSel>;
impl<'a, REG> TxSwingPreEmpModeSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TX swing and pre emphasis control by register dp_reserv2&lt;7:0>"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(TxSwingPreEmpModeSel::B1)
    }
    #[doc = "TX swing and pre emphasis control by register chx_reg_swing&lt;7:0> and chx_reg_pre&lt;7:0>"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(TxSwingPreEmpModeSel::B0)
    }
}
impl R {
    #[doc = "Bits 0:2 - TX terminal resistor control \n\nwhen tx_common&lt;3>=0 \n\n000: 58.54 \n\n. . . . . . \n\n011:54.6 \n\n. . . . . . \n\n111:50 \n\nwhen tx_common&lt;6>=1 \n\n000: 49 \n\n. . . . . . \n\n011:46 \n\n. . . . . . \n\n111:42.6"]
    #[inline(always)]
    pub fn resistor_ctrl(&self) -> ResistorCtrlR {
        ResistorCtrlR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - TX terminal resistor MSB control"]
    #[inline(always)]
    pub fn resistor_msb_ctrl(&self) -> ResistorMsbCtrlR {
        ResistorMsbCtrlR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Low power mode control for clock regulator"]
    #[inline(always)]
    pub fn lp_mode_clk_regulator(&self) -> LpModeClkRegulatorR {
        LpModeClkRegulatorR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Pre-driver extra power control"]
    #[inline(always)]
    pub fn pre_driver_pw_ctrl1(&self) -> PreDriverPwCtrl1R {
        PreDriverPwCtrl1R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - TX swing and pre emphasis control mode \n\nselection"]
    #[inline(always)]
    pub fn tx_swing_pre_emp_mode_sel(&self) -> TxSwingPreEmpModeSelR {
        TxSwingPreEmpModeSelR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - TX terminal resistor control \n\nwhen tx_common&lt;3>=0 \n\n000: 58.54 \n\n. . . . . . \n\n011:54.6 \n\n. . . . . . \n\n111:50 \n\nwhen tx_common&lt;6>=1 \n\n000: 49 \n\n. . . . . . \n\n011:46 \n\n. . . . . . \n\n111:42.6"]
    #[inline(always)]
    #[must_use]
    pub fn resistor_ctrl(&mut self) -> ResistorCtrlW<TxCommonSpec> {
        ResistorCtrlW::new(self, 0)
    }
    #[doc = "Bit 3 - TX terminal resistor MSB control"]
    #[inline(always)]
    #[must_use]
    pub fn resistor_msb_ctrl(&mut self) -> ResistorMsbCtrlW<TxCommonSpec> {
        ResistorMsbCtrlW::new(self, 3)
    }
    #[doc = "Bit 4 - Low power mode control for clock regulator"]
    #[inline(always)]
    #[must_use]
    pub fn lp_mode_clk_regulator(&mut self) -> LpModeClkRegulatorW<TxCommonSpec> {
        LpModeClkRegulatorW::new(self, 4)
    }
    #[doc = "Bits 5:6 - Pre-driver extra power control"]
    #[inline(always)]
    #[must_use]
    pub fn pre_driver_pw_ctrl1(&mut self) -> PreDriverPwCtrl1W<TxCommonSpec> {
        PreDriverPwCtrl1W::new(self, 5)
    }
    #[doc = "Bit 7 - TX swing and pre emphasis control mode \n\nselection"]
    #[inline(always)]
    #[must_use]
    pub fn tx_swing_pre_emp_mode_sel(&mut self) -> TxSwingPreEmpModeSelW<TxCommonSpec> {
        TxSwingPreEmpModeSelW::new(self, 7)
    }
}
#[doc = "Tx terminal resistor control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_common::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_common::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxCommonSpec;
impl crate::RegisterSpec for TxCommonSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_common::R`](R) reader structure"]
impl crate::Readable for TxCommonSpec {}
#[doc = "`write(|w| ..)` method takes [`tx_common::W`](W) writer structure"]
impl crate::Writable for TxCommonSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x7f;
}
#[doc = "`reset()` method sets TX_COMMON to value 0x3a"]
impl crate::Resettable for TxCommonSpec {
    const RESET_VALUE: u32 = 0x3a;
}
