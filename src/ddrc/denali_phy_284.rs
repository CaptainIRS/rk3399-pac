#[doc = "Register `DENALI_PHY_284` reader"]
pub type R = crate::R<DenaliPhy284Spec>;
#[doc = "Register `DENALI_PHY_284` writer"]
pub type W = crate::W<DenaliPhy284Spec>;
#[doc = "Field `PHY_USER_PATT0_2` reader - User-defined pattern to be used during write data leveling for slice 2. This register holds the bytes 3 to 0 written/read from device."]
pub type PhyUserPatt0_2R = crate::FieldReader<u32>;
#[doc = "Field `PHY_USER_PATT0_2` writer - User-defined pattern to be used during write data leveling for slice 2. This register holds the bytes 3 to 0 written/read from device."]
pub type PhyUserPatt0_2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - User-defined pattern to be used during write data leveling for slice 2. This register holds the bytes 3 to 0 written/read from device."]
    #[inline(always)]
    pub fn phy_user_patt0_2(&self) -> PhyUserPatt0_2R {
        PhyUserPatt0_2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - User-defined pattern to be used during write data leveling for slice 2. This register holds the bytes 3 to 0 written/read from device."]
    #[inline(always)]
    #[must_use]
    pub fn phy_user_patt0_2(&mut self) -> PhyUserPatt0_2W<DenaliPhy284Spec> {
        PhyUserPatt0_2W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_284::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_284::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy284Spec;
impl crate::RegisterSpec for DenaliPhy284Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_284::R`](R) reader structure"]
impl crate::Readable for DenaliPhy284Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_284::W`](W) writer structure"]
impl crate::Writable for DenaliPhy284Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_284 to value 0"]
impl crate::Resettable for DenaliPhy284Spec {
    const RESET_VALUE: u32 = 0;
}
