#[doc = "Register `DDR_DENALI_PHY_399` reader"]
pub type R = crate::R<DdrDenaliPhy399Spec>;
#[doc = "Register `DDR_DENALI_PHY_399` writer"]
pub type W = crate::W<DdrDenaliPhy399Spec>;
#[doc = "Field `PHY_DFI40_POLARITY_3` reader - Indicates the dfi_wrdata_cs_n and dfi_rddata_cs_n is low active or high active."]
pub type PhyDfi40Polarity3R = crate::BitReader;
#[doc = "Field `PHY_DFI40_POLARITY_3` writer - Indicates the dfi_wrdata_cs_n and dfi_rddata_cs_n is low active or high active."]
pub type PhyDfi40Polarity3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_LP4_PST_AMBLE_3` reader - Controls the read postamble extension for LPDDR4 for slice 3."]
pub type PhyLp4PstAmble3R = crate::FieldReader;
#[doc = "Field `PHY_LP4_PST_AMBLE_3` writer - Controls the read postamble extension for LPDDR4 for slice 3."]
pub type PhyLp4PstAmble3W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - Indicates the dfi_wrdata_cs_n and dfi_rddata_cs_n is low active or high active."]
    #[inline(always)]
    pub fn phy_dfi40_polarity_3(&self) -> PhyDfi40Polarity3R {
        PhyDfi40Polarity3R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:9 - Controls the read postamble extension for LPDDR4 for slice 3."]
    #[inline(always)]
    pub fn phy_lp4_pst_amble_3(&self) -> PhyLp4PstAmble3R {
        PhyLp4PstAmble3R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Indicates the dfi_wrdata_cs_n and dfi_rddata_cs_n is low active or high active."]
    #[inline(always)]
    #[must_use]
    pub fn phy_dfi40_polarity_3(&mut self) -> PhyDfi40Polarity3W<DdrDenaliPhy399Spec> {
        PhyDfi40Polarity3W::new(self, 0)
    }
    #[doc = "Bits 8:9 - Controls the read postamble extension for LPDDR4 for slice 3."]
    #[inline(always)]
    #[must_use]
    pub fn phy_lp4_pst_amble_3(&mut self) -> PhyLp4PstAmble3W<DdrDenaliPhy399Spec> {
        PhyLp4PstAmble3W::new(self, 8)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_399::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_399::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy399Spec;
impl crate::RegisterSpec for DdrDenaliPhy399Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_399::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy399Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_399::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy399Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_399 to value 0"]
impl crate::Resettable for DdrDenaliPhy399Spec {
    const RESET_VALUE: u32 = 0;
}
