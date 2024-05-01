#[doc = "Register `WB_YRGB_MST` reader"]
pub type R = crate::R<WbYrgbMstSpec>;
#[doc = "Register `WB_YRGB_MST` writer"]
pub type W = crate::W<WbYrgbMstSpec>;
#[doc = "Field `WB_YRGB_MST` reader - YRGB mst address"]
pub type WbYrgbMstR = crate::FieldReader<u32>;
#[doc = "Field `WB_YRGB_MST` writer - YRGB mst address"]
pub type WbYrgbMstW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - YRGB mst address"]
    #[inline(always)]
    pub fn wb_yrgb_mst(&self) -> WbYrgbMstR {
        WbYrgbMstR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - YRGB mst address"]
    #[inline(always)]
    #[must_use]
    pub fn wb_yrgb_mst(&mut self) -> WbYrgbMstW<WbYrgbMstSpec> {
        WbYrgbMstW::new(self, 0)
    }
}
#[doc = "write back yrgb mst\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wb_yrgb_mst::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wb_yrgb_mst::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WbYrgbMstSpec;
impl crate::RegisterSpec for WbYrgbMstSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wb_yrgb_mst::R`](R) reader structure"]
impl crate::Readable for WbYrgbMstSpec {}
#[doc = "`write(|w| ..)` method takes [`wb_yrgb_mst::W`](W) writer structure"]
impl crate::Writable for WbYrgbMstSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WB_YRGB_MST to value 0"]
impl crate::Resettable for WbYrgbMstSpec {
    const RESET_VALUE: u32 = 0;
}
