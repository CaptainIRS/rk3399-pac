#[doc = "Register `DDR_DENALI_CTL_115` reader"]
pub type R = crate::R<DdrDenaliCtl115Spec>;
#[doc = "Register `DDR_DENALI_CTL_115` writer"]
pub type W = crate::W<DdrDenaliCtl115Spec>;
#[doc = "Field `DFS_PHY_REG_WRITE_DATA_F2` reader - Register data which will be written during a frequency change for frequency copy 2."]
pub type DfsPhyRegWriteDataF2R = crate::FieldReader<u32>;
#[doc = "Field `DFS_PHY_REG_WRITE_DATA_F2` writer - Register data which will be written during a frequency change for frequency copy 2."]
pub type DfsPhyRegWriteDataF2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Register data which will be written during a frequency change for frequency copy 2."]
    #[inline(always)]
    pub fn dfs_phy_reg_write_data_f2(&self) -> DfsPhyRegWriteDataF2R {
        DfsPhyRegWriteDataF2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Register data which will be written during a frequency change for frequency copy 2."]
    #[inline(always)]
    #[must_use]
    pub fn dfs_phy_reg_write_data_f2(&mut self) -> DfsPhyRegWriteDataF2W<DdrDenaliCtl115Spec> {
        DfsPhyRegWriteDataF2W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_ctl_115::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_ctl_115::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliCtl115Spec;
impl crate::RegisterSpec for DdrDenaliCtl115Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_ctl_115::R`](R) reader structure"]
impl crate::Readable for DdrDenaliCtl115Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_ctl_115::W`](W) writer structure"]
impl crate::Writable for DdrDenaliCtl115Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_CTL_115 to value 0"]
impl crate::Resettable for DdrDenaliCtl115Spec {
    const RESET_VALUE: u32 = 0;
}
