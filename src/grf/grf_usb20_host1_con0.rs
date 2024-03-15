#[doc = "Register `GRF_USB20_HOST1_CON0` reader"]
pub type R = crate::R<GrfUsb20Host1Con0Spec>;
#[doc = "Register `GRF_USB20_HOST1_CON0` writer"]
pub type W = crate::W<GrfUsb20Host1Con0Spec>;
#[doc = "Field `APP_PRT_OVRCUR` reader - app_prt_ovrcur"]
pub type AppPrtOvrcurR = crate::BitReader;
#[doc = "Field `APP_PRT_OVRCUR` writer - app_prt_ovrcur"]
pub type AppPrtOvrcurW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APP_START_CLK` reader - app_start_clk"]
pub type AppStartClkR = crate::BitReader;
#[doc = "Field `APP_START_CLK` writer - app_start_clk"]
pub type AppStartClkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARB_PAUSE` reader - arb_pause"]
pub type ArbPauseR = crate::BitReader;
#[doc = "Field `ARB_PAUSE` writer - arb_pause"]
pub type ArbPauseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTOPPD_ON_OVERCUR_EN` reader - autoppd_on_overcur_en"]
pub type AutoppdOnOvercurEnR = crate::BitReader;
#[doc = "Field `AUTOPPD_ON_OVERCUR_EN` writer - autoppd_on_overcur_en"]
pub type AutoppdOnOvercurEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HUBSETUP_MIN` reader - hubsetup_min"]
pub type HubsetupMinR = crate::BitReader;
#[doc = "Field `HUBSETUP_MIN` writer - hubsetup_min"]
pub type HubsetupMinW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "incr16_en\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Incr16En {
    #[doc = "1: disable AHB INCR16 burst"]
    B1 = 1,
    #[doc = "0: disable AHB INCR16 burst"]
    B0 = 0,
}
impl From<Incr16En> for bool {
    #[inline(always)]
    fn from(variant: Incr16En) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INCR16_EN` reader - incr16_en"]
pub type Incr16EnR = crate::BitReader<Incr16En>;
impl Incr16EnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Incr16En {
        match self.bits {
            true => Incr16En::B1,
            false => Incr16En::B0,
        }
    }
    #[doc = "disable AHB INCR16 burst"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Incr16En::B1
    }
    #[doc = "disable AHB INCR16 burst"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Incr16En::B0
    }
}
#[doc = "Field `INCR16_EN` writer - incr16_en"]
pub type Incr16EnW<'a, REG> = crate::BitWriter<'a, REG, Incr16En>;
impl<'a, REG> Incr16EnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable AHB INCR16 burst"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Incr16En::B1)
    }
    #[doc = "disable AHB INCR16 burst"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Incr16En::B0)
    }
}
#[doc = "incr4_en\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Incr4En {
    #[doc = "1: disable AHB INCR4 burst"]
    B1 = 1,
    #[doc = "0: disable AHB INCR4 burst"]
    B0 = 0,
}
impl From<Incr4En> for bool {
    #[inline(always)]
    fn from(variant: Incr4En) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INCR4_EN` reader - incr4_en"]
pub type Incr4EnR = crate::BitReader<Incr4En>;
impl Incr4EnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Incr4En {
        match self.bits {
            true => Incr4En::B1,
            false => Incr4En::B0,
        }
    }
    #[doc = "disable AHB INCR4 burst"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Incr4En::B1
    }
    #[doc = "disable AHB INCR4 burst"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Incr4En::B0
    }
}
#[doc = "Field `INCR4_EN` writer - incr4_en"]
pub type Incr4EnW<'a, REG> = crate::BitWriter<'a, REG, Incr4En>;
impl<'a, REG> Incr4EnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable AHB INCR4 burst"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Incr4En::B1)
    }
    #[doc = "disable AHB INCR4 burst"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Incr4En::B0)
    }
}
#[doc = "incr8_en\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Incr8En {
    #[doc = "1: disable AHB INCR8 burst"]
    B1 = 1,
    #[doc = "0: disable AHB INCR8 burst"]
    B0 = 0,
}
impl From<Incr8En> for bool {
    #[inline(always)]
    fn from(variant: Incr8En) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INCR8_EN` reader - incr8_en"]
pub type Incr8EnR = crate::BitReader<Incr8En>;
impl Incr8EnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Incr8En {
        match self.bits {
            true => Incr8En::B1,
            false => Incr8En::B0,
        }
    }
    #[doc = "disable AHB INCR8 burst"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Incr8En::B1
    }
    #[doc = "disable AHB INCR8 burst"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Incr8En::B0
    }
}
#[doc = "Field `INCR8_EN` writer - incr8_en"]
pub type Incr8EnW<'a, REG> = crate::BitWriter<'a, REG, Incr8En>;
impl<'a, REG> Incr8EnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable AHB INCR8 burst"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Incr8En::B1)
    }
    #[doc = "disable AHB INCR8 burst"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Incr8En::B0)
    }
}
#[doc = "incrx_en Forces AHB master to start INCR4/8/16 busts only on burst boundaries. AHB requires that double word width burst be addressed-aligned only to the double-word boundary.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IncrxEn {
    #[doc = "1: Normal AHB operation; start bursts on any double word boundary Note: When this function is enabled, the burst are started only when the lowest bits of haddr are: INCR4: haddr\\[3:0\\]
== 4'b0000 INCR8: haddr\\[4:0\\]
== 5'b00000 INCR16: haddr\\[5:0\\]
== 6'b000000"]
    B1 = 1,
    #[doc = "0: Normal AHB operation; start bursts on any double word boundary Note: When this function is enabled, the burst are started only when the lowest bits of haddr are: INCR4: haddr\\[3:0\\]
== 4'b0000 INCR8: haddr\\[4:0\\]
== 5'b00000 INCR16: haddr\\[5:0\\]
== 6'b000000"]
    B0 = 0,
}
impl From<IncrxEn> for bool {
    #[inline(always)]
    fn from(variant: IncrxEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INCRX_EN` reader - incrx_en Forces AHB master to start INCR4/8/16 busts only on burst boundaries. AHB requires that double word width burst be addressed-aligned only to the double-word boundary."]
pub type IncrxEnR = crate::BitReader<IncrxEn>;
impl IncrxEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IncrxEn {
        match self.bits {
            true => IncrxEn::B1,
            false => IncrxEn::B0,
        }
    }
    #[doc = "Normal AHB operation; start bursts on any double word boundary Note: When this function is enabled, the burst are started only when the lowest bits of haddr are: INCR4: haddr\\[3:0\\]
== 4'b0000 INCR8: haddr\\[4:0\\]
== 5'b00000 INCR16: haddr\\[5:0\\]
== 6'b000000"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IncrxEn::B1
    }
    #[doc = "Normal AHB operation; start bursts on any double word boundary Note: When this function is enabled, the burst are started only when the lowest bits of haddr are: INCR4: haddr\\[3:0\\]
