#[doc = "Register `DENALI_PHY_158` reader"]
pub type R = crate::R<DenaliPhy158Spec>;
#[doc = "Register `DENALI_PHY_158` writer"]
pub type W = crate::W<DenaliPhy158Spec>;
#[doc = "Field `PHY_USER_PATT2_1` reader - User-defined pattern to be used during write data leveling for slice 1. This register holds the bytes 11 to 8 written/read from device."]
pub type PhyUserPatt2_1R = crate::FieldReader<u32>;
#[doc = "Field `PHY_USER_PATT2_1` writer - User-defined pattern to be used during write data leveling for slice 1. This register holds the bytes 11 to 8 written/read from device."]
pub type PhyUserPatt2_1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - User-defined pattern to be used during write data leveling for slice 1. This register holds the bytes 11 to 8 written/read from device."]
    #[inline(always)]
    pub fn phy_user_patt2_1(&self) -> PhyUserPatt2_1R {
        PhyUserPatt2_1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - User-defined pattern to be used during write data leveling for slice 1. This register holds the bytes 11 to 8 written/read from device."]
    #[inline(always)]
    #[must_use]
    pub fn phy_user_patt2_1(&mut self) -> PhyUserPatt2_1W<DenaliPhy158Spec> {
        PhyUserPatt2_1W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_158::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_158::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy158Spec;
impl crate::RegisterSpec for DenaliPhy158Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_158::R`](R) reader structure"]
impl crate::Readable for DenaliPhy158Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_158::W`](W) writer structure"]
impl crate::Writable for DenaliPhy158Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_158 to value 0"]
impl crate::Resettable for DenaliPhy158Spec {
    const RESET_VALUE: u32 = 0;
}
