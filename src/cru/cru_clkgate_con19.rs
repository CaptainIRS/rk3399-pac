#[doc = "Register `CRU_CLKGATE_CON19` reader"]
pub type R = crate::R<CruClkgateCon19Spec>;
#[doc = "Register `CRU_CLKGATE_CON19` writer"]
pub type W = crate::W<CruClkgateCon19Spec>;
#[doc = "Field `ACLK_CENTER_MAIN_NOC_EN` reader - aclk_center_main_noc clock disable bit When HIGH, disable clock Suggest always on"]
pub type AclkCenterMainNocEnR = crate::BitReader;
#[doc = "Field `ACLK_CENTER_MAIN_NOC_EN` writer - aclk_center_main_noc clock disable bit When HIGH, disable clock Suggest always on"]
pub type AclkCenterMainNocEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACLK_CENTER_PERI_NOC_EN` reader - aclk_center_peri_noc clock disable bit When HIGH, disable clock Suggest always on"]
pub type AclkCenterPeriNocEnR = crate::BitReader;
#[doc = "Field `ACLK_CENTER_PERI_NOC_EN` writer - aclk_center_peri_noc clock disable bit When HIGH, disable clock Suggest always on"]
pub type AclkCenterPeriNocEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCLK_DDR_SGRF_EN` reader - pclk_ddr_sgrf clock disable bit When HIGH, disable clock Suggest always on"]
pub type PclkDdrSgrfEnR = crate::BitReader;
#[doc = "Field `PCLK_DDR_SGRF_EN` writer - pclk_ddr_sgrf clock disable bit When HIGH, disable clock Suggest always on"]
pub type PclkDdrSgrfEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_MASK` writer - write mask bits When every bit HIGH, enable the writing corresponding bit When every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - aclk_center_main_noc clock disable bit When HIGH, disable clock Suggest always on"]
    #[inline(always)]
    pub fn aclk_center_main_noc_en(&self) -> AclkCenterMainNocEnR {
        AclkCenterMainNocEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - aclk_center_peri_noc clock disable bit When HIGH, disable clock Suggest always on"]
    #[inline(always)]
    pub fn aclk_center_peri_noc_en(&self) -> AclkCenterPeriNocEnR {
        AclkCenterPeriNocEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - pclk_ddr_sgrf clock disable bit When HIGH, disable clock Suggest always on"]
    #[inline(always)]
    pub fn pclk_ddr_sgrf_en(&self) -> PclkDdrSgrfEnR {
        PclkDdrSgrfEnR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - aclk_center_main_noc clock disable bit When HIGH, disable clock Suggest always on"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_center_main_noc_en(&mut self) -> AclkCenterMainNocEnW<CruClkgateCon19Spec> {
        AclkCenterMainNocEnW::new(self, 0)
    }
    #[doc = "Bit 1 - aclk_center_peri_noc clock disable bit When HIGH, disable clock Suggest always on"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_center_peri_noc_en(&mut self) -> AclkCenterPeriNocEnW<CruClkgateCon19Spec> {
        AclkCenterPeriNocEnW::new(self, 1)
    }
    #[doc = "Bit 2 - pclk_ddr_sgrf clock disable bit When HIGH, disable clock Suggest always on"]
    #[inline(always)]
    #[must_use]
    pub fn pclk_ddr_sgrf_en(&mut self) -> PclkDdrSgrfEnW<CruClkgateCon19Spec> {
        PclkDdrSgrfEnW::new(self, 2)
    }
    #[doc = "Bits 16:31 - write mask bits When every bit HIGH, enable the writing corresponding bit When every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<CruClkgateCon19Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Internal clock gating register19\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clkgate_con19::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clkgate_con19::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CruClkgateCon19Spec;
impl crate::RegisterSpec for CruClkgateCon19Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cru_clkgate_con19::R`](R) reader structure"]
impl crate::Readable for CruClkgateCon19Spec {}
#[doc = "`write(|w| ..)` method takes [`cru_clkgate_con19::W`](W) writer structure"]
impl crate::Writable for CruClkgateCon19Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRU_CLKGATE_CON19 to value 0"]
impl crate::Resettable for CruClkgateCon19Spec {
    const RESET_VALUE: u32 = 0;
}
