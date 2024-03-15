#[doc = "Register `SARADC_CTRL` reader"]
pub type R = crate::R<SaradcCtrlSpec>;
#[doc = "Register `SARADC_CTRL` writer"]
pub type W = crate::W<SaradcCtrlSpec>;
#[doc = "ADC input source selection(CH_SEL\\[2:0\\]).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AdcInputSrcSel {
    #[doc = "0: Input source 5 (SARADC_AIN\\[5\\]) Others : Reserved"]
    B000 = 0,
    #[doc = "1: Input source 5 (SARADC_AIN\\[5\\]) Others : Reserved"]
    B001 = 1,
    #[doc = "2: Input source 5 (SARADC_AIN\\[5\\]) Others : Reserved"]
    B010 = 2,
    #[doc = "3: Input source 5 (SARADC_AIN\\[5\\]) Others : Reserved"]
    B011 = 3,
    #[doc = "4: Input source 5 (SARADC_AIN\\[5\\]) Others : Reserved"]
    B100 = 4,
    #[doc = "5: Input source 5 (SARADC_AIN\\[5\\]) Others : Reserved"]
    B101 = 5,
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
            2 => Some(AdcInputSrcSel::B010),
            3 => Some(AdcInputSrcSel::B011),
            4 => Some(AdcInputSrcSel::B100),
            5 => Some(AdcInputSrcSel::B101),
            _ => None,
        }
    }
    #[doc = "Input source 5 (SARADC_AIN\\[5\\]) Others : Reserved"]
    #[inline(always)]
    pub fn is_b000(&self) -> bool {
        *self == AdcInputSrcSel::B000
    }
    #[doc = "Input source 5 (SARADC_AIN\\[5\\]) Others : Reserved"]
    #[inline(always)]
    pub fn is_b001(&self) -> bool {
        *self == AdcInputSrcSel::B001
    }
    #[doc = "Input source 5 (SARADC_AIN\\[5\\]) Others : Reserved"]
    #[inline(always)]
    pub fn is_b010(&self) -> bool {
        *self == AdcInputSrcSel::B010
    }
    #[doc = "Input source 5 (SARADC_AIN\\[5\\]) Others : Reserved"]
    #[inline(always)]
    pub fn is_b011(&self) -> bool {
        *self == AdcInputSrcSel::B011
    }
    #[doc = "Input source 5 (SARADC_AIN\\[5\\]) Others : Reserved"]
    #[inline(always)]
    pub fn is_b100(&self) -> bool {
        *self == AdcInputSrcSel::B100
    }
    #[doc = "Input source 5 (SARADC_AIN\\[5\\]) Others : Reserved"]
    #[inline(always)]
    pub fn is_b101(&self) -> bool {
        *self == AdcInputSrcSel::B101
    }
}
#[doc = "Field `ADC_INPUT_SRC_SEL` writer - ADC input source selection(CH_SEL\\[2:0\\])."]
pub type AdcInputSrcSelW<'a, REG> = crate::FieldWriter<'a, REG, 3, AdcInputSrcSel>;
impl<'a, REG> AdcInputSrcSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input source 5 (SARADC_AIN\\[5\\]) Others : Reserved"]
    #[inline(always)]
    pub fn b000(self) -> &'a mut crate::W<REG> {
        self.variant(AdcInputSrcSel::B000)
    }
    #[doc = "Input source 5 (SARADC_AIN\\[5\\]) Others : Reserved"]
    #[inline(always)]
    pub fn b001(self) -> &'a mut crate::W<REG> {
        self.variant(AdcInputSrcSel::B001)
    }
    #[doc = "Input source 5 (SARADC_AIN\\[5\\]) Others : Reserved"]
    #[inline(always)]
    pub fn b010(self) -> &'a mut crate::W<REG> {
        self.variant(AdcInputSrcSel::B010)
    }
    #[doc = "Input source 5 (SARADC_AIN\\[5\\]) Others : Reserved"]
    #[inline(always)]
    pub fn b011(self) -> &'a mut crate::W<REG> {
        self.variant(AdcInputSrcSel::B011)
    }
    #[doc = "Input source 5 (SARADC_AIN\\[5\\]) Others : Reserved"]
    #[inline(always)]
    pub fn b100(self) -> &'a mut crate::W<REG> {
        self.variant(AdcInputSrcSel::B100)
    }
    #[doc = "Input source 5 (SARADC_AIN\\[5\\]) Others : Reserved"]
    #[inline(always)]
    pub fn b101(self) -> &'a mut crate::W<REG> {
        self.variant(AdcInputSrcSel::B101)
    }
}
#[doc = "ADC power down control bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AdcPowerCtrl {
    #[doc = "0: ADC power up and reset. start signal will be asserted (DLY_PU_SOC + 2) sclk clock period later after power up"]
    B0 = 0,
    #[doc = "1: ADC power up and reset. start signal will be asserted (DLY_PU_SOC + 2) sclk clock period later after power up"]
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
    #[doc = "ADC power up and reset. start signal will be asserted (DLY_PU_SOC + 2) sclk clock period later after power up"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == AdcPowerCtrl::B0
    }
    #[doc = "ADC power up and reset. start signal will be asserted (DLY_PU_SOC + 2) sclk clock period later after power up"]
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
    #[doc = "ADC power up and reset. start signal will be asserted (DLY_PU_SOC + 2) sclk clock period later after power up"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(AdcPowerCtrl::B0)
    }
    #[doc = "ADC power up and reset. start signal will be asserted (DLY_PU_SOC + 2) sclk clock period later after power up"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(AdcPowerCtrl::B1)
    }
}
#[doc = "Interrupt enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEn {
    #[doc = "0: Enable"]
    B0 = 0,
    #[doc = "1: Enable"]
    B1 = 1,
}
impl From<IntEn> for bool {
    #[inline(always)]
    fn from(variant: IntEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EN` reader - Interrupt enable."]
pub type IntEnR = crate::BitReader<IntEn>;
impl IntEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEn {
        match self.bits {
            false => IntEn::B0,
            true => IntEn::B1,
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IntEn::B0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IntEn::B1
    }
}
#[doc = "Field `INT_EN` writer - Interrupt enable."]
pub type IntEnW<'a, REG> = crate::BitWriter<'a, REG, IntEn>;
impl<'a, REG> IntEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(IntEn::B0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(IntEn::B1)
    }
}
#[doc = "Field `INT_STATUS` reader - Interrupt status. This bit will be set to 1 when end-of-conversion. Set 0 to clear the interrupt."]
pub type IntStatusR = crate::BitReader;
#[doc = "Field `INT_STATUS` writer - Interrupt status. This bit will be set to 1 when end-of-conversion. Set 0 to clear the interrupt."]
pub type IntStatusW<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 5 - Interrupt enable."]
    #[inline(always)]
    pub fn int_en(&self) -> IntEnR {
        IntEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Interrupt status. This bit will be set to 1 when end-of-conversion. Set 0 to clear the interrupt."]
    #[inline(always)]
    pub fn int_status(&self) -> IntStatusR {
        IntStatusR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - ADC input source selection(CH_SEL\\[2:0\\])."]
    #[inline(always)]
    #[must_use]
    pub fn adc_input_src_sel(&mut self) -> AdcInputSrcSelW<SaradcCtrlSpec> {
        AdcInputSrcSelW::new(self, 0)
    }
    #[doc = "Bit 3 - ADC power down control bit"]
    #[inline(always)]
    #[must_use]
    pub fn adc_power_ctrl(&mut self) -> AdcPowerCtrlW<SaradcCtrlSpec> {
        AdcPowerCtrlW::new(self, 3)
    }
    #[doc = "Bit 5 - Interrupt enable."]
    #[inline(always)]
    #[must_use]
    pub fn int_en(&mut self) -> IntEnW<SaradcCtrlSpec> {
        IntEnW::new(self, 5)
    }
    #[doc = "Bit 6 - Interrupt status. This bit will be set to 1 when end-of-conversion. Set 0 to clear the interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn int_status(&mut self) -> IntStatusW<SaradcCtrlSpec> {
        IntStatusW::new(self, 6)
    }
}
#[doc = "The control register of A/D Converter.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`saradc_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`saradc_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SaradcCtrlSpec;
impl crate::RegisterSpec for SaradcCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`saradc_ctrl::R`](R) reader structure"]
impl crate::Readable for SaradcCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`saradc_ctrl::W`](W) writer structure"]
impl crate::Writable for SaradcCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SARADC_CTRL to value 0"]
impl crate::Resettable for SaradcCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
