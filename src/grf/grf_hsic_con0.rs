#[doc = "Register `GRF_HSIC_CON0` reader"]
pub type R = crate::R<GrfHsicCon0Spec>;
#[doc = "Register `GRF_HSIC_CON0` writer"]
pub type W = crate::W<GrfHsicCon0Spec>;
#[doc = "Field `HSIC_APP_PRT_OVRCUR` reader - app_prt_ovrcur"]
pub type HsicAppPrtOvrcurR = crate::BitReader;
#[doc = "Field `HSIC_APP_PRT_OVRCUR` writer - app_prt_ovrcur"]
pub type HsicAppPrtOvrcurW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSIC_APP_START_CLK` reader - app_start_clk"]
pub type HsicAppStartClkR = crate::BitReader;
#[doc = "Field `HSIC_APP_START_CLK` writer - app_start_clk"]
pub type HsicAppStartClkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSIC_AUTOPPD_ON_OVERCUR` reader - autoppd_on_overcur"]
pub type HsicAutoppdOnOvercurR = crate::BitReader;
#[doc = "Field `HSIC_AUTOPPD_ON_OVERCUR` writer - autoppd_on_overcur"]
pub type HsicAutoppdOnOvercurW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSIC_HUBSETUP_MIN` reader - hubsetup_min"]
pub type HsicHubsetupMinR = crate::BitReader;
#[doc = "Field `HSIC_HUBSETUP_MIN` writer - hubsetup_min"]
pub type HsicHubsetupMinW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "incr16_en\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HsicIncr16En {
    #[doc = "1: disable AHB INCR16 burst"]
    B1 = 1,
    #[doc = "0: disable AHB INCR16 burst"]
    B0 = 0,
}
impl From<HsicIncr16En> for bool {
    #[inline(always)]
    fn from(variant: HsicIncr16En) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSIC_INCR16_EN` reader - incr16_en"]
