#[doc = "Register `DDR_DENALI_PHY_143` reader"]
pub type R = crate::R<DdrDenaliPhy143Spec>;
#[doc = "Register `DDR_DENALI_PHY_143` writer"]
pub type W = crate::W<DdrDenaliPhy143Spec>;
#[doc = "Field `PHY_DFI40_POLARITY_1` reader - Indicates the dfi_wrdata_cs_n and dfi_rddata_cs_n is low active or high active."]
pub type PhyDfi40Polarity1R = crate::BitReader;
#[doc = "Field `PHY_DFI40_POLARITY_1` writer - Indicates the dfi_wrdata_cs_n and dfi_rddata_cs_n is low active or high active."]
pub type PhyDfi40Polarity1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_LP4_PST_AMBLE_1` reader - Controls the read postamble extension for LPDDR4 for slice 1."]
pub type PhyLp4PstAmble1R = crate::FieldReader;
#[doc = "Field `PHY_LP4_PST_AMBLE_1` writer - Controls the read postamble extension for LPDDR4 for slice 1."]
pub type PhyLp4PstAmble1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - Indicates the dfi_wrdata_cs_n and dfi_rddata_cs_n is low active or high active."]
    #[inline(always)]
    pub fn phy_dfi40_polarity_1(&self) -> PhyDfi40Polarity1R {
        PhyDfi40Polarity1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:9 - Controls the read postamble extension for LPDDR4 for slice 1."]
    #[inline(always)]
    pub fn phy_lp4_pst_amble_1(&self) -> PhyLp4PstAmble1R {
        PhyLp4PstAmble1R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Indicates the dfi_wrdata_cs_n and dfi_rddata_cs_n is low active or high active."]
    #[inline(always)]
    #[must_use]
    pub fn phy_dfi40_polarity_1(&mut self) -> PhyDfi40Polarity1W<DdrDenaliPhy143Spec> {
        PhyDfi40Polarity1W::new(self, 0)
    }
    #[doc = "Bits 8:9 - Controls the read postamble extension for LPDDR4 for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_lp4_pst_amble_1(&mut self) -> PhyLp4PstAmble1W<DdrDenaliPhy143Spec> {
        PhyLp4PstAmble1W::new(self, 8)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_143::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_143::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy143Spec;
impl crate::RegisterSpec for DdrDenaliPhy143Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_143::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy143Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_143::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy143Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_143 to value 0"]
impl crate::Resettable for DdrDenaliPhy143Spec {
    const RESET_VALUE: u32 = 0;
}
