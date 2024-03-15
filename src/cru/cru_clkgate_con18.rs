#[doc = "Register `CRU_CLKGATE_CON18` reader"]
pub type R = crate::R<CruClkgateCon18Spec>;
#[doc = "Register `CRU_CLKGATE_CON18` writer"]
pub type W = crate::W<CruClkgateCon18Spec>;
#[doc = "Field `CLK_DDR0_MSCH_EN` reader - clk_ddr0_msch clock disable bit When HIGH, disable clock"]
pub type ClkDdr0MschEnR = crate::BitReader;
#[doc = "Field `CLK_DDR0_MSCH_EN` writer - clk_ddr0_msch clock disable bit When HIGH, disable clock"]
pub type ClkDdr0MschEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_DDRC0_EN` reader - clk_ddrc0 clock disable bit When HIGH, disable clock"]
pub type ClkDdrc0EnR = crate::BitReader;
#[doc = "Field `CLK_DDRC0_EN` writer - clk_ddrc0 clock disable bit When HIGH, disable clock"]
pub type ClkDdrc0EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_DDRPHY_CTRL0_EN` reader - clk_ddrphy_ctrl0 clock disable bit When HIGH, disable clock"]
pub type ClkDdrphyCtrl0EnR = crate::BitReader;
#[doc = "Field `CLK_DDRPHY_CTRL0_EN` writer - clk_ddrphy_ctrl0 clock disable bit When HIGH, disable clock"]
pub type ClkDdrphyCtrl0EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_DDRPHY0_EN` reader - clk_ddrphy0 clock disable bit When HIGH, disable clock"]
pub type ClkDdrphy0EnR = crate::BitReader;
#[doc = "Field `CLK_DDRPHY0_EN` writer - clk_ddrphy0 clock disable bit When HIGH, disable clock"]
pub type ClkDdrphy0EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_DDRCFG_MSCH0_EN` reader - clk_ddrcfg_msch0 clock disable bit When HIGH, disable clock"]
pub type ClkDdrcfgMsch0EnR = crate::BitReader;
#[doc = "Field `CLK_DDRCFG_MSCH0_EN` writer - clk_ddrcfg_msch0 clock disable bit When HIGH, disable clock"]
pub type ClkDdrcfgMsch0EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_DDR1_MSCH_EN` reader - clk_ddr1_msch clock disable bit When HIGH, disable clock"]
pub type ClkDdr1MschEnR = crate::BitReader;
#[doc = "Field `CLK_DDR1_MSCH_EN` writer - clk_ddr1_msch clock disable bit When HIGH, disable clock"]
pub type ClkDdr1MschEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_DDRC1_EN` reader - clk_ddrc1 clock disable bit When HIGH, disable clock"]
pub type ClkDdrc1EnR = crate::BitReader;
#[doc = "Field `CLK_DDRC1_EN` writer - clk_ddrc1 clock disable bit When HIGH, disable clock"]
pub type ClkDdrc1EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_DDRPHY_CTRL1_EN` reader - clk_ddrphy_ctrl1 clock disable bit When HIGH, disable clock"]
pub type ClkDdrphyCtrl1EnR = crate::BitReader;
#[doc = "Field `CLK_DDRPHY_CTRL1_EN` writer - clk_ddrphy_ctrl1 clock disable bit When HIGH, disable clock"]
pub type ClkDdrphyCtrl1EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_DDRPHY1_EN` reader - clk_ddrphy1 clock disable bit When HIGH, disable clock"]
pub type ClkDdrphy1EnR = crate::BitReader;
#[doc = "Field `CLK_DDRPHY1_EN` writer - clk_ddrphy1 clock disable bit When HIGH, disable clock"]
pub type ClkDdrphy1EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_DDRCFG_MSCH1_EN` reader - clk_ddrcfg_msch1 clock disable bit When HIGH, disable clock"]
pub type ClkDdrcfgMsch1EnR = crate::BitReader;
#[doc = "Field `CLK_DDRCFG_MSCH1_EN` writer - clk_ddrcfg_msch1 clock disable bit When HIGH, disable clock"]
pub type ClkDdrcfgMsch1EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCLK_CENTER_MAIN_NOC_EN` reader - pclk_center_main_noc clock disable bit When HIGH, disable clock Suggest always on"]
pub type PclkCenterMainNocEnR = crate::BitReader;
#[doc = "Field `PCLK_CENTER_MAIN_NOC_EN` writer - pclk_center_main_noc clock disable bit When HIGH, disable clock Suggest always on"]
pub type PclkCenterMainNocEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_DDR_CIC_EN` reader - clk_ddr_cic clock disable bit When HIGH, disable clock"]
pub type ClkDdrCicEnR = crate::BitReader;
#[doc = "Field `CLK_DDR_CIC_EN` writer - clk_ddr_cic clock disable bit When HIGH, disable clock"]
pub type ClkDdrCicEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCLK_DDR_MON_EN` reader - pclk_ddr_mon clock disable bit When HIGH, disable clock"]
pub type PclkDdrMonEnR = crate::BitReader;
#[doc = "Field `PCLK_DDR_MON_EN` writer - pclk_ddr_mon clock disable bit When HIGH, disable clock"]
pub type PclkDdrMonEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_DDR_MON_EN` reader - clk_ddr_mon clock disable bit When HIGH, disable clock"]
pub type ClkDdrMonEnR = crate::BitReader;
#[doc = "Field `CLK_DDR_MON_EN` writer - clk_ddr_mon clock disable bit When HIGH, disable clock"]
pub type ClkDdrMonEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_DDR_MON_TIMER_EN` reader - clk_ddr_mon_timer clock disable bit When HIGH, disable clock"]
pub type ClkDdrMonTimerEnR = crate::BitReader;
#[doc = "Field `CLK_DDR_MON_TIMER_EN` writer - clk_ddr_mon_timer clock disable bit When HIGH, disable clock"]
pub type ClkDdrMonTimerEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCLK_CIC_EN` reader - pclk_cic clock disable bit When HIGH, disable clock"]
pub type PclkCicEnR = crate::BitReader;
#[doc = "Field `PCLK_CIC_EN` writer - pclk_cic clock disable bit When HIGH, disable clock"]
pub type PclkCicEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_MASK` writer - write mask bits When every bit HIGH, enable the writing corresponding bit When every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - clk_ddr0_msch clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_ddr0_msch_en(&self) -> ClkDdr0MschEnR {
        ClkDdr0MschEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - clk_ddrc0 clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_ddrc0_en(&self) -> ClkDdrc0EnR {
        ClkDdrc0EnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - clk_ddrphy_ctrl0 clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_ddrphy_ctrl0_en(&self) -> ClkDdrphyCtrl0EnR {
        ClkDdrphyCtrl0EnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - clk_ddrphy0 clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_ddrphy0_en(&self) -> ClkDdrphy0EnR {
        ClkDdrphy0EnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - clk_ddrcfg_msch0 clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_ddrcfg_msch0_en(&self) -> ClkDdrcfgMsch0EnR {
        ClkDdrcfgMsch0EnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - clk_ddr1_msch clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_ddr1_msch_en(&self) -> ClkDdr1MschEnR {
        ClkDdr1MschEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - clk_ddrc1 clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_ddrc1_en(&self) -> ClkDdrc1EnR {
        ClkDdrc1EnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - clk_ddrphy_ctrl1 clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_ddrphy_ctrl1_en(&self) -> ClkDdrphyCtrl1EnR {
        ClkDdrphyCtrl1EnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - clk_ddrphy1 clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_ddrphy1_en(&self) -> ClkDdrphy1EnR {
        ClkDdrphy1EnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - clk_ddrcfg_msch1 clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_ddrcfg_msch1_en(&self) -> ClkDdrcfgMsch1EnR {
        ClkDdrcfgMsch1EnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - pclk_center_main_noc clock disable bit When HIGH, disable clock Suggest always on"]
    #[inline(always)]
    pub fn pclk_center_main_noc_en(&self) -> PclkCenterMainNocEnR {
        PclkCenterMainNocEnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - clk_ddr_cic clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_ddr_cic_en(&self) -> ClkDdrCicEnR {
        ClkDdrCicEnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - pclk_ddr_mon clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn pclk_ddr_mon_en(&self) -> PclkDdrMonEnR {
        PclkDdrMonEnR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - clk_ddr_mon clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_ddr_mon_en(&self) -> ClkDdrMonEnR {
        ClkDdrMonEnR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - clk_ddr_mon_timer clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_ddr_mon_timer_en(&self) -> ClkDdrMonTimerEnR {
        ClkDdrMonTimerEnR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - pclk_cic clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn pclk_cic_en(&self) -> PclkCicEnR {
        PclkCicEnR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - clk_ddr0_msch clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_ddr0_msch_en(&mut self) -> ClkDdr0MschEnW<CruClkgateCon18Spec> {
        ClkDdr0MschEnW::new(self, 0)
    }
    #[doc = "Bit 1 - clk_ddrc0 clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_ddrc0_en(&mut self) -> ClkDdrc0EnW<CruClkgateCon18Spec> {
        ClkDdrc0EnW::new(self, 1)
    }
    #[doc = "Bit 2 - clk_ddrphy_ctrl0 clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_ddrphy_ctrl0_en(&mut self) -> ClkDdrphyCtrl0EnW<CruClkgateCon18Spec> {
        ClkDdrphyCtrl0EnW::new(self, 2)
    }
    #[doc = "Bit 3 - clk_ddrphy0 clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_ddrphy0_en(&mut self) -> ClkDdrphy0EnW<CruClkgateCon18Spec> {
        ClkDdrphy0EnW::new(self, 3)
    }
    #[doc = "Bit 4 - clk_ddrcfg_msch0 clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_ddrcfg_msch0_en(&mut self) -> ClkDdrcfgMsch0EnW<CruClkgateCon18Spec> {
        ClkDdrcfgMsch0EnW::new(self, 4)
    }
    #[doc = "Bit 5 - clk_ddr1_msch clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_ddr1_msch_en(&mut self) -> ClkDdr1MschEnW<CruClkgateCon18Spec> {
        ClkDdr1MschEnW::new(self, 5)
    }
    #[doc = "Bit 6 - clk_ddrc1 clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_ddrc1_en(&mut self) -> ClkDdrc1EnW<CruClkgateCon18Spec> {
        ClkDdrc1EnW::new(self, 6)
    }
    #[doc = "Bit 7 - clk_ddrphy_ctrl1 clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_ddrphy_ctrl1_en(&mut self) -> ClkDdrphyCtrl1EnW<CruClkgateCon18Spec> {
        ClkDdrphyCtrl1EnW::new(self, 7)
    }
    #[doc = "Bit 8 - clk_ddrphy1 clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_ddrphy1_en(&mut self) -> ClkDdrphy1EnW<CruClkgateCon18Spec> {
        ClkDdrphy1EnW::new(self, 8)
    }
    #[doc = "Bit 9 - clk_ddrcfg_msch1 clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_ddrcfg_msch1_en(&mut self) -> ClkDdrcfgMsch1EnW<CruClkgateCon18Spec> {
        ClkDdrcfgMsch1EnW::new(self, 9)
    }
    #[doc = "Bit 10 - pclk_center_main_noc clock disable bit When HIGH, disable clock Suggest always on"]
    #[inline(always)]
    #[must_use]
    pub fn pclk_center_main_noc_en(&mut self) -> PclkCenterMainNocEnW<CruClkgateCon18Spec> {
        PclkCenterMainNocEnW::new(self, 10)
    }
    #[doc = "Bit 11 - clk_ddr_cic clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_ddr_cic_en(&mut self) -> ClkDdrCicEnW<CruClkgateCon18Spec> {
        ClkDdrCicEnW::new(self, 11)
    }
    #[doc = "Bit 12 - pclk_ddr_mon clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn pclk_ddr_mon_en(&mut self) -> PclkDdrMonEnW<CruClkgateCon18Spec> {
        PclkDdrMonEnW::new(self, 12)
    }
    #[doc = "Bit 13 - clk_ddr_mon clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_ddr_mon_en(&mut self) -> ClkDdrMonEnW<CruClkgateCon18Spec> {
        ClkDdrMonEnW::new(self, 13)
    }
    #[doc = "Bit 14 - clk_ddr_mon_timer clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_ddr_mon_timer_en(&mut self) -> ClkDdrMonTimerEnW<CruClkgateCon18Spec> {
        ClkDdrMonTimerEnW::new(self, 14)
    }
    #[doc = "Bit 15 - pclk_cic clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn pclk_cic_en(&mut self) -> PclkCicEnW<CruClkgateCon18Spec> {
        PclkCicEnW::new(self, 15)
    }
    #[doc = "Bits 16:31 - write mask bits When every bit HIGH, enable the writing corresponding bit When every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<CruClkgateCon18Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Internal clock gating register18\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clkgate_con18::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clkgate_con18::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CruClkgateCon18Spec;
impl crate::RegisterSpec for CruClkgateCon18Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cru_clkgate_con18::R`](R) reader structure"]
impl crate::Readable for CruClkgateCon18Spec {}
#[doc = "`write(|w| ..)` method takes [`cru_clkgate_con18::W`](W) writer structure"]
impl crate::Writable for CruClkgateCon18Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRU_CLKGATE_CON18 to value 0"]
impl crate::Resettable for CruClkgateCon18Spec {
    const RESET_VALUE: u32 = 0;
}
