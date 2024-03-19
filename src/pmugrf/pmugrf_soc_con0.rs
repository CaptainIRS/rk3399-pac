#[doc = "Register `PMUGRF_SOC_CON0` reader"]
pub type R = crate::R<PmugrfSocCon0Spec>;
#[doc = "Register `PMUGRF_SOC_CON0` writer"]
pub type W = crate::W<PmugrfSocCon0Spec>;
#[doc = "chip 32K clock source select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chip32kSrc {
    #[doc = "0: from external"]
    B0 = 0,
    #[doc = "1: from internal, pvtm"]
    B1 = 1,
}
impl From<Chip32kSrc> for bool {
    #[inline(always)]
    fn from(variant: Chip32kSrc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHIP_32K_SRC` reader - chip 32K clock source select"]
pub type Chip32kSrcR = crate::BitReader<Chip32kSrc>;
impl Chip32kSrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chip32kSrc {
        match self.bits {
            false => Chip32kSrc::B0,
            true => Chip32kSrc::B1,
        }
    }
    #[doc = "from external"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Chip32kSrc::B0
    }
    #[doc = "from internal, pvtm"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Chip32kSrc::B1
    }
}
#[doc = "Field `CHIP_32K_SRC` writer - chip 32K clock source select"]
pub type Chip32kSrcW<'a, REG> = crate::BitWriter<'a, REG, Chip32kSrc>;
impl<'a, REG> Chip32kSrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "from external"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Chip32kSrc::B0)
    }
    #[doc = "from internal, pvtm"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Chip32kSrc::B1)
    }
}
#[doc = "When pmu noc meet illegal access, the noc\n\nwill\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PmuNocStall {
    #[doc = "0: error reponse"]
    B0 = 0,
    #[doc = "1: stall"]
    B1 = 1,
}
impl From<PmuNocStall> for bool {
    #[inline(always)]
    fn from(variant: PmuNocStall) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PMU_NOC_STALL` reader - When pmu noc meet illegal access, the noc\n\nwill"]
pub type PmuNocStallR = crate::BitReader<PmuNocStall>;
impl PmuNocStallR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PmuNocStall {
        match self.bits {
            false => PmuNocStall::B0,
            true => PmuNocStall::B1,
        }
    }
    #[doc = "error reponse"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PmuNocStall::B0
    }
    #[doc = "stall"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PmuNocStall::B1
    }
}
#[doc = "Field `PMU_NOC_STALL` writer - When pmu noc meet illegal access, the noc\n\nwill"]
pub type PmuNocStallW<'a, REG> = crate::BitWriter<'a, REG, PmuNocStall>;
impl<'a, REG> PmuNocStallW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "error reponse"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PmuNocStall::B0)
    }
    #[doc = "stall"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PmuNocStall::B1)
    }
}
#[doc = "Field `PMU_MCU_NIU_OBSRV` reader - pmu_mcu_niu_obsrv"]
pub type PmuMcuNiuObsrvR = crate::BitReader;
#[doc = "Field `PMU_MCU_NIU_OBSRV` writer - pmu_mcu_niu_obsrv"]
pub type PmuMcuNiuObsrvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PMU_NOC_OBSRV` reader - pmu_noc_obsrv"]
pub type PmuNocObsrvR = crate::BitReader;
#[doc = "Field `PMU_NOC_OBSRV` writer - pmu_noc_obsrv"]
pub type PmuNocObsrvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CruPmuPclkGate {
    #[doc = "1: gate clock ;"]
    B1 = 1,
    #[doc = "0: not gate ."]
    B0 = 0,
}
impl From<CruPmuPclkGate> for bool {
    #[inline(always)]
    fn from(variant: CruPmuPclkGate) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRU_PMU_PCLK_GATE` reader - "]
