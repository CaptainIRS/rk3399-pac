#[doc = "Register `DDR_PI_REG_91` reader"]
pub type R = crate::R<DdrPiReg91Spec>;
#[doc = "Register `DDR_PI_REG_91` writer"]
pub type W = crate::W<DdrPiReg91Spec>;
#[doc = "Field `PI_WRLAT_ADJ_F2` reader - Indicates the adjustment value for PHY write timing. The suffix\n\n\"_f2\" of the parameter name is omitted when in non-DFS mode."]
pub type PiWrlatAdjF2R = crate::FieldReader;
#[doc = "Field `PI_WRLAT_ADJ_F2` writer - Indicates the adjustment value for PHY write timing. The suffix\n\n\"_f2\" of the parameter name is omitted when in non-DFS mode."]
pub type PiWrlatAdjF2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PI_TDFI_PHY_WRLAT` reader - Holds the calculated DFI tPHY_WRLAT timing parameter (in DFI\n\nPHY clocks), the maximum cycles between a write command and a\n\ndfi_wrdata_en assertion."]
pub type PiTdfiPhyWrlatR = crate::FieldReader;
#[doc = "Field `PI_TDFI_WRCSLAT_F0` reader - Defines the DFI tPHY_WRCSLAT timing parameter (in DFI PHY\n\nclocks), the maximum cycles between a write command and a\n\ndfi_wrdata_cs_n assertion. The suffix \"_f0\" of the parameter name\n\nis omitted when in non-DFS mode."]
pub type PiTdfiWrcslatF0R = crate::FieldReader;
#[doc = "Field `PI_TDFI_WRCSLAT_F0` writer - Defines the DFI tPHY_WRCSLAT timing parameter (in DFI PHY\n\nclocks), the maximum cycles between a write command and a\n\ndfi_wrdata_cs_n assertion. The suffix \"_f0\" of the parameter name\n\nis omitted when in non-DFS mode."]
pub type PiTdfiWrcslatF0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PI_TDFI_WRCSLAT_F1` reader - Defines the DFI tPHY_WRCSLAT timing parameter (in DFI PHY\n\nclocks), the maximum cycles between a write command and a\n\ndfi_wrdata_cs_n assertion. The suffix \"_f1\" of the parameter name\n\nis omitted when in non-DFS mode."]
pub type PiTdfiWrcslatF1R = crate::FieldReader;
#[doc = "Field `PI_TDFI_WRCSLAT_F1` writer - Defines the DFI tPHY_WRCSLAT timing parameter (in DFI PHY\n\nclocks), the maximum cycles between a write command and a\n\ndfi_wrdata_cs_n assertion. The suffix \"_f1\" of the parameter name\n\nis omitted when in non-DFS mode."]
pub type PiTdfiWrcslatF1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Indicates the adjustment value for PHY write timing. The suffix\n\n\"_f2\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_wrlat_adj_f2(&self) -> PiWrlatAdjF2R {
        PiWrlatAdjF2R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Holds the calculated DFI tPHY_WRLAT timing parameter (in DFI\n\nPHY clocks), the maximum cycles between a write command and a\n\ndfi_wrdata_en assertion."]
    #[inline(always)]
    pub fn pi_tdfi_phy_wrlat(&self) -> PiTdfiPhyWrlatR {
        PiTdfiPhyWrlatR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Defines the DFI tPHY_WRCSLAT timing parameter (in DFI PHY\n\nclocks), the maximum cycles between a write command and a\n\ndfi_wrdata_cs_n assertion. The suffix \"_f0\" of the parameter name\n\nis omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_tdfi_wrcslat_f0(&self) -> PiTdfiWrcslatF0R {
        PiTdfiWrcslatF0R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Defines the DFI tPHY_WRCSLAT timing parameter (in DFI PHY\n\nclocks), the maximum cycles between a write command and a\n\ndfi_wrdata_cs_n assertion. The suffix \"_f1\" of the parameter name\n\nis omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_tdfi_wrcslat_f1(&self) -> PiTdfiWrcslatF1R {
        PiTdfiWrcslatF1R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Indicates the adjustment value for PHY write timing. The suffix\n\n\"_f2\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_wrlat_adj_f2(&mut self) -> PiWrlatAdjF2W<DdrPiReg91Spec> {
        PiWrlatAdjF2W::new(self, 0)
    }
    #[doc = "Bits 16:23 - Defines the DFI tPHY_WRCSLAT timing parameter (in DFI PHY\n\nclocks), the maximum cycles between a write command and a\n\ndfi_wrdata_cs_n assertion. The suffix \"_f0\" of the parameter name\n\nis omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tdfi_wrcslat_f0(&mut self) -> PiTdfiWrcslatF0W<DdrPiReg91Spec> {
        PiTdfiWrcslatF0W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Defines the DFI tPHY_WRCSLAT timing parameter (in DFI PHY\n\nclocks), the maximum cycles between a write command and a\n\ndfi_wrdata_cs_n assertion. The suffix \"_f1\" of the parameter name\n\nis omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tdfi_wrcslat_f1(&mut self) -> PiTdfiWrcslatF1W<DdrPiReg91Spec> {
        PiTdfiWrcslatF1W::new(self, 24)
    }
}
#[doc = "DDR PHY Independent Register 91\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_91::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_91::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrPiReg91Spec;
impl crate::RegisterSpec for DdrPiReg91Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_pi_reg_91::R`](R) reader structure"]
impl crate::Readable for DdrPiReg91Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_pi_reg_91::W`](W) writer structure"]
impl crate::Writable for DdrPiReg91Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_PI_REG_91 to value 0"]
impl crate::Resettable for DdrPiReg91Spec {
    const RESET_VALUE: u32 = 0;
}