pub type HsicIncr16EnR = crate::BitReader<HsicIncr16En>;
impl HsicIncr16EnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HsicIncr16En {
        match self.bits {
            true => HsicIncr16En::B1,
            false => HsicIncr16En::B0,
        }
    }
    #[doc = "disable AHB INCR16 burst"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == HsicIncr16En::B1
    }
    #[doc = "disable AHB INCR16 burst"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == HsicIncr16En::B0
    }
}
#[doc = "Field `HSIC_INCR16_EN` writer - incr16_en"]
pub type HsicIncr16EnW<'a, REG> = crate::BitWriter<'a, REG, HsicIncr16En>;
impl<'a, REG> HsicIncr16EnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable AHB INCR16 burst"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(HsicIncr16En::B1)
    }
    #[doc = "disable AHB INCR16 burst"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(HsicIncr16En::B0)
    }
}
#[doc = "incr4_en\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HsicIncr4En {
    #[doc = "1: disable AHB INCR4 burst"]
    B1 = 1,
    #[doc = "0: disable AHB INCR4 burst"]
    B0 = 0,
}
impl From<HsicIncr4En> for bool {
    #[inline(always)]
    fn from(variant: HsicIncr4En) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSIC_INCR4_EN` reader - incr4_en"]
pub type HsicIncr4EnR = crate::BitReader<HsicIncr4En>;
impl HsicIncr4EnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HsicIncr4En {
        match self.bits {
            true => HsicIncr4En::B1,
            false => HsicIncr4En::B0,
        }
    }
    #[doc = "disable AHB INCR4 burst"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == HsicIncr4En::B1
    }
    #[doc = "disable AHB INCR4 burst"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == HsicIncr4En::B0
    }
}
#[doc = "Field `HSIC_INCR4_EN` writer - incr4_en"]
pub type HsicIncr4EnW<'a, REG> = crate::BitWriter<'a, REG, HsicIncr4En>;
impl<'a, REG> HsicIncr4EnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable AHB INCR4 burst"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(HsicIncr4En::B1)
    }
    #[doc = "disable AHB INCR4 burst"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(HsicIncr4En::B0)
    }
}
#[doc = "incr8_en\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HsicIncr8En {
    #[doc = "1: disable AHB INCR8 burst"]
    B1 = 1,
    #[doc = "0: disable AHB INCR8 burst"]
    B0 = 0,
}
impl From<HsicIncr8En> for bool {
    #[inline(always)]
    fn from(variant: HsicIncr8En) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSIC_INCR8_EN` reader - incr8_en"]
pub type HsicIncr8EnR = crate::BitReader<HsicIncr8En>;
impl HsicIncr8EnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HsicIncr8En {
        match self.bits {
            true => HsicIncr8En::B1,
            false => HsicIncr8En::B0,
        }
    }
    #[doc = "disable AHB INCR8 burst"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == HsicIncr8En::B1
    }
    #[doc = "disable AHB INCR8 burst"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == HsicIncr8En::B0
    }
}
#[doc = "Field `HSIC_INCR8_EN` writer - incr8_en"]
pub type HsicIncr8EnW<'a, REG> = crate::BitWriter<'a, REG, HsicIncr8En>;
impl<'a, REG> HsicIncr8EnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable AHB INCR8 burst"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(HsicIncr8En::B1)
    }
    #[doc = "disable AHB INCR8 burst"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(HsicIncr8En::B0)
    }
}
#[doc = "Burst Alignment Enable Forces AHB master to start INCR4/8/16 busts only on burst boundaries. AHB requires that double word width burst be addressed-aligned only to the double-word boundary.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HsicIncrxEn {
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
impl From<HsicIncrxEn> for bool {
    #[inline(always)]
    fn from(variant: HsicIncrxEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSIC_INCRX_EN` reader - Burst Alignment Enable Forces AHB master to start INCR4/8/16 busts only on burst boundaries. AHB requires that double word width burst be addressed-aligned only to the double-word boundary."]
pub type HsicIncrxEnR = crate::BitReader<HsicIncrxEn>;
impl HsicIncrxEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HsicIncrxEn {
        match self.bits {
            true => HsicIncrxEn::B1,
            false => HsicIncrxEn::B0,
        }
    }
    #[doc = "Normal AHB operation; start bursts on any double word boundary Note: When this function is enabled, the burst are started only when the lowest bits of haddr are: INCR4: haddr\\[3:0\\]
== 4'b0000 INCR8: haddr\\[4:0\\]
== 5'b00000 INCR16: haddr\\[5:0\\]
== 6'b000000"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == HsicIncrxEn::B1
    }
    #[doc = "Normal AHB operation; start bursts on any double word boundary Note: When this function is enabled, the burst are started only when the lowest bits of haddr are: INCR4: haddr\\[3:0\\]
== 4'b0000 INCR8: haddr\\[4:0\\]
== 5'b00000 INCR16: haddr\\[5:0\\]
== 6'b000000"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == HsicIncrxEn::B0
    }
}
#[doc = "Field `HSIC_INCRX_EN` writer - Burst Alignment Enable Forces AHB master to start INCR4/8/16 busts only on burst boundaries. AHB requires that double word width burst be addressed-aligned only to the double-word boundary."]
pub type HsicIncrxEnW<'a, REG> = crate::BitWriter<'a, REG, HsicIncrxEn>;
impl<'a, REG> HsicIncrxEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal AHB operation; start bursts on any double word boundary Note: When this function is enabled, the burst are started only when the lowest bits of haddr are: INCR4: haddr\\[3:0\\]
== 4'b0000 INCR8: haddr\\[4:0\\]
== 5'b00000 INCR16: haddr\\[5:0\\]
== 6'b000000"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(HsicIncrxEn::B1)
    }
    #[doc = "Normal AHB operation; start bursts on any double word boundary Note: When this function is enabled, the burst are started only when the lowest bits of haddr are: INCR4: haddr\\[3:0\\]
== 4'b0000 INCR8: haddr\\[4:0\\]
== 5'b00000 INCR16: haddr\\[5:0\\]
== 6'b000000"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(HsicIncrxEn::B0)
    }
}
#[doc = "Field `HSIC_SIM_MODE` reader - sim_mode Simulation only."]
pub type HsicSimModeR = crate::BitReader;
#[doc = "Field `HSIC_SIM_MODE` writer - sim_mode Simulation only."]
pub type HsicSimModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "word_if\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HsicWordIf {
    #[doc = "1: select 8bit utmi interface Note: HSICPHY only support 16bit utmi interface."]
    B1 = 1,
    #[doc = "0: select 8bit utmi interface Note: HSICPHY only support 16bit utmi interface."]
    B0 = 0,
}
impl From<HsicWordIf> for bool {
    #[inline(always)]
    fn from(variant: HsicWordIf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSIC_WORD_IF` reader - word_if"]
pub type HsicWordIfR = crate::BitReader<HsicWordIf>;
impl HsicWordIfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HsicWordIf {
        match self.bits {
            true => HsicWordIf::B1,
            false => HsicWordIf::B0,
        }
    }
    #[doc = "select 8bit utmi interface Note: HSICPHY only support 16bit utmi interface."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == HsicWordIf::B1
    }
    #[doc = "select 8bit utmi interface Note: HSICPHY only support 16bit utmi interface."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == HsicWordIf::B0
    }
}
#[doc = "Field `HSIC_WORD_IF` writer - word_if"]
pub type HsicWordIfW<'a, REG> = crate::BitWriter<'a, REG, HsicWordIf>;
impl<'a, REG> HsicWordIfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "select 8bit utmi interface Note: HSICPHY only support 16bit utmi interface."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(HsicWordIf::B1)
    }
    #[doc = "select 8bit utmi interface Note: HSICPHY only support 16bit utmi interface."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(HsicWordIf::B0)
    }
}
#[doc = "Field `WRITE_ENABLE` reader - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
pub type WriteEnableR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - app_prt_ovrcur"]
    #[inline(always)]
    pub fn hsic_app_prt_ovrcur(&self) -> HsicAppPrtOvrcurR {
        HsicAppPrtOvrcurR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - app_start_clk"]
    #[inline(always)]
    pub fn hsic_app_start_clk(&self) -> HsicAppStartClkR {
        HsicAppStartClkR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - autoppd_on_overcur"]
    #[inline(always)]
    pub fn hsic_autoppd_on_overcur(&self) -> HsicAutoppdOnOvercurR {
        HsicAutoppdOnOvercurR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - hubsetup_min"]
    #[inline(always)]
    pub fn hsic_hubsetup_min(&self) -> HsicHubsetupMinR {
        HsicHubsetupMinR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - incr16_en"]
    #[inline(always)]
    pub fn hsic_incr16_en(&self) -> HsicIncr16EnR {
        HsicIncr16EnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - incr4_en"]
    #[inline(always)]
    pub fn hsic_incr4_en(&self) -> HsicIncr4EnR {
        HsicIncr4EnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - incr8_en"]
    #[inline(always)]
    pub fn hsic_incr8_en(&self) -> HsicIncr8EnR {
        HsicIncr8EnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Burst Alignment Enable Forces AHB master to start INCR4/8/16 busts only on burst boundaries. AHB requires that double word width burst be addressed-aligned only to the double-word boundary."]
    #[inline(always)]
    pub fn hsic_incrx_en(&self) -> HsicIncrxEnR {
        HsicIncrxEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - sim_mode Simulation only."]
    #[inline(always)]
    pub fn hsic_sim_mode(&self) -> HsicSimModeR {
        HsicSimModeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - word_if"]
    #[inline(always)]
    pub fn hsic_word_if(&self) -> HsicWordIfR {
        HsicWordIfR::new(((self.bits >> 9) & 1) != 0)
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
    pub fn hsic_app_prt_ovrcur(&mut self) -> HsicAppPrtOvrcurW<GrfHsicCon0Spec> {
        HsicAppPrtOvrcurW::new(self, 0)
    }
    #[doc = "Bit 1 - app_start_clk"]
    #[inline(always)]
    #[must_use]
    pub fn hsic_app_start_clk(&mut self) -> HsicAppStartClkW<GrfHsicCon0Spec> {
        HsicAppStartClkW::new(self, 1)
    }
    #[doc = "Bit 2 - autoppd_on_overcur"]
    #[inline(always)]
    #[must_use]
    pub fn hsic_autoppd_on_overcur(&mut self) -> HsicAutoppdOnOvercurW<GrfHsicCon0Spec> {
        HsicAutoppdOnOvercurW::new(self, 2)
    }
    #[doc = "Bit 3 - hubsetup_min"]
    #[inline(always)]
    #[must_use]
    pub fn hsic_hubsetup_min(&mut self) -> HsicHubsetupMinW<GrfHsicCon0Spec> {
        HsicHubsetupMinW::new(self, 3)
    }
    #[doc = "Bit 4 - incr16_en"]
    #[inline(always)]
    #[must_use]
    pub fn hsic_incr16_en(&mut self) -> HsicIncr16EnW<GrfHsicCon0Spec> {
        HsicIncr16EnW::new(self, 4)
    }
    #[doc = "Bit 5 - incr4_en"]
    #[inline(always)]
    #[must_use]
    pub fn hsic_incr4_en(&mut self) -> HsicIncr4EnW<GrfHsicCon0Spec> {
        HsicIncr4EnW::new(self, 5)
    }
    #[doc = "Bit 6 - incr8_en"]
    #[inline(always)]
    #[must_use]
    pub fn hsic_incr8_en(&mut self) -> HsicIncr8EnW<GrfHsicCon0Spec> {
        HsicIncr8EnW::new(self, 6)
    }
    #[doc = "Bit 7 - Burst Alignment Enable Forces AHB master to start INCR4/8/16 busts only on burst boundaries. AHB requires that double word width burst be addressed-aligned only to the double-word boundary."]
    #[inline(always)]
    #[must_use]
    pub fn hsic_incrx_en(&mut self) -> HsicIncrxEnW<GrfHsicCon0Spec> {
        HsicIncrxEnW::new(self, 7)
    }
    #[doc = "Bit 8 - sim_mode Simulation only."]
    #[inline(always)]
    #[must_use]
    pub fn hsic_sim_mode(&mut self) -> HsicSimModeW<GrfHsicCon0Spec> {
        HsicSimModeW::new(self, 8)
    }
    #[doc = "Bit 9 - word_if"]
    #[inline(always)]
    #[must_use]
    pub fn hsic_word_if(&mut self) -> HsicWordIfW<GrfHsicCon0Spec> {
        HsicWordIfW::new(self, 9)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<GrfHsicCon0Spec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "HSIC controller GRF register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_hsic_con0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_hsic_con0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrfHsicCon0Spec;
impl crate::RegisterSpec for GrfHsicCon0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grf_hsic_con0::R`](R) reader structure"]
impl crate::Readable for GrfHsicCon0Spec {}
#[doc = "`write(|w| ..)` method takes [`grf_hsic_con0::W`](W) writer structure"]
impl crate::Writable for GrfHsicCon0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GRF_HSIC_CON0 to value 0x02f0"]
impl crate::Resettable for GrfHsicCon0Spec {
    const RESET_VALUE: u32 = 0x02f0;
}
