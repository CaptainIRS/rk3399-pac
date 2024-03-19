#[doc = "Register `DDR_DENALI_PHY_542` reader"]
pub type R = crate::R<DdrDenaliPhy542Spec>;
#[doc = "Register `DDR_DENALI_PHY_542` writer"]
pub type W = crate::W<DdrDenaliPhy542Spec>;
#[doc = "Field `PHY_ADR_LP4_BOOT_SLV_DELAY_0` reader - Address slave delay setting during the LPDDR4 boot frequency operation for address slice 0."]
pub type PhyAdrLp4BootSlvDelay0R = crate::FieldReader<u16>;
#[doc = "Field `PHY_ADR_LP4_BOOT_SLV_DELAY_0` writer - Address slave delay setting during the LPDDR4 boot frequency operation for address slice 0."]
pub type PhyAdrLp4BootSlvDelay0W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `PHY_ADR_BIT_MASK_0` reader - Mask bit for address slice 0. Set to 1 to indicate that the bit is masked (not used)."]
pub type PhyAdrBitMask0R = crate::FieldReader;
#[doc = "Field `PHY_ADR_BIT_MASK_0` writer - Mask bit for address slice 0. Set to 1 to indicate that the bit is masked (not used)."]
pub type PhyAdrBitMask0W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PHY_ADR_SEG_MASK_0` reader - Segment mask bit for address slice 0. Set to 1 to indicate that the bit is either CA 4 or CA 9."]
pub type PhyAdrSegMask0R = crate::FieldReader;
#[doc = "Field `PHY_ADR_SEG_MASK_0` writer - Segment mask bit for address slice 0. Set to 1 to indicate that the bit is either CA 4 or CA 9."]
pub type PhyAdrSegMask0W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:9 - Address slave delay setting during the LPDDR4 boot frequency operation for address slice 0."]
    #[inline(always)]
    pub fn phy_adr_lp4_boot_slv_delay_0(&self) -> PhyAdrLp4BootSlvDelay0R {
        PhyAdrLp4BootSlvDelay0R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:21 - Mask bit for address slice 0. Set to 1 to indicate that the bit is masked (not used)."]
    #[inline(always)]
    pub fn phy_adr_bit_mask_0(&self) -> PhyAdrBitMask0R {
        PhyAdrBitMask0R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - Segment mask bit for address slice 0. Set to 1 to indicate that the bit is either CA 4 or CA 9."]
    #[inline(always)]
    pub fn phy_adr_seg_mask_0(&self) -> PhyAdrSegMask0R {
        PhyAdrSegMask0R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9 - Address slave delay setting during the LPDDR4 boot frequency operation for address slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_lp4_boot_slv_delay_0(&mut self) -> PhyAdrLp4BootSlvDelay0W<DdrDenaliPhy542Spec> {
        PhyAdrLp4BootSlvDelay0W::new(self, 0)
    }
    #[doc = "Bits 16:21 - Mask bit for address slice 0. Set to 1 to indicate that the bit is masked (not used)."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_bit_mask_0(&mut self) -> PhyAdrBitMask0W<DdrDenaliPhy542Spec> {
        PhyAdrBitMask0W::new(self, 16)
    }
    #[doc = "Bits 24:29 - Segment mask bit for address slice 0. Set to 1 to indicate that the bit is either CA 4 or CA 9."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_seg_mask_0(&mut self) -> PhyAdrSegMask0W<DdrDenaliPhy542Spec> {
        PhyAdrSegMask0W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_542::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_542::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy542Spec;
impl crate::RegisterSpec for DdrDenaliPhy542Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_542::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy542Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_542::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy542Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_542 to value 0"]
impl crate::Resettable for DdrDenaliPhy542Spec {
    const RESET_VALUE: u32 = 0;
}
