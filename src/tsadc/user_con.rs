#[doc = "Register `USER_CON` reader"]
pub type R = crate::R<UserConSpec>;
#[doc = "Register `USER_CON` writer"]
pub type W = crate::W<UserConSpec>;
#[doc = "ADC input source selection(CH_SEL\\[2:0\\]).\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AdcInputSrcSel {
    #[doc = "0: Input source 0 (SARADC_AIN\\[0\\])"]
    B000 = 0,
    #[doc = "1: Input source 1 (SARADC_AIN\\[1\\]) Others : Reserved"]
    B001 = 1,
}
impl From<AdcInputSrcSel> for u8 {
    #[inline(always)]
    fn from(variant: AdcInputSrcSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AdcInputSrcSel {
    type Ux = u8;
}
#[doc = "Field `ADC_INPUT_SRC_SEL` reader - ADC input source selection(CH_SEL\\[2:0\\])."]
pub type AdcInputSrcSelR = crate::FieldReader<AdcInputSrcSel>;
impl AdcInputSrcSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<AdcInputSrcSel> {
        match self.bits {
            0 => Some(AdcInputSrcSel::B000),
            1 => Some(AdcInputSrcSel::B001),
            _ => None,
        }
    }
    #[doc = "Input source 0 (SARADC_AIN\\[0\\])"]
    #[inline(always)]
    pub fn is_b000(&self) -> bool {
        *self == AdcInputSrcSel::B000
    }
    #[doc = "Input source 1 (SARADC_AIN\\[1\\]) Others : Reserved"]
    #[inline(always)]
    pub fn is_b001(&self) -> bool {
        *self == AdcInputSrcSel::B001
    }
}
#[doc = "Field `ADC_INPUT_SRC_SEL` writer - ADC input source selection(CH_SEL\\[2:0\\])."]
pub type AdcInputSrcSelW<'a, REG> = crate::FieldWriter<'a, REG, 3, AdcInputSrcSel>;
impl<'a, REG> AdcInputSrcSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input source 0 (SARADC_AIN\\[0\\])"]
    #[inline(always)]
    pub fn b000(self) -> &'a mut crate::W<REG> {
        self.variant(AdcInputSrcSel::B000)
    }
    #[doc = "Input source 1 (SARADC_AIN\\[1\\]) Others : Reserved"]
    #[inline(always)]
    pub fn b001(self) -> &'a mut crate::W<REG> {
        self.variant(AdcInputSrcSel::B001)
    }
}
#[doc = "ADC power down control bit\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AdcPowerCtrl {
    #[doc = "0: ADC power down"]
    B0 = 0,
    #[doc = "1: ADC power up and reset"]
    B1 = 1,
}
impl From<AdcPowerCtrl> for bool {
    #[inline(always)]
    fn from(variant: AdcPowerCtrl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC_POWER_CTRL` reader - ADC power down control bit"]
pub type AdcPowerCtrlR = crate::BitReader<AdcPowerCtrl>;
impl AdcPowerCtrlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AdcPowerCtrl {
        match self.bits {
            false => AdcPowerCtrl::B0,
            true => AdcPowerCtrl::B1,
        }
    }
    #[doc = "ADC power down"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == AdcPowerCtrl::B0
    }
    #[doc = "ADC power up and reset"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == AdcPowerCtrl::B1
    }
}
#[doc = "Field `ADC_POWER_CTRL` writer - ADC power down control bit"]
pub type AdcPowerCtrlW<'a, REG> = crate::BitWriter<'a, REG, AdcPowerCtrl>;
impl<'a, REG> AdcPowerCtrlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADC power down"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(AdcPowerCtrl::B0)
    }
    #[doc = "ADC power up and reset"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(AdcPowerCtrl::B1)
    }
}
#[doc = "start mode.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StartMode {
    #[doc = "0: tsadc controller will asert start_of_conversion after 'inter_pd_soc' cycles."]
    B0 = 0,
    #[doc = "1: the start_of_conversion will be controlled by TSADC_USER_CON\\[5\\]."]
    B1 = 1,
}
impl From<StartMode> for bool {
    #[inline(always)]
    fn from(variant: StartMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `START_MODE` reader - start mode."]
pub type StartModeR = crate::BitReader<StartMode>;
impl StartModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> StartMode {
        match self.bits {
            false => StartMode::B0,
            true => StartMode::B1,
        }
    }
    #[doc = "tsadc controller will asert start_of_conversion after 'inter_pd_soc' cycles."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == StartMode::B0
    }
    #[doc = "the start_of_conversion will be controlled by TSADC_USER_CON\\[5\\]."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == StartMode::B1
    }
}
#[doc = "Field `START_MODE` writer - start mode."]
pub type StartModeW<'a, REG> = crate::BitWriter<'a, REG, StartMode>;
impl<'a, REG> StartModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "tsadc controller will asert start_of_conversion after 'inter_pd_soc' cycles."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(StartMode::B0)
    }
    #[doc = "the start_of_conversion will be controlled by TSADC_USER_CON\\[5\\]."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(StartMode::B1)
    }
}
#[doc = "Field `START` reader - When software write 1 to this bit , start_of_conversion will be\n\nassert.\n\nThis bit will be cleared after TSADC access finishing.\n\nWhen TSADC_USER_CON\\[4\\]
= 1'b1 take effect."]
pub type StartR = crate::BitReader;
#[doc = "Field `START` writer - When software write 1 to this bit , start_of_conversion will be\n\nassert.\n\nThis bit will be cleared after TSADC access finishing.\n\nWhen TSADC_USER_CON\\[4\\]
= 1'b1 take effect."]
pub type StartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTER_PD_SOC` reader - interleave between power down and start of conversion"]
pub type InterPdSocR = crate::FieldReader;
#[doc = "Field `INTER_PD_SOC` writer - interleave between power down and start of conversion"]
pub type InterPdSocW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "ADC status (EOC)\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AdcStatus {
    #[doc = "0: ADC stop"]
    B0 = 0,
    #[doc = "1: Conversion in progress"]
    B1 = 1,
}
impl From<AdcStatus> for bool {
    #[inline(always)]
    fn from(variant: AdcStatus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC_STATUS` reader - ADC status (EOC)"]
pub type AdcStatusR = crate::BitReader<AdcStatus>;
impl AdcStatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AdcStatus {
        match self.bits {
            false => AdcStatus::B0,
            true => AdcStatus::B1,
        }
    }
    #[doc = "ADC stop"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == AdcStatus::B0
    }
    #[doc = "Conversion in progress"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == AdcStatus::B1
    }
}
impl R {
    #[doc = "Bits 0:2 - ADC input source selection(CH_SEL\\[2:0\\])."]
    #[inline(always)]
    pub fn adc_input_src_sel(&self) -> AdcInputSrcSelR {
        AdcInputSrcSelR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - ADC power down control bit"]
    #[inline(always)]
    pub fn adc_power_ctrl(&self) -> AdcPowerCtrlR {
        AdcPowerCtrlR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - start mode."]
    #[inline(always)]
    pub fn start_mode(&self) -> StartModeR {
        StartModeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - When software write 1 to this bit , start_of_conversion will be\n\nassert.\n\nThis bit will be cleared after TSADC access finishing.\n\nWhen TSADC_USER_CON\\[4\\]
= 1'b1 take effect."]
    #[inline(always)]
    pub fn start(&self) -> StartR {
        StartR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:11 - interleave between power down and start of conversion"]
    #[inline(always)]
    pub fn inter_pd_soc(&self) -> InterPdSocR {
        InterPdSocR::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bit 12 - ADC status (EOC)"]
    #[inline(always)]
    pub fn adc_status(&self) -> AdcStatusR {
        AdcStatusR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - ADC input source selection(CH_SEL\\[2:0\\])."]
    #[inline(always)]
    #[must_use]
    pub fn adc_input_src_sel(&mut self) -> AdcInputSrcSelW<UserConSpec> {
        AdcInputSrcSelW::new(self, 0)
    }
    #[doc = "Bit 3 - ADC power down control bit"]
    #[inline(always)]
    #[must_use]
    pub fn adc_power_ctrl(&mut self) -> AdcPowerCtrlW<UserConSpec> {
        AdcPowerCtrlW::new(self, 3)
    }
    #[doc = "Bit 4 - start mode."]
    #[inline(always)]
    #[must_use]
    pub fn start_mode(&mut self) -> StartModeW<UserConSpec> {
        StartModeW::new(self, 4)
    }
    #[doc = "Bit 5 - When software write 1 to this bit , start_of_conversion will be\n\nassert.\n\nThis bit will be cleared after TSADC access finishing.\n\nWhen TSADC_USER_CON\\[4\\]
= 1'b1 take effect."]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> StartW<UserConSpec> {
        StartW::new(self, 5)
    }
    #[doc = "Bits 6:11 - interleave between power down and start of conversion"]
    #[inline(always)]
    #[must_use]
    pub fn inter_pd_soc(&mut self) -> InterPdSocW<UserConSpec> {
        InterPdSocW::new(self, 6)
    }
}
#[doc = "The control register of A/D Converter.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`user_con::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`user_con::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UserConSpec;
impl crate::RegisterSpec for UserConSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`user_con::R`](R) reader structure"]
impl crate::Readable for UserConSpec {}
#[doc = "`write(|w| ..)` method takes [`user_con::W`](W) writer structure"]
impl crate::Writable for UserConSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets USER_CON to value 0x0208"]
impl crate::Resettable for UserConSpec {
    const RESET_VALUE: u32 = 0x0208;
}
