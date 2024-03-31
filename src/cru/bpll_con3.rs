#[doc = "Register `BPLL_CON3` reader"]
pub type R = crate::R<BpllCon3Spec>;
#[doc = "Register `BPLL_CON3` writer"]
pub type W = crate::W<BpllCon3Spec>;
#[doc = "Global power down\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PowerDown {
    #[doc = "0: no power down"]
    B0 = 0,
    #[doc = "1: power down"]
    B1 = 1,
}
impl From<PowerDown> for bool {
    #[inline(always)]
    fn from(variant: PowerDown) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POWER_DOWN` reader - Global power down"]
pub type PowerDownR = crate::BitReader<PowerDown>;
impl PowerDownR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PowerDown {
        match self.bits {
            false => PowerDown::B0,
            true => PowerDown::B1,
        }
    }
    #[doc = "no power down"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PowerDown::B0
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PowerDown::B1
    }
}
#[doc = "Field `POWER_DOWN` writer - Global power down"]
pub type PowerDownW<'a, REG> = crate::BitWriter<'a, REG, PowerDown>;
impl<'a, REG> PowerDownW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no power down"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PowerDown::B0)
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PowerDown::B1)
    }
}
#[doc = "PLL Bypass. FREF bypasses PLL to FOUTPOSTDIV\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bypass {
    #[doc = "0: no bypass"]
    B0 = 0,
    #[doc = "1: bypass"]
    B1 = 1,
}
impl From<Bypass> for bool {
    #[inline(always)]
    fn from(variant: Bypass) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BYPASS` reader - PLL Bypass. FREF bypasses PLL to FOUTPOSTDIV"]
pub type BypassR = crate::BitReader<Bypass>;
impl BypassR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bypass {
        match self.bits {
            false => Bypass::B0,
            true => Bypass::B1,
        }
    }
    #[doc = "no bypass"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bypass::B0
    }
    #[doc = "bypass"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bypass::B1
    }
}
#[doc = "Field `BYPASS` writer - PLL Bypass. FREF bypasses PLL to FOUTPOSTDIV"]
pub type BypassW<'a, REG> = crate::BitWriter<'a, REG, Bypass>;
impl<'a, REG> BypassW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no bypass"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bypass::B0)
    }
    #[doc = "bypass"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bypass::B1)
    }
}
#[doc = "Power down quantization noise cancellation DAC\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dacpd {
    #[doc = "0: no power down"]
    B0 = 0,
    #[doc = "1: power down"]
    B1 = 1,
}
impl From<Dacpd> for bool {
    #[inline(always)]
    fn from(variant: Dacpd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DACPD` reader - Power down quantization noise cancellation DAC"]
pub type DacpdR = crate::BitReader<Dacpd>;
impl DacpdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dacpd {
        match self.bits {
            false => Dacpd::B0,
            true => Dacpd::B1,
        }
    }
    #[doc = "no power down"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Dacpd::B0
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Dacpd::B1
    }
}
#[doc = "Field `DACPD` writer - Power down quantization noise cancellation DAC"]
pub type DacpdW<'a, REG> = crate::BitWriter<'a, REG, Dacpd>;
impl<'a, REG> DacpdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no power down"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Dacpd::B0)
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Dacpd::B1)
    }
}
#[doc = "PLL saturation behavior enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dsmpd {
    #[doc = "0: no power down"]
    B0 = 0,
    #[doc = "1: power down DSMPD = 1'b1 ( modulator is disabled, 'integer mode')"]
    B1 = 1,
}
impl From<Dsmpd> for bool {
    #[inline(always)]
    fn from(variant: Dsmpd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DSMPD` reader - PLL saturation behavior enable"]
pub type DsmpdR = crate::BitReader<Dsmpd>;
impl DsmpdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dsmpd {
        match self.bits {
            false => Dsmpd::B0,
            true => Dsmpd::B1,
        }
    }
    #[doc = "no power down"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Dsmpd::B0
    }
    #[doc = "power down DSMPD = 1'b1 ( modulator is disabled, 'integer mode')"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Dsmpd::B1
    }
}
#[doc = "Field `DSMPD` writer - PLL saturation behavior enable"]
pub type DsmpdW<'a, REG> = crate::BitWriter<'a, REG, Dsmpd>;
impl<'a, REG> DsmpdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no power down"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Dsmpd::B0)
    }
    #[doc = "power down DSMPD = 1'b1 ( modulator is disabled, 'integer mode')"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Dsmpd::B1)
    }
}
#[doc = "Power down all outputs except for buffered VCO clock\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Foutpostdivpd {
    #[doc = "0: no power down"]
    B0 = 0,
    #[doc = "1: power down"]
    B1 = 1,
}
impl From<Foutpostdivpd> for bool {
    #[inline(always)]
    fn from(variant: Foutpostdivpd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FOUTPOSTDIVPD` reader - Power down all outputs except for buffered VCO clock"]
pub type FoutpostdivpdR = crate::BitReader<Foutpostdivpd>;
impl FoutpostdivpdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Foutpostdivpd {
        match self.bits {
            false => Foutpostdivpd::B0,
            true => Foutpostdivpd::B1,
        }
    }
    #[doc = "no power down"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Foutpostdivpd::B0
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Foutpostdivpd::B1
    }
}
#[doc = "Field `FOUTPOSTDIVPD` writer - Power down all outputs except for buffered VCO clock"]
pub type FoutpostdivpdW<'a, REG> = crate::BitWriter<'a, REG, Foutpostdivpd>;
impl<'a, REG> FoutpostdivpdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no power down"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Foutpostdivpd::B0)
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Foutpostdivpd::B1)
    }
}
#[doc = "Power down buffered VCO clock\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Foutvcopd {
    #[doc = "0: no power down"]
    B0 = 0,
    #[doc = "1: power down"]
    B1 = 1,
}
impl From<Foutvcopd> for bool {
    #[inline(always)]
    fn from(variant: Foutvcopd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FOUTVCOPD` reader - Power down buffered VCO clock"]
pub type FoutvcopdR = crate::BitReader<Foutvcopd>;
impl FoutvcopdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Foutvcopd {
        match self.bits {
            false => Foutvcopd::B0,
            true => Foutvcopd::B1,
        }
    }
    #[doc = "no power down"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Foutvcopd::B0
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Foutvcopd::B1
    }
}
#[doc = "Field `FOUTVCOPD` writer - Power down buffered VCO clock"]
pub type FoutvcopdW<'a, REG> = crate::BitWriter<'a, REG, Foutvcopd>;
impl<'a, REG> FoutvcopdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no power down"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Foutvcopd::B0)
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Foutvcopd::B1)
    }
}
#[doc = "Power down 4-phase clocks and 2X, 3X, 4X clocks\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fout4phasepd {
    #[doc = "0: no power down"]
    B0 = 0,
    #[doc = "1: power down"]
    B1 = 1,
}
impl From<Fout4phasepd> for bool {
    #[inline(always)]
    fn from(variant: Fout4phasepd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FOUT4PHASEPD` reader - Power down 4-phase clocks and 2X, 3X, 4X clocks"]
pub type Fout4phasepdR = crate::BitReader<Fout4phasepd>;
impl Fout4phasepdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fout4phasepd {
        match self.bits {
            false => Fout4phasepd::B0,
            true => Fout4phasepd::B1,
        }
    }
    #[doc = "no power down"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Fout4phasepd::B0
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Fout4phasepd::B1
    }
}
#[doc = "Field `FOUT4PHASEPD` writer - Power down 4-phase clocks and 2X, 3X, 4X clocks"]
pub type Fout4phasepdW<'a, REG> = crate::BitWriter<'a, REG, Fout4phasepd>;
impl<'a, REG> Fout4phasepdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no power down"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Fout4phasepd::B0)
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Fout4phasepd::B1)
    }
}
#[doc = "PLL work mode select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PllWorkMode {
    #[doc = "0: Slow mode, clock from external 24MHz/26MHz OSC (default)"]
    B00 = 0,
    #[doc = "1: Normal mode, clock from PLL output"]
    B01 = 1,
    #[doc = "2: Deep slow mode, clock from external 32.768kHz"]
    B10 = 2,
}
impl From<PllWorkMode> for u8 {
    #[inline(always)]
    fn from(variant: PllWorkMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PllWorkMode {
    type Ux = u8;
}
#[doc = "Field `PLL_WORK_MODE` reader - PLL work mode select"]
pub type PllWorkModeR = crate::FieldReader<PllWorkMode>;
impl PllWorkModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PllWorkMode> {
        match self.bits {
            0 => Some(PllWorkMode::B00),
            1 => Some(PllWorkMode::B01),
            2 => Some(PllWorkMode::B10),
            _ => None,
        }
    }
    #[doc = "Slow mode, clock from external 24MHz/26MHz OSC (default)"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == PllWorkMode::B00
    }
    #[doc = "Normal mode, clock from PLL output"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == PllWorkMode::B01
    }
    #[doc = "Deep slow mode, clock from external 32.768kHz"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == PllWorkMode::B10
    }
}
#[doc = "Field `PLL_WORK_MODE` writer - PLL work mode select"]
pub type PllWorkModeW<'a, REG> = crate::FieldWriter<'a, REG, 2, PllWorkMode>;
impl<'a, REG> PllWorkModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Slow mode, clock from external 24MHz/26MHz OSC (default)"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(PllWorkMode::B00)
    }
    #[doc = "Normal mode, clock from PLL output"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(PllWorkMode::B01)
    }
    #[doc = "Deep slow mode, clock from external 32.768kHz"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(PllWorkMode::B10)
    }
}
#[doc = "Field `WRITE_MASK` writer - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - Global power down"]
    #[inline(always)]
    pub fn power_down(&self) -> PowerDownR {
        PowerDownR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PLL Bypass. FREF bypasses PLL to FOUTPOSTDIV"]
    #[inline(always)]
    pub fn bypass(&self) -> BypassR {
        BypassR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Power down quantization noise cancellation DAC"]
    #[inline(always)]
    pub fn dacpd(&self) -> DacpdR {
        DacpdR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PLL saturation behavior enable"]
    #[inline(always)]
    pub fn dsmpd(&self) -> DsmpdR {
        DsmpdR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Power down all outputs except for buffered VCO clock"]
    #[inline(always)]
    pub fn foutpostdivpd(&self) -> FoutpostdivpdR {
        FoutpostdivpdR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Power down buffered VCO clock"]
    #[inline(always)]
    pub fn foutvcopd(&self) -> FoutvcopdR {
        FoutvcopdR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Power down 4-phase clocks and 2X, 3X, 4X clocks"]
    #[inline(always)]
    pub fn fout4phasepd(&self) -> Fout4phasepdR {
        Fout4phasepdR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:9 - PLL work mode select"]
    #[inline(always)]
    pub fn pll_work_mode(&self) -> PllWorkModeR {
        PllWorkModeR::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Global power down"]
    #[inline(always)]
    #[must_use]
    pub fn power_down(&mut self) -> PowerDownW<BpllCon3Spec> {
        PowerDownW::new(self, 0)
    }
    #[doc = "Bit 1 - PLL Bypass. FREF bypasses PLL to FOUTPOSTDIV"]
    #[inline(always)]
    #[must_use]
    pub fn bypass(&mut self) -> BypassW<BpllCon3Spec> {
        BypassW::new(self, 1)
    }
    #[doc = "Bit 2 - Power down quantization noise cancellation DAC"]
    #[inline(always)]
    #[must_use]
    pub fn dacpd(&mut self) -> DacpdW<BpllCon3Spec> {
        DacpdW::new(self, 2)
    }
    #[doc = "Bit 3 - PLL saturation behavior enable"]
    #[inline(always)]
    #[must_use]
    pub fn dsmpd(&mut self) -> DsmpdW<BpllCon3Spec> {
        DsmpdW::new(self, 3)
    }
    #[doc = "Bit 4 - Power down all outputs except for buffered VCO clock"]
    #[inline(always)]
    #[must_use]
    pub fn foutpostdivpd(&mut self) -> FoutpostdivpdW<BpllCon3Spec> {
        FoutpostdivpdW::new(self, 4)
    }
    #[doc = "Bit 5 - Power down buffered VCO clock"]
    #[inline(always)]
    #[must_use]
    pub fn foutvcopd(&mut self) -> FoutvcopdW<BpllCon3Spec> {
        FoutvcopdW::new(self, 5)
    }
    #[doc = "Bit 6 - Power down 4-phase clocks and 2X, 3X, 4X clocks"]
    #[inline(always)]
    #[must_use]
    pub fn fout4phasepd(&mut self) -> Fout4phasepdW<BpllCon3Spec> {
        Fout4phasepdW::new(self, 6)
    }
    #[doc = "Bits 8:9 - PLL work mode select"]
    #[inline(always)]
    #[must_use]
    pub fn pll_work_mode(&mut self) -> PllWorkModeW<BpllCon3Spec> {
        PllWorkModeW::new(self, 8)
    }
    #[doc = "Bits 16:31 - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<BpllCon3Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "BPLL configuration register3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bpll_con3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bpll_con3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BpllCon3Spec;
impl crate::RegisterSpec for BpllCon3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bpll_con3::R`](R) reader structure"]
impl crate::Readable for BpllCon3Spec {}
#[doc = "`write(|w| ..)` method takes [`bpll_con3::W`](W) writer structure"]
impl crate::Writable for BpllCon3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BPLL_CON3 to value 0x08"]
impl crate::Resettable for BpllCon3Spec {
    const RESET_VALUE: u32 = 0x08;
}
