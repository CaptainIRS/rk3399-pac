#[doc = "Register `MC_PHYRSTZ` reader"]
pub type R = crate::R<McPhyrstzSpec>;
#[doc = "Register `MC_PHYRSTZ` writer"]
pub type W = crate::W<McPhyrstzSpec>;
#[doc = "Field `PHYRSTZ` reader - HDMI Source PHY active low reset control for PHY GEN1, active high reset control for PHY GEN2. Reset Value: (PHY_GEN2== 1) ? 1 : 0"]
pub type PhyrstzR = crate::BitReader;
#[doc = "Field `PHYRSTZ` writer - HDMI Source PHY active low reset control for PHY GEN1, active high reset control for PHY GEN2. Reset Value: (PHY_GEN2== 1) ? 1 : 0"]
pub type PhyrstzW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - HDMI Source PHY active low reset control for PHY GEN1, active high reset control for PHY GEN2. Reset Value: (PHY_GEN2== 1) ? 1 : 0"]
    #[inline(always)]
    pub fn phyrstz(&self) -> PhyrstzR {
        PhyrstzR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - HDMI Source PHY active low reset control for PHY GEN1, active high reset control for PHY GEN2. Reset Value: (PHY_GEN2== 1) ? 1 : 0"]
    #[inline(always)]
    #[must_use]
    pub fn phyrstz(&mut self) -> PhyrstzW<McPhyrstzSpec> {
        PhyrstzW::new(self, 0)
    }
}
#[doc = "HDMI Source PHY active low reset control for PHY GEN1, active high reset control for PHY GEN2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mc_phyrstz::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mc_phyrstz::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McPhyrstzSpec;
impl crate::RegisterSpec for McPhyrstzSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`mc_phyrstz::R`](R) reader structure"]
impl crate::Readable for McPhyrstzSpec {}
#[doc = "`write(|w| ..)` method takes [`mc_phyrstz::W`](W) writer structure"]
impl crate::Writable for McPhyrstzSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets MC_PHYRSTZ to value 0"]
impl crate::Resettable for McPhyrstzSpec {
    const RESET_VALUE: u8 = 0;
}
