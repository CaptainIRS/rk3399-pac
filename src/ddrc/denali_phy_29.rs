#[doc = "Register `DENALI_PHY_29` reader"]
pub type R = crate::R<DenaliPhy29Spec>;
#[doc = "Register `DENALI_PHY_29` writer"]
pub type W = crate::W<DenaliPhy29Spec>;
#[doc = "Field `PHY_USER_PATT1_0` reader - User-defined pattern to be used during write data leveling for slice 0. This register holds the bytes 7 to 4 written/read from device."]
pub type PhyUserPatt1_0R = crate::FieldReader<u32>;
#[doc = "Field `PHY_USER_PATT1_0` writer - User-defined pattern to be used during write data leveling for slice 0. This register holds the bytes 7 to 4 written/read from device."]
pub type PhyUserPatt1_0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - User-defined pattern to be used during write data leveling for slice 0. This register holds the bytes 7 to 4 written/read from device."]
    #[inline(always)]
    pub fn phy_user_patt1_0(&self) -> PhyUserPatt1_0R {
        PhyUserPatt1_0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - User-defined pattern to be used during write data leveling for slice 0. This register holds the bytes 7 to 4 written/read from device."]
    #[inline(always)]
    #[must_use]
    pub fn phy_user_patt1_0(&mut self) -> PhyUserPatt1_0W<DenaliPhy29Spec> {
        PhyUserPatt1_0W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_29::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_29::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy29Spec;
impl crate::RegisterSpec for DenaliPhy29Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_29::R`](R) reader structure"]
impl crate::Readable for DenaliPhy29Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_29::W`](W) writer structure"]
impl crate::Writable for DenaliPhy29Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_29 to value 0"]
impl crate::Resettable for DenaliPhy29Spec {
    const RESET_VALUE: u32 = 0;
}