== 4'b0000 INCR8: haddr\\[4:0\\]
== 5'b00000 INCR16: haddr\\[5:0\\]
== 6'b000000"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IncrxEn::B0
    }
}
#[doc = "Field `INCRX_EN` writer - incrx_en Forces AHB master to start INCR4/8/16 busts only on burst boundaries. AHB requires that double word width burst be addressed-aligned only to the double-word boundary."]
pub type IncrxEnW<'a, REG> = crate::BitWriter<'a, REG, IncrxEn>;
impl<'a, REG> IncrxEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal AHB operation; start bursts on any double word boundary Note: When this function is enabled, the burst are started only when the lowest bits of haddr are: INCR4: haddr\\[3:0\\]
== 4'b0000 INCR8: haddr\\[4:0\\]
== 5'b00000 INCR16: haddr\\[5:0\\]
== 6'b000000"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(IncrxEn::B1)
    }
    #[doc = "Normal AHB operation; start bursts on any double word boundary Note: When this function is enabled, the burst are started only when the lowest bits of haddr are: INCR4: haddr\\[3:0\\]
== 4'b0000 INCR8: haddr\\[4:0\\]
== 5'b00000 INCR16: haddr\\[5:0\\]
== 6'b000000"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(IncrxEn::B0)
    }
}
#[doc = "Field `OHCI_CLKCKTRST` reader - ohci_clkcktrst"]
pub type OhciClkcktrstR = crate::BitReader;
#[doc = "Field `OHCI_CLKCKTRST` writer - ohci_clkcktrst"]
pub type OhciClkcktrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OHCI_CNTSEL` reader - ohci_cntsel"]
pub type OhciCntselR = crate::BitReader;
#[doc = "Field `OHCI_CNTSEL` writer - ohci_cntsel"]
pub type OhciCntselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OHCI_SUSP_LGCY` reader - ohci_susp_lgcy"]
pub type OhciSuspLgcyR = crate::BitReader;
#[doc = "Field `OHCI_SUSP_LGCY` writer - ohci_susp_lgcy"]
pub type OhciSuspLgcyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SIM_MODE` reader - sim_mode Simulation only."]
pub type SimModeR = crate::BitReader;
#[doc = "Field `SIM_MODE` writer - sim_mode Simulation only."]
pub type SimModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "word_if\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WordIf {
    #[doc = "1: select 8bit utmi interface Note: usb2phy only support 16bit interface."]
    B1 = 1,
    #[doc = "0: select 8bit utmi interface Note: usb2phy only support 16bit interface."]
    B0 = 0,
}
impl From<WordIf> for bool {
    #[inline(always)]
    fn from(variant: WordIf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WORD_IF` reader - word_if"]
