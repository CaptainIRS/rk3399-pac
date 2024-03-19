#[doc = "Register `DDR_DENALI_CTL_116` reader"]
pub type R = crate::R<DdrDenaliCtl116Spec>;
#[doc = "Register `DDR_DENALI_CTL_116` writer"]
pub type W = crate::W<DdrDenaliCtl116Spec>;
#[doc = "Field `DFS_PHY_REG_WRITE_MASK` reader - Register mask which will be written during a frequency change."]
pub type DfsPhyRegWriteMaskR = crate::FieldReader;
#[doc = "Field `DFS_PHY_REG_WRITE_MASK` writer - Register mask which will be written during a frequency change."]
pub type DfsPhyRegWriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Register mask which will be written during a frequency change."]
    #[inline(always)]
    pub fn dfs_phy_reg_write_mask(&self) -> DfsPhyRegWriteMaskR {
        DfsPhyRegWriteMaskR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Register mask which will be written during a frequency change."]
    #[inline(always)]
    #[must_use]
    pub fn dfs_phy_reg_write_mask(&mut self) -> DfsPhyRegWriteMaskW<DdrDenaliCtl116Spec> {
        DfsPhyRegWriteMaskW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_ctl_116::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_ctl_116::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliCtl116Spec;
impl crate::RegisterSpec for DdrDenaliCtl116Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_ctl_116::R`](R) reader structure"]
impl crate::Readable for DdrDenaliCtl116Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_ctl_116::W`](W) writer structure"]
impl crate::Writable for DdrDenaliCtl116Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_CTL_116 to value 0"]
impl crate::Resettable for DdrDenaliCtl116Spec {
    const RESET_VALUE: u32 = 0;
}