pub type CruPmuPclkGateR = crate::BitReader<CruPmuPclkGate>;
impl CruPmuPclkGateR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CruPmuPclkGate {
        match self.bits {
            true => CruPmuPclkGate::B1,
            false => CruPmuPclkGate::B0,
        }
    }
    #[doc = "gate clock ;"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == CruPmuPclkGate::B1
    }
    #[doc = "not gate ."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == CruPmuPclkGate::B0
    }
}
#[doc = "Field `CRU_PMU_PCLK_GATE` writer - "]
pub type CruPmuPclkGateW<'a, REG> = crate::BitWriter<'a, REG, CruPmuPclkGate>;
impl<'a, REG> CruPmuPclkGateW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "gate clock ;"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(CruPmuPclkGate::B1)
    }
    #[doc = "not gate ."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(CruPmuPclkGate::B0)
    }
}
#[doc = "Use 2 optional IOs for pwm3.\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwm3Sel {
    #[doc = "0: pwm3a"]
    B0 = 0,
    #[doc = "1: pwm3b"]
    B1 = 1,
}
impl From<Pwm3Sel> for bool {
    #[inline(always)]
    fn from(variant: Pwm3Sel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWM3_SEL` reader - Use 2 optional IOs for pwm3."]
pub type Pwm3SelR = crate::BitReader<Pwm3Sel>;
impl Pwm3SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwm3Sel {
        match self.bits {
            false => Pwm3Sel::B0,
            true => Pwm3Sel::B1,
        }
    }
    #[doc = "pwm3a"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Pwm3Sel::B0
    }
    #[doc = "pwm3b"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Pwm3Sel::B1
    }
}
#[doc = "Field `PWM3_SEL` writer - Use 2 optional IOs for pwm3."]
pub type Pwm3SelW<'a, REG> = crate::BitWriter<'a, REG, Pwm3Sel>;
impl<'a, REG> Pwm3SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "pwm3a"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Pwm3Sel::B0)
    }
    #[doc = "pwm3b"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Pwm3Sel::B1)
    }
}
#[doc = "pd_alive pclk_niu gating.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PclkAliveNiuEn {
    #[doc = "1: gating ;"]
    B1 = 1,
    #[doc = "0: not gating ."]
    B0 = 0,
}
impl From<PclkAliveNiuEn> for bool {
    #[inline(always)]
    fn from(variant: PclkAliveNiuEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PCLK_ALIVE_NIU_EN` reader - pd_alive pclk_niu gating."]
pub type PclkAliveNiuEnR = crate::BitReader<PclkAliveNiuEn>;
impl PclkAliveNiuEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PclkAliveNiuEn {
        match self.bits {
            true => PclkAliveNiuEn::B1,
            false => PclkAliveNiuEn::B0,
        }
    }
    #[doc = "gating ;"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PclkAliveNiuEn::B1
    }
    #[doc = "not gating ."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PclkAliveNiuEn::B0
    }
}
#[doc = "Field `PCLK_ALIVE_NIU_EN` writer - pd_alive pclk_niu gating."]
pub type PclkAliveNiuEnW<'a, REG> = crate::BitWriter<'a, REG, PclkAliveNiuEn>;
impl<'a, REG> PclkAliveNiuEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "gating ;"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PclkAliveNiuEn::B1)
    }
    #[doc = "not gating ."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PclkAliveNiuEn::B0)
    }
}
#[doc = "pmu GPIO1 1.8v/3.0v control source select.\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pmu1830Volsel {
    #[doc = "0: controlled by IO_GPIO0B1 ;"]
    B0 = 0,
    #[doc = "1: controlled by PMUGRF.SOC_CON0.pmu1830_vol"]
    B1 = 1,
}
impl From<Pmu1830Volsel> for bool {
    #[inline(always)]
    fn from(variant: Pmu1830Volsel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PMU1830_VOLSEL` reader - pmu GPIO1 1.8v/3.0v control source select."]
pub type Pmu1830VolselR = crate::BitReader<Pmu1830Volsel>;
impl Pmu1830VolselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pmu1830Volsel {
        match self.bits {
            false => Pmu1830Volsel::B0,
            true => Pmu1830Volsel::B1,
        }
    }
    #[doc = "controlled by IO_GPIO0B1 ;"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Pmu1830Volsel::B0
    }
    #[doc = "controlled by PMUGRF.SOC_CON0.pmu1830_vol"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Pmu1830Volsel::B1
    }
}
#[doc = "Field `PMU1830_VOLSEL` writer - pmu GPIO1 1.8v/3.0v control source select."]
pub type Pmu1830VolselW<'a, REG> = crate::BitWriter<'a, REG, Pmu1830Volsel>;
impl<'a, REG> Pmu1830VolselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "controlled by IO_GPIO0B1 ;"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Pmu1830Volsel::B0)
    }
    #[doc = "controlled by PMUGRF.SOC_CON0.pmu1830_vol"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Pmu1830Volsel::B1)
    }
}
#[doc = "pmu IO 1.8v/3.0v select.\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pmu1830Vol {
    #[doc = "0: 3.0v ;"]
    B0 = 0,
    #[doc = "1: 1.8v ;"]
    B1 = 1,
}
impl From<Pmu1830Vol> for bool {
    #[inline(always)]
    fn from(variant: Pmu1830Vol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PMU1830_VOL` reader - pmu IO 1.8v/3.0v select."]
pub type Pmu1830VolR = crate::BitReader<Pmu1830Vol>;
impl Pmu1830VolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pmu1830Vol {
        match self.bits {
            false => Pmu1830Vol::B0,
            true => Pmu1830Vol::B1,
        }
    }
    #[doc = "3.0v ;"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Pmu1830Vol::B0
    }
    #[doc = "1.8v ;"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Pmu1830Vol::B1
    }
}
#[doc = "Field `PMU1830_VOL` writer - pmu IO 1.8v/3.0v select."]
pub type Pmu1830VolW<'a, REG> = crate::BitWriter<'a, REG, Pmu1830Vol>;
impl<'a, REG> Pmu1830VolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "3.0v ;"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Pmu1830Vol::B0)
    }
    #[doc = "1.8v ;"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Pmu1830Vol::B1)
    }
}
#[doc = "Field `WRITE_ENABLE` reader - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - chip 32K clock source select"]
    #[inline(always)]
    pub fn chip_32k_src(&self) -> Chip32kSrcR {
        Chip32kSrcR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - When pmu noc meet illegal access, the noc\n\nwill"]
    #[inline(always)]
    pub fn pmu_noc_stall(&self) -> PmuNocStallR {
        PmuNocStallR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - pmu_mcu_niu_obsrv"]
    #[inline(always)]
    pub fn pmu_mcu_niu_obsrv(&self) -> PmuMcuNiuObsrvR {
        PmuMcuNiuObsrvR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - pmu_noc_obsrv"]
    #[inline(always)]
    pub fn pmu_noc_obsrv(&self) -> PmuNocObsrvR {
        PmuNocObsrvR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cru_pmu_pclk_gate(&self) -> CruPmuPclkGateR {
        CruPmuPclkGateR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Use 2 optional IOs for pwm3."]
    #[inline(always)]
    pub fn pwm3_sel(&self) -> Pwm3SelR {
        Pwm3SelR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - pd_alive pclk_niu gating."]
    #[inline(always)]
    pub fn pclk_alive_niu_en(&self) -> PclkAliveNiuEnR {
        PclkAliveNiuEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - pmu GPIO1 1.8v/3.0v control source select."]
    #[inline(always)]
    pub fn pmu1830_volsel(&self) -> Pmu1830VolselR {
        Pmu1830VolselR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - pmu IO 1.8v/3.0v select."]
    #[inline(always)]
    pub fn pmu1830_vol(&self) -> Pmu1830VolR {
        Pmu1830VolR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    pub fn write_enable(&self) -> WriteEnableR {
        WriteEnableR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - chip 32K clock source select"]
    #[inline(always)]
    #[must_use]
    pub fn chip_32k_src(&mut self) -> Chip32kSrcW<PmugrfSocCon0Spec> {
        Chip32kSrcW::new(self, 0)
    }
    #[doc = "Bit 1 - When pmu noc meet illegal access, the noc\n\nwill"]
    #[inline(always)]
    #[must_use]
    pub fn pmu_noc_stall(&mut self) -> PmuNocStallW<PmugrfSocCon0Spec> {
        PmuNocStallW::new(self, 1)
    }
    #[doc = "Bit 2 - pmu_mcu_niu_obsrv"]
    #[inline(always)]
    #[must_use]
    pub fn pmu_mcu_niu_obsrv(&mut self) -> PmuMcuNiuObsrvW<PmugrfSocCon0Spec> {
        PmuMcuNiuObsrvW::new(self, 2)
    }
    #[doc = "Bit 3 - pmu_noc_obsrv"]
    #[inline(always)]
    #[must_use]
    pub fn pmu_noc_obsrv(&mut self) -> PmuNocObsrvW<PmugrfSocCon0Spec> {
        PmuNocObsrvW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn cru_pmu_pclk_gate(&mut self) -> CruPmuPclkGateW<PmugrfSocCon0Spec> {
        CruPmuPclkGateW::new(self, 4)
    }
    #[doc = "Bit 5 - Use 2 optional IOs for pwm3."]
    #[inline(always)]
    #[must_use]
    pub fn pwm3_sel(&mut self) -> Pwm3SelW<PmugrfSocCon0Spec> {
        Pwm3SelW::new(self, 5)
    }
    #[doc = "Bit 6 - pd_alive pclk_niu gating."]
    #[inline(always)]
    #[must_use]
    pub fn pclk_alive_niu_en(&mut self) -> PclkAliveNiuEnW<PmugrfSocCon0Spec> {
        PclkAliveNiuEnW::new(self, 6)
    }
    #[doc = "Bit 8 - pmu GPIO1 1.8v/3.0v control source select."]
    #[inline(always)]
    #[must_use]
    pub fn pmu1830_volsel(&mut self) -> Pmu1830VolselW<PmugrfSocCon0Spec> {
        Pmu1830VolselW::new(self, 8)
    }
    #[doc = "Bit 9 - pmu IO 1.8v/3.0v select."]
    #[inline(always)]
    #[must_use]
    pub fn pmu1830_vol(&mut self) -> Pmu1830VolW<PmugrfSocCon0Spec> {
        Pmu1830VolW::new(self, 9)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<PmugrfSocCon0Spec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "SoC control register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmugrf_soc_con0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmugrf_soc_con0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmugrfSocCon0Spec;
impl crate::RegisterSpec for PmugrfSocCon0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmugrf_soc_con0::R`](R) reader structure"]
impl crate::Readable for PmugrfSocCon0Spec {}
#[doc = "`write(|w| ..)` method takes [`pmugrf_soc_con0::W`](W) writer structure"]
impl crate::Writable for PmugrfSocCon0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PMUGRF_SOC_CON0 to value 0x0320"]
impl crate::Resettable for PmugrfSocCon0Spec {
    const RESET_VALUE: u32 = 0x0320;
}
