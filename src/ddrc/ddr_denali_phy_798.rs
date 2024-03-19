#[doc = "Register `DDR_DENALI_PHY_798` reader"]
pub type R = crate::R<DdrDenaliPhy798Spec>;
#[doc = "Register `DDR_DENALI_PHY_798` writer"]
pub type W = crate::W<DdrDenaliPhy798Spec>;
#[doc = "Field `PHY_ADR_LP4_BOOT_SLV_DELAY_2` reader - Address slave delay setting during the LPDDR4 boot frequency operation for address slice 2."]
pub type PhyAdrLp4BootSlvDelay2R = crate::FieldReader<u16>;
#[doc = "Field `PHY_ADR_LP4_BOOT_SLV_DELAY_2` writer - Address slave delay setting during the LPDDR4 boot frequency operation for address slice 2."]
pub type PhyAdrLp4BootSlvDelay2W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `PHY_ADR_BIT_MASK_2` reader - Mask bit for address slice 2. Set to 1 to indicate that the bit is masked (not used)."]
pub type PhyAdrBitMask2R = crate::FieldReader;
#[doc = "Field `PHY_ADR_BIT_MASK_2` writer - Mask bit for address slice 2. Set to 1 to indicate that the bit is masked (not used)."]
pub type PhyAdrBitMask2W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PHY_ADR_SEG_MASK_2` reader - Segment mask bit for address slice 2. Set to 1 to indicate that the bit is either CA 4 or CA 9."]
pub type PhyAdrSegMask2R = crate::FieldReader;
#[doc = "Field `PHY_ADR_SEG_MASK_2` writer - Segment mask bit for address slice 2. Set to 1 to indicate that the bit is either CA 4 or CA 9."]
pub type PhyAdrSegMask2W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:9 - Address slave delay setting during the LPDDR4 boot frequency operation for address slice 2."]
    #[inline(always)]
    pub fn phy_adr_lp4_boot_slv_delay_2(&self) -> PhyAdrLp4BootSlvDelay2R {
        PhyAdrLp4BootSlvDelay2R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:21 - Mask bit for address slice 2. Set to 1 to indicate that the bit is masked (not used)."]
    #[inline(always)]
    pub fn phy_adr_bit_mask_2(&self) -> PhyAdrBitMask2R {
        PhyAdrBitMask2R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - Segment mask bit for address slice 2. Set to 1 to indicate that the bit is either CA 4 or CA 9."]
    #[inline(always)]
    pub fn phy_adr_seg_mask_2(&self) -> PhyAdrSegMask2R {
        PhyAdrSegMask2R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9 - Address slave delay setting during the LPDDR4 boot frequency operation for address slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_lp4_boot_slv_delay_2(&mut self) -> PhyAdrLp4BootSlvDelay2W<DdrDenaliPhy798Spec> {
        PhyAdrLp4BootSlvDelay2W::new(self, 0)
    }
    #[doc = "Bits 16:21 - Mask bit for address slice 2. Set to 1 to indicate that the bit is masked (not used)."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_bit_mask_2(&mut self) -> PhyAdrBitMask2W<DdrDenaliPhy798Spec> {
        PhyAdrBitMask2W::new(self, 16)
    }
    #[doc = "Bits 24:29 - Segment mask bit for address slice 2. Set to 1 to indicate that the bit is either CA 4 or CA 9."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_seg_mask_2(&mut self) -> PhyAdrSegMask2W<DdrDenaliPhy798Spec> {
        PhyAdrSegMask2W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_798::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_798::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy798Spec;
impl crate::RegisterSpec for DdrDenaliPhy798Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_798::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy798Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_798::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy798Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_798 to value 0"]
impl crate::Resettable for DdrDenaliPhy798Spec {
    const RESET_VALUE: u32 = 0;
}
