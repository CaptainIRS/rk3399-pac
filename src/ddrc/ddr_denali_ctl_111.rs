#[doc = "Register `DDR_DENALI_CTL_111` reader"]
pub type R = crate::R<DdrDenaliCtl111Spec>;
#[doc = "Register `DDR_DENALI_CTL_111` writer"]
pub type W = crate::W<DdrDenaliCtl111Spec>;
#[doc = "Field `TDFI_INIT_COMPLETE_F2` reader - Defines the DFI tINIT_COMPLETE timing parameter (in DFI clocks) for frequency copy 2, the maximum cycles between a dfi_init_start de- assertion and a dfi_init_complete assertion from the PHY."]
pub type TdfiInitCompleteF2R = crate::FieldReader<u16>;
#[doc = "Field `TDFI_INIT_COMPLETE_F2` writer - Defines the DFI tINIT_COMPLETE timing parameter (in DFI clocks) for frequency copy 2, the maximum cycles between a dfi_init_start de- assertion and a dfi_init_complete assertion from the PHY."]
pub type TdfiInitCompleteF2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `CURRENT_REG_COPY` reader - Indicates the current copy of timing parameters that is in use by the controller. READ-ONLY"]
pub type CurrentRegCopyR = crate::FieldReader;
#[doc = "Field `DFS_PHY_REG_WRITE_EN` reader - Enable a register write to the PHY during a frequency change. Set to 1 to enable."]
pub type DfsPhyRegWriteEnR = crate::BitReader;
#[doc = "Field `DFS_PHY_REG_WRITE_EN` writer - Enable a register write to the PHY during a frequency change. Set to 1 to enable."]
pub type DfsPhyRegWriteEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - Defines the DFI tINIT_COMPLETE timing parameter (in DFI clocks) for frequency copy 2, the maximum cycles between a dfi_init_start de- assertion and a dfi_init_complete assertion from the PHY."]
    #[inline(always)]
    pub fn tdfi_init_complete_f2(&self) -> TdfiInitCompleteF2R {
        TdfiInitCompleteF2R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:17 - Indicates the current copy of timing parameters that is in use by the controller. READ-ONLY"]
    #[inline(always)]
    pub fn current_reg_copy(&self) -> CurrentRegCopyR {
        CurrentRegCopyR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 24 - Enable a register write to the PHY during a frequency change. Set to 1 to enable."]
    #[inline(always)]
    pub fn dfs_phy_reg_write_en(&self) -> DfsPhyRegWriteEnR {
        DfsPhyRegWriteEnR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Defines the DFI tINIT_COMPLETE timing parameter (in DFI clocks) for frequency copy 2, the maximum cycles between a dfi_init_start de- assertion and a dfi_init_complete assertion from the PHY."]
    #[inline(always)]
    #[must_use]
    pub fn tdfi_init_complete_f2(&mut self) -> TdfiInitCompleteF2W<DdrDenaliCtl111Spec> {
        TdfiInitCompleteF2W::new(self, 0)
    }
    #[doc = "Bit 24 - Enable a register write to the PHY during a frequency change. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn dfs_phy_reg_write_en(&mut self) -> DfsPhyRegWriteEnW<DdrDenaliCtl111Spec> {
        DfsPhyRegWriteEnW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_ctl_111::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_ctl_111::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliCtl111Spec;
impl crate::RegisterSpec for DdrDenaliCtl111Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_ctl_111::R`](R) reader structure"]
impl crate::Readable for DdrDenaliCtl111Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_ctl_111::W`](W) writer structure"]
impl crate::Writable for DdrDenaliCtl111Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_CTL_111 to value 0"]
impl crate::Resettable for DdrDenaliCtl111Spec {
    const RESET_VALUE: u32 = 0;
}
