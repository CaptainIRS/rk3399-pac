#[doc = "Register `DENALI_PHY_799` reader"]
pub type R = crate::R<DenaliPhy799Spec>;
#[doc = "Register `DENALI_PHY_799` writer"]
pub type W = crate::W<DenaliPhy799Spec>;
#[doc = "Field `PHY_ADR_CALVL_TRAIN_MASK_2` reader - Mask bit for CA training participation for address slice 2. Set to 1 to indicate that the bit is participating in training."]
pub type PhyAdrCalvlTrainMask2R = crate::FieldReader;
#[doc = "Field `PHY_ADR_CALVL_TRAIN_MASK_2` writer - Mask bit for CA training participation for address slice 2. Set to 1 to indicate that the bit is participating in training."]
pub type PhyAdrCalvlTrainMask2W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Mask bit for CA training participation for address slice 2. Set to 1 to indicate that the bit is participating in training."]
    #[inline(always)]
    pub fn phy_adr_calvl_train_mask_2(&self) -> PhyAdrCalvlTrainMask2R {
        PhyAdrCalvlTrainMask2R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Mask bit for CA training participation for address slice 2. Set to 1 to indicate that the bit is participating in training."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_calvl_train_mask_2(&mut self) -> PhyAdrCalvlTrainMask2W<DenaliPhy799Spec> {
        PhyAdrCalvlTrainMask2W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_799::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_799::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy799Spec;
impl crate::RegisterSpec for DenaliPhy799Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_799::R`](R) reader structure"]
impl crate::Readable for DenaliPhy799Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_799::W`](W) writer structure"]
impl crate::Writable for DenaliPhy799Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_799 to value 0"]
impl crate::Resettable for DenaliPhy799Spec {
    const RESET_VALUE: u32 = 0;
}
