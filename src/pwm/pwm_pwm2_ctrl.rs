#[doc = "Register `PWM_PWM2_CTRL` reader"]
pub type R = crate::R<PwmPwm2CtrlSpec>;
#[doc = "Register `PWM_PWM2_CTRL` writer"]
pub type W = crate::W<PwmPwm2CtrlSpec>;
#[doc = "PWM channel enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PwmEn {
    #[doc = "0: disabled"]
    B0 = 0,
    #[doc = "1: enabled. If the PWM is worked in the one-shot mode, this bit will be cleared at the end of operation"]
    B1 = 1,
}
impl From<PwmEn> for bool {
    #[inline(always)]
    fn from(variant: PwmEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWM_EN` reader - PWM channel enable"]
pub type PwmEnR = crate::BitReader<PwmEn>;
impl PwmEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PwmEn {
        match self.bits {
            false => PwmEn::B0,
            true => PwmEn::B1,
        }
    }
    #[doc = "disabled"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PwmEn::B0
    }
    #[doc = "enabled. If the PWM is worked in the one-shot mode, this bit will be cleared at the end of operation"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PwmEn::B1
    }
}
#[doc = "Field `PWM_EN` writer - PWM channel enable"]
pub type PwmEnW<'a, REG> = crate::BitWriter<'a, REG, PwmEn>;
impl<'a, REG> PwmEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disabled"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PwmEn::B0)
    }
    #[doc = "enabled. If the PWM is worked in the one-shot mode, this bit will be cleared at the end of operation"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PwmEn::B1)
    }
}
#[doc = "PWM Operation Mode\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PwmMode {
    #[doc = "0: One shot mode. PWM produces the waveform within the repeated times defined by PWMx_CTRL_rpt."]
    B00 = 0,
    #[doc = "1: Continuous mode. PWM produces the waveform continuously"]
    B01 = 1,
    #[doc = "2: Capture mode. PWM measures the cycles of high/low polarity of input waveform."]
    B10 = 2,
    #[doc = "3: reserved"]
    B11 = 3,
}
impl From<PwmMode> for u8 {
    #[inline(always)]
    fn from(variant: PwmMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PwmMode {
    type Ux = u8;
}
#[doc = "Field `PWM_MODE` reader - PWM Operation Mode"]
pub type PwmModeR = crate::FieldReader<PwmMode>;
impl PwmModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PwmMode {
        match self.bits {
            0 => PwmMode::B00,
            1 => PwmMode::B01,
            2 => PwmMode::B10,
            3 => PwmMode::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "One shot mode. PWM produces the waveform within the repeated times defined by PWMx_CTRL_rpt."]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == PwmMode::B00
    }
    #[doc = "Continuous mode. PWM produces the waveform continuously"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == PwmMode::B01
    }
    #[doc = "Capture mode. PWM measures the cycles of high/low polarity of input waveform."]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == PwmMode::B10
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == PwmMode::B11
    }
}
#[doc = "Field `PWM_MODE` writer - PWM Operation Mode"]
pub type PwmModeW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, PwmMode>;
impl<'a, REG> PwmModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "One shot mode. PWM produces the waveform within the repeated times defined by PWMx_CTRL_rpt."]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(PwmMode::B00)
    }
    #[doc = "Continuous mode. PWM produces the waveform continuously"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(PwmMode::B01)
    }
    #[doc = "Capture mode. PWM measures the cycles of high/low polarity of input waveform."]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(PwmMode::B10)
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(PwmMode::B11)
    }
}
#[doc = "Duty Cycle Output Polarity\n\nThis defines the polarity for duty cycle. PWM starts the output\n\nwaveform with duty cycle.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DutyPol {
    #[doc = "0: negative"]
    B0 = 0,
    #[doc = "1: positive"]
    B1 = 1,
}
impl From<DutyPol> for bool {
    #[inline(always)]
    fn from(variant: DutyPol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DUTY_POL` reader - Duty Cycle Output Polarity\n\nThis defines the polarity for duty cycle. PWM starts the output\n\nwaveform with duty cycle."]
pub type DutyPolR = crate::BitReader<DutyPol>;
impl DutyPolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DutyPol {
        match self.bits {
            false => DutyPol::B0,
            true => DutyPol::B1,
        }
    }
    #[doc = "negative"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == DutyPol::B0
    }
    #[doc = "positive"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == DutyPol::B1
    }
}
#[doc = "Field `DUTY_POL` writer - Duty Cycle Output Polarity\n\nThis defines the polarity for duty cycle. PWM starts the output\n\nwaveform with duty cycle."]
pub type DutyPolW<'a, REG> = crate::BitWriter<'a, REG, DutyPol>;
impl<'a, REG> DutyPolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "negative"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(DutyPol::B0)
    }
    #[doc = "positive"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(DutyPol::B1)
    }
}
#[doc = "Inactive State Output Polarity\n\nThis defines the output waveform polarity when PWM channel is\n\nin inactive state. The inactive state means that PWM finishes the\n\ncomplete waveform in one-shot mode or PWM channel is\n\ndisabled.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InactivePol {
    #[doc = "0: negative"]
    B0 = 0,
    #[doc = "1: positive"]
    B1 = 1,
}
impl From<InactivePol> for bool {
    #[inline(always)]
    fn from(variant: InactivePol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INACTIVE_POL` reader - Inactive State Output Polarity\n\nThis defines the output waveform polarity when PWM channel is\n\nin inactive state. The inactive state means that PWM finishes the\n\ncomplete waveform in one-shot mode or PWM channel is\n\ndisabled."]
pub type InactivePolR = crate::BitReader<InactivePol>;
impl InactivePolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> InactivePol {
        match self.bits {
            false => InactivePol::B0,
            true => InactivePol::B1,
        }
    }
    #[doc = "negative"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == InactivePol::B0
    }
    #[doc = "positive"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == InactivePol::B1
    }
}
#[doc = "Field `INACTIVE_POL` writer - Inactive State Output Polarity\n\nThis defines the output waveform polarity when PWM channel is\n\nin inactive state. The inactive state means that PWM finishes the\n\ncomplete waveform in one-shot mode or PWM channel is\n\ndisabled."]
pub type InactivePolW<'a, REG> = crate::BitWriter<'a, REG, InactivePol>;
impl<'a, REG> InactivePolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "negative"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(InactivePol::B0)
    }
    #[doc = "positive"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(InactivePol::B1)
    }
}
#[doc = "PWM Output mode\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OutputMode {
    #[doc = "0: left aligned mode"]
    B0 = 0,
    #[doc = "1: center aligned mode"]
    B1 = 1,
}
impl From<OutputMode> for bool {
    #[inline(always)]
    fn from(variant: OutputMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OUTPUT_MODE` reader - PWM Output mode"]
pub type OutputModeR = crate::BitReader<OutputMode>;
impl OutputModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OutputMode {
        match self.bits {
            false => OutputMode::B0,
            true => OutputMode::B1,
        }
    }
    #[doc = "left aligned mode"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == OutputMode::B0
    }
    #[doc = "center aligned mode"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == OutputMode::B1
    }
}
#[doc = "Field `OUTPUT_MODE` writer - PWM Output mode"]
pub type OutputModeW<'a, REG> = crate::BitWriter<'a, REG, OutputMode>;
impl<'a, REG> OutputModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "left aligned mode"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(OutputMode::B0)
    }
    #[doc = "center aligned mode"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(OutputMode::B1)
    }
}
#[doc = "Low Power Mode Enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LpEn {
    #[doc = "0: disabled"]
    B0 = 0,
    #[doc = "1: enabled When PWM channel is inactive state and Low Power Mode is enabled, the path to PWM Clock prescale module is blocked to reduce power consumption."]
    B1 = 1,
}
impl From<LpEn> for bool {
    #[inline(always)]
    fn from(variant: LpEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LP_EN` reader - Low Power Mode Enable"]
pub type LpEnR = crate::BitReader<LpEn>;
impl LpEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LpEn {
        match self.bits {
            false => LpEn::B0,
            true => LpEn::B1,
        }
    }
    #[doc = "disabled"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == LpEn::B0
    }
    #[doc = "enabled When PWM channel is inactive state and Low Power Mode is enabled, the path to PWM Clock prescale module is blocked to reduce power consumption."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == LpEn::B1
    }
}
#[doc = "Field `LP_EN` writer - Low Power Mode Enable"]
pub type LpEnW<'a, REG> = crate::BitWriter<'a, REG, LpEn>;
impl<'a, REG> LpEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disabled"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(LpEn::B0)
    }
    #[doc = "enabled When PWM channel is inactive state and Low Power Mode is enabled, the path to PWM Clock prescale module is blocked to reduce power consumption."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(LpEn::B1)
    }
}
#[doc = "Clock Source Select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClkSel {
    #[doc = "0: non-scaled clock is selected as PWM clock source. It means that the prescale clock is directly used as the PWM clock source"]
    B0 = 0,
    #[doc = "1: scaled clock is selected as PWM clock source"]
    B1 = 1,
}
impl From<ClkSel> for bool {
    #[inline(always)]
    fn from(variant: ClkSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLK_SEL` reader - Clock Source Select"]
pub type ClkSelR = crate::BitReader<ClkSel>;
impl ClkSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClkSel {
        match self.bits {
            false => ClkSel::B0,
            true => ClkSel::B1,
        }
    }
    #[doc = "non-scaled clock is selected as PWM clock source. It means that the prescale clock is directly used as the PWM clock source"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ClkSel::B0
    }
    #[doc = "scaled clock is selected as PWM clock source"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ClkSel::B1
    }
}
#[doc = "Field `CLK_SEL` writer - Clock Source Select"]
pub type ClkSelW<'a, REG> = crate::BitWriter<'a, REG, ClkSel>;
impl<'a, REG> ClkSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "non-scaled clock is selected as PWM clock source. It means that the prescale clock is directly used as the PWM clock source"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ClkSel::B0)
    }
    #[doc = "scaled clock is selected as PWM clock source"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ClkSel::B1)
    }
}
#[doc = "Field `PRESCALE` reader - Prescale Factor\n\nThis field defines the prescale factor applied to input clock. The\n\nvalue N means that the input clock is divided by 2^N."]
pub type PrescaleR = crate::FieldReader;
#[doc = "Field `PRESCALE` writer - Prescale Factor\n\nThis field defines the prescale factor applied to input clock. The\n\nvalue N means that the input clock is divided by 2^N."]
pub type PrescaleW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SCALE` reader - Scale Factor\n\nThis field defines the scale factor applied to prescaled clock. The\n\nvalue N means the clock is divided by 2*N. If N is 0, it means\n\nthat the clock is divided by 512(2*256)."]
pub type ScaleR = crate::FieldReader;
#[doc = "Field `SCALE` writer - Scale Factor\n\nThis field defines the scale factor applied to prescaled clock. The\n\nvalue N means the clock is divided by 2*N. If N is 0, it means\n\nthat the clock is divided by 512(2*256)."]
pub type ScaleW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RPT` reader - Repeat Counter\n\nThis field defines the repeated effective periods of output\n\nwaveform in one-shot mode. The value N means N+1 repeated\n\neffective periods."]
pub type RptR = crate::FieldReader;
#[doc = "Field `RPT` writer - Repeat Counter\n\nThis field defines the repeated effective periods of output\n\nwaveform in one-shot mode. The value N means N+1 repeated\n\neffective periods."]
pub type RptW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - PWM channel enable"]
    #[inline(always)]
    pub fn pwm_en(&self) -> PwmEnR {
        PwmEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - PWM Operation Mode"]
    #[inline(always)]
    pub fn pwm_mode(&self) -> PwmModeR {
        PwmModeR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - Duty Cycle Output Polarity\n\nThis defines the polarity for duty cycle. PWM starts the output\n\nwaveform with duty cycle."]
    #[inline(always)]
    pub fn duty_pol(&self) -> DutyPolR {
        DutyPolR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Inactive State Output Polarity\n\nThis defines the output waveform polarity when PWM channel is\n\nin inactive state. The inactive state means that PWM finishes the\n\ncomplete waveform in one-shot mode or PWM channel is\n\ndisabled."]
    #[inline(always)]
    pub fn inactive_pol(&self) -> InactivePolR {
        InactivePolR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PWM Output mode"]
    #[inline(always)]
    pub fn output_mode(&self) -> OutputModeR {
        OutputModeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Low Power Mode Enable"]
    #[inline(always)]
    pub fn lp_en(&self) -> LpEnR {
        LpEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Clock Source Select"]
    #[inline(always)]
    pub fn clk_sel(&self) -> ClkSelR {
        ClkSelR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 12:14 - Prescale Factor\n\nThis field defines the prescale factor applied to input clock. The\n\nvalue N means that the input clock is divided by 2^N."]
    #[inline(always)]
    pub fn prescale(&self) -> PrescaleR {
        PrescaleR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:23 - Scale Factor\n\nThis field defines the scale factor applied to prescaled clock. The\n\nvalue N means the clock is divided by 2*N. If N is 0, it means\n\nthat the clock is divided by 512(2*256)."]
    #[inline(always)]
    pub fn scale(&self) -> ScaleR {
        ScaleR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Repeat Counter\n\nThis field defines the repeated effective periods of output\n\nwaveform in one-shot mode. The value N means N+1 repeated\n\neffective periods."]
    #[inline(always)]
    pub fn rpt(&self) -> RptR {
        RptR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - PWM channel enable"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_en(&mut self) -> PwmEnW<PwmPwm2CtrlSpec> {
        PwmEnW::new(self, 0)
    }
    #[doc = "Bits 1:2 - PWM Operation Mode"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_mode(&mut self) -> PwmModeW<PwmPwm2CtrlSpec> {
        PwmModeW::new(self, 1)
    }
    #[doc = "Bit 3 - Duty Cycle Output Polarity\n\nThis defines the polarity for duty cycle. PWM starts the output\n\nwaveform with duty cycle."]
    #[inline(always)]
    #[must_use]
    pub fn duty_pol(&mut self) -> DutyPolW<PwmPwm2CtrlSpec> {
        DutyPolW::new(self, 3)
    }
    #[doc = "Bit 4 - Inactive State Output Polarity\n\nThis defines the output waveform polarity when PWM channel is\n\nin inactive state. The inactive state means that PWM finishes the\n\ncomplete waveform in one-shot mode or PWM channel is\n\ndisabled."]
    #[inline(always)]
    #[must_use]
    pub fn inactive_pol(&mut self) -> InactivePolW<PwmPwm2CtrlSpec> {
        InactivePolW::new(self, 4)
    }
    #[doc = "Bit 5 - PWM Output mode"]
    #[inline(always)]
    #[must_use]
    pub fn output_mode(&mut self) -> OutputModeW<PwmPwm2CtrlSpec> {
        OutputModeW::new(self, 5)
    }
    #[doc = "Bit 8 - Low Power Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lp_en(&mut self) -> LpEnW<PwmPwm2CtrlSpec> {
        LpEnW::new(self, 8)
    }
    #[doc = "Bit 9 - Clock Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn clk_sel(&mut self) -> ClkSelW<PwmPwm2CtrlSpec> {
        ClkSelW::new(self, 9)
    }
    #[doc = "Bits 12:14 - Prescale Factor\n\nThis field defines the prescale factor applied to input clock. The\n\nvalue N means that the input clock is divided by 2^N."]
    #[inline(always)]
    #[must_use]
    pub fn prescale(&mut self) -> PrescaleW<PwmPwm2CtrlSpec> {
        PrescaleW::new(self, 12)
    }
    #[doc = "Bits 16:23 - Scale Factor\n\nThis field defines the scale factor applied to prescaled clock. The\n\nvalue N means the clock is divided by 2*N. If N is 0, it means\n\nthat the clock is divided by 512(2*256)."]
    #[inline(always)]
    #[must_use]
    pub fn scale(&mut self) -> ScaleW<PwmPwm2CtrlSpec> {
        ScaleW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Repeat Counter\n\nThis field defines the repeated effective periods of output\n\nwaveform in one-shot mode. The value N means N+1 repeated\n\neffective periods."]
    #[inline(always)]
    #[must_use]
    pub fn rpt(&mut self) -> RptW<PwmPwm2CtrlSpec> {
        RptW::new(self, 24)
    }
}
#[doc = "PWM Channel 2 Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwm_pwm2_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwm_pwm2_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwmPwm2CtrlSpec;
impl crate::RegisterSpec for PwmPwm2CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwm_pwm2_ctrl::R`](R) reader structure"]
impl crate::Readable for PwmPwm2CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`pwm_pwm2_ctrl::W`](W) writer structure"]
impl crate::Writable for PwmPwm2CtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PWM_PWM2_CTRL to value 0"]
impl crate::Resettable for PwmPwm2CtrlSpec {
    const RESET_VALUE: u32 = 0;
}
