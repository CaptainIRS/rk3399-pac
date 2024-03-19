#[doc = "Register `DDR_DENALI_PHY_670` reader"]
pub type R = crate::R<DdrDenaliPhy670Spec>;
#[doc = "Register `DDR_DENALI_PHY_670` writer"]
pub type W = crate::W<DdrDenaliPhy670Spec>;
#[doc = "Field `PHY_ADR_LP4_BOOT_SLV_DELAY_1` reader - Address slave delay setting during the LPDDR4 boot frequency operation for address slice 1."]
pub type PhyAdrLp4BootSlvDelay1R = crate::FieldReader<u16>;
#[doc = "Field `PHY_ADR_LP4_BOOT_SLV_DELAY_1` writer - Address slave delay setting during the LPDDR4 boot frequency operation for address slice 1."]
pub type PhyAdrLp4BootSlvDelay1W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `PHY_ADR_BIT_MASK_1` reader - Mask bit for address slice 1. Set to 1 to indicate that the bit is masked (not used)."]
pub type PhyAdrBitMask1R = crate::FieldReader;
#[doc = "Field `PHY_ADR_BIT_MASK_1` writer - Mask bit for address slice 1. Set to 1 to indicate that the bit is masked (not used)."]
pub type PhyAdrBitMask1W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PHY_ADR_SEG_MASK_1` reader - Segment mask bit for address slice 1. Set to 1 to indicate that the bit is either CA 4 or CA 9."]
pub type PhyAdrSegMask1R = crate::FieldReader;
#[doc = "Field `PHY_ADR_SEG_MASK_1` writer - Segment mask bit for address slice 1. Set to 1 to indicate that the bit is either CA 4 or CA 9."]
pub type PhyAdrSegMask1W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:9 - Address slave delay setting during the LPDDR4 boot frequency operation for address slice 1."]
    #[inline(always)]
    pub fn phy_adr_lp4_boot_slv_delay_1(&self) -> PhyAdrLp4BootSlvDelay1R {
        PhyAdrLp4BootSlvDelay1R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:21 - Mask bit for address slice 1. Set to 1 to indicate that the bit is masked (not used)."]
    #[inline(always)]
    pub fn phy_adr_bit_mask_1(&self) -> PhyAdrBitMask1R {
        PhyAdrBitMask1R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - Segment mask bit for address slice 1. Set to 1 to indicate that the bit is either CA 4 or CA 9."]
    #[inline(always)]
    pub fn phy_adr_seg_mask_1(&self) -> PhyAdrSegMask1R {
        PhyAdrSegMask1R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9 - Address slave delay setting during the LPDDR4 boot frequency operation for address slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_lp4_boot_slv_delay_1(&mut self) -> PhyAdrLp4BootSlvDelay1W<DdrDenaliPhy670Spec> {
        PhyAdrLp4BootSlvDelay1W::new(self, 0)
    }
    #[doc = "Bits 16:21 - Mask bit for address slice 1. Set to 1 to indicate that the bit is masked (not used)."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_bit_mask_1(&mut self) -> PhyAdrBitMask1W<DdrDenaliPhy670Spec> {
        PhyAdrBitMask1W::new(self, 16)
    }
    #[doc = "Bits 24:29 - Segment mask bit for address slice 1. Set to 1 to indicate that the bit is either CA 4 or CA 9."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_seg_mask_1(&mut self) -> PhyAdrSegMask1W<DdrDenaliPhy670Spec> {
        PhyAdrSegMask1W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_670::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_670::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy670Spec;
impl crate::RegisterSpec for DdrDenaliPhy670Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_670::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy670Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_670::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy670Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_670 to value 0"]
impl crate::Resettable for DdrDenaliPhy670Spec {
    const RESET_VALUE: u32 = 0;
}
