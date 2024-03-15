#[doc = "Register `DENALI_CTL_299` reader"]
pub type R = crate::R<DenaliCtl299Spec>;
#[doc = "Register `DENALI_CTL_299` writer"]
pub type W = crate::W<DenaliCtl299Spec>;
#[doc = "Field `TDFI_CTRL_DELAY_F2` reader - Defines the DFI tCTRL_DELAY timing parameter (in DFI clocks), the delay between a DFI command change and a memory command."]
pub type TdfiCtrlDelayF2R = crate::FieldReader;
#[doc = "Field `TDFI_CTRL_DELAY_F2` writer - Defines the DFI tCTRL_DELAY timing parameter (in DFI clocks), the delay between a DFI command change and a memory command."]
pub type TdfiCtrlDelayF2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TDFI_DRAM_CLK_DISABLE` reader - Defines the DFI tDRAM_CLK_DISABLE timing parameter (in DFI clocks), the delay between a dfi_dram_clock_disable assertion and the memory clock disable."]
pub type TdfiDramClkDisableR = crate::FieldReader;
#[doc = "Field `TDFI_DRAM_CLK_DISABLE` writer - Defines the DFI tDRAM_CLK_DISABLE timing parameter (in DFI clocks), the delay between a dfi_dram_clock_disable assertion and the memory clock disable."]
pub type TdfiDramClkDisableW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TDFI_DRAM_CLK_ENABLE` reader - Defines the DFI tDRAM_CLK_ENABLE timing parameter (in DFI clocks), the delay between a dfi_dram_clk_disable de-assertion and the memory clock enable."]
pub type TdfiDramClkEnableR = crate::FieldReader;
#[doc = "Field `TDFI_DRAM_CLK_ENABLE` writer - Defines the DFI tDRAM_CLK_ENABLE timing parameter (in DFI clocks), the delay between a dfi_dram_clk_disable de-assertion and the memory clock enable."]
pub type TdfiDramClkEnableW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TDFI_WRLVL_EN` reader - Defines the DFI tWRLVL_EN timing parameter (in DFI clocks), the minimum cycles from a dfi_wrlvl_en assertion to the first dfi_wrlvl_strobe assertion."]
pub type TdfiWrlvlEnR = crate::FieldReader;
#[doc = "Field `TDFI_WRLVL_EN` writer - Defines the DFI tWRLVL_EN timing parameter (in DFI clocks), the minimum cycles from a dfi_wrlvl_en assertion to the first dfi_wrlvl_strobe assertion."]
pub type TdfiWrlvlEnW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:3 - Defines the DFI tCTRL_DELAY timing parameter (in DFI clocks), the delay between a DFI command change and a memory command."]
    #[inline(always)]
    pub fn tdfi_ctrl_delay_f2(&self) -> TdfiCtrlDelayF2R {
        TdfiCtrlDelayF2R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Defines the DFI tDRAM_CLK_DISABLE timing parameter (in DFI clocks), the delay between a dfi_dram_clock_disable assertion and the memory clock disable."]
    #[inline(always)]
    pub fn tdfi_dram_clk_disable(&self) -> TdfiDramClkDisableR {
        TdfiDramClkDisableR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Defines the DFI tDRAM_CLK_ENABLE timing parameter (in DFI clocks), the delay between a dfi_dram_clk_disable de-assertion and the memory clock enable."]
    #[inline(always)]
    pub fn tdfi_dram_clk_enable(&self) -> TdfiDramClkEnableR {
        TdfiDramClkEnableR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:31 - Defines the DFI tWRLVL_EN timing parameter (in DFI clocks), the minimum cycles from a dfi_wrlvl_en assertion to the first dfi_wrlvl_strobe assertion."]
    #[inline(always)]
    pub fn tdfi_wrlvl_en(&self) -> TdfiWrlvlEnR {
        TdfiWrlvlEnR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Defines the DFI tCTRL_DELAY timing parameter (in DFI clocks), the delay between a DFI command change and a memory command."]
    #[inline(always)]
    #[must_use]
    pub fn tdfi_ctrl_delay_f2(&mut self) -> TdfiCtrlDelayF2W<DenaliCtl299Spec> {
        TdfiCtrlDelayF2W::new(self, 0)
    }
    #[doc = "Bits 8:11 - Defines the DFI tDRAM_CLK_DISABLE timing parameter (in DFI clocks), the delay between a dfi_dram_clock_disable assertion and the memory clock disable."]
    #[inline(always)]
    #[must_use]
    pub fn tdfi_dram_clk_disable(&mut self) -> TdfiDramClkDisableW<DenaliCtl299Spec> {
        TdfiDramClkDisableW::new(self, 8)
    }
    #[doc = "Bits 16:19 - Defines the DFI tDRAM_CLK_ENABLE timing parameter (in DFI clocks), the delay between a dfi_dram_clk_disable de-assertion and the memory clock enable."]
    #[inline(always)]
    #[must_use]
    pub fn tdfi_dram_clk_enable(&mut self) -> TdfiDramClkEnableW<DenaliCtl299Spec> {
        TdfiDramClkEnableW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Defines the DFI tWRLVL_EN timing parameter (in DFI clocks), the minimum cycles from a dfi_wrlvl_en assertion to the first dfi_wrlvl_strobe assertion."]
    #[inline(always)]
    #[must_use]
    pub fn tdfi_wrlvl_en(&mut self) -> TdfiWrlvlEnW<DenaliCtl299Spec> {
        TdfiWrlvlEnW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_299::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_299::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl299Spec;
impl crate::RegisterSpec for DenaliCtl299Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_299::R`](R) reader structure"]
impl crate::Readable for DenaliCtl299Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_299::W`](W) writer structure"]
impl crate::Writable for DenaliCtl299Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_299 to value 0x02"]
impl crate::Resettable for DenaliCtl299Spec {
    const RESET_VALUE: u32 = 0x02;
}
