#[doc = "Register `DDR_DENALI_CTL_112` reader"]
pub type R = crate::R<DdrDenaliCtl112Spec>;
#[doc = "Register `DDR_DENALI_CTL_112` writer"]
pub type W = crate::W<DdrDenaliCtl112Spec>;
#[doc = "Field `DFS_PHY_REG_WRITE_ADDR` reader - Register address which will be written during a frequency change. Must be a PHY register address."]
pub type DfsPhyRegWriteAddrR = crate::FieldReader<u32>;
#[doc = "Field `DFS_PHY_REG_WRITE_ADDR` writer - Register address which will be written during a frequency change. Must be a PHY register address."]
pub type DfsPhyRegWriteAddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Register address which will be written during a frequency change. Must be a PHY register address."]
    #[inline(always)]
    pub fn dfs_phy_reg_write_addr(&self) -> DfsPhyRegWriteAddrR {
        DfsPhyRegWriteAddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Register address which will be written during a frequency change. Must be a PHY register address."]
    #[inline(always)]
    #[must_use]
    pub fn dfs_phy_reg_write_addr(&mut self) -> DfsPhyRegWriteAddrW<DdrDenaliCtl112Spec> {
        DfsPhyRegWriteAddrW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_ctl_112::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_ctl_112::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliCtl112Spec;
impl crate::RegisterSpec for DdrDenaliCtl112Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_ctl_112::R`](R) reader structure"]
impl crate::Readable for DdrDenaliCtl112Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_ctl_112::W`](W) writer structure"]
impl crate::Writable for DdrDenaliCtl112Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_CTL_112 to value 0"]
impl crate::Resettable for DdrDenaliCtl112Spec {
    const RESET_VALUE: u32 = 0;
}
