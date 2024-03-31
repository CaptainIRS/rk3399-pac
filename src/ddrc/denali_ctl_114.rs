#[doc = "Register `DENALI_CTL_114` reader"]
pub type R = crate::R<DenaliCtl114Spec>;
#[doc = "Register `DENALI_CTL_114` writer"]
pub type W = crate::W<DenaliCtl114Spec>;
#[doc = "Field `DFS_PHY_REG_WRITE_DATA_F1` reader - Register data which will be written during a frequency change for frequency copy 1."]
pub type DfsPhyRegWriteDataF1R = crate::FieldReader<u32>;
#[doc = "Field `DFS_PHY_REG_WRITE_DATA_F1` writer - Register data which will be written during a frequency change for frequency copy 1."]
pub type DfsPhyRegWriteDataF1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Register data which will be written during a frequency change for frequency copy 1."]
    #[inline(always)]
    pub fn dfs_phy_reg_write_data_f1(&self) -> DfsPhyRegWriteDataF1R {
        DfsPhyRegWriteDataF1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Register data which will be written during a frequency change for frequency copy 1."]
    #[inline(always)]
    #[must_use]
    pub fn dfs_phy_reg_write_data_f1(&mut self) -> DfsPhyRegWriteDataF1W<DenaliCtl114Spec> {
        DfsPhyRegWriteDataF1W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_114::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_114::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl114Spec;
impl crate::RegisterSpec for DenaliCtl114Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_114::R`](R) reader structure"]
impl crate::Readable for DenaliCtl114Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_114::W`](W) writer structure"]
impl crate::Writable for DenaliCtl114Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_114 to value 0"]
impl crate::Resettable for DenaliCtl114Spec {
    const RESET_VALUE: u32 = 0;
}
