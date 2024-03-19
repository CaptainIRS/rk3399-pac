#[doc = "Register `DDR_DENALI_PHY_955` reader"]
pub type R = crate::R<DdrDenaliPhy955Spec>;
#[doc = "Register `DDR_DENALI_PHY_955` writer"]
pub type W = crate::W<DdrDenaliPhy955Spec>;
#[doc = "Field `PHY_ADR_DISABLE` reader - Disable the unused adr slice to save power."]
pub type PhyAdrDisableR = crate::FieldReader;
#[doc = "Field `PHY_ADR_DISABLE` writer - Disable the unused adr slice to save power."]
pub type PhyAdrDisableW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PHY_ADRCTL_MSTR_DLY_ENC_SEL` reader - Select the adr_slicesadrctl_mstr_dly_enc.'"]
pub type PhyAdrctlMstrDlyEncSelR = crate::FieldReader;
#[doc = "Field `PHY_ADRCTL_MSTR_DLY_ENC_SEL` writer - Select the adr_slicesadrctl_mstr_dly_enc.'"]
pub type PhyAdrctlMstrDlyEncSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PHY_CS_DLY_UPT_PER_AC_SLICE` reader - cs grp slave delay line is updated per CS"]
pub type PhyCsDlyUptPerAcSliceR = crate::BitReader;
#[doc = "Field `PHY_CS_DLY_UPT_PER_AC_SLICE` writer - cs grp slave delay line is updated per CS"]
pub type PhyCsDlyUptPerAcSliceW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Disable the unused adr slice to save power."]
    #[inline(always)]
    pub fn phy_adr_disable(&self) -> PhyAdrDisableR {
        PhyAdrDisableR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:9 - Select the adr_slicesadrctl_mstr_dly_enc.'"]
    #[inline(always)]
    pub fn phy_adrctl_mstr_dly_enc_sel(&self) -> PhyAdrctlMstrDlyEncSelR {
        PhyAdrctlMstrDlyEncSelR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 16 - cs grp slave delay line is updated per CS"]
    #[inline(always)]
    pub fn phy_cs_dly_upt_per_ac_slice(&self) -> PhyCsDlyUptPerAcSliceR {
        PhyCsDlyUptPerAcSliceR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Disable the unused adr slice to save power."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_disable(&mut self) -> PhyAdrDisableW<DdrDenaliPhy955Spec> {
        PhyAdrDisableW::new(self, 0)
    }
    #[doc = "Bits 8:9 - Select the adr_slicesadrctl_mstr_dly_enc.'"]
    #[inline(always)]
    #[must_use]
    pub fn phy_adrctl_mstr_dly_enc_sel(&mut self) -> PhyAdrctlMstrDlyEncSelW<DdrDenaliPhy955Spec> {
        PhyAdrctlMstrDlyEncSelW::new(self, 8)
    }
    #[doc = "Bit 16 - cs grp slave delay line is updated per CS"]
    #[inline(always)]
    #[must_use]
    pub fn phy_cs_dly_upt_per_ac_slice(&mut self) -> PhyCsDlyUptPerAcSliceW<DdrDenaliPhy955Spec> {
        PhyCsDlyUptPerAcSliceW::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_955::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_955::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy955Spec;
impl crate::RegisterSpec for DdrDenaliPhy955Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_955::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy955Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_955::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy955Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_955 to value 0"]
impl crate::Resettable for DdrDenaliPhy955Spec {
    const RESET_VALUE: u32 = 0;
}
