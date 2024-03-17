#[doc = "Register `DENALI_PHY_287` reader"]
pub type R = crate::R<DenaliPhy287Spec>;
#[doc = "Register `DENALI_PHY_287` writer"]
pub type W = crate::W<DenaliPhy287Spec>;
#[doc = "Field `PHY_USER_PATT3_2` reader - User-defined pattern to be used during write data leveling for slice 2. This register holds the bytes 15 to 12 written/read from device."]
pub type PhyUserPatt3_2R = crate::FieldReader<u32>;
#[doc = "Field `PHY_USER_PATT3_2` writer - User-defined pattern to be used during write data leveling for slice 2. This register holds the bytes 15 to 12 written/read from device."]
pub type PhyUserPatt3_2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - User-defined pattern to be used during write data leveling for slice 2. This register holds the bytes 15 to 12 written/read from device."]
    #[inline(always)]
    pub fn phy_user_patt3_2(&self) -> PhyUserPatt3_2R {
        PhyUserPatt3_2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - User-defined pattern to be used during write data leveling for slice 2. This register holds the bytes 15 to 12 written/read from device."]
    #[inline(always)]
    #[must_use]
    pub fn phy_user_patt3_2(&mut self) -> PhyUserPatt3_2W<DenaliPhy287Spec> {
        PhyUserPatt3_2W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_287::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_287::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy287Spec;
impl crate::RegisterSpec for DenaliPhy287Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_287::R`](R) reader structure"]
impl crate::Readable for DenaliPhy287Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_287::W`](W) writer structure"]
impl crate::Writable for DenaliPhy287Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_287 to value 0"]
impl crate::Resettable for DenaliPhy287Spec {
    const RESET_VALUE: u32 = 0;
}
