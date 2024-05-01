#[doc = "Register `SYS_CTRL1` reader"]
pub type R = crate::R<SysCtrl1Spec>;
#[doc = "Register `SYS_CTRL1` writer"]
pub type W = crate::W<SysCtrl1Spec>;
#[doc = "Field `NOC_HURRY_EN` reader - Noc hurry enable"]
pub type NocHurryEnR = crate::BitReader;
#[doc = "Field `NOC_HURRY_EN` writer - Noc hurry enable"]
pub type NocHurryEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOC_HURRY_VALUE` reader - Noc hurry value"]
pub type NocHurryValueR = crate::FieldReader;
#[doc = "Field `NOC_HURRY_VALUE` writer - Noc hurry value"]
pub type NocHurryValueW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `NOC_HURRY_THRESHOLD` reader - Noc hurry threshold value"]
pub type NocHurryThresholdR = crate::FieldReader;
#[doc = "Field `NOC_HURRY_THRESHOLD` writer - Noc hurry threshold value"]
pub type NocHurryThresholdW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `NOC_QOS_EN` reader - Noc qos enable"]
pub type NocQosEnR = crate::BitReader;
#[doc = "Field `NOC_QOS_EN` writer - Noc qos enable"]
pub type NocQosEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOC_WIN_QOS` reader - Noc win qos"]
pub type NocWinQosR = crate::FieldReader;
#[doc = "Field `NOC_WIN_QOS` writer - Noc win qos"]
pub type NocWinQosW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `AXI_MAX_OUTSTANDING_EN` reader - axi bus max outstanding enable"]
pub type AxiMaxOutstandingEnR = crate::BitReader;
#[doc = "Field `AXI_MAX_OUTSTANDING_EN` writer - axi bus max outstanding enable"]
pub type AxiMaxOutstandingEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AXI_OUTSTANDING_MAX_NUM` reader - axi bus max outstanding number"]
pub type AxiOutstandingMaxNumR = crate::FieldReader;
#[doc = "Field `AXI_OUTSTANDING_MAX_NUM` writer - axi bus max outstanding number"]
pub type AxiOutstandingMaxNumW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum NocHurryWMode {
    #[doc = "0: noc_hurry_w disable"]
    B00 = 0,
    #[doc = "1: left 1/4 fifo empty"]
    B01 = 1,
    #[doc = "2: left 1/2 fifo empty"]
    B10 = 2,
    #[doc = "3: left 3/4 fifo empty"]
    B11 = 3,
}
impl From<NocHurryWMode> for u8 {
    #[inline(always)]
    fn from(variant: NocHurryWMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for NocHurryWMode {
    type Ux = u8;
}
#[doc = "Field `NOC_HURRY_W_MODE` reader - "]
pub type NocHurryWModeR = crate::FieldReader<NocHurryWMode>;
impl NocHurryWModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> NocHurryWMode {
        match self.bits {
            0 => NocHurryWMode::B00,
            1 => NocHurryWMode::B01,
            2 => NocHurryWMode::B10,
            3 => NocHurryWMode::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "noc_hurry_w disable"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == NocHurryWMode::B00
    }
    #[doc = "left 1/4 fifo empty"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == NocHurryWMode::B01
    }
    #[doc = "left 1/2 fifo empty"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == NocHurryWMode::B10
    }
    #[doc = "left 3/4 fifo empty"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == NocHurryWMode::B11
    }
}
#[doc = "Field `NOC_HURRY_W_MODE` writer - "]
pub type NocHurryWModeW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, NocHurryWMode>;
impl<'a, REG> NocHurryWModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "noc_hurry_w disable"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(NocHurryWMode::B00)
    }
    #[doc = "left 1/4 fifo empty"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(NocHurryWMode::B01)
    }
    #[doc = "left 1/2 fifo empty"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(NocHurryWMode::B10)
    }
    #[doc = "left 3/4 fifo empty"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(NocHurryWMode::B11)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum NocHurryWValue {
    #[doc = "0: low priority"]
    B00 = 0,
    #[doc = "3: high priority"]
    B11 = 3,
}
impl From<NocHurryWValue> for u8 {
    #[inline(always)]
    fn from(variant: NocHurryWValue) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for NocHurryWValue {
    type Ux = u8;
}
#[doc = "Field `NOC_HURRY_W_VALUE` reader - "]
pub type NocHurryWValueR = crate::FieldReader<NocHurryWValue>;
impl NocHurryWValueR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<NocHurryWValue> {
        match self.bits {
            0 => Some(NocHurryWValue::B00),
            3 => Some(NocHurryWValue::B11),
            _ => None,
        }
    }
    #[doc = "low priority"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == NocHurryWValue::B00
    }
    #[doc = "high priority"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == NocHurryWValue::B11
    }
}
#[doc = "Field `NOC_HURRY_W_VALUE` writer - "]
pub type NocHurryWValueW<'a, REG> = crate::FieldWriter<'a, REG, 2, NocHurryWValue>;
impl<'a, REG> NocHurryWValueW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "low priority"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(NocHurryWValue::B00)
    }
    #[doc = "high priority"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(NocHurryWValue::B11)
    }
}
#[doc = "Field `REG_DONE_FRM` reader - 1’b0: every frame valid\n\n1’b1: every field valid"]
pub type RegDoneFrmR = crate::BitReader;
#[doc = "Field `REG_DONE_FRM` writer - 1’b0: every frame valid\n\n1’b1: every field valid"]
pub type RegDoneFrmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSP_FP_STANDBY` reader - dsp_fp_standby"]
pub type DspFpStandbyR = crate::BitReader;
#[doc = "Field `DSP_FP_STANDBY` writer - dsp_fp_standby"]
pub type DspFpStandbyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Noc hurry enable"]
    #[inline(always)]
    pub fn noc_hurry_en(&self) -> NocHurryEnR {
        NocHurryEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Noc hurry value"]
    #[inline(always)]
    pub fn noc_hurry_value(&self) -> NocHurryValueR {
        NocHurryValueR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:8 - Noc hurry threshold value"]
    #[inline(always)]
    pub fn noc_hurry_threshold(&self) -> NocHurryThresholdR {
        NocHurryThresholdR::new(((self.bits >> 3) & 0x3f) as u8)
    }
    #[doc = "Bit 9 - Noc qos enable"]
    #[inline(always)]
    pub fn noc_qos_en(&self) -> NocQosEnR {
        NocQosEnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - Noc win qos"]
    #[inline(always)]
    pub fn noc_win_qos(&self) -> NocWinQosR {
        NocWinQosR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - axi bus max outstanding enable"]
    #[inline(always)]
    pub fn axi_max_outstanding_en(&self) -> AxiMaxOutstandingEnR {
        AxiMaxOutstandingEnR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:17 - axi bus max outstanding number"]
    #[inline(always)]
    pub fn axi_outstanding_max_num(&self) -> AxiOutstandingMaxNumR {
        AxiOutstandingMaxNumR::new(((self.bits >> 13) & 0x1f) as u8)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn noc_hurry_w_mode(&self) -> NocHurryWModeR {
        NocHurryWModeR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn noc_hurry_w_value(&self) -> NocHurryWValueR {
        NocHurryWValueR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bit 24 - 1’b0: every frame valid\n\n1’b1: every field valid"]
    #[inline(always)]
    pub fn reg_done_frm(&self) -> RegDoneFrmR {
        RegDoneFrmR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 31 - dsp_fp_standby"]
    #[inline(always)]
    pub fn dsp_fp_standby(&self) -> DspFpStandbyR {
        DspFpStandbyR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Noc hurry enable"]
    #[inline(always)]
    #[must_use]
    pub fn noc_hurry_en(&mut self) -> NocHurryEnW<SysCtrl1Spec> {
        NocHurryEnW::new(self, 0)
    }
    #[doc = "Bits 1:2 - Noc hurry value"]
    #[inline(always)]
    #[must_use]
    pub fn noc_hurry_value(&mut self) -> NocHurryValueW<SysCtrl1Spec> {
        NocHurryValueW::new(self, 1)
    }
    #[doc = "Bits 3:8 - Noc hurry threshold value"]
    #[inline(always)]
    #[must_use]
    pub fn noc_hurry_threshold(&mut self) -> NocHurryThresholdW<SysCtrl1Spec> {
        NocHurryThresholdW::new(self, 3)
    }
    #[doc = "Bit 9 - Noc qos enable"]
    #[inline(always)]
    #[must_use]
    pub fn noc_qos_en(&mut self) -> NocQosEnW<SysCtrl1Spec> {
        NocQosEnW::new(self, 9)
    }
    #[doc = "Bits 10:11 - Noc win qos"]
    #[inline(always)]
    #[must_use]
    pub fn noc_win_qos(&mut self) -> NocWinQosW<SysCtrl1Spec> {
        NocWinQosW::new(self, 10)
    }
    #[doc = "Bit 12 - axi bus max outstanding enable"]
    #[inline(always)]
    #[must_use]
    pub fn axi_max_outstanding_en(&mut self) -> AxiMaxOutstandingEnW<SysCtrl1Spec> {
        AxiMaxOutstandingEnW::new(self, 12)
    }
    #[doc = "Bits 13:17 - axi bus max outstanding number"]
    #[inline(always)]
    #[must_use]
    pub fn axi_outstanding_max_num(&mut self) -> AxiOutstandingMaxNumW<SysCtrl1Spec> {
        AxiOutstandingMaxNumW::new(self, 13)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    #[must_use]
    pub fn noc_hurry_w_mode(&mut self) -> NocHurryWModeW<SysCtrl1Spec> {
        NocHurryWModeW::new(self, 20)
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    #[must_use]
    pub fn noc_hurry_w_value(&mut self) -> NocHurryWValueW<SysCtrl1Spec> {
        NocHurryWValueW::new(self, 22)
    }
    #[doc = "Bit 24 - 1’b0: every frame valid\n\n1’b1: every field valid"]
    #[inline(always)]
    #[must_use]
    pub fn reg_done_frm(&mut self) -> RegDoneFrmW<SysCtrl1Spec> {
        RegDoneFrmW::new(self, 24)
    }
    #[doc = "Bit 31 - dsp_fp_standby"]
    #[inline(always)]
    #[must_use]
    pub fn dsp_fp_standby(&mut self) -> DspFpStandbyW<SysCtrl1Spec> {
        DspFpStandbyW::new(self, 31)
    }
}
#[doc = "System control register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_ctrl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_ctrl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysCtrl1Spec;
impl crate::RegisterSpec for SysCtrl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_ctrl1::R`](R) reader structure"]
impl crate::Readable for SysCtrl1Spec {}
#[doc = "`write(|w| ..)` method takes [`sys_ctrl1::W`](W) writer structure"]
impl crate::Writable for SysCtrl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SYS_CTRL1 to value 0x0003_a000"]
impl crate::Resettable for SysCtrl1Spec {
    const RESET_VALUE: u32 = 0x0003_a000;
}
