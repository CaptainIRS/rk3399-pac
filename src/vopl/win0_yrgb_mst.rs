#[doc = "Register `WIN0_YRGB_MST` reader"]
pub type R = crate::R<Win0YrgbMstSpec>;
#[doc = "Register `WIN0_YRGB_MST` writer"]
pub type W = crate::W<Win0YrgbMstSpec>;
#[doc = "Field `WIN0_YRGB_MST` reader - win0 YRGB frame buffer memory start address"]
pub type Win0YrgbMstR = crate::FieldReader<u32>;
#[doc = "Field `WIN0_YRGB_MST` writer - win0 YRGB frame buffer memory start address"]
pub type Win0YrgbMstW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - win0 YRGB frame buffer memory start address"]
    #[inline(always)]
    pub fn win0_yrgb_mst(&self) -> Win0YrgbMstR {
        Win0YrgbMstR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - win0 YRGB frame buffer memory start address"]
    #[inline(always)]
    #[must_use]
    pub fn win0_yrgb_mst(&mut self) -> Win0YrgbMstW<Win0YrgbMstSpec> {
        Win0YrgbMstW::new(self, 0)
    }
}
#[doc = "Win0 YRGB memory start address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win0_yrgb_mst::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win0_yrgb_mst::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Win0YrgbMstSpec;
impl crate::RegisterSpec for Win0YrgbMstSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`win0_yrgb_mst::R`](R) reader structure"]
impl crate::Readable for Win0YrgbMstSpec {}
#[doc = "`write(|w| ..)` method takes [`win0_yrgb_mst::W`](W) writer structure"]
impl crate::Writable for Win0YrgbMstSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WIN0_YRGB_MST to value 0"]
impl crate::Resettable for Win0YrgbMstSpec {
    const RESET_VALUE: u32 = 0;
}