pub type WordIfR = crate::BitReader<WordIf>;
impl WordIfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WordIf {
        match self.bits {
            true => WordIf::B1,
            false => WordIf::B0,
        }
    }
    #[doc = "select 8bit utmi interface Note: usb2phy only support 16bit interface."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == WordIf::B1
    }
    #[doc = "select 8bit utmi interface Note: usb2phy only support 16bit interface."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == WordIf::B0
    }
}
#[doc = "Field `WORD_IF` writer - word_if"]
pub type WordIfW<'a, REG> = crate::BitWriter<'a, REG, WordIf>;
impl<'a, REG> WordIfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "select 8bit utmi interface Note: usb2phy only support 16bit interface."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(WordIf::B1)
    }
    #[doc = "select 8bit utmi interface Note: usb2phy only support 16bit interface."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(WordIf::B0)
    }
}
#[doc = "Field `WRITE_ENABLE` reader - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
pub type WriteEnableR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - app_prt_ovrcur"]
    #[inline(always)]
    pub fn app_prt_ovrcur(&self) -> AppPrtOvrcurR {
        AppPrtOvrcurR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - app_start_clk"]
    #[inline(always)]
    pub fn app_start_clk(&self) -> AppStartClkR {
        AppStartClkR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - arb_pause"]
    #[inline(always)]
    pub fn arb_pause(&self) -> ArbPauseR {
        ArbPauseR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - autoppd_on_overcur_en"]
    #[inline(always)]
    pub fn autoppd_on_overcur_en(&self) -> AutoppdOnOvercurEnR {
        AutoppdOnOvercurEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - hubsetup_min"]
    #[inline(always)]
    pub fn hubsetup_min(&self) -> HubsetupMinR {
        HubsetupMinR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - incr16_en"]
    #[inline(always)]
    pub fn incr16_en(&self) -> Incr16EnR {
        Incr16EnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - incr4_en"]
    #[inline(always)]
    pub fn incr4_en(&self) -> Incr4EnR {
        Incr4EnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - incr8_en"]
    #[inline(always)]
    pub fn incr8_en(&self) -> Incr8EnR {
        Incr8EnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - incrx_en Forces AHB master to start INCR4/8/16 busts only on burst boundaries. AHB requires that double word width burst be addressed-aligned only to the double-word boundary."]
    #[inline(always)]
    pub fn incrx_en(&self) -> IncrxEnR {
        IncrxEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - ohci_clkcktrst"]
    #[inline(always)]
    pub fn ohci_clkcktrst(&self) -> OhciClkcktrstR {
        OhciClkcktrstR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - ohci_cntsel"]
    #[inline(always)]
    pub fn ohci_cntsel(&self) -> OhciCntselR {
        OhciCntselR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - ohci_susp_lgcy"]
    #[inline(always)]
    pub fn ohci_susp_lgcy(&self) -> OhciSuspLgcyR {
        OhciSuspLgcyR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - sim_mode Simulation only."]
    #[inline(always)]
    pub fn sim_mode(&self) -> SimModeR {
        SimModeR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - word_if"]
    #[inline(always)]
    pub fn word_if(&self) -> WordIfR {
        WordIfR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
    #[inline(always)]
    pub fn write_enable(&self) -> WriteEnableR {
        WriteEnableR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - app_prt_ovrcur"]
    #[inline(always)]
    #[must_use]
    pub fn app_prt_ovrcur(&mut self) -> AppPrtOvrcurW<GrfUsb20Host1Con0Spec> {
        AppPrtOvrcurW::new(self, 0)
    }
    #[doc = "Bit 1 - app_start_clk"]
    #[inline(always)]
    #[must_use]
    pub fn app_start_clk(&mut self) -> AppStartClkW<GrfUsb20Host1Con0Spec> {
        AppStartClkW::new(self, 1)
    }
    #[doc = "Bit 2 - arb_pause"]
    #[inline(always)]
    #[must_use]
    pub fn arb_pause(&mut self) -> ArbPauseW<GrfUsb20Host1Con0Spec> {
        ArbPauseW::new(self, 2)
    }
    #[doc = "Bit 3 - autoppd_on_overcur_en"]
    #[inline(always)]
    #[must_use]
    pub fn autoppd_on_overcur_en(&mut self) -> AutoppdOnOvercurEnW<GrfUsb20Host1Con0Spec> {
        AutoppdOnOvercurEnW::new(self, 3)
    }
    #[doc = "Bit 4 - hubsetup_min"]
    #[inline(always)]
    #[must_use]
    pub fn hubsetup_min(&mut self) -> HubsetupMinW<GrfUsb20Host1Con0Spec> {
        HubsetupMinW::new(self, 4)
    }
    #[doc = "Bit 5 - incr16_en"]
    #[inline(always)]
    #[must_use]
    pub fn incr16_en(&mut self) -> Incr16EnW<GrfUsb20Host1Con0Spec> {
        Incr16EnW::new(self, 5)
    }
    #[doc = "Bit 6 - incr4_en"]
    #[inline(always)]
    #[must_use]
    pub fn incr4_en(&mut self) -> Incr4EnW<GrfUsb20Host1Con0Spec> {
        Incr4EnW::new(self, 6)
    }
    #[doc = "Bit 7 - incr8_en"]
    #[inline(always)]
    #[must_use]
    pub fn incr8_en(&mut self) -> Incr8EnW<GrfUsb20Host1Con0Spec> {
        Incr8EnW::new(self, 7)
    }
    #[doc = "Bit 8 - incrx_en Forces AHB master to start INCR4/8/16 busts only on burst boundaries. AHB requires that double word width burst be addressed-aligned only to the double-word boundary."]
    #[inline(always)]
    #[must_use]
    pub fn incrx_en(&mut self) -> IncrxEnW<GrfUsb20Host1Con0Spec> {
        IncrxEnW::new(self, 8)
    }
    #[doc = "Bit 9 - ohci_clkcktrst"]
    #[inline(always)]
    #[must_use]
    pub fn ohci_clkcktrst(&mut self) -> OhciClkcktrstW<GrfUsb20Host1Con0Spec> {
        OhciClkcktrstW::new(self, 9)
    }
    #[doc = "Bit 10 - ohci_cntsel"]
    #[inline(always)]
    #[must_use]
    pub fn ohci_cntsel(&mut self) -> OhciCntselW<GrfUsb20Host1Con0Spec> {
        OhciCntselW::new(self, 10)
    }
    #[doc = "Bit 11 - ohci_susp_lgcy"]
    #[inline(always)]
    #[must_use]
    pub fn ohci_susp_lgcy(&mut self) -> OhciSuspLgcyW<GrfUsb20Host1Con0Spec> {
        OhciSuspLgcyW::new(self, 11)
    }
    #[doc = "Bit 12 - sim_mode Simulation only."]
    #[inline(always)]
    #[must_use]
    pub fn sim_mode(&mut self) -> SimModeW<GrfUsb20Host1Con0Spec> {
        SimModeW::new(self, 12)
    }
    #[doc = "Bit 13 - word_if"]
    #[inline(always)]
    #[must_use]
    pub fn word_if(&mut self) -> WordIfW<GrfUsb20Host1Con0Spec> {
        WordIfW::new(self, 13)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<GrfUsb20Host1Con0Spec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "USB20 Host1 GRF register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_usb20_host1_con0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_usb20_host1_con0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrfUsb20Host1Con0Spec;
impl crate::RegisterSpec for GrfUsb20Host1Con0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grf_usb20_host1_con0::R`](R) reader structure"]
impl crate::Readable for GrfUsb20Host1Con0Spec {}
#[doc = "`write(|w| ..)` method takes [`grf_usb20_host1_con0::W`](W) writer structure"]
impl crate::Writable for GrfUsb20Host1Con0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GRF_USB20_HOST1_CON0 to value 0x23e0"]
impl crate::Resettable for GrfUsb20Host1Con0Spec {
    const RESET_VALUE: u32 = 0x23e0;
}
