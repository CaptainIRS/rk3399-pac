#[doc = "Register `WIN1_YRGB_MST` reader"]
pub type R = crate::R<Win1YrgbMstSpec>;
#[doc = "Register `WIN1_YRGB_MST` writer"]
pub type W = crate::W<Win1YrgbMstSpec>;
#[doc = "Field `WIN1_YRGB_MST` reader - win1 YRGB frame buffer memory start address"]
pub type Win1YrgbMstR = crate::FieldReader<u32>;
#[doc = "Field `WIN1_YRGB_MST` writer - win1 YRGB frame buffer memory start address"]
pub type Win1YrgbMstW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - win1 YRGB frame buffer memory start address"]
    #[inline(always)]
    pub fn win1_yrgb_mst(&self) -> Win1YrgbMstR {
        Win1YrgbMstR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - win1 YRGB frame buffer memory start address"]
    #[inline(always)]
    #[must_use]
    pub fn win1_yrgb_mst(&mut self) -> Win1YrgbMstW<Win1YrgbMstSpec> {
        Win1YrgbMstW::new(self, 0)
    }
}
#[doc = "Win1 YRGB memory start address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win1_yrgb_mst::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win1_yrgb_mst::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Win1YrgbMstSpec;
impl crate::RegisterSpec for Win1YrgbMstSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`win1_yrgb_mst::R`](R) reader structure"]
impl crate::Readable for Win1YrgbMstSpec {}
#[doc = "`write(|w| ..)` method takes [`win1_yrgb_mst::W`](W) writer structure"]
impl crate::Writable for Win1YrgbMstSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WIN1_YRGB_MST to value 0"]
impl crate::Resettable for Win1YrgbMstSpec {
    const RESET_VALUE: u32 = 0;
}
