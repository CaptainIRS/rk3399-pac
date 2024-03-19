#[doc = "Register `MC_HEACPHY_RST` reader"]
pub type R = crate::R<McHeacphyRstSpec>;
#[doc = "Register `MC_HEACPHY_RST` writer"]
pub type W = crate::W<McHeacphyRstSpec>;
#[doc = "Field `HEACPHYRST` reader - HEAC PHY reset (active high)"]
pub type HeacphyrstR = crate::BitReader;
#[doc = "Field `HEACPHYRST` writer - HEAC PHY reset (active high)"]
pub type HeacphyrstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - HEAC PHY reset (active high)"]
    #[inline(always)]
    pub fn heacphyrst(&self) -> HeacphyrstR {
        HeacphyrstR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - HEAC PHY reset (active high)"]
    #[inline(always)]
    #[must_use]
    pub fn heacphyrst(&mut self) -> HeacphyrstW<McHeacphyRstSpec> {
        HeacphyrstW::new(self, 0)
    }
}
#[doc = "Main Controller HEAC PHY Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mc_heacphy_rst::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mc_heacphy_rst::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McHeacphyRstSpec;
impl crate::RegisterSpec for McHeacphyRstSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`mc_heacphy_rst::R`](R) reader structure"]
impl crate::Readable for McHeacphyRstSpec {}
#[doc = "`write(|w| ..)` method takes [`mc_heacphy_rst::W`](W) writer structure"]
impl crate::Writable for McHeacphyRstSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets MC_HEACPHY_RST to value 0x01"]
impl crate::Resettable for McHeacphyRstSpec {
    const RESET_VALUE: u8 = 0x01;
}
